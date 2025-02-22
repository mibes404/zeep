use super::{
    file_header::FileHeader,
    helpers::Helpers,
    node::collect_namespaces_on_node,
    soap::{binding::SoapBinding, message::SoapMessage, port::SoapPort, service::SoapService},
};
use crate::{
    error::WriterResult,
    model::{Namespace, node::RustNode},
    reader::{WELL_KNOWN_NAMESPACES, WriteXml},
};
use roxmltree::Document;
use std::{collections::HashMap, rc::Rc};

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

    pub fn find_node_by_xml_name(&self, xml_name: &str, namespace: Option<&Namespace>) -> Option<&Rc<RustNode>> {
        self.nodes.iter().find(|node| {
            node.rust_type.xml_name().is_some_and(|n| n == xml_name) && node.in_namespace.as_deref() == namespace
        })
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

fn create_mod_name_for_namespace(abbreviation: &str) -> String {
    format!("mod_{abbreviation}")
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
        for binding in &self.soap_bindings {
            binding.write_xml(writer)?;
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
