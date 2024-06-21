use crate::{
    debug::DebugBuffer,
    element::{
        root, Element, ElementType, NamespacedElement, ParentElement, StaticElement,
        WritableElement,
    },
    error::{WriterError, WriterResult},
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
use log::warn;
use roxmltree::Node;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{stdout, Write},
    ops::Deref,
    path::PathBuf,
    rc::Rc,
};

const MESSAGES_MOD: &str = "messages";
const TYPES_MOD: &str = "types";
const PORTS_MOD: &str = "ports";
const BINDINGS_MOD: &str = "bindings";
const SERVICES_MOD: &str = "services";
const MULTIREF_MOD: &str = "multiref";
const ANY_TYPE: &str = "AnyType";
const ANY_TYPE_DEFINITION: &str = "Option<String>";
const ERROR_IMPL: &str = include_str!("../template/soap_fault_error.tmpl.rs");
const MULTIREF_TMPL: &str = include_str!("../template/multiref.tmpl.rs");

const SIGNATURE: &str = r"//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_NS_PREFIX: &str = "tns";
const IMPORT_PREFIX: &str = "nsi";

pub struct FileWriter {
    base_path: PathBuf,
    writer: Option<Box<dyn std::io::Write>>,
    target_name_space: Option<String>,
    port_types: HashMap<String, PortType>,
    message_types: HashMap<String, String>,
    namespaces: HashMap<String, String>,
    import_count: u32,
    ns_prefix: String,
    default_namespace: Option<String>,
    root: Element,
}

#[derive(Clone)]
struct PortType {
    name: String,
    input_type: Option<(String, Option<String>)>,
    output_type: Option<(String, Option<String>)>,
    fault_type: Option<(String, Option<String>)>,
}

impl Default for FileWriter {
    fn default() -> Self {
        FileWriter {
            base_path: PathBuf::default(),
            writer: Option::Some(Box::new(stdout())),
            target_name_space: Option::None,
            port_types: HashMap::new(),
            message_types: HashMap::new(),
            namespaces: HashMap::new(),
            import_count: 0,
            ns_prefix: DEFAULT_NS_PREFIX.to_string(),
            default_namespace: Option::None,
            root: root(),
        }
    }
}

impl FileWriter {
    #[allow(clippy::field_reassign_with_default)]
    #[must_use]
    pub fn new(ns_prefix: Option<String>, default_namespace: Option<String>) -> Self {
        let mut fw = FileWriter::default();
        fw.ns_prefix = ns_prefix.unwrap_or_else(|| DEFAULT_NS_PREFIX.to_string());
        fw.default_namespace = default_namespace;
        fw
    }

