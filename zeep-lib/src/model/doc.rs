use super::WriteXml;
use crate::{
    error::WriterResult,
    model::{node::RustNode, Namespace},
};
use reqwest::dns::Name;
use std::{collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct RustDocument {
    pub namespace_references: HashMap<String, Rc<Namespace>>,
    pub target_namespaces: Vec<Rc<Namespace>>,
    pub current_target_namespace: Option<Rc<Namespace>>,
    pub nodes: Vec<RustNode>,
}

impl RustDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_namespace_reference(&mut self, node_name: &str, url: &str) {
        // check if the namespace is already in the list
        let Some(original_abbreviation) = node_name.split(':').last() else {
            // not a namespace reference
            return;
        };

        if !self.namespace_references.contains_key(original_abbreviation) {
            let ns = Rc::new(Namespace {
                abbreviation: make_abbreviated_namespace(url, &self.target_namespaces),
                namespace: url.to_string(),
            });

            // extract the original abbreviation from the node_name
            self.namespace_references.insert(original_abbreviation.to_string(), ns);
        }
    }

    pub fn find_module_name_from_namespace_reference(&self, abbreviation: &str) -> Option<String> {
        self.find_namespace(abbreviation)
            .map(|ns| create_mod_name_for_namespace(ns))
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
                .map(|ns| ns.clone())
                .unwrap_or_else(|| {
                    Rc::new(Namespace {
                        abbreviation: make_abbreviated_namespace(namespace, &self.target_namespaces),
                        namespace: namespace.to_string(),
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

impl<W> WriteXml<W> for RustDocument
where
    W: std::io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        for namespace in &self.target_namespaces {
            let module = create_mod_name_for_namespace(namespace);
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

pub fn create_mod_name_for_namespace(namespace: &Namespace) -> String {
    format!("ns_{}", namespace.abbreviation)
}

fn make_abbreviated_namespace(namespace: &str, existing_namespaces: &[Rc<Namespace>]) -> String {
    fn take_three_chars_max(namespace: &str) -> String {
        namespace.chars().take(3).collect()
    }

    let append: Option<u8> = None;

    loop {
        let mut abbreviation = if let Some(last_segment) = namespace.split('/').next_back() {
            if let Some(slashed) = last_segment.split('-').next_back() {
                take_three_chars_max(slashed)
            } else {
                take_three_chars_max(last_segment)
            }
        } else {
            take_three_chars_max(namespace)
        };

        if let Some(append) = append {
            abbreviation.push_str(&append.to_string());
        }

        let abbreviation = abbreviation.to_lowercase();

        if !existing_namespaces.iter().any(|ns| ns.abbreviation == abbreviation) {
            return abbreviation;
        }

        let append = match append {
            None => 1,
            Some(n) => n + 1,
        };

        assert_ne!(append, 255, "Too many namespaces with the same abbreviation");
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
        })];

        let abbr = make_abbreviated_namespace("http://www.w3.org/2001/XMLSchema-instance", &existing_namespaces);
        assert_eq!(abbr, "ins1");
    }

    #[test]
    fn can_add_namespace_references_to_doc() {
        let mut doc = RustDocument::default();
        doc.add_namespace_reference("xmlns:t", "http://schemas.microsoft.com/exchange/services/2006/types");
        doc.add_namespace_reference("xmlns:xs", "http://www.w3.org/2001/XMLSchema");
        // skip non-namespace references
        doc.add_namespace_reference("xmls", "http://schemas.xmlsoap.org/wsdl/");
        // should have 2 references
        assert_eq!(doc.namespace_references.len(), 2);
    }
}
