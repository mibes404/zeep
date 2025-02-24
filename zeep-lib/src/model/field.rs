use super::{Namespace, TryFromNode, doc::RustDocument};
use crate::{
    error::{WriterError, WriterResult},
    reader::WriteXml,
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
use roxmltree::Node;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Field {
    pub xml_name: String,
    pub rust_name: String,
    pub rust_type: RustFieldType,
    pub is_optional: bool,
    pub is_vec: bool,
    pub target_namespace: Option<Rc<Namespace>>,
    pub is_attribute: bool,
    pub is_choice: bool,
    pub is_any: bool,
}

impl<'n> TryFromNode<'n> for Field {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> WriterResult<Self> {
        if !node.is_element() {
            return Err(WriterError::NotAnElement);
        }

        let mut target_namespace = None;
        if let Some(use_target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(use_target_namespace);
        }

        target_namespace.clone_from(&doc.current_target_namespace);

        let is_attribute = node.tag_name().name() == "attribute";

        let is_optional = if is_attribute {
            node.attribute("use") != Some("required")
        } else {
            node.attribute("minOccurs") == Some("0")
        };

        let is_vec = Node::attribute(&node, "maxOccurs") == Some("unbounded");
        let is_choice = node.parent().is_some_and(|n| n.tag_name().name() == "choice");

        // check if this is an any type
        if node.tag_name().name() == "any" {
            return Ok(Field {
                xml_name: "body".to_string(),
                rust_name: "body".to_string(),
                rust_type: RustFieldType::String,
                is_optional: true,
                is_vec: false,
                target_namespace: None,
                is_attribute: false,
                is_choice: false,
                is_any: true,
            });
        }

        if let Some(ref_name) = node.attribute("ref") {
            let (xml_name, namespace_ref) = split_type(ref_name);
            let rust_name = rename_keywords(&to_snake_case(xml_name)).to_string();

            if ref_name.starts_with("xml") {
                return Ok(Field {
                    xml_name: xml_name.to_string(),
                    rust_name: rust_name.to_string(),
                    rust_type: RustFieldType::String,
                    is_optional: false,
                    is_vec: false,
                    target_namespace: None,
                    is_attribute: false,
                    is_choice: false,
                    is_any: false,
                });
            }

            let namespace: Option<Rc<Namespace>> = namespace_ref
                .and_then(|ns| doc.find_namespace_by_abbreviation(ns))
                .cloned();

            let ref_node = doc.find_node_by_xml_name(&node, xml_name, namespace.as_deref());
            let ref_node = ref_node
                .as_ref()
                .ok_or_else(|| WriterError::NodeNotFound(ref_name.to_string()))?;

            let is_choice = node.parent().is_some_and(|n| n.tag_name().name() == "choice");
            let module = namespace.map(|n| n.rust_mod_name.clone());

            let xml_name = ref_node.xml_name().ok_or(WriterError::InvalidReference)?;
            let rust_type = RustFieldType::Other(OtherRustType {
                name: to_pascal_case(xml_name),
                module,
            });

            return Ok(Field {
                xml_name: xml_name.to_string(),
                rust_name,
                rust_type,
                is_optional: false,
                is_vec: false,
                target_namespace: None,
                is_attribute: false,
                is_choice,
                is_any: false,
            });
        }

        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::attribute_missing(&node, "name"))?
            .to_string();

        let rust_name = rename_keywords(&to_snake_case(&xml_name)).to_string();

        let rust_type = node
            .attribute("type")
            .map_or(RustFieldType::String, |t| as_rust_type(t, doc));

        Ok(Field {
            xml_name,
            rust_name,
            rust_type,
            is_optional,
            is_vec,
            target_namespace,
            is_attribute,
            is_choice,
            is_any: false,
        })
    }
}