    #[must_use]
    pub fn new_file(
        dest_file_name: File,
        ns_prefix: Option<String>,
        default_namespace: Option<String>,
    ) -> Self {
        FileWriter {
            base_path: PathBuf::default(),
            writer: Option::Some(Box::new(dest_file_name)),
            target_name_space: Option::None,
            port_types: HashMap::new(),
            message_types: HashMap::new(),
            namespaces: HashMap::new(),
            import_count: 0,
            ns_prefix: ns_prefix.unwrap_or_else(|| DEFAULT_NS_PREFIX.to_string()),
            default_namespace,
            root: root(),
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn new_buffer(
        ns_prefix: Option<String>,
        default_namespace: Option<String>,
        buffer: DebugBuffer,
    ) -> Self {
        let mut fw = FileWriter::new(ns_prefix, default_namespace);
        fw.writer = Option::Some(Box::from(buffer));
        fw
    }

    fn init_modules(&mut self) {
        self.root.add(Element::new_module(MESSAGES_MOD));
        self.root.add(Element::new_module(TYPES_MOD));
        self.root.add(Element::new_module(PORTS_MOD));
        self.root.add(Element::new_module(BINDINGS_MOD));
        self.root.add(Element::new_module(SERVICES_MOD));
        self.root.add(Element::new_module_with_content(
            MULTIREF_MOD,
            MULTIREF_TMPL.to_string(),
        ));
    }

    pub fn process_file(&mut self, base_path: &str, file_name: &str) -> WriterResult<()> {
        self.base_path = PathBuf::from(base_path);
        self.print_global_header();
        self.print_common_structs();
        self.init_modules();
        self.process_file_in_path(file_name, true)?;
        Ok(())
    }

    fn process_file_in_path(&mut self, file_name: &str, print_when_done: bool) -> WriterResult<()> {
        let xml = self.read_to_string(file_name)?;
        let doc = roxmltree::Document::parse(&xml).map_err(|e| WriterError {
            message: format!("Unable to parse file {file_name}: {e}"),
        })?;
        doc.root().children().try_for_each(|n| self.print(&n))?;

        if !print_when_done {
            return Ok(());
        }

        // once all elements are processed, write them to output
        if let Some(mut writer) = self.writer.take() {
            writer.write_all(self.root.render().as_bytes())?;
            self.writer.replace(writer);
        }

        Ok(())
    }

    fn read_to_string(&self, file_name: &str) -> WriterResult<String> {
        if file_name.starts_with("http://") || file_name.starts_with("https://") {
            let body = reqwest::blocking::get(file_name)
                .map_err(|e| WriterError {
                    message: format!("Unable to retrieve {file_name}: {e}"),
                })?
                .text()
                .map_err(|e| WriterError {
                    message: format!("Unable to get body from {file_name}: {e}"),
                })?;
            return Ok(body);
        }

        let mut f_in = self.base_path.clone();
        f_in.push(file_name);

        std::fs::read_to_string(&f_in).map_err(|e| WriterError {
            message: format!("Unable to read file {f_in:?}: {e}"),
        })
    }

    #[must_use]
    pub fn have_seen_type(&self, type_def: &str, module: &Element) -> bool {
        module.has_child(type_def)
    }

    fn print_global_header(&mut self) {
        let mut global_header = Element::new("global_header", ElementType::Static);
        global_header.set_content(SIGNATURE);
        global_header.append_content(format!("//! version: {VERSION}\n//!\n").as_str());
        global_header.append_content(
            r#"
            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde_derive::{YaSerialize, YaDeserialize};
            use std::io::{Read, Write};
            use log::{warn, debug};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            "#,
        );

        self.root.add(global_header);
    }

    fn print_common_structs(&mut self) {
        let header = Element::new("Header", ElementType::Struct);
        let mut soap_fault = Element::new("SoapFault", ElementType::Struct);
        soap_fault.xml_name = Option::Some("Fault".to_string());
        soap_fault.add_ns("soapenv", "http://schemas.xmlsoap.org/soap/envelope/");
        soap_fault.prefix = Option::Some("soapenv".to_string());
        soap_fault.add(Element::new_field(
            "fault_code",
            "faultcode",
            "String",
            true,
        ));
        soap_fault.add(Element::new_field(
            "fault_string",
            "faultstring",
            "String",
            true,
        ));

        // Error implementation for SoapFault
        let mut soap_fault_error = Element::new("SoapFaultError", ElementType::Static);
        soap_fault_error.set_content(ERROR_IMPL);

        let mut soap_response = Element::new("SoapResponse", ElementType::Alias);
        soap_response.field_type =
            Option::Some("Result<(reqwest::StatusCode, String), reqwest::Error>".to_string());

        self.root.add(header);
        self.root.add(soap_fault);
        self.root.add(soap_fault_error);
        self.root.add(soap_response);
    }

    /// print parses the root of the XML file
    fn print(&mut self, node: &Node) -> WriterResult<()> {
        if !node.is_element() {
            return Ok(());
        }

        match node.tag_name().name() {
            "definitions" => self.print_definitions(node)?,
            "schema" => self.print_xsd(node)?,
            _ => {}
        }

        Ok(())
    }

    fn print_definitions(&mut self, node: &Node) -> WriterResult<()> {
        node.children()
            .filter(|child| child.tag_name().name() == "types")
            .try_for_each(|node| self.print_types(&node))?;

        node.children()
            .filter(|child| child.tag_name().name() == "message")
            .for_each(|node| self.print_message(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "portType")
            .for_each(|node| self.print_port_type(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "binding")
            .for_each(|node| self.print_binding(&node));

        node.children()
            .filter(|child| child.tag_name().name() == "service")
            .for_each(|node| self.print_service(&node));

        Ok(())
    }

    fn print_types(&mut self, node: &Node) -> WriterResult<()> {
        node.children()
            .filter(|c| c.has_tag_name("schema"))
            .try_for_each(|c| self.print_xsd(&c))?;

        Ok(())
    }

    fn print_xsd(&mut self, node: &Node) -> WriterResult<()> {
        self.target_name_space =
            Self::get_some_attribute(node, "targetNamespace").map(std::string::ToString::to_string);

        self.find_namespaces(node);

        node.children()
            .try_for_each(|child| match child.tag_name().name() {
                "import" => self.import_file(&child),
                "element" => {
                    let module = self.pick_section(TYPES_MOD);
                    let mut_module = &mut *module.deref().borrow_mut();
                    self.print_element(&child, true, &mut None, mut_module)
                }
                "complexType" => {
                    if let Some(n) = Self::get_some_attribute(&child, "name") {
                        let module = self.pick_section(TYPES_MOD);
                        let mut_module = &mut *module.deref().borrow_mut();
                        self.print_complex_element(&child, n, false, mut_module)
                    } else {
                        Ok(())
                    }
                }
                "simpleType" => {
                    if let Some(n) = Self::get_some_attribute(&child, "name") {
                        let module = self.pick_section(TYPES_MOD);
                        let mut_module = &mut *module.deref().borrow_mut();
                        self.print_simplex_element(&child, n, mut_module);
                        Ok(())
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            })?;

        Ok(())
    }

    fn find_namespaces(&mut self, node: &Node) {
        node.namespaces().for_each(|ns| {
            if let Some(name) = ns.name() {
                self.namespaces
                    .insert(name.to_string(), ns.uri().to_string());
            }
        });
    }

    fn import_file(&mut self, node: &Node) -> WriterResult<()> {
        let name = match Self::get_some_attribute(node, "schemaLocation") {
            None => return Ok(()),
            Some(n) => n,
        };

        let namespace = match Self::get_some_attribute(node, "namespace") {
            None => self.target_name_space.clone().unwrap_or_default(),
            Some(n) => n.to_string(),
        };

        self.import_count += 1;
        let prefix = format!("{}{}", IMPORT_PREFIX, self.import_count);

        let my_tns = self.target_name_space.replace(namespace);
        let my_prefix = self.ns_prefix.clone();
        self.ns_prefix = prefix;

        self.process_file_in_path(name, false)?;

        // restore old namespace from before import
        if let Some(old_tns) = my_tns {
            self.target_name_space.replace(old_tns);
        }

        self.ns_prefix = my_prefix;

        Ok(())
    }

    fn on_default_namespace(&self) -> bool {
        if let (Some(default_namespace), Some(namespace)) =
            (&self.default_namespace, &self.target_name_space)
        {
            return default_namespace == namespace;
        }

        false
    }

    #[allow(clippy::too_many_lines)]
    fn print_element(
        &mut self,
        node: &Node,
        is_top_level: bool,
        parent: &mut Option<&mut Element>,
        module: &mut Element,
    ) -> WriterResult<()> {
        let as_enum = node.has_tag_name("choice");
        if as_enum {
            let seq: Vec<Node> = node
                .children()
                .filter(|child| child.has_tag_name("sequence"))
                .collect();

            for node in seq {
                self.print_sequence(&node, parent, module)?;
            }

            let elements: Vec<Node> = node
                .children()
                .filter(|child| child.has_tag_name("element"))
                .collect();

            for node in elements {
                self.print_element(&node, false, parent, module)?;
            }
        }

        let element_name = match Self::get_some_attribute(node, "name") {
            None => return Ok(()),
            Some(n) => n,
        };

        let as_vec = {
            !matches!(
                Self::get_some_attribute(node, "maxOccurs"),
                Some("1") | None
            )
        };

        let as_option = {
            matches!(
                (
                    Self::get_some_attribute(node, "nillable"),
                    Self::get_some_attribute(node, "minOccurs"),
                ),
                (Some(_), _) | (_, Some("0"))
            )
        };

        let maybe_complex = node
            .children()
            .find(|child| child.has_tag_name("complexType"));

        let maybe_simplex = node
            .children()
            .find(|child| child.has_tag_name("simpleType"));

        let empty_element = node.children().count() == 0;

        let mut type_name = match Self::get_some_attribute(node, "type") {
            None => {
                if empty_element {
                    ANY_TYPE.to_string()
                } else {
                    to_pascal_case(element_name)
                }
            }
            Some(t) => t.to_string(),
        };

        if is_top_level {
            // top-level == type alias
            let top_level_name = to_pascal_case(element_name);
            let alias = Self::fetch_type(&type_name);

            if top_level_name != alias {
                let mut alias_element = Element::new(top_level_name.as_str(), ElementType::Alias);
                alias_element.field_type = Option::Some(alias);
                module.add(alias_element);

                // todo: add the AnyType definition
                if type_name == ANY_TYPE && !module.has_child(ANY_TYPE) {
                    let mut alias_element = Element::new(ANY_TYPE, ElementType::Alias);
                    alias_element.field_type = Option::Some(ANY_TYPE_DEFINITION.to_string());
                    module.add(alias_element);
                }

                return Ok(());
            }
        } else {
            let snake_name = to_snake_case(element_name);
            let field_name = Self::shield_reserved_names(&snake_name);

            // fields
            let mut element = if let Some(_tns) = &self.target_name_space {
                if is_top_level {
                    let mut e = Element::new(field_name, ElementType::Field);
                    e.xml_name = Option::Some(element_name.to_string());
                    e.add_ns("xsi", "http://www.w3.org/2001/XMLSchema-instance");
                    e
                } else if self.on_default_namespace() {
                    let mut e = Element::new(field_name, ElementType::Field);
                    e.xml_name = Option::Some(element_name.to_string());
                    e
                } else {
                    let mut e = Element::new(field_name, ElementType::Field);
                    e.xml_name = Option::Some(element_name.to_string());
                    e.prefix = Option::Some(self.ns_prefix.to_string());
                    e
                }
            } else {
                let mut e = Element::new(field_name, ElementType::Field);
                e.xml_name = Option::Some(element_name.to_string());
                e
            };

            if let Some(simple) = maybe_simplex {
                type_name = match Self::deconstruct_simplex_element(&simple) {
                    Ok(tn) => tn,
                    Err(_) => type_name,
                };
            }

            // add the element to the owning structure
            element.field_type = Some(Self::fetch_type(&type_name));
            element.vector = as_vec;
            element.optional = as_option;

            if let Some(p) = parent {
                // Self-referencing structs need to wrap the field in MultiRef<T>
                element.multi_ref = p.xml_name == element.field_type;
                p.add(element);
            }
        }

        if let Some(complex) = maybe_complex {
            self.print_complex_element(&complex, element_name, is_top_level, module)?;
        }

        Ok(())
    }

    fn get_some_attribute<'a>(node: &'a Node, attr_name: &str) -> Option<&'a str> {
        match node.attributes().find(|a| a.name() == attr_name) {
            None => None,
            Some(a) => Some(a.value()),
        }
    }

    fn get_some_attribute_as_string(node: &Node, attr_name: &str) -> Option<String> {
        node.attributes()
            .find(|a| a.name() == attr_name)
            .map(|a| a.value().to_string())
    }

    fn fetch_type(node_type: &str) -> String {
        match Self::split_type(node_type) {
            "byte" => "i8".to_string(),
            "string" | "normalizedString" | "base64Binary" | "hexBinary" | "anyURI" => {
                "String".to_string()
            }
            "decimal" | "double" => "f64".to_string(),
            "float" => "f32".to_string(),
            "integer" | "int" | "negativeInteger" | "nonNegativeInteger" | "nonPositiveInteger"
            | "positiveInteger" => "i32".to_string(),
            "long" => "i64".to_string(),
            "unsignedLong" => "u64".to_string(),
            "unsignedInt" => "u32".to_string(),
            "unsignedShort" => "u16".to_string(),
            "unsignedByte" => "u8".to_string(),
            "short" => "i16".to_string(),
            "boolean" => "bool".to_string(),
            // use String for date types
            "date" | "dateTime" | "time" => "String".to_string(),
            v => to_pascal_case(v),
        }
    }

    fn is_primitive(field_type: &str) -> bool {
        matches!(
            field_type,
            "i8" | "u8" | "i16" | "f32" | "i32" | "u32" | "f64" | "i64" | "u64" | "String"
        )
    }

    fn split_type(node_type: &str) -> &str {
        node_type.split(':').last().unwrap_or("String")
    }

    fn init_element(&self, name: &str, is_top_level: bool) -> Element {
        let some_tns = self.target_name_space.clone();

        if let Some(tns) = some_tns {
            let element_name = to_pascal_case(name);
            let mut e = Element::new(&element_name, ElementType::Struct);

            if is_top_level {
                e.prefix = Option::Some(self.ns_prefix.to_string());
                e.xml_name = Option::Some(name.to_string());

                // declare all namespaces
                // e.add_ns("tns", &tns);
                e.add_ns(&self.ns_prefix, &tns);
                e.add_ns("xsi", "http://www.w3.org/2001/XMLSchema-instance");
                e
            } else if self.on_default_namespace() {
                e.xml_name = Option::Some(name.to_string());
                e
            } else {
                e.prefix = Option::Some(self.ns_prefix.to_string());
                e.xml_name = Option::Some(name.to_string());
                e.add_ns(&self.ns_prefix, &tns);
                e
            }
        } else {
            let element_name = to_pascal_case(name);
            let mut e = Element::new(&element_name, ElementType::Struct);
            e.xml_name = Option::Some(name.to_string());
            e
        }
    }

    fn print_simplex_element(&mut self, node: &Node, name: &str, module: &mut Element) {
        if self.have_seen_type(name, module) {
            return;
        }

        let mut parent_element = self.init_element(name, false);
        let type_name = match Self::deconstruct_simplex_element(node) {
            Ok(tn) => tn,
            Err(_) => to_pascal_case(name),
        };

        let mut field = Element::new("body", ElementType::Field);
        let field_type = Self::fetch_type(&type_name);
        field.text_field = field_type == "String";
        field.flatten = !Self::is_primitive(&field_type);
        field.field_type = Option::Some(field_type);
        field.xml_name = None;

        parent_element.add(field);

        if !self.have_seen_type(&parent_element.name, module) {
            module.add(parent_element);
        }
    }

    fn print_complex_element(
        &mut self,
        node: &Node,
        name: &str,
        is_top_level: bool,
        module: &mut Element,
    ) -> WriterResult<()> {
        if self.have_seen_type(name, module) {
            return Ok(());
        }

        let mut element = self.init_element(name, is_top_level);

        let maybe_sequence = node.children().find(|child| child.has_tag_name("sequence"));

        let maybe_complex = node
            .children()
            .find(|child| child.has_tag_name("complexContent"));

        node.children()
            .filter(|c| c.has_tag_name("attribute"))
            .for_each(|c| {
                Self::print_attribute(&c, &mut element);
            });

        if let Some(sequence) = maybe_sequence {
            self.print_sequence(&sequence, &mut Some(&mut element), module)?;
        }

        if let Some(complex) = maybe_complex {
            self.print_complex_content(&complex, &mut Some(&mut element), module)?;
        }

        if !self.have_seen_type(&element.name, module) {
            module.add(element);
        }

        Ok(())
    }

    fn print_attribute(node: &Node, parent: &mut Element) {
        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let element_type = match Self::get_some_attribute(node, "type") {
            None => return,
            Some(n) => Self::fetch_type(n),
        };

        let optional = match Self::get_some_attribute(node, "use") {
            None => false,
            Some(a) => a != "required",
        };

        let mut element =
            Element::new(to_snake_case(element_name).as_str(), ElementType::Attribute);

        element.xml_name = Option::Some(element_name.to_string());
        element.field_type = Option::Some(element_type);
        element.optional = optional;
        parent.add(element);
    }

    fn deconstruct_simplex_element(node: &Node) -> WriterResult<String> {
        let restriction = match node.children().find(|c| c.has_tag_name("restriction")) {
            None => {
                return Err(WriterError {
                    message: "restriction element is missing".to_string(),
                })
            }
            Some(b) => b,
        };

        let base = match Self::get_some_attribute(&restriction, "base") {
            None => {
                return Err(WriterError {
                    message: "base type is missing".to_string(),
                })
            }
            Some(b) => b,
        };

        Ok(base.to_string())
    }

    fn print_sequence(
        &mut self,
        node: &Node,
        parent: &mut Option<&mut Element>,
        module: &mut Element,
    ) -> WriterResult<()> {
        node.children().try_for_each(|child| {
            if let Some(p) = parent {
                self.print_element(&child, false, &mut Some(p), module)
            } else {
                Ok(())
            }
        })?;
        Ok(())
    }

    fn print_complex_content(
        &mut self,
        node: &Node,
        parent: &mut Option<&mut Element>,
        module: &mut Element,
    ) -> WriterResult<()> {
        if let Some(extension) = node
            .children()
            .find(|child| child.has_tag_name("extension"))
        {
            Self::print_extension(&extension, parent);

            let maybe_sequence = extension
                .children()
                .find(|ext_child| ext_child.has_tag_name("sequence"));

            if let Some(sequence) = maybe_sequence {
                self.print_sequence(&sequence, parent, module)?;
            }
        }

        self.print_sequence(node, parent, module)
    }

    fn print_extension(node: &Node, parent: &mut Option<&mut Element>) {
        if let Some(p) = parent {
            let base = match Self::get_some_attribute(node, "base") {
                None => return,
                Some(n) => n,
            };

            let mut element = Element::new(
                to_snake_case(&Self::fetch_type(base)).as_str(),
                ElementType::Field,
            );
            let field_type = Self::fetch_type(base);
            element.flatten = !Self::is_primitive(&field_type);
            element.field_type = Option::Some(field_type);
            p.add(element);

            let type_name = Self::fetch_type(base);
            let mut xsi = Element::new("xsi_type", ElementType::Attribute);
            xsi.field_type = Option::Some("String".to_string());
            xsi.prefix = Option::Some("xsi".to_string());
            xsi.xml_name = Option::Some("type".to_string());
            xsi.comment = Option::Some(type_name);
            p.add(xsi);
        }
    }

    fn shield_reserved_names(type_name: &str) -> &str {
        match type_name {
            "type" => "rs_type",
            other => other,
        }
    }

    fn pick_section(&mut self, target: &str) -> Rc<RefCell<Element>> {
        self.root
            .child(target)
            .expect("modules have not been initialized properly")
    }

    // WSDL Messages

    fn print_message(&mut self, node: &Node) {
        let parent = self.pick_section(MESSAGES_MOD);
        let mut_parent = &mut *parent.deref().borrow_mut();

        if let Some(name) = Self::get_some_attribute(node, "name") {
            let mut element = Element::new(to_pascal_case(name).as_str(), ElementType::Struct);
            element.xml_name = Option::Some(name.to_string());

            let maybe_part = node.children().find(|child| child.has_tag_name("part"));

            if let Some(part) = maybe_part {
                if let Some(type_name) = Self::get_some_attribute(&part, "type") {
                    // simple type
                    self.print_simple_part(name, &part, type_name, &mut element);
                } else {
                    self.print_element_part(name, &part, &mut element);
                }
            }

            if !self.have_seen_type(&element.name, mut_parent) {
                mut_parent.add(element);
            }
        }
    }

    fn print_element_part(&mut self, message_name: &str, node: &Node, parent: &mut Element) {
        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        if let Some(type_name) = Self::get_some_attribute(node, "element") {
            let type_name = format!("{}::{}", TYPES_MOD, Self::fetch_type(type_name));

            let mut element = Element::new(
                Self::shield_reserved_names(&to_snake_case(element_name)),
                ElementType::Field,
            );
            element.flatten = !Self::is_primitive(&type_name);
            element.field_type = Option::Some(type_name.clone());
            parent.add(element);

            self.message_types
                .insert(message_name.to_string(), type_name);
        }
    }

    fn print_simple_part(
        &mut self,
        message_name: &str,
        node: &Node,
        type_name: &str,
        parent: &mut Element,
    ) {
        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let element = Element::new_field(
            Self::shield_reserved_names(&to_snake_case(element_name)),
            element_name,
            Self::fetch_type(type_name).as_str(),
            false,
        );

        parent.add(element);

        self.message_types
            .insert(message_name.to_string(), type_name.to_string());
    }

    // WSDL Port Types
    fn print_port_type(&mut self, node: &Node) {
        let parent = self.pick_section(PORTS_MOD);
        let mut_parent = &mut *parent.deref().borrow_mut();

        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let struct_name = to_pascal_case(element_name);
        let mut element = Element::new(struct_name.as_str(), ElementType::Trait);

        node.children().for_each(|child| {
            self.print_operation(
                to_pascal_case(element_name).as_str(),
                &child,
                &mut element,
                mut_parent,
            );
        });

        mut_parent.add(element);
    }

    // WSDL bindings

    fn print_binding(&mut self, node: &Node) {
        let parent = self.pick_section(BINDINGS_MOD);
        let mut_parent = &mut *parent.deref().borrow_mut();

        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let type_name = match Self::get_some_attribute(node, "type") {
            None => return,
            Some(n) => n,
        };

        let struct_name = to_pascal_case(element_name);
        let trait_name = Self::fetch_type(type_name);

        if !self.have_seen_type(&struct_name, mut_parent) {
            Self::print_binding_helpers(&struct_name, mut_parent);
        }

        let mut client = Element::new(&struct_name, ElementType::Static);
        client.set_content(
            format!(
                r#"pub struct {struct_name} {{
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }}
                "#
            )
            .as_str(),
        );

        let mut t_impl = Element::new(&struct_name, ElementType::TraitImpl);
        t_impl.field_type = Option::Some(format!("{PORTS_MOD}::{trait_name}"));

        node.children().for_each(|child| {
            self.print_binding_operation(&trait_name, &child, &mut t_impl, mut_parent);
        });

        self.print_default_constructor(struct_name.as_str(), mut_parent);
        Self::print_constructor(struct_name.as_str(), mut_parent);

        mut_parent.add(client);
        mut_parent.add(t_impl);
    }

    fn print_binding_helpers(struct_name: &str, parent: &mut Element) {
        let mut e = Element::new(struct_name, ElementType::Static);
        e.set_content(format!(r#"
            impl {struct_name} {{
                async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {{
                    let body = to_string(request).expect("failed to generate xml");
                    debug!("SOAP Request: {{}}", body);
                    let mut req = self
                        .client
                        .post(&self.url)
                        .body(body)
                        .header("Content-Type", "text/xml")
                        .header("Soapaction", action);
                    if let Some(credentials) = &self.credentials {{
                        req = req.basic_auth(
                            credentials.0.to_string(),
                            Option::Some(credentials.1.to_string()),
                        );
                    }}
                    let res = req.send().await?;
                    let status = res.status();
                    debug!("SOAP Status: {{}}", status);
                    let txt = res.text().await.unwrap_or_default();
                    debug!("SOAP Response: {{}}", txt);
                    Ok((status, txt))
                }}
            }}
            "#).as_str());

        parent.add(e);
    }

    fn print_default_constructor(&mut self, struct_name: &str, parent: &mut Element) {
        let url = match &self.target_name_space {
            None => "String::new()".to_string(),
            Some(tns) => tns.to_string(),
        };

        let mut e = Element::new(struct_name, ElementType::Static);
        e.set_content(
            format!(
                r#"impl Default for {struct_name} {{
                fn default() -> Self {{
                    {struct_name} {{
                        client: reqwest::Client::new(),
                        url: "{url}".to_string(),
                        credentials: Option::None,
                     }}
                }}
            }}
            "#
            )
            .as_str(),
        );

        parent.add(e);
    }

    fn print_constructor(struct_name: &str, parent: &mut Element) {
        let mut e = Element::new(struct_name, ElementType::Static);
        e.set_content(
            format!(
                r#"impl {struct_name} {{
                #[must_use]
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {{
                    {struct_name} {{
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }}
                }}
        }}
        "#
            )
            .as_str(),
        );

        parent.add(e);
    }

