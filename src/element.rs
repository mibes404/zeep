use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum ElementType {
    Root,
    Struct,
    Field,
    Static,
    Alias,
    Module,
    Attribute,
    Trait,
    TraitImpl,
    Function,
}

pub struct Element {
    pub element_type: ElementType,
    pub prefix: Option<String>,
    pub xml_name: Option<String>,
    pub namespaces: Vec<String>,
    pub name: String,
    pub children: Vec<Rc<RefCell<Element>>>,
    pub children_idx: HashMap<String, usize>,
    static_content: Option<String>,
    pub optional: bool,
    pub field_type: Option<String>,
    pub vector: bool,
    pub flatten: bool,
    pub comment: Option<String>,
    pub function_args: Option<FunctionArgs>,
}

pub struct FunctionArgs {
    pub input_type: String,
    pub input_name: String,
    pub output_type: Option<String>,
    pub fault_type: Option<String>,
    pub comment: Option<String>,
}

pub fn root() -> Element {
    Element {
        element_type: ElementType::Root,
        prefix: None,
        xml_name: None,
        name: "root".to_string(),
        children: vec![],
        children_idx: HashMap::new(),
        static_content: None,
        namespaces: vec![],
        optional: false,
        field_type: None,
        vector: false,
        flatten: false,
        comment: None,
        function_args: None,
    }
}

pub trait WritableElement {
    fn render(&self) -> String;
}

pub trait StaticElement {
    fn set_content(&mut self, content: &str);
    fn append_content(&mut self, content: &str);
}

pub trait ParentElement {
    fn add(&mut self, child: Element);
    fn child(&self, name: &str) -> Option<Rc<RefCell<Element>>>;
    fn has_children(&self) -> bool;
    fn has_child(&self, name: &str) -> bool;
}

pub trait NamespacedElement {
    fn add_ns(&mut self, prefix: &str, ns: &str);
}

impl WritableElement for Element {
    fn render(&self) -> String {
        match self.element_type {
            ElementType::Root => self.render_root(),
            ElementType::Struct => self.render_struct(),
            ElementType::Field => self.render_field(),
            ElementType::Static => self.render_static(),
            ElementType::Alias => self.render_alias(),
            ElementType::Module => self.render_module(),
            ElementType::Attribute => self.render_atribute(),
            ElementType::Trait => self.render_trait(),
            ElementType::TraitImpl => self.render_trait_impl(),
            ElementType::Function => self.render_function(),
        }
    }
}

impl NamespacedElement for Element {
    fn add_ns(&mut self, prefix: &str, ns: &str) {
        self.namespaces.push(format!("{}: {}", prefix, ns))
    }
}

impl StaticElement for Element {
    fn set_content(&mut self, content: &str) {
        self.static_content = Option::Some(content.to_string());
    }

    fn append_content(&mut self, additional_content: &str) {
        if let Some(mut content) = self.static_content.take() {
            content.push_str(additional_content);
            self.static_content.replace(content);
        }
    }
}

impl ParentElement for Element {
    fn add(&mut self, child: Element) {
        let name = child.name.clone();
        let pos = self.children.len();
        self.children.push(Rc::new(RefCell::new(child)));
        self.children_idx.insert(name, pos);
    }

    fn child(&self, name: &str) -> Option<Rc<RefCell<Element>>> {
        if let Some(pos) = self.children_idx.get(name) {
            self.children.get(*pos).cloned()
        } else {
            None
        }
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn has_child(&self, name: &str) -> bool {
        if let Some(_pos) = self.children_idx.get(name) {
            true
        } else {
            false
        }
    }
}

impl Element {
    pub fn new(name: &str, element_type: ElementType) -> Self {
        Element {
            element_type,
            prefix: None,
            xml_name: None,
            name: name.to_string(),
            static_content: None,
            children: vec![],
            children_idx: HashMap::new(),
            namespaces: vec![],
            optional: false,
            field_type: None,
            vector: false,
            flatten: false,
            comment: None,
            function_args: None,
        }
    }

    pub fn new_field(field_name: &str, xml_name: &str, field_type: &str, optional: bool) -> Self {
        Element {
            element_type: ElementType::Field,
            prefix: None,
            xml_name: Option::Some(xml_name.to_string()),
            namespaces: vec![],
            name: field_name.to_string(),
            children: vec![],
            children_idx: HashMap::new(),
            static_content: None,
            optional,
            field_type: Option::Some(field_type.to_string()),
            vector: false,
            flatten: false,
            comment: None,
            function_args: None,
        }
    }

    pub fn new_module(module_name: &str) -> Self {
        Element::new(module_name, ElementType::Module)
    }

    pub fn new_function(function_name: &str, input_name: &str, input_type: &str) -> Self {
        let mut e = Element::new(function_name, ElementType::Function);
        e.function_args = Option::Some(FunctionArgs {
            input_type: input_type.to_string(),
            input_name: input_name.to_string(),
            output_type: None,
            fault_type: None,
            comment: None,
        });
        e
    }

    fn render_root(&self) -> String {
        self.children.iter().map(|c| c.borrow().render()).collect()
    }

    fn render_struct(&self) -> String {
        let mut result = if let Some(comment) = &self.comment {
            format!("//* {}\n */", comment)
        } else {
            "".to_string()
        };

        result.push_str("#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]\n");

        let mut has_options = false;

        let mut options = "#[yaserde(\n".to_string();

        if let Some(xml_name) = &self.xml_name {
            options.push_str(&format!("\troot = \"{}\",\n", xml_name));
            has_options = true;
        }

        for namespace in &self.namespaces {
            options.push_str(&format!("\tnamespace = \"{}\",\n", namespace));
            has_options = true;
        }

        if let Some(prefix) = &self.prefix {
            options.push_str(&format!("\tprefix = \"{}\",\n", prefix));
            has_options = true;
        }

        if self.flatten {
            options.push_str("\tflatten, \n");
            has_options = true;
        }

        options.push_str(")]\n");

        if has_options {
            result.push_str(&options);
        }

        result.push_str(&format!("pub struct {} {{\n", self.name));

        if self.has_children() {
            let r: String = self.children.iter().map(|c| c.borrow().render()).collect();
            result.push_str(&r);
        }

        result.push_str("}\n");
        result
    }