impl<W> WriteXml<W> for Field
where
    W: std::io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        let possibly_optional_field = if self.is_vec {
            format!("Vec<{}>", self.rust_type)
        } else if self.is_optional {
            format!("Option<{}>", self.rust_type)
        } else {
            self.rust_type.to_string()
        };

        let attribute_header = if self.is_attribute { ", attribute = true" } else { "" };

        if let Some(tns) = &self.target_namespace {
            writeln!(
                writer,
                "    #[yaserde(prefix = \"{}\", rename = \"{}\"{attribute_header})]",
                tns.abbreviation, self.xml_name
            )?;
        } else {
            writeln!(
                writer,
                "    #[yaserde(rename = \"{}\"{attribute_header})]",
                self.xml_name
            )?;
        }

        writeln!(writer, "    pub {}: {},", self.rust_name, possibly_optional_field)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum RustFieldType {
    #[default]
    String,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    F32,
    F64,
    Bool,
    Other(OtherRustType),
    U64,
    U32,
}

impl RustFieldType {
    pub fn is_string(&self) -> bool {
        matches!(self, RustFieldType::String)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, RustFieldType::Other(_))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OtherRustType {
    pub name: String,
    pub module: Option<String>,
}

impl OtherRustType {
    pub fn new(name: String, module: Option<String>) -> Self {
        OtherRustType { name, module }
    }
}

impl Display for RustFieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustFieldType::String => write!(f, "String"),
            RustFieldType::I8 => write!(f, "i8"),
            RustFieldType::I16 => write!(f, "i16"),
            RustFieldType::I32 => write!(f, "i32"),
            RustFieldType::I64 => write!(f, "i64"),
            RustFieldType::U8 => write!(f, "u8"),
            RustFieldType::U16 => write!(f, "u16"),
            RustFieldType::U32 => write!(f, "u32"),
            RustFieldType::U64 => write!(f, "u64"),
            RustFieldType::F32 => write!(f, "f32"),
            RustFieldType::F64 => write!(f, "f64"),
            RustFieldType::Bool => write!(f, "bool"),
            RustFieldType::Other(OtherRustType { name, module }) => {
                if let Some(module) = module {
                    write!(f, "{module}::{name}")
                } else {
                    write!(f, "{name}")
                }
            }
        }
    }
}

fn split_type(node_type: &str) -> (&str, Option<&str>) {
    // split once. If there is a target namespace, it will be in the second part
    if let Some((abbreviation, type_name)) = node_type.split_once(':') {
        (type_name, Some(abbreviation))
    } else {
        (node_type, None)
    }
}

pub fn resolve_type<'n>(node_type: &'n str, doc: &RustDocument) -> (&'n str, Option<Rc<Namespace>>) {
    let (node_type, namespace) = split_type(node_type);
    let namespace = namespace.and_then(|ns| doc.find_namespace_by_abbreviation(ns));
    (node_type, namespace.cloned())
}

pub fn as_rust_type(node_type: &str, doc: &RustDocument) -> RustFieldType {
    let (node_type, namespace) = split_type(node_type);

    match node_type {
        "byte" => RustFieldType::I8,
        "string" | "normalizedString" | "base64Binary" | "hexBinary" | "anyURI" | "date" | "dateTime" | "time"
        | "language" | "duration" => RustFieldType::String,
        "decimal" | "double" => RustFieldType::F64,
        "float" => RustFieldType::F32,
        "integer" | "int" | "negativeInteger" | "nonNegativeInteger" | "nonPositiveInteger" | "positiveInteger" => {
            RustFieldType::I32
        }
        "long" => RustFieldType::I64,
        "unsignedLong" => RustFieldType::U64,
        "unsignedInt" => RustFieldType::U32,
        "unsignedShort" => RustFieldType::U16,
        "unsignedByte" => RustFieldType::U8,
        "short" => RustFieldType::I16,
        "boolean" => RustFieldType::Bool,
        v => RustFieldType::Other(OtherRustType {
            name: to_pascal_case(v),
            module: namespace.and_then(|ns| {
                doc.find_module_name_from_namespace_reference(ns)
                    .map(ToString::to_string)
            }),
        }),
    }
}

pub fn as_field_name(xml_name: &str) -> String {
    let field_name = to_snake_case(xml_name);
    rename_keywords(&field_name).to_string()
}

