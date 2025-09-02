use super::{
    TryFromNode,
    file_header::FileHeader,
    helpers::Helpers,
    node::collect_namespaces_on_node,
    soap::{binding::SoapBinding, message::SoapMessage, port::SoapPort, service::SoapService},
};
use crate::{
    error::{WriterError, WriterResult},
    model::{Namespace, field::resolve_type, node::RustNode},
    reader::{WELL_KNOWN_NAMESPACES, WriteXml},
};
use roxmltree::{Document, Node};
use std::{collections::{HashMap, HashSet}, rc::Rc};

pub struct RustDocument {
    pub(crate) namespace_lookup: HashMap<String, Rc<Namespace>>,
    pub(crate) namespaces: Vec<Rc<Namespace>>,
    pub(crate) target_namespaces: Vec<Rc<Namespace>>,
    pub(crate) current_target_namespace: Option<Rc<Namespace>>,
    pub(crate) nodes: Vec<Rc<RustNode>>,
    pub(crate) soap_messages: Vec<Rc<SoapMessage>>,
    pub(crate) soap_ports: Vec<Rc<SoapPort>>,
    pub(crate) soap_bindings: Vec<Rc<SoapBinding>>,
    pub(crate) soap_services: Vec<SoapService>,
}

impl RustDocument {
    pub fn init(doc: &Document) -> Self {
        let mut me = Self::empty();
        // parse namespaces on the root element
        collect_namespaces_on_node(doc.root_element(), &mut me);
        me
    }

    pub fn extend(&mut self, other: RustDocument) {
        self.namespace_lookup.extend(other.namespace_lookup);

        extend_no_duplicates(&mut self.namespaces, other.namespaces);
        extend_no_duplicates(&mut self.target_namespaces, other.target_namespaces);

        self.nodes.extend(other.nodes);
        self.soap_messages.extend(other.soap_messages);
        self.soap_ports.extend(other.soap_ports);
        self.soap_bindings.extend(other.soap_bindings);
        self.soap_services.extend(other.soap_services);
    }

    pub fn empty() -> Self {
        Self {
            namespace_lookup: HashMap::new(),
            namespaces: Vec::new(),
            target_namespaces: Vec::new(),
            current_target_namespace: None,
            nodes: Vec::new(),
            soap_messages: Vec::new(),
            soap_ports: Vec::new(),
            soap_bindings: Vec::new(),
            soap_services: Vec::new(),
        }
    }

    pub fn add_namespace_reference(&mut self, original_abbreviation: &str, url: &str) {
        if original_abbreviation.is_empty() || url.is_empty() {
            return;
        }

        if WELL_KNOWN_NAMESPACES.contains(&url) {
            return;
        }

        // check if the abbreviation is already in use
        if self.namespace_lookup.contains_key(original_abbreviation) {
            return;
        }

        if let Some(existing) = self.namespaces.iter().find(|ns| ns.namespace == url) {
            self.namespace_lookup
                .insert(original_abbreviation.to_string(), existing.clone());
            return;
        }

        let abbreviation = make_abbreviated_namespace(url, &self.namespaces);

        let rust_mod_name = create_mod_name_for_namespace(&abbreviation);
        let ns = Rc::new(Namespace {
            abbreviation,
            namespace: url.to_string(),
            rust_mod_name,
        });

        self.namespace_lookup
            .insert(original_abbreviation.to_string(), ns.clone());

        self.namespaces.push(ns);
    }

    pub fn find_module_name_from_namespace_reference(&self, abbreviation: &str) -> Option<&str> {
        self.find_namespace_by_abbreviation(abbreviation)
            .map(|ns| ns.rust_mod_name.as_str())
    }

    pub fn find_namespace_by_abbreviation(&self, abbreviation: &str) -> Option<&Rc<Namespace>> {
        self.namespace_lookup.get(abbreviation)
    }

    pub fn find_namespace(&self, url: &str) -> Option<&Rc<Namespace>> {
        self.namespaces.iter().find(|ns| ns.namespace == url)
    }

    pub fn switch_to_target_namespace(&mut self, namespace: &str) {
        // check if the namespace is already in the list
        if !self.target_namespaces.iter().any(|ns| ns.namespace == namespace) {
            // Check if we already have a reference to this namespace. If so, use that one, otherwise create a new one.
            let tns = self
                .namespaces
                .iter()
                .find(|ns| ns.namespace == namespace)
                .cloned()
                .unwrap_or_else(|| {
                    let abbreviation = make_abbreviated_namespace(namespace, &self.target_namespaces);
                    let rust_mod_name = create_mod_name_for_namespace(&abbreviation);

                    Rc::new(Namespace {
                        abbreviation,
                        namespace: namespace.to_string(),
                        rust_mod_name,
                    })
                });

            self.target_namespaces.push(tns.clone());
            self.namespaces.push(tns.clone());
            self.current_target_namespace = Some(tns);
        }
    }

    pub fn find_node_by_xml_name<'n>(
        &mut self,
        start_node: &Node<'n, 'n>,
        xml_name: &str,
        namespace: Option<&Namespace>,
    ) -> Option<Rc<RustNode>> {
        let rust_node = self.nodes.iter().find(|node| {
            node.rust_type.xml_name().is_some_and(|n| n == xml_name) && node.in_namespace.as_deref() == namespace
        });

        if let Some(rust_node) = rust_node {
            return Some(rust_node.clone());
        }

        let alt_node = try_to_find_node_by_xml_name_in_xml_doc(start_node, xml_name, namespace, self).ok()?;
        Some(alt_node.into())
    }

    pub fn find_message_by_xml_name(&self, xml_name: &str, _namespace: Option<&Namespace>) -> Option<&Rc<SoapMessage>> {
        self.soap_messages.iter().find(|msg| msg.xml_name == xml_name)
    }

    pub fn find_port_by_xml_name(&self, xml_name: &str, _namespace: Option<&Namespace>) -> Option<&Rc<SoapPort>> {
        self.soap_ports.iter().find(|port| port.xml_name == xml_name)
    }

    pub fn find_binding_by_xml_name(&self, xml_name: &str, _namespace: Option<&Namespace>) -> Option<&Rc<SoapBinding>> {
        self.soap_bindings.iter().find(|port| port.name == xml_name)
    }
}

