use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use log::{info, warn};
use roxmltree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{stdout, Cursor, Read, Write};
use std::mem::discriminant;

const MESSAGES_MOD: &str = "messages";
const TYPES_MOD: &str = "types";
const PORTS_MOD: &str = "ports";
const BINDINGS_MOD: &str = "bindings";
const SOAP_ENV: &str = "soapenv";

pub struct FileWriter {
    base_path: String,
    current_section: Section,
    mod_writers: HashMap<Section, ModWriter>,
    level: usize,
    writer: Option<Box<dyn std::io::Write>>,
    target_name_space: Option<String>,
    port_types: HashMap<String, PortType>,
}

struct ModWriter {
    level: usize,
    section: Section,
    buffers: Vec<Cursor<Vec<u8>>>,
    delayed_buffer: Cursor<Vec<u8>>,
    final_stage: Cursor<Vec<u8>>,
    defined_types: Vec<String>,
}

#[derive(Clone)]
struct PortType {
    name: String,
    input_type: Option<(String, Option<String>)>,
    output_type: Option<(String, Option<String>)>,
    fault_type: Option<(String, Option<String>)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Section {
    Root,
    Types,
    Messages,
    PortTypes,
    Bindings,
}

impl Default for FileWriter {
    fn default() -> Self {
        FileWriter {
            base_path: String::default(),
            current_section: Section::Root,
            mod_writers: FileWriter::init_mod_writers(),
            level: 0,
            writer: Option::Some(Box::new(stdout())),
            target_name_space: Option::None,
            port_types: HashMap::new(),
        }
    }
}

impl FileWriter {
    pub fn new_file(dest_file_name: File) -> Self {
        FileWriter {
            base_path: String::default(),
            current_section: Section::Root,
            mod_writers: FileWriter::init_mod_writers(),
            level: 0,
            writer: Option::Some(Box::new(dest_file_name)),
            target_name_space: Option::None,
            port_types: HashMap::new(),
        }
    }

    fn init_mod_writers() -> HashMap<Section, ModWriter> {
        let mut mod_writers = HashMap::new();
        mod_writers.insert(Section::Root, ModWriter::new(Section::Root));
        mod_writers.insert(Section::Messages, ModWriter::new(Section::Messages));
        mod_writers.insert(Section::Types, ModWriter::new(Section::Types));
        mod_writers.insert(Section::PortTypes, ModWriter::new(Section::PortTypes));
        mod_writers.insert(Section::Bindings, ModWriter::new(Section::Bindings));
        mod_writers
    }

    pub fn process_file(&mut self, base_path: &str, file_name: &str) {
        self.base_path = base_path.to_string();
        self.print_header();
        self.print_common_structs();
        self.process_file_in_path(file_name, true);
    }

    fn process_file_in_path(&mut self, file_name: &str, print_when_done: bool) {
        let f_in = format!("{}/{}", self.base_path, file_name);
        let xml = std::fs::read_to_string(f_in).expect("can not read file");
        let doc = roxmltree::Document::parse(&xml).unwrap();
        doc.descendants().for_each(|n| self.print(&n));

        if !print_when_done {
            return;
        }

        // once all elements are processed, write them to output
        for (_section, mw) in self.mod_writers.iter_mut() {
            let reader_ref = mw.read_for_output();
            let mut reader = reader_ref.into_inner();

            if let Some(mut writer) = self.writer.take() {
                if let Err(err) = io::copy(&mut reader, &mut writer) {
                    warn!("Failed to flush final stage to output: {:?}", err);
                }

                // return writer for next loop
                self.writer = Option::Some(writer);
            }
        }
    }

    fn write(&mut self, buf: String) {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.write(buf, self.level)
        }
    }