    fn map_name_message(node: &Node) -> (String, Option<String>) {
        let msg = Self::get_some_attribute_as_string(node, "message").map(|m| Self::fetch_type(&m));
        let name = Self::get_some_attribute_as_string(node, "name");

        let name = match name {
            None => match &msg {
                None => String::new(),
                Some(msg) => to_pascal_case(msg.as_str()),
            },
            Some(name) => name,
        };

        (name, msg)
    }

    fn print_operation(
        &mut self,
        port_type_name: &str,
        node: &Node,
        parent: &mut Element,
        module: &mut Element,
    ) {
        let element_name = match Self::get_some_attribute(node, "name") {
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
            .map(|c| Self::map_name_message(&c));

        let some_output = node
            .children()
            .find(|c| c.has_tag_name("output"))
            .map(|c| Self::map_name_message(&c));

        let some_fault = node
            .children()
            .find(|c| c.has_tag_name("fault"))
            .map(|c| Self::map_name_message(&c));

        let port_type = PortType {
            name: format!("{port_type_name}::{element_name}"),
            input_type: some_input,
            output_type: some_output,
            fault_type: some_fault,
        };

        let input_type_element = match &port_type.input_type {
            Some((type_name, Some(message_type_name))) => {
                let mut e = Element::new(to_pascal_case(type_name).as_str(), ElementType::Alias);
                e.field_type = Option::Some(format!("{MESSAGES_MOD}::{message_type_name}",));

                Option::Some(e)
            }
            _ => None,
        };

        let mut function_element = match &port_type.input_type {
            Some((name, Some(_msg))) => Element::new_function(
                &func_name,
                to_snake_case(name.as_str()).as_str(),
                to_pascal_case(name.as_str()).as_str(),
            ),
            _ => return,
        };

        let (output_type_element, fault_type_element) = match &port_type.output_type {
            Some((type_name, Some(msg))) => {
                let mut e = Element::new(to_pascal_case(type_name).as_str(), ElementType::Alias);
                e.field_type = Option::Some(format!("{MESSAGES_MOD}::{msg}"));

                if let Some((fault_name, Some(fault_type))) = &port_type.fault_type {
                    let mut f = Element::new(fault_name.as_str(), ElementType::Alias);
                    f.field_type = Option::Some(format!("{MESSAGES_MOD}::{fault_type}",));

                    if !self.have_seen_type(fault_name, module) {
                        self.fault_soap_wrapper(fault_name, fault_type, module);
                    }

                    if let Some(mut args) = function_element.function_args.take() {
                        args.output_type = Option::Some(to_pascal_case(type_name));
                        args.fault_type =
                            Option::Some(format!("Option<Soap{}>", to_pascal_case(fault_name)));
                        function_element.function_args.replace(args);
                    }

                    (Option::Some(e), Option::Some(f))
                } else {
                    if let Some(mut args) = function_element.function_args.take() {
                        args.output_type = Option::Some(to_pascal_case(type_name));
                        args.fault_type = Option::Some("Option<SoapFault>".to_string());
                        function_element.function_args.replace(args);
                    }

                    (Option::Some(e), Option::None)
                }
            }
            _ => (Option::None, Option::None),
        };

        if let Some(input_type_element) = input_type_element {
            if !self.have_seen_type(&input_type_element.name, module) {
                module.add(input_type_element);
            }
        }

        if let Some(output_type_element) = output_type_element {
            if !self.have_seen_type(&output_type_element.name, module) {
                module.add(output_type_element);
            }
        }

        if let Some(fault_type_element) = fault_type_element {
            if !self.have_seen_type(&fault_type_element.name, module) {
                module.add(fault_type_element);
            }
        }

        if let Some(doc) = some_documentation {
            function_element.comment = Option::Some(doc.to_string());
        }

        parent.add(function_element);
        self.port_types.insert(port_type.name.clone(), port_type);
    }

    fn fault_soap_wrapper(&self, fault_name: &str, fault_type: &str, parent: &mut Element) {
        let soap_fault_name = format!("Soap{fault_name}");

        let mut e = Element::new(&soap_fault_name, ElementType::Struct);
        e.xml_name = Option::Some("Fault".to_string());
        e.add_ns("soapenv", "http://schemas.xmlsoap.org/soap/envelope/");
        e.prefix = Option::Some("soapenv".to_string());

        let fault_code = Element::new_field("fault_code", "faultcode", "String", true);
        let fault_string = Element::new_field("fault_string", "faultstring", "String", true);
        let detail = Element::new_field("detail", fault_type, fault_name, true);

        e.add(fault_code);
        e.add(fault_string);
        e.add(detail);

        if !self.have_seen_type(&e.name, parent) {
            parent.add(e);
        }
    }

    fn construct_soap_wrapper(&self, soap_name: &str, body_type: &str) -> String {
        let tns = match &self.target_name_space {
            None => "Option::None".to_string(),
            Some(t) => format!("Option::Some(\"{t}\".to_string())"),
        };

        format!(
            r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct {0}SoapEnvelope {{
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: Option<String>,
            #[yaserde(rename = "{3}", prefix = "xmlns", attribute)]
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
            #[must_use]
            pub fn new(body: {1}) -> Self {{
                {0}SoapEnvelope {{
                    encoding_style: Some(SOAP_ENCODING.to_string()),
                    tnsattr: {2},
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }}
            }}
        }}        
        "#,
            soap_name, body_type, tns, self.ns_prefix
        )
    }

    #[allow(clippy::too_many_lines)]
    fn print_binding_operation(
        &mut self,
        bind_type_name: &str,
        node: &Node,
        parent: &mut Element,
        module: &mut Element,
    ) {
        let operation_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let message_type_name = match self.message_types.get(operation_name) {
            None => operation_name.to_string(),
            Some(mt) => Self::split_type(mt).to_string(),
        };

        let port_type_name = format!("{bind_type_name}::{operation_name}");

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
                let soap_name = format!("Soap{input_type}");

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
            format!("{input_name}: {PORTS_MOD}::{input_type}")
        } else {
            String::new()
        };

        let soap_wrapper_in = if has_input {
            if self.have_seen_type(&input_soap_name, parent) {
                Option::None
            } else {
                Option::Some(format!(
                    r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct {0} {{
                        #[yaserde(rename = "{3}", default)]
                        pub body: {2}::{1},
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }}
                    {4}
                    "#,
                    input_soap_name,
                    input_type,
                    PORTS_MOD,
                    // this should be renamed when renaming is used
                    message_type_name,
                    self.construct_soap_wrapper(input_type.as_str(), input_soap_name.as_str())
                ))
            }
        } else {
            Option::None
        };

        let (output_type, output_soap_name, output_xml_type, has_output) =
            match &port_type.output_type {
                Some((output_name, Some(output_type))) => {
                    let soap_name = format!("Soap{output_type}");
                    (output_type.clone(), soap_name, output_name.clone(), true)
                }
                _ => (String::new(), String::new(), String::new(), false),
            };

        let output_xml_type = match self.message_types.get(&output_xml_type) {
            None => output_xml_type.to_string(),
            Some(mt) => Self::split_type(mt).to_string(),
        };

        let (_fault_type, _fault_xml_type, fault_soap_name, has_fault) = match &port_type.fault_type
        {
            Some((fault_name, Some(fault_type))) => {
                let soap_name = format!("Soap{fault_type}");
                (
                    fault_type.to_string(),
                    fault_name.to_string(),
                    soap_name,
                    true,
                )
            }
            _ => (String::new(), String::new(), String::new(), false),
        };

        let soap_fault = if has_fault {
            format!(
                r#"     #[yaserde(rename = "Fault", default)]
                            pub fault: Option<{PORTS_MOD}::{fault_soap_name}>,
                            "#,
            )
        } else {
            r#"     #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            "#
            .to_string()
        };

        let soap_wrapper_out = if has_output {
            if self.have_seen_type(&output_soap_name, parent) {
                Option::None
            } else {
                Option::Some(format!(
                    r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct {0} {{
                    #[yaserde(rename = "{3}", default)]
                    pub body: {2}::{1},
                    {4}
                }}
                {5}
                "#,
                    output_soap_name,
                    output_type,
                    PORTS_MOD,
                    output_xml_type,
                    soap_fault,
                    self.construct_soap_wrapper(output_type.as_str(), output_soap_name.as_str()),
                ))
            }
        } else {
            Option::None
        };

        let output_template = if has_output {
            if has_fault {
                format!(
                    "-> Result<{PORTS_MOD}::{output_type}, Option<{PORTS_MOD}::{fault_soap_name}>>",
                )
            } else {
                format!("-> Result<{PORTS_MOD}::{output_type}, Option<SoapFault>>")
            }
        } else {
            String::new()
        };

        let some_soap_action = node
            .children()
            .find(|c| c.has_tag_name("operation"))
            .map(|opp| opp.attribute("soapAction"))
            .unwrap_or_default();

        // todo: convert this to function
        let mut e = Element::new(&func_name, ElementType::Static);
        e.set_content(
            format!("\tasync fn {func_name} (&self, {input_template}) {output_template} {{\n",)
                .as_str(),
        );

        if has_input && has_output {
            self.print_reqwest_body(
                input_name.as_str(),
                input_type.as_str(),
                output_type.as_str(),
                operation_name,
                some_soap_action,
                &mut e,
            );
        }

        e.append_content("}");

        if !self.have_seen_type(&e.name, parent) {
            parent.add(e);
        }

        if let Some(soap_wrapper_in) = soap_wrapper_in {
            if !self.have_seen_type(&input_soap_name, module) {
                let mut e_in = Element::new(&input_soap_name, ElementType::Static);
                e_in.set_content(&soap_wrapper_in);
                module.add(e_in);
            }
        }

        if let Some(soap_wrapper_out) = soap_wrapper_out {
            if !self.have_seen_type(&output_soap_name, module) {
                let mut e_out = Element::new(&output_soap_name, ElementType::Static);
                e_out.set_content(&soap_wrapper_out);
                module.add(e_out);
            }
        }
    }

