use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use log::warn;
use roxmltree::Node;
use std::fs::File;
use std::io;
use std::io::{stdout, Cursor, Write};

pub struct NodeWriter {
    writer: Box<dyn std::io::Write>,
    level: usize,
    buffers: Vec<Cursor<Vec<u8>>>,
    base_path: String,
}

impl Default for NodeWriter {
    fn default() -> Self {
        NodeWriter {
            level: 0,
            writer: Box::new(stdout()),
            buffers: vec![],
            base_path: String::default(),
        }
    }
}

impl NodeWriter {
    pub fn new_file(dest_file_name: File) -> Self {
        NodeWriter {
            level: 0,
            writer: Box::new(dest_file_name),
            buffers: vec![],
            base_path: String::default(),
        }
    }

    pub fn process_file(&mut self, base_path: &str, file_name: &str) {
        self.base_path = base_path.to_string();
        self.print_header();
        self.process_file_in_path(file_name);
    }

    fn process_file_in_path(&mut self, file_name: &str) {
        let f_in = format!("{}/{}", self.base_path, file_name);
        let xml = std::fs::read_to_string(f_in).expect("can not read file");
        let doc = roxmltree::Document::parse(&xml).unwrap();
        doc.descendants().for_each(|n| self.print(&n));
    }

    fn write(&mut self, buf: String) {
        if self.level == 0 {
            // write to output
            self.writer
                .write_all(buf.as_bytes())
                .expect("can not write to output");
        } else if let Some(buffer) = self.buffers.get_mut(self.level - 1) {
            // store in buffer
            buffer
                .write_all(buf.as_bytes())
                .expect("can not write buffer");
        }
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;

        if level == 0 {
            self.flush_buffers();
        }
    }

    fn inc_level(&mut self) {
        self.set_level(self.level + 1);

        if self.buffers.len() < (self.level as usize) {
            let b = Cursor::new(Vec::new());
            self.buffers.push(b);
        }
    }

    fn dec_level(&mut self) {
        self.set_level(self.level - 1);
    }

    fn flush_buffers(&mut self) {
        while let Some(mut cursor) = self.buffers.pop() {
            cursor.set_position(0);
            if let Err(err) = io::copy(&mut cursor, &mut self.writer) {
                warn!("Failed to flush buffer: {:?}", err);
            }
        }
    }

    fn print_header(&mut self) {
        self.write("use serde::{{Serialize, Deserialize}};\n\n".to_string());
    }

    fn print(&mut self, node: &Node) {
        if !node.is_element() {
            return;
        }

        match node.tag_name().name() {
            "definitions" => self.print_definitions(node),
            "schema" => self.print_xsd(node),
            _ => {}
        }
    }

    fn print_definitions(&mut self, node: &Node) {
        node.children()
            .for_each(|child| match child.tag_name().name() {
                "types" => self.print_xsd(&child),
                _ => {}
            })
    }

    fn print_xsd(&mut self, node: &Node) {
        node.children()
            .for_each(|child| match child.tag_name().name() {
                "import" => self.import_file(&child),
                "element" => self.print_element(&child),
                "complexType" => {
                    if let Some(n) = self.get_some_attribute(&child, "name") {
                        self.print_complex_element(&child, n)
                    };
                }
                _ => {}
            })
    }

    fn import_file(&mut self, node: &Node) {
        let name = match self.get_some_attribute(node, "schemaLocation") {
            None => return,
            Some(n) => n,
        };

        self.process_file_in_path(name);
    }

    fn print_element(&mut self, node: &Node) {
        let name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let as_vec = self.get_some_attribute(node, "maxOccurs").is_some();
        let as_option = self.get_some_attribute(node, "nillable").is_some();

        let maybe_complex = node
            .children()
            .find(|child| child.tag_name().name() == "complexType");

        // fields
        if let Some(complex) = maybe_complex {
            self.print_complex_element(&complex, name)
        } else if let Some(element_name) = self.get_some_attribute(node, "name") {
            if let Some(type_name) = self.get_some_attribute(node, "type") {
                if self.level == 0 {
                    // top-level == type alias
                    self.write(format!(
                        "pub type {} = {};\n\n",
                        to_pascal_case(element_name),
                        self.fetch_type(type_name)
                    ));
                    return;
                }

                self.write(format!(
                    "\t#[serde(rename = \"{}\", default)]\n",
                    element_name,
                ));

                if as_vec || as_option {
                    self.write(format!(
                        "\tpub {}: {}<{}>,\n",
                        self.shield_reserved_names(&to_snake_case(element_name)),
                        if as_vec { "Vec" } else { "Option" },
                        self.fetch_type(type_name)
                    ));
                } else {
                    self.write(format!(
                        "\tpub {}: {},\n",
                        self.shield_reserved_names(&to_snake_case(element_name)),
                        self.fetch_type(type_name)
                    ));
                }
            }
        }
    }

    fn get_some_attribute<'a>(&self, node: &'a Node, attr_name: &str) -> Option<&'a str> {
        match node.attributes().iter().find(|a| a.name() == attr_name) {
            None => None,
            Some(a) => Some(a.value()),
        }
    }

    fn fetch_type(&self, node_type: &str) -> String {
        match self.split_type(node_type) {
            "string" | "base64Binary" => "String".to_string(),
            "decimal" => "f64".to_string(),
            "integer" | "int" | "long" => "u64".to_string(),
            "short" => "u8".to_string(),
            "boolean" => "bool".to_string(),
            "date" | "xs:time" => "SystemTime".to_string(),
            v => to_pascal_case(v),
        }
    }

    fn split_type<'a>(&self, node_type: &'a str) -> &'a str {
        match node_type.split(':').last() {
            None => "String",
            Some(v) => v,
        }
    }

    fn print_complex_element(&mut self, node: &Node, name: &str) {
        self.inc_level();
        self.write("#[derive(Debug, Default, Serialize, Deserialize)]\n".to_string());

        self.write(format!(
            "#[serde(rename = \"{}\", default)]\npub struct {} {{\n",
            name,
            to_pascal_case(name)
        ));

        let maybe_sequence = node
            .children()
            .find(|child| child.tag_name().name() == "sequence");

        let maybe_complex = node
            .children()
            .find(|child| child.tag_name().name() == "complexContent");

        if let Some(sequence) = maybe_sequence {
            self.print_sequence(&sequence);
        }

        if let Some(complex) = maybe_complex {
            self.print_complex_content(&complex);
        }

        self.write("}\n\n".to_string());
        self.dec_level();
    }

    fn print_sequence(&mut self, node: &Node) {
        node.children().for_each(|child| self.print_element(&child));
    }

    fn print_complex_content(&mut self, node: &Node) {
        if let Some(extension) = node
            .children()
            .find(|child| child.tag_name().name() == "extension")
        {
            self.write("\t#[serde(flatten)]\n".to_string());
            self.print_extension(&extension);

            let maybe_sequence = extension
                .children()
                .find(|ext_child| ext_child.tag_name().name() == "sequence");

            if let Some(sequence) = maybe_sequence {
                self.print_sequence(&sequence);
            }
        }

        self.print_sequence(node);
    }

    fn print_extension(&mut self, node: &Node) {
        let base = match self.get_some_attribute(node, "base") {
            None => return,
            Some(n) => n,
        };

        self.write(format!(
            "\tpub {}: {},\n",
            to_snake_case(&self.fetch_type(base)),
            self.fetch_type(base)
        ));
    }

    fn shield_reserved_names<'a>(&self, type_name: &'a str) -> &'a str {
        match type_name {
            "type" => "rs_type",
            other => other,
        }
    }
}