/// renamed the Rust keyword and quote the field name
pub fn rename_keywords(field_name: &str) -> &str {
    match field_name {
        "type" => "r#type",
        "as" => "r#as",
        "where" => "r#where",
        "break" => "r#break",
        "override" => "r#override",
        "continue" => "r#continue",
        "crate" => "r#crate",
        "else" => "r#else",
        "enum" => "r#enum",
        "extern" => "r#extern",
        "false" => "r#false",
        "true" => "r#true",
        "fn" => "r#fn",
        "for" => "r#for",
        "if" => "r#if",
        "impl" => "r#impl",
        "in" => "r#in",
        "let" => "r#let",
        "loop" => "r#loop",
        "match" => "r#match",
        "mod" => "r#mod",
        "move" => "r#move",
        "mut" => "r#mut",
        "pub" => "r#pub",
        "ref" => "r#ref",
        "return" => "r#return",
        "self" => "r#self",
        _ => field_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::field::{Field, RustFieldType};

    #[test]
    fn can_parse_field_from_node() {
        const STRING_FIELD: &str = r#"<element name="Id" type="string" minOccurs="0"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let mut rust_doc = RustDocument::init(&doc);
        let field = Field::try_from_node(node, &mut rust_doc).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                is_optional: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn can_parse_attribute_from_node() {
        const STRING_FIELD: &str = r#"<xs:attribute name="HeaderName" type="xs:string" use="required" xmlns:xs="http://www.w3.org/2001/XMLSchema"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let mut rust_doc = RustDocument::init(&doc);
        let field = Field::try_from_node(node, &mut rust_doc).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "HeaderName".to_string(),
                rust_name: "header_name".to_string(),
                rust_type: RustFieldType::String,
                is_attribute: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn can_parse_field_from_node_with_namespace() {
        const STRING_FIELD: &str =
            r#"<xs:element name="Id" type="xs:string" minOccurs="0" xmlns:xs="http://www.w3.org/2001/XMLSchema"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let mut rust_doc = RustDocument::init(&doc);
        let field = Field::try_from_node(node, &mut rust_doc).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                is_optional: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn can_parse_field_from_node_with_tns() {
        const STRING_FIELD: &str = r#"<element name="Id" type="string" minOccurs="0" targetNamespace="http://schemas.microsoft.com/exchange/services/2006/types"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let mut rust_doc = RustDocument::init(&doc);
        let field = Field::try_from_node(node, &mut rust_doc).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                is_optional: true,
                target_namespace: Some(Rc::new(Namespace {
                    namespace: "http://schemas.microsoft.com/exchange/services/2006/types".to_string(),
                    abbreviation: "typ".to_string(),
                    rust_mod_name: "mod_typ".to_string(),
                })),
                ..Default::default()
            }
        );
    }

    #[test]
    fn can_write_field_in_rust() {
        let field = Field {
            xml_name: "Id".to_string(),
            rust_name: "id".to_string(),
            rust_type: RustFieldType::String,
            ..Default::default()
        };
        let mut buffer = Vec::new();
        field.write_xml(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "    #[yaserde(rename = \"Id\")]\n    pub id: String,\n"
        );
    }

    #[test]
    fn can_write_optional_field_in_rust() {
        let field = Field {
            xml_name: "Id".to_string(),
            rust_name: "id".to_string(),
            rust_type: RustFieldType::String,
            is_optional: true,
            ..Default::default()
        };
        let mut buffer = Vec::new();
        field.write_xml(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "    #[yaserde(rename = \"Id\")]\n    pub id: Option<String>,\n"
        );
    }

    #[test]
    fn can_write_vec_field_in_rust() {
        let field = Field {
            xml_name: "Id".to_string(),
            rust_name: "id".to_string(),
            rust_type: RustFieldType::String,
            is_optional: true,
            is_vec: true,
            ..Default::default()
        };
        let mut buffer = Vec::new();
        field.write_xml(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "    #[yaserde(rename = \"Id\")]\n    pub id: Vec<String>,\n"
        );
    }

    #[test]
    fn can_write_attribute_in_rust() {
        let field = Field {
            xml_name: "HeaderName".to_string(),
            rust_name: "header_name".to_string(),
            rust_type: RustFieldType::String,
            is_attribute: true,
            ..Default::default()
        };

        let mut buffer = Vec::new();
        field.write_xml(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "    #[yaserde(rename = \"HeaderName\", attribute = true)]\n    pub header_name: String,\n"
        );
    }
}