    fn delayed_write(&mut self, buf: String) {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.delayed_write(buf)
        }
    }

    fn flush_delayed_buffer(&mut self) {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.flush_delayed_buffer()
        }
    }

    pub fn seen_type(&mut self, type_def: String) {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.seen_type(type_def);
        }
    }

    pub fn reset_defined_types(&mut self) {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.reset_defined_types();
        }
    }

    pub fn have_seen_type(&mut self, type_def: String) -> bool {
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.have_seen_type(type_def)
        } else {
            false
        }
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;
        if let Some(mw) = self.mod_writers.get_mut(&self.current_section) {
            mw.set_level(level);
        }
    }

    fn inc_level(&mut self) {
        self.set_level(self.level + 1);
    }

    fn dec_level(&mut self) {
        self.set_level(self.level - 1);
    }

    fn print_header(&mut self) {
        self.write(
            r#"use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            "#
            .to_string(),
        );
    }

    fn print_common_structs(&mut self) {
        self.write(
            r#"
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct Header {}
    "#
            .to_string(),
        );
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
            .filter(|child| child.tag_name().name() == "types")
            .for_each(|node| self.print_xsd(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "message")
            .for_each(|node| self.print_message(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "portType")
            .for_each(|node| self.print_port_type(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "binding")
            .for_each(|node| self.print_binding(&node));
    }

    fn print_xsd(&mut self, node: &Node) {
        self.check_section(Section::Types);

        self.target_name_space = self
            .get_some_attribute(node, "targetNamespace")
            .map(|s| s.to_string());

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

        self.process_file_in_path(name, false);
    }

    fn format_type(&mut self, id: &str, definition: String) -> String {
        if !self.have_seen_type(id.to_string()) {
            self.seen_type(id.to_string());
            definition
        } else {
            String::new()
        }
    }

    fn print_type(&mut self, id: &str, definition: String) {
        let out = self.format_type(id, definition);
        if !out.is_empty() {
            self.write(out);
        }
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
            .find(|child| child.has_tag_name("complexType"));

        // fields
        if let Some(complex) = maybe_complex {
            self.print_complex_element(&complex, name)
        } else if let Some(element_name) = self.get_some_attribute(node, "name") {
            if let Some(type_name) = self.get_some_attribute(node, "type") {
                if self.level == 0 {
                    // top-level == type alias
                    let top_level = to_pascal_case(element_name);
                    let alias = self.fetch_type(type_name);

                    if top_level != alias {
                        self.print_type(
                            &top_level,
                            format!("pub type {} = {};\n\n", top_level, alias),
                        );
                    }

                    return;
                }

                if let Some(_tns) = &self.target_name_space {
                    self.write(format!(
                        "\t#[yaserde(prefix = \"ns\", rename = \"{}\", default)]\n",
                        element_name,
                    ));
                } else {
                    self.write(format!(
                        "\t#[yaserde(rename = \"{}\", default)]\n",
                        element_name,
                    ));
                }

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

    fn get_some_attribute_as_string(&self, node: &Node, attr_name: &str) -> Option<String> {
        match node.attributes().iter().find(|a| a.name() == attr_name) {
            None => None,
            Some(a) => Some(a.value().to_string()),
        }
    }

    fn fetch_type(&self, node_type: &str) -> String {
        match self.split_type(node_type) {
            "string" | "base64Binary" => "String".to_string(),
            "decimal" => "f64".to_string(),
            "integer" | "int" | "long" => "u64".to_string(),
            "short" => "u8".to_string(),
            "boolean" => "bool".to_string(),
            "date" | "dateTime" | "xs:time" => "SystemTime".to_string(),
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
        self.write("#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n".to_string());

        if let Some(tns) = &self.target_name_space {
            self.write(format!(
                "#[yaserde(prefix = \"ns\", namespace = \"ns: {}\", rename = \"{}\", default)]\npub struct {} {{\n",
                tns,
                name,
                to_pascal_case(name)
            ));
        } else {
            self.write(format!(
                "#[yaserde(rename = \"{}\", default)]\npub struct {} {{\n",
                name,
                to_pascal_case(name)
            ));
        }

        let maybe_sequence = node.children().find(|child| child.has_tag_name("sequence"));

        let maybe_complex = node
            .children()
            .find(|child| child.has_tag_name("complexContent"));

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
            .find(|child| child.has_tag_name("extension"))
        {
            self.write("\t#[yaserde(flatten)]\n".to_string());
            self.print_extension(&extension);

            let maybe_sequence = extension
                .children()
                .find(|ext_child| ext_child.has_tag_name("sequence"));

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

    fn check_section(&mut self, target: Section) {
        if discriminant(&self.current_section) != discriminant(&target) {
            self.current_section = target;
        }
    }

    // WSDL Messages

    fn print_message(&mut self, node: &Node) {
        self.check_section(Section::Messages);

        if let Some(name) = self.get_some_attribute(node, "name") {
            self.write("#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n".to_string());

            self.write(format!(
                "#[yaserde(rename = \"{}\", default)]\npub struct {} {{\n",
                name,
                to_pascal_case(name)
            ));

            let maybe_part = node.children().find(|child| child.has_tag_name("part"));

            if let Some(part) = maybe_part {
                self.print_part(&part);
            }

            self.write("}\n\n".to_string());
        }
    }

    fn print_part(&mut self, node: &Node) {
        let element_name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        if let Some(type_name) = self.get_some_attribute(node, "element") {
            self.write("\t#[yaserde(flatten)]\n".to_string());

            self.write(format!(
                "\tpub {}: {}::{},\n",
                self.shield_reserved_names(&to_snake_case(element_name)),
                TYPES_MOD,
                self.fetch_type(type_name)
            ));
        }
    }

    // WSDL Port Types
    fn print_port_type(&mut self, node: &Node) {
        self.check_section(Section::PortTypes);
        let element_name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let struct_name = to_pascal_case(element_name);
        self.write(format!("#[async_trait]\npub trait {0} {{\n", struct_name));
        node.children()
            .for_each(|child| self.print_operation(element_name, &child));
        self.write("}\n\n".to_string());
        self.flush_delayed_buffer();
    }

    // WSDL bindings

    fn print_binding(&mut self, node: &Node) {
        self.check_section(Section::Bindings);
        let element_name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let type_name = match self.get_some_attribute(node, "type") {
            None => return,
            Some(n) => n,
        };

        let struct_name = to_pascal_case(element_name);
        let trait_name = self.fetch_type(type_name);

        self.write(format!(
            r#"pub struct {0} {{
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }}
                
                #[async_trait]
                impl {2}::{1} for {0} {{
                "#,
            struct_name, trait_name, PORTS_MOD,
        ));

        node.children()
            .for_each(|child| self.print_binding_operation(&trait_name, &child));

        self.write("}\n\n".to_string());
        self.print_default_constructor(struct_name.as_str());
        self.print_constructor(struct_name.as_str());
        self.flush_delayed_buffer();
    }

    fn print_default_constructor(&mut self, struct_name: &str) {
        let url = match &self.target_name_space {
            None => "String::new()".to_string(),
            Some(tns) => tns.to_string(),
        };

        self.write(format!(
            r#"impl Default for {0} {{
                fn default() -> Self {{
                    {0} {{
                        client: reqwest::Client::new(),
                        url: {1},
                        credentials: Option::None,
                     }}
                }}
            }}
            "#,
            struct_name, url
        ));
    }

    fn print_constructor(&mut self, struct_name: &str) {
        self.write(format!(
            r#"impl {0} {{
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {{
                    {0} {{
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }}
                }}
        }}
        "#,
            struct_name
        ));
    }

    fn map_name_message(&self, node: &Node) -> (String, Option<String>) {
        let msg = self
            .get_some_attribute_as_string(node, "message")
            .map(|m| self.fetch_type(&m));

        let name = self.get_some_attribute_as_string(node, "name");

        let name = match name {
            None => match &msg {
                None => String::new(),
                Some(msg) => to_snake_case(msg.as_str()),
            },
            Some(name) => name,
        };

        (name, msg)
    }

    fn print_operation(&mut self, port_type_name: &str, node: &Node) {
        let element_name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let func_name = to_snake_case(element_name);

        let some_documentation = node
            .children()
            .find(|c| c.has_tag_name("documentation"))
            .map(|c| c.text().unwrap_or_default());

        let some_input = node
            .children()
            .find(|c| c.has_tag_name("input"))
            .map(|c| self.map_name_message(&c));

        let some_output = node
            .children()
            .find(|c| c.has_tag_name("output"))
            .map(|c| self.map_name_message(&c));

        let some_fault = node
            .children()
            .find(|c| c.has_tag_name("fault"))
            .map(|c| self.map_name_message(&c));

        let port_type = PortType {
            name: format!("{}::{}", port_type_name, element_name.to_string()),
            input_type: some_input,
            output_type: some_output,
            fault_type: some_fault,
        };

        let input_type_template = match &port_type.input_type {
            Some((name, Some(message_type_name))) => {
                let type_name = to_pascal_case(name.as_str());

                self.format_type(
                    &type_name,
                    format!(
                        "pub type {0} = {1}::{2};\n",
                        type_name, MESSAGES_MOD, message_type_name,
                    ),
                )
            }
            _ => "".to_string(),
        };

        let input_template = match &port_type.input_type {
            Some((name, Some(msg))) => format!(
                "{}: {}",
                to_snake_case(name.as_str()),
                to_pascal_case(name.as_str())
            ),
            _ => ("".to_string()),
        };

        let (output_type_template, fault_type_template, output_template) =
            match &port_type.output_type {
                Some((name, Some(msg))) => {
                    if let Some((fault_name, Some(fault_type))) = &port_type.fault_type {
                        (
                            self.format_type(
                                &to_pascal_case(name.as_str()),
                                format!(
                                    "pub type {} = {}::{};\n",
                                    to_pascal_case(name.as_str()),
                                    MESSAGES_MOD,
                                    msg
                                ),
                            ),
                            Option::Some(self.format_type(
                                &to_pascal_case(fault_name.as_str()),
                                format!(
                                    "pub type {} = {}::{};\n",
                                    to_pascal_case(fault_name.as_str()),
                                    MESSAGES_MOD,
                                    fault_type
                                ),
                            )),
                            format!(
                                "-> Result<{0}, {1}>",
                                to_pascal_case(name.as_str()),
                                to_pascal_case(fault_name.as_str())
                            ),
                        )
                    } else {
                        (
                            self.format_type(
                                to_pascal_case(name.as_str()).as_str(),
                                format!(
                                    "pub type {} = {}::{};\n",
                                    to_pascal_case(name.as_str()),
                                    MESSAGES_MOD,
                                    msg.as_str()
                                ),
                            ),
                            Option::None,
                            format!("-> {}", to_pascal_case(name.as_str())),
                        )
                    }
                }
                _ => ("".to_string(), Option::None, "".to_string()),
            };

        self.queue_port_types(
            &input_type_template,
            &output_type_template,
            fault_type_template,
        );

        if let Some(doc) = some_documentation {
            self.write(format!("\t/// {}\n", doc))
        }

        self.write(format!(
            "\tasync fn {} (&mut self, {}) {};\n",
            func_name, input_template, output_template,
        ));

        self.port_types.insert(port_type.name.clone(), port_type);
    }

    fn queue_port_types(&mut self, input: &str, output: &str, fault: Option<String>) {
        // make sure these messages get written at the end of the module
        self.delayed_write(input.to_string());
        self.delayed_write(output.to_string());

        if let Some(f) = fault {
            self.delayed_write(f);
        }
    }

    fn construct_soap_wrapper(&self, soap_name: &str, body_type: &str) -> String {
        let tns = match &self.target_name_space {
            None => "Option::None".to_string(),
            Some(t) => t.to_string(),
        };

        format!(
            r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            root = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct {0}SoapEnvelope {{
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: {1},
        }}
        
        impl {0}SoapEnvelope {{
            pub fn new(body: {1}) -> Self {{
                {0}SoapEnvelope {{
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: {2},
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }}
            }}
        }}        
        "#,
            soap_name, body_type, tns
        )
    }

    fn print_binding_operation(&mut self, bind_type_name: &str, node: &Node) {
        let operation_name = match self.get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let port_type_name = format!("{}::{}", bind_type_name, operation_name);

        let port_type = match self.port_types.get(&port_type_name) {
            None => {
                warn!(
                    "failed to find matching port type for binding: {} with type: {}",
                    operation_name, port_type_name
                );
                return;
            }
            Some(pt) => pt.clone(),
        };

        let func_name = to_snake_case(operation_name);

        let (input_name, input_type, input_soap_name, has_input) = match &port_type.input_type {
            Some((input_name, Some(input_type))) => {
                let soap_name = format!("Soap{}", input_type);

                (
                    to_snake_case(input_name),
                    input_type.clone(),
                    soap_name,
                    true,
                )
            }
            _ => (String::new(), String::new(), String::new(), false),
        };

        let input_template = if has_input {
            format!("{}: {}::{}", input_name, PORTS_MOD, input_type)
        } else {
            String::new()
        };

        let soap_wrapper_in = if has_input {
            if !self.have_seen_type(input_soap_name.clone()) {
                self.seen_type(input_soap_name.clone());
                Option::from(format!(
                    "#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct {0} {{\n\t#[yaserde(rename = \"{3}\", default)]\n\tpub body: {2}::{1},\n}}\n{4}\n",
                    input_soap_name,
                    input_type,
                    PORTS_MOD,
                    operation_name,
                    self.construct_soap_wrapper(input_type.as_str(), input_soap_name.as_str())
                ))
            } else {
                Option::None
            }
        } else {
            Option::None
        };

        let (output_type, output_soap_name, output_xml_type, has_output) =
            match &port_type.output_type {
                Some((output_name, Some(output_type))) => {
                    let soap_name = format!("Soap{}", output_type);
                    (output_type.clone(), soap_name, output_name.clone(), true)
                }
                _ => (String::new(), String::new(), String::new(), false),
            };

        let fault_type = match &port_type.fault_type {
            Some((fault_name, Some(fault_type))) => Option::from(fault_type),
            _ => Option::None,
        };

        let soap_wrapper_out = if has_output {
            if !self.have_seen_type(output_soap_name.clone()) {
                self.seen_type(output_soap_name.clone());
                Option::from(format!(
                "#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct {0} {{\n\t#[yaserde(rename = \"{3}\", default)]\n\tpub body: {2}::{1},\n}}\n{4}\n",
                output_soap_name,
                output_type,
                PORTS_MOD,
                output_xml_type,
                self.construct_soap_wrapper(output_type.as_str(), output_soap_name.as_str())
            ))
            } else {
                Option::None
            }
        } else {
            Option::None
        };

        let output_template = if has_output {
            if let Some(fault_type) = fault_type {
                format!(
                    "-> Result<{2}::{0}, {2}::{1}>",
                    output_type, fault_type, PORTS_MOD,
                )
            } else {
                format!("-> {}::{}", PORTS_MOD, output_type)
            }
        } else {
            String::new()
        };

        self.write(format!(
            "\tasync fn {} (&mut self, {}) {} {{\n",
            func_name, input_template, output_template,
        ));

        if has_input && has_output {
            self.print_reqwest_body(
                input_name.as_str(),
                input_type.as_str(),
                output_type.as_str(),
                fault_type,
                operation_name,
            )
        }

        self.write("}\n".to_string());

        if let Some(soap_wrapper_in) = soap_wrapper_in {
            self.delayed_write(soap_wrapper_in);
        }

        if let Some(soap_wrapper_out) = soap_wrapper_out {
            self.delayed_write(soap_wrapper_out);
        }
    }

    fn print_reqwest_body(
        &mut self,
        input_variable: &str,
        input_type: &str,
        output_type: &str,
        fault_type: Option<&String>,
        operation_name: &str,
    ) {
        self.write(format!(
            r#"
        let __request = {1}SoapEnvelope::new(Soap{1} {{
            body: {0},
        }});            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/{3}",
        );
        
        if let Some(credentials) = &self.credentials {{
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }}
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: {2}SoapEnvelope = from_str(&txt).expect("can not unmarshal");
        "#,
            input_variable, input_type, output_type, operation_name
        ));

        if fault_type.is_some() {
            self.write("Ok(r.body.body)".to_string())
        } else {
            self.write("r.body.body".to_string())
        }
    }
}

impl ModWriter {
    fn new(section: Section) -> Self {
        let mut mw = ModWriter {
            section,
            buffers: vec![],
            delayed_buffer: Cursor::new(vec![]),
            final_stage: Cursor::new(vec![]),
            level: 0,
            defined_types: vec![],
        };

        match &mw.section {
            Section::Root => {}
            Section::Types => mw.print_mod_header(TYPES_MOD),
            Section::Messages => mw.print_mod_header(MESSAGES_MOD),
            Section::PortTypes => mw.print_mod_header(PORTS_MOD),
            Section::Bindings => mw.print_mod_header(BINDINGS_MOD),
        }

        mw
    }

    fn print_mod_header(&mut self, mod_name: &str) {
        self.write(format!("pub mod {} {{\n", mod_name), 0);
        self.print_header();
        self.write("use super::*;\n\n".to_string(), 0);
    }

    fn print_header(&mut self) {
        self.write(
            r#"use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use std::time::SystemTime;
            "#
            .to_string(),
            0,
        );
    }

    fn print_footer(&mut self) {
        if let Section::Root = self.section {
        } else {
            self.write("}\n\n".to_string(), 0);
        }
    }

    fn flush_buffers(&mut self) {
        while let Some(mut cursor) = self.buffers.pop() {
            cursor.set_position(0);
            if let Err(err) = io::copy(&mut cursor, &mut self.final_stage) {
                warn!("Failed to flush buffer: {:?}", err);
            }
        }
    }

    fn flush_delayed_buffer(&mut self) {
        self.delayed_buffer.set_position(0);
        if self.level == 0 {
            if let Err(err) = io::copy(&mut self.delayed_buffer, &mut self.final_stage) {
                warn!("Failed to flush buffer: {:?}", err);
            }
        } else if let Some(mut buffer) = self.buffers.get_mut(self.level - 1) {
            if let Err(err) = io::copy(&mut self.delayed_buffer, &mut buffer) {
                warn!("Failed to flush buffer: {:?}", err);
            }
        }
        self.delayed_buffer = Cursor::new(vec![]);
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;

        if level == 0 {
            self.flush_buffers();
        }

        if self.buffers.len() < (self.level as usize) {
            let b = Cursor::new(Vec::new());
            self.buffers.push(b);
        }
    }

    pub fn write(&mut self, buf: String, level: usize) {
        if level == 0 {
            // write to output
            self.final_stage
                .write_all(buf.as_bytes())
                .expect("can not write to output");
        } else if let Some(buffer) = self.buffers.get_mut(level - 1) {
            // store in buffer
            buffer
                .write_all(buf.as_bytes())
                .expect("can not write buffer");
        }
    }

    pub fn delayed_write(&mut self, buf: String) {
        self.delayed_buffer
            .write_all(buf.as_bytes())
            .expect("can not write to delayed buffer");
    }

    pub fn read_for_output(&mut self) -> RefCell<impl Read> {
        self.print_footer();
        self.final_stage.set_position(0);
        RefCell::new(self.final_stage.clone())
    }

    pub fn seen_type(&mut self, type_def: String) {
        self.defined_types.push(type_def);
    }

    pub fn reset_defined_types(&mut self) {
        self.defined_types.clear();
    }

    pub fn have_seen_type(&self, type_def: String) -> bool {
        self.defined_types.contains(&type_def)
    }
}
