use super::TargetNamespace;
use crate::{
    error::{WriterError, WriterResult},
    model::{doc::RustDocument, split_type, TryFromNode, WriteNode},
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
use roxmltree::Node;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub xml_name: String,
    pub rust_name: String,
    pub rust_type: RustFieldType,
    pub optional: bool,
    pub vec: bool,
    pub target_namespace: Option<Rc<TargetNamespace>>,
}

impl<'n> TryFromNode<'n> for Field {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> WriterResult<Self> {
        if !node.is_element() {
            return Err(WriterError::NotAnElement);
        }

        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
        }

        if let Some(ref_name) = node.attribute("ref") {
            let xml_name = split_type(ref_name);
            let ref_node = doc
                .find_node_by_xml_name(xml_name)
                .and_then(|n| n.rust_type.try_as_element())
                .ok_or_else(|| WriterError::NodeNotFound(ref_name.to_string()))?;

            let rust_name = rename_keywords(&to_snake_case(xml_name)).to_string();

            return Ok(Field {
                xml_name: ref_node.xml_name.clone(),
                rust_name,
                rust_type: ref_node.rust_type.clone(),
                optional: false,
                vec: false,
                target_namespace: None,
            });
        }

        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        let rust_name = rename_keywords(&to_snake_case(&xml_name)).to_string();
        let rust_type = node
            .attribute("type")
            .map(as_rust_type)
            .ok_or_else(|| WriterError::AttributeMissing("type".to_string()))?;
        let optional = node.attribute("minOccurs") == Some("0");
        let vec = Node::attribute(&node, "maxOccurs") == Some("unbounded");

        Ok(Field {
            xml_name,
            rust_name,
            rust_type,
            optional,
            vec,
            target_namespace: doc.current_target_namespace.clone(),
        })
    }
}

impl<W> WriteNode<W> for Field
where
    W: std::io::Write,
{
    fn write_node(&self, writer: &mut W) -> WriterResult<()> {
        let possibly_vec_field = if self.vec {
            format!("Vec<{}>", self.rust_type)
        } else {
            self.rust_type.to_string()
        };

        let possibly_optional_field = if self.optional {
            format!("Option<{possibly_vec_field}>")
        } else {
            possibly_vec_field
        };

        if let Some(tns) = &self.target_namespace {
            let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
            writeln!(
                writer,
                "    #[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
                tns.abbreviation, namespaces, self.xml_name
            )?;
        } else {
            writeln!(writer, "    #[yaserde(rename = \"{}\")]", self.xml_name)?;
        }

        writeln!(writer, "    pub {}: {},", self.rust_name, possibly_optional_field)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RustFieldType {
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
    Other(String),
    U64,
    U32,
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
            RustFieldType::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<&str> for RustFieldType {
    fn from(value: &str) -> Self {
        as_rust_type(value)
    }
}

fn as_rust_type(node_type: &str) -> RustFieldType {
    match split_type(node_type) {
        "byte" => RustFieldType::I8,
        "string" | "normalizedString" | "base64Binary" | "hexBinary" | "anyURI" | "date" | "dateTime" | "time"
        | "language" => RustFieldType::String,
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
        v => RustFieldType::Other(to_pascal_case(v)),
    }
}

/// renamed the Rust keyword and quote the field name
fn rename_keywords(field_name: &str) -> &str {
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
        let field = Field::try_from_node(node, &mut RustDocument::new()).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                optional: true,
                vec: false,
                target_namespace: None,
            }
        );
    }

    #[test]
    fn can_parse_field_from_node_with_namespace() {
        const STRING_FIELD: &str =
            r#"<xs:element name="Id" type="xs:string" minOccurs="0" xmlns:xs="http://www.w3.org/2001/XMLSchema"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let field = Field::try_from_node(node, &mut RustDocument::new()).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                optional: true,
                vec: false,
                target_namespace: None
            }
        );
    }

    #[test]
    fn can_parse_field_from_node_with_tns() {
        const STRING_FIELD: &str = r#"<element name="Id" type="string" minOccurs="0" targetNamespace="http://schemas.microsoft.com/exchange/services/2006/types"/>"#;
        let doc = roxmltree::Document::parse(STRING_FIELD).unwrap();
        let node = doc.root_element();
        let field = Field::try_from_node(node, &mut RustDocument::new()).unwrap();
        assert_eq!(
            field,
            Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                optional: true,
                vec: false,
                target_namespace: Some(Rc::new(TargetNamespace {
                    namespace: "http://schemas.microsoft.com/exchange/services/2006/types".to_string(),
                    abbreviation: "typ".to_string()
                }))
            }
        );
    }

    #[test]
    fn can_write_field_in_rust() {
        let field = Field {
            xml_name: "Id".to_string(),
            rust_name: "id".to_string(),
            rust_type: RustFieldType::String,
            optional: false,
            vec: false,
            target_namespace: None,
        };
        let mut buffer = Vec::new();
        field.write_node(&mut buffer).unwrap();
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
            optional: true,
            vec: false,
            target_namespace: None,
        };
        let mut buffer = Vec::new();
        field.write_node(&mut buffer).unwrap();
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
            optional: true,
            vec: true,
            target_namespace: None,
        };
        let mut buffer = Vec::new();
        field.write_node(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "    #[yaserde(rename = \"Id\")]\n    pub id: Option<Vec<String>>,\n"
        );
    }
}