fn extend_no_duplicates<T: PartialEq>(me: &mut Vec<T>, other: Vec<T>) {
    for item in other {
        if !me.contains(&item) {
            me.push(item);
        }
    }
}

fn create_mod_name_for_namespace(abbreviation: &str) -> String {
    format!("mod_{abbreviation}")
}

fn try_to_find_node_by_xml_name_in_xml_doc<'n>(
    start_node: &'n Node<'n, 'n>,
    xml_name: &str,
    _namespace: Option<&Namespace>,
    doc: &mut RustDocument,
) -> WriterResult<RustNode> {
    // get to the root of the document from the start node
    let mut start_node = *start_node;
    while let Some(parent) = start_node.parent() {
        start_node = parent;
    }

    // iterate over all subsequent nodes in the XML tree to find the node with the given name
    for node in start_node.descendants() {
        if node.is_element() {
            // do a quick check on the name of the node, so we can skip the more expensive try_from_node
            if let Some(node_name) = node.attribute("name") {
                let (node_name, _node_namespace) = resolve_type(node_name, doc);
                if node_name != xml_name {
                    continue;
                }

                let rust_node = RustNode::try_from_node(node, doc)?;
                return Ok(rust_node);
            }
        }
    }
    Err(WriterError::NodeNotFound(xml_name.to_string()))
}

impl<W> WriteXml<W> for RustDocument
where
    W: std::io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        FileHeader.write_xml(writer)?;

        for namespace in &self.target_namespaces {
            let module = namespace.rust_mod_name.as_str();
            writeln!(writer, "pub mod {module} {{")?;
            writeln!(writer, "    use super::*;")?;
            writeln!(writer, "    use restrictions::CheckRestrictions;")?;
            for node in self
                .nodes
                .iter()
                .filter(|n| n.in_namespace.as_deref() == Some(namespace))
            {
                node.write_xml(writer)?;
            }
            writeln!(writer, "}}")?;
        }

        // finally write the types that do not have a namespace
        for node in self.nodes.iter().filter(|n| n.in_namespace.is_none()) {
            node.write_xml(writer)?;
        }

        // write the soap bindings
        let used_bindings = self.soap_services.iter().map(|s| &s.binding.name).collect::<HashSet<_>>();
        for binding in &self.soap_bindings {
            if used_bindings.contains(&&binding.name) {
                binding.write_xml(writer)?;
            }
        }

        // write the soap services
        for service in &self.soap_services {
            service.write_xml(writer)?;
        }

        Helpers.write_xml(writer)?;

        Ok(())
    }
}

fn make_abbreviated_namespace(namespace: &str, existing_namespaces: &[Rc<Namespace>]) -> String {
    fn take_three_chars_max(namespace: &str) -> String {
        namespace.chars().filter(|c| c != &'.').take(3).collect()
    }

    let mut append: Option<u8> = None;

    let abbreviation = if let Some(last_segment) = namespace.split('/').next_back() {
        if let Some(slashed) = last_segment.split('-').next_back() {
            take_three_chars_max(slashed)
        } else {
            take_three_chars_max(last_segment)
        }
    } else {
        take_three_chars_max(namespace)
    };

    let abbreviation = abbreviation.to_lowercase();

    loop {
        let use_abbreviation = if let Some(append) = append {
            format!("{abbreviation}{append}")
        } else {
            abbreviation.clone()
        };

        if !existing_namespaces.iter().any(|ns| ns.abbreviation == use_abbreviation) {
            return use_abbreviation;
        }

        append = match append {
            None => Some(1),
            Some(n) => Some(n + 1),
        };

        assert_ne!(append, Some(255), "Too many namespaces with the same abbreviation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_abbreviated_namespace() {
        let abbr = make_abbreviated_namespace("https://example.com/a", &[]);
        assert_eq!(abbr, "a");

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema", &[]);
        assert_eq!(abbr, "xml");

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema-instance", &[]);
        assert_eq!(abbr, "ins");

        let existing_namespaces = vec![Rc::new(Namespace {
            namespace: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            abbreviation: abbr,
            rust_mod_name: "mod_ins".to_string(),
        })];

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema-instance", &existing_namespaces);
        assert_eq!(abbr, "ins1");
    }

    #[test]
    fn can_add_namespace_references_to_doc() {
        let mut doc = RustDocument::empty();
        doc.add_namespace_reference("t", "http://schemas.microsoft.com/exchange/services/2006/types");
        doc.add_namespace_reference("xs", "http://www.w3.org/2001/XMLSchema");
        // skip non-namespace references
        doc.add_namespace_reference("", "http://schemas.xmlsoap.org/wsdl/");
        // should have 2 references
        assert_eq!(doc.namespaces.len(), 1);
        let ns = doc
            .namespaces
            .iter()
            .find(|ns| ns.namespace == "http://schemas.microsoft.com/exchange/services/2006/types")
            .unwrap();
        assert_eq!(
            ns.namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(ns.abbreviation, "typ");

        // can lookup the namespace by abbreviation
        let ns = doc.find_namespace_by_abbreviation("t").unwrap();
        assert_eq!(
            ns.namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(ns.abbreviation, "typ");
        assert_eq!(ns.rust_mod_name, "mod_typ");
    }
}
