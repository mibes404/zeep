use super::{WriteXml, node::collect_namespaces_on_node};
use crate::{
    error::WriterResult,
    model::{Namespace, node::RustNode},
};
use roxmltree::Document;
use std::{collections::HashMap, rc::Rc};

pub struct RustDocument {
    pub namespace_references: HashMap<String, Rc<Namespace>>,
    pub target_namespaces: Vec<Rc<Namespace>>,
    pub current_target_namespace: Option<Rc<Namespace>>,
    pub nodes: Vec<RustNode>,
}

impl RustDocument {
    pub fn init<'n>(doc: &Document) -> Self {
        let mut me = Self {
            namespace_references: HashMap::new(),
            target_namespaces: Vec::new(),
            current_target_namespace: None,
            nodes: Vec::new(),
        };

        // parse namespaces on the root element
        collect_namespaces_on_node(doc.root_element(), &mut me);
        me
    }

    pub fn empty() -> Self {
        Self {
            namespace_references: HashMap::new(),
            target_namespaces: Vec::new(),
            current_target_namespace: None,
            nodes: Vec::new(),
        }
    }

    pub fn add_namespace_reference(&mut self, original_abbreviation: &str, url: &str) {
        if original_abbreviation.is_empty() {
            return;
        }

        // check if the namespace is already in the list
        if !self.namespace_references.contains_key(original_abbreviation) {
            let abbreviation = make_abbreviated_namespace(url, &self.target_namespaces);
            let rust_mod_name = create_mod_name_for_namespace(&abbreviation);
            let ns = Rc::new(Namespace {
                abbreviation,
                namespace: url.to_string(),
                rust_mod_name,
            });

            // extract the original abbreviation from the node_name
            self.namespace_references.insert(original_abbreviation.to_string(), ns);
        }
    }

    pub fn find_module_name_from_namespace_reference(&self, abbreviation: &str) -> Option<&str> {
        self.find_namespace(abbreviation).map(|ns| ns.rust_mod_name.as_str())
    }

    pub fn find_namespace(&self, abbreviation: &str) -> Option<&Rc<Namespace>> {
        self.namespace_references.get(abbreviation)
    }

    pub fn switch_to_target_namespace(&mut self, namespace: &str) {
        // check if the namespace is already in the list
        if !self.target_namespaces.iter().any(|ns| ns.namespace == namespace) {
            // Check if we already have a reference to this namespace. If so, use that one, otherwise create a new one.
            let tns = self
                .namespace_references
                .values()
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
            self.current_target_namespace = Some(tns);
        }
    }

    pub fn find_node_by_xml_name(&self, xml_name: &str, namespace: Option<&Namespace>) -> Option<&RustNode> {
        self.nodes.iter().find(|node| {
            node.rust_type.xml_name().is_some_and(|n| n == xml_name) && node.in_namespace.as_deref() == namespace
        })
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
        for namespace in &self.target_namespaces {
            let module = namespace.rust_mod_name.as_str();
            writeln!(writer, "pub mod {module} {{")?;
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

        Ok(())
    }
}

fn make_abbreviated_namespace(namespace: &str, existing_namespaces: &[Rc<Namespace>]) -> String {
    fn take_three_chars_max(namespace: &str) -> String {
        namespace.chars().take(3).collect()
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
        assert_eq!(doc.namespace_references.len(), 2);
        assert_eq!(
            doc.namespace_references.get("t").unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(
            doc.namespace_references.get("xs").unwrap().namespace,
            "http://www.w3.org/2001/XMLSchema"
        );
    }
}