    fn print_reqwest_body(
        &mut self,
        input_variable: &str,
        input_type: &str,
        output_type: &str,
        operation_name: &str,
        soap_action: Option<&str>,
        parent: &mut Element,
    ) {
        let action = match soap_action {
            None => match &self.target_name_space {
                None => "undefined".to_string(),
                Some(tns) => format!("{tns}/{operation_name}"),
            },
            Some(sa) => sa.to_string(),
        };

        let xmlns = match &self.target_name_space {
            None => "Option::None".to_string(),
            Some(tns) => format!("Option::Some(\"{tns}\".to_string())"),
        };

        parent.append_content(
            format!(
                r#"
        let __request = {input_type}SoapEnvelope::new(Soap{input_type} {{
            body: {input_variable},
            xmlns: {xmlns},
        }});            
        
        let (status, response) = self.send_soap_request(&__request, "{action}")
                    .await
                    .map_err(|err| {{
                        warn!("Failed to send SOAP request: {{:?}}", err);
                        None
                    }})?;

        let r: {output_type}SoapEnvelope = from_str(&response).map_err(|err| {{
                        warn!("Failed to unmarshal SOAP response: {{:?}}", err);
                        None
                    }})?;
        "#
            )
            .as_str(),
        );

        parent.append_content(
            r"if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }",
        );
    }

