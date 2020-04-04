use std::collections::HashMap;

pub enum ElementType {
    Root,
    Struct,
    Field,
    Static,
}

pub struct Element {
    pub element_type: ElementType,
    pub prefix: Option<String>,
    pub xml_name: Option<String>,
    pub namespace: Option<String>,
    pub name: String,
    pub children: Vec<Element>,
    pub children_idx: HashMap<String, usize>,
    static_content: Option<String>,
    pub optional: bool,
    pub field_type: Option<String>,
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
        namespace: None,
        optional: false,
        field_type: None,
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
    fn child(&mut self, name: String) -> Option<&mut Element>;
    fn has_children(&self) -> bool;
}

impl WritableElement for Element {
    fn render(&self) -> String {
        match self.element_type {
            ElementType::Root => "".to_string(),
            ElementType::Struct => self.render_struct(),
            ElementType::Field => self.render_field(),
            ElementType::Static => self.render_static(),
        }
    }
}

impl StaticElement for Element {
    fn set_content(&mut self, content: &str) {
        self.static_content = Option::from(content.to_string());
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
        self.children.push(child);
        self.children_idx.insert(name, pos);
    }

    fn child(&mut self, name: String) -> Option<&mut Element> {
        if let Some(pos) = self.children_idx.get(&name) {
            self.children.get_mut(*pos)
        } else {
            None
        }
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
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
            namespace: None,
            optional: false,
            field_type: None,
        }
    }

    pub fn new_field(field_name: &str, xml_name: &str, field_type: &str, optional: bool) -> Self {
        Element {
            element_type: ElementType::Field,
            prefix: None,
            xml_name: Option::from(xml_name.to_string()),
            namespace: None,
            name: field_name.to_string(),
            children: vec![],
            children_idx: HashMap::new(),
            static_content: None,
            optional,
            field_type: Option::from(field_type.to_string()),
        }
    }

    fn render_struct(&self) -> String {
        let mut result = "#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n".to_string();

        let mut has_options = false;

        let mut options = "#[yaserde(\n".to_string();

        if let Some(xml_name) = &self.xml_name {
            options.push_str(&format!("\troot = \"{}\",\n", xml_name));
            has_options = true;
        }

        if let Some(namespace) = &self.namespace {
            options.push_str(&format!("\tnamespace = \"{}\",\n", namespace));
            has_options = true;
        }

        if let Some(prefix) = &self.prefix {
            options.push_str(&format!("\tprefix = \"{}\",\n", prefix));
            has_options = true;
        }

        options.push_str(")]\n");

        if has_options {
            result.push_str(&options);
        }

        result.push_str(&format!("pub struct {} {{\n", self.name));

        if self.has_children() {
            let r: String = self.children.iter().map(|c| c.render()).collect();
            result.push_str(&r);
        }

        result.push_str("}\n");
        result
    }

    fn render_field(&self) -> String {
        if let Some(xml_name) = &self.xml_name {
            format!(
                "\t#[yaserde(rename = \"{0}\", default)]\n\tpub {1}: {2},\n",
                xml_name,
                self.name,
                self.render_field_type()
            )
        } else {
            format!(
                "\t#[yaserde(default)]\n\tpub {0}: {1},\n",
                self.name,
                self.render_field_type()
            )
        }
    }

    fn render_field_type(&self) -> String {
        if let Some(field_type) = &self.field_type {
            if self.optional {
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
            Some(c) => c.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_struct() {
        let expected = r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Header {
}
"#;
        let header = Element::new("Header", ElementType::Struct);
        assert_eq!(header.render(), expected.to_string());
    }

    #[test]
    fn test_struct_with_fields() {
        let expected = r#"#[derive(Debug, Default, YaSerialize, YaDeserialize)]
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
        soap_fault.xml_name = Option::from("Fault".to_string());
        soap_fault.namespace =
            Option::from("soapenv: http://schemas.xmlsoap.org/soap/envelope/".to_string());
        soap_fault.prefix = Option::from("soapenv".to_string());
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
            use yaserde::{{YaSerialize, YaDeserialize}};
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
            use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            "#,
        );

        assert_eq!(global_header.render(), expected);
    }
}