    fn render_trait(&self) -> String {
        let mut result = format!("#[async_trait]\npub trait {0} {{\n", self.name);
        let r: String = self.children.iter().map(|c| c.borrow().render()).collect();
        result.push_str(&r);
        result.push_str("}\n");
        result
    }

    fn render_trait_impl(&self) -> String {
        let field_type = match &self.field_type {
            None => return String::new(),
            Some(ft) => ft,
        };

        let mut result = format!(
            "#[async_trait]\n\timpl {0} for {1} {{\n",
            field_type, self.name
        );

        let r: String = self.children.iter().map(|c| c.borrow().render()).collect();
        result.push_str(&r);
        result.push_str("}\n");
        result
    }

    fn render_field(&self) -> String {
        let flatten = if self.flatten { "flatten, " } else { "" };
        let comment = match &self.comment {
            None => "".to_string(),
            Some(c) => format!("// {}", c),
        };

        let prefix = match &self.prefix {
            Some(p) => format!("prefix = \"{}\", ", p),
            None => "".to_string(),
        };

        if let Some(xml_name) = &self.xml_name {
            format!(
                "\t#[yaserde(rename = \"{0}\", {3}{5}default)]\n\tpub {1}: {2}, {4}\n",
                xml_name,
                self.name,
                self.render_field_type(),
                flatten,
                comment,
                prefix,
            )
        } else {
            format!(
                "\t#[yaserde({2}{4}default)]\n\tpub {0}: {1}, {3}\n",
                self.name,
                self.render_field_type(),
                flatten,
                comment,
                prefix
            )
        }
    }

    fn render_atribute(&self) -> String {
        let field_type = match &self.field_type {
            None => return String::new(),
            Some(ft) => ft,
        };

        let xml_name = match &self.xml_name {
            None => return String::new(),
            Some(xn) => xn,
        };

        let prefix = match &self.prefix {
            Some(p) => format!("prefix = \"{}\", ", p),
            None => "".to_string(),
        };

        if self.optional {
            format!(
                "#[yaserde({}rename=\"{}\", attribute)]\npub {}: Option<{}>,\n",
                prefix, xml_name, self.name, field_type
            )
        } else {
            format!(
                "#[yaserde({}rename=\"{}\", attribute)]\npub {}: {},\n",
                prefix, xml_name, self.name, field_type
            )
        }
    }

    fn render_field_type(&self) -> String {
        if let Some(field_type) = &self.field_type {
            if self.vector {
                format!("Vec<{}>", field_type)
            } else if self.optional {
                format!("Option<{}>", field_type)
            } else {
                field_type.to_string()
            }
        } else {
            "".to_string()
        }
    }

    fn render_static(&self) -> String {
        match &self.static_content {
            None => String::new(),
            Some(c) => {
                if let Some(comment) = &self.comment {
                    format!("/** {}\n */\n{}", comment, c)
                } else {
                    c.clone()
                }
            }
        }
    }

    fn render_alias(&self) -> String {
        if let Some(field_type) = &self.field_type {
            format!("pub type {} = {};\n\n", self.name, field_type)
        } else {
            String::new()
        }
    }

    fn render_module(&self) -> String {
        let mut result = format!("pub mod {} {{\n", self.name);
        result.push_str(
            r#"use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            "#,
        );

        let child_content: String = self.children.iter().map(|c| c.borrow().render()).collect();
        result.push_str(child_content.as_str());

        result.push_str("}\n\n");
        result
    }

    fn render_function(&self) -> String {
        let args = match &self.function_args {
            None => return String::new(),
            Some(a) => a,
        };

        let function_result = match &args.fault_type {
            None => match &args.output_type {
                None => String::new(),
                Some(o) => format!("-> {}", o),
            },
            Some(fault) => match &args.output_type {
                None => format!("-> {}", fault),
                Some(o) => format!("-> Result<{},{}>", o, fault),
            },
        };

        let function_input = format!("{}: {}", args.input_name, args.input_type);

        format!(
            "\tasync fn {} (&self, {}) {};\n",
            self.name, function_input, function_result
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_struct() {
        let expected = r#"#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {
}
"#;
        let header = Element::new("Header", ElementType::Struct);
        assert_eq!(header.render(), expected.to_string());
    }

    #[test]
    fn test_struct_with_fields() {
        let expected = r#"#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
}
"#;

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
        assert_eq!(soap_fault.render(), expected.to_string())
    }

    #[test]
    fn test_static_element() {
        let expected = r#"
            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde::{YaSerialize, YaDeserialize};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            "#
        .to_string();

        let mut global_header = Element::new("global_header", ElementType::Static);
        global_header.set_content("");
        global_header.append_content(
            r#"
            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde::{YaSerialize, YaDeserialize};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            "#,
        );

        assert_eq!(global_header.render(), expected);
    }

    #[test]
    fn test_alias() {
        let expected = r#"pub type SomeElement = other_mod::SomeElement;

"#
        .to_string();
        let mut alias_element = Element::new("SomeElement", ElementType::Alias);
        alias_element.field_type = Option::Some("other_mod::SomeElement".to_string());
        assert_eq!(alias_element.render(), expected)
    }
}