    // WSDL Services

    fn print_service(&mut self, node: &Node) {
        let parent = self.pick_section(SERVICES_MOD);
        let mut_parent = &mut *parent.deref().borrow_mut();

        let element_name = match Self::get_some_attribute(node, "name") {
            None => return,
            Some(n) => n,
        };

        let some_documentation = node
            .children()
            .find(|c| c.has_tag_name("documentation"))
            .map(|c| c.text().unwrap_or_default());

        let some_port = node.children().find(|c| c.has_tag_name("port"));

        let port = match some_port {
            None => return,
            Some(p) => p,
        };

        let some_binding = port
            .attributes()
            .find(|a| a.name() == "binding")
            .map(|a| Self::fetch_type(a.value()));

        let binding = match some_binding {
            None => return,
            Some(b) => b,
        };

        let some_address = port.children().find(|c| c.has_tag_name("address"));

        let address = match some_address {
            None => return,
            Some(a) => a,
        };

        let location = address
            .attributes()
            .find(|a| a.name() == "location")
            .map(|a| a.value())
            .unwrap_or_default();

        let struct_name = to_pascal_case(element_name);

        if self.have_seen_type(&struct_name, mut_parent) {
            return;
        }

        let mut e = Element::new(&struct_name, ElementType::Static);

        if let Some(doc) = some_documentation {
            e.comment = Option::Some(doc.to_string());
        }

        e.set_content(
            format!(
                r#"pub struct {struct_name} {{}}
               impl {struct_name} {{
                "#,
            )
            .as_str(),
        );

        e.append_content(
            format!(
                r#"
            #[must_use]
            pub fn new_client(credentials: Option<(String, String)>) -> {2}::{1} {{
                Self::new_client_with_url("{0}", credentials)
            }}

            #[must_use]
            pub fn new_client_with_url(url: &str, credentials: Option<(String, String)>) -> {2}::{1} {{
                {2}::{1}::new(url, credentials)
            }}
        "#,
                location,
                to_pascal_case(binding.as_str()),
                BINDINGS_MOD,
            )
            .as_str(),
        );

        e.append_content("}\n");

        mut_parent.add(e);
    }
}

#[cfg(test)]
mod test_xsd {
    use super::*;
    use crate::debug::DebugBuffer;
    use std::io::Read;

    fn prepare_output(
        base_path: &str,
        filename: &str,
        ns_prefix: Option<String>,
        default_ns: Option<String>,
    ) -> String {
        let mut buffer = DebugBuffer::default();
        let mut fw = FileWriter::new_buffer(ns_prefix, default_ns, buffer.clone());
        fw.process_file(base_path, filename)
            .expect("can not open xsd");

        let mut result = String::new();
        buffer
            .read_to_string(&mut result)
            .expect("failed to get content");
        result
    }

    fn prepare_smgr_output(ns_prefix: Option<String>, default_ns: Option<String>) -> String {
        prepare_output(
            "../resources/smgr/",
            "agentCommProfile.xsd",
            ns_prefix,
            default_ns,
        )
    }

    #[test]
    fn test_attributes() {
        let result = prepare_smgr_output(
            None,
            Some("http://xml.avaya.com/schema/import_csm_agent".to_string()),
        );
        assert!(
            result.contains(r#"#[yaserde(rename="createTenantIfNotAlreadyPresent", attribute)]"#)
        );
    }

    #[test]
    fn test_default_namespace() {
        let result = prepare_smgr_output(
            None,
            Some("http://xml.avaya.com/schema/import_csm_agent".to_string()),
        );

        // no prefix for default namespace
        assert!(result.contains(
            r#"#[yaserde(
	rename = "xmlAgentProfile",
)]"#
        ));
    }

    #[test]
    fn test_no_default_namespace() {
        let result = prepare_smgr_output(None, None);

        assert!(result.contains(
            r#"#[yaserde(
	rename = "xmlAgentProfile",
	namespace = "tns: http://xml.avaya.com/schema/import_csm_agent",
	prefix = "tns",
)]"#
        ));
    }

    #[test]
    fn test_ns_prefix() {
        let result = prepare_smgr_output(Some("ns2".to_string()), None);

        assert!(result.contains(
            r#"#[yaserde(
	rename = "xmlAgentProfile",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_agent",
	prefix = "ns2",
)]"#
        ));
    }

    #[test]
    fn test_import() {
        let result = prepare_smgr_output(None, None);
        assert!(result.contains(r"xmlContact"));
    }

    #[test]
    fn test_simple_xsd() {
        let result = prepare_output("../resources/simple/", "simple.xsd", None, None);
        println!("{result}");
        assert!(result.contains(r"pub type ThisIsAString = String;"));
        assert!(result.contains(r"pub type CouldBeAnything = AnyType;"));
        assert!(result.contains(&format!("pub type AnyType = {ANY_TYPE_DEFINITION};")));
    }
}

#[cfg(test)]
mod test_wsdl {
    use super::*;
    use std::io::Read;

    fn prepare_output(ns_prefix: Option<String>, default_ns: Option<String>) -> String {
        let mut buffer = DebugBuffer::default();
        let mut fw = FileWriter::new_buffer(ns_prefix, default_ns, buffer.clone());
        fw.process_file("../resources/temp_converter/", "tempconverter.wsdl")
            .expect("can not open wsdl");

        let mut result = String::new();
        buffer
            .read_to_string(&mut result)
            .expect("failed to get content");
        result
    }

    #[test]
    fn test_service() {
        let result = prepare_output(None, None);
        assert!(result.contains(r#"bindings::TempConverterEndpointServiceSoapBinding::new("https://apps.learnwebservices.com:443/services/tempconverter", credentials)"#));
    }
}
