use log::{info, warn};
use roxmltree::Node;
use std::io::{stdout, Write};
use std::time::SystemTime;

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    // Find element by id.
    let xml = std::fs::read_to_string("resources/work.xml").expect("can not read file");
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let mut writer = NodeWriter::default();
    doc.descendants().for_each(|n| writer.print(&n));
}

struct NodeWriter {
    writer: Box<dyn std::io::Write>,
}

impl Default for NodeWriter {
    fn default() -> Self {
        NodeWriter {
            writer: Box::new(stdout()),
        }
    }
}

impl NodeWriter {
    fn print(&mut self, node: &Node) {
        if !node.is_element() {
            return;
        }

        match node.tag_name().name() {
            "definitions" => self.print_definitions(node),
            "schema" => self.print_xsd(node),
            _ => return,
        }
    }

    fn print_definitions(&mut self, node: &Node) {
        node.children()
            .for_each(|child| match child.tag_name().name() {
                "types" => self.print_xsd(&child),
                _ => return,
            })
    }

    fn print_xsd(&mut self, node: &Node) {
        node.children()
            .for_each(|child| match child.tag_name().name() {
                "element" => self.print_element(&child),
                _ => return,
            })
    }

    fn print_element(&mut self, node: &Node) {
        let maybe_complex = node
            .children()
            .find(|child| child.tag_name().name() == "complexType")
            .take();

        if let Some(complex) = maybe_complex {
            self.print_complex_element(&complex)
        } else {
            if let Some(node_type) = node.attributes().iter().find(|a| a.name() == "type").take() {
                self.writer
                    .write_fmt(format_args!("{}\n", self.fetch_type(node_type.value())));
            }
        }
    }

    fn fetch_type(&self, node_type: &str) -> String {
        match self.split_type(node_type) {
            "string" => "String".to_string(),
            "decimal" => "f64".to_string(),
            "integer" => "u64".to_string(),
            "boolean" => "bool".to_string(),
            "date" | "xs:time" => "SystemTime".to_string(),
            v => v.to_string(),
        }
    }

    fn split_type<'a>(&self, node_type: &'a str) -> &'a str {
        match node_type.split(":").last() {
            None => "String",
            Some(v) => v,
        }
    }

    fn print_complex_element(&mut self, node: &Node) {
        let maybe_sequence = node
            .children()
            .find(|child| child.tag_name().name() == "sequence")
            .take();

        if let Some(sequence) = maybe_sequence {
            sequence
                .children()
                .for_each(|child| self.print_element(&child));
        }
    }
}
