pub mod complex;
pub mod element;
pub mod restrictions;
pub mod simple;

use super::{TryFromNode, field::RustFieldType};
use crate::{
    error::{WriterError, WriterResult},
    model::{Namespace, doc::RustDocument, field::Field},
    reader::WriteXml,
};
use complex::ComplexProps;
use element::{ElementProps, ElementType};
use inflector::cases::pascalcase::to_pascal_case;
use roxmltree::Node;
use simple::SimpleProps;
use std::{io, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum RustType {
    Ignore,
    Complex(Box<ComplexProps>),
    Simple(Box<SimpleProps>),
    Element(Box<ElementProps>),
}

impl RustType {
    pub fn xml_name(&self) -> Option<&str> {
        match self {
            RustType::Complex(props) => Some(&props.xml_name),
            RustType::Simple(props) => Some(&props.xml_name),
            RustType::Element(props) => Some(&props.xml_name),
            RustType::Ignore => None,
        }
    }

    pub fn try_as_element(&self) -> Option<&ElementProps> {
        match self {
            RustType::Element(props) => Some(&**props),
            _ => None,
        }
    }
}

/// check for documentation
fn parse_comment<'n>(node: Node<'n, 'n>) -> Option<String> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == "annotation")
        .and_then(|n| {
            n.children()
                .find(|n| n.is_element() && n.tag_name().name() == "documentation")
                .and_then(|n| n.text())
        })
        .map(|s| {
            // strip all whitespace and newlines from start and end
            s.trim().to_string()
        })
}

pub fn xml_name_to_rust_name(xml_name: &str) -> String {
    to_pascal_case(xml_name)
}

impl<W> WriteXml<W> for RustType
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        match self {
            RustType::Ignore => Ok(()),
            RustType::Complex(props) => write_complex_type(writer, props),
            RustType::Simple(props) => {
                let SimpleProps {
                    xml_name,
                    rust_type,
                    target_namespace: _,
                    restrictions: _,
                    comment,
                } = &**props;

                let rust_name = xml_name_to_rust_name(xml_name);
                if rust_type.to_string().eq(&rust_name) {
                    // NOOP
                    return Ok(());
                }

                // for now, write this as a type alias; we may want to change this to a newtype
                // in the future
                if let Some(comment) = comment {
                    writeln!(writer, "/// {comment}")?;
                }

                writeln!(writer, "pub type {rust_name} = {rust_type};")?;
                Ok(())
            }
            RustType::Element(props) => {
                let ElementProps { xml_name, element_type } = &**props;
                let rust_name = xml_name_to_rust_name(xml_name);

                match element_type {
                    ElementType::RustType(rust_type) => {
                        if rust_type.to_string().eq(&rust_name) {
                            // NOOP
                            return Ok(());
                        }

                        writeln!(writer, "pub type {rust_name} = {rust_type};")?;
                    }
                    ElementType::ComplexType(props) => {
                        write_complex_type(writer, props)?;
                    }
                    ElementType::Unsupported => {
                        // NOOP
                    }
                }

                Ok(())
            }
        }
    }
}

fn write_complex_type<W>(writer: &mut W, props: &ComplexProps) -> WriterResult<()>
where
    W: io::Write,
{
    let ComplexProps {
        xml_name,
        fields,
        target_namespace,
        comment,
    } = &props;

    let rust_name = xml_name_to_rust_name(xml_name);

    if let Some(comment) = comment {
        writeln!(writer, "/// {comment}")?;
    }

    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    if let Some(tns) = &target_namespace {
        let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
        writeln!(
            writer,
            "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
            tns.abbreviation, namespaces, xml_name
        )?;
    }
    writeln!(writer, "pub struct {rust_name} {{")?;
    for field in fields {
        field.write_xml(writer)?;
    }

    writeln!(writer, "}}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::field::RustFieldType;

    #[test]
    fn can_write_a_struct_type_to_rust() {
        let mut writer = Vec::new();
        let props = prep_struct_props(None);
        let rust_type = RustType::Complex(props.into());
        rust_type.write_xml(&mut writer).unwrap();

        let expected = r#"/// A person
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Person {
    #[yaserde(rename = "name")]
    pub name: String,
    #[yaserde(rename = "age")]
    pub age: i32,
}
"#;
        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    #[test]
    fn can_write_a_struct_type_with_namespace_to_rust() {
        const EXPECTED: &str = r#"/// A person
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "Person")]
pub struct Person {
    #[yaserde(rename = "name")]
    pub name: String,
    #[yaserde(rename = "age")]
    pub age: i32,
}
"#;

        let mut writer = Vec::new();
        let props = prep_struct_props(Some(Rc::new(Namespace {
            namespace: "http://example.com".to_string(),
            abbreviation: "ex".to_string(),
            rust_mod_name: "mod_ex".to_string(),
        })));
        let rust_type = RustType::Complex(props.into());
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    fn prep_struct_props(target_namespace: Option<Rc<Namespace>>) -> ComplexProps {
        ComplexProps {
            xml_name: "Person".to_string(),
            fields: vec![
                Field {
                    xml_name: "name".to_string(),
                    rust_name: "name".to_string(),
                    rust_type: RustFieldType::String,
                    optional: false,
                    vec: false,
                    target_namespace: None,
                },
                Field {
                    xml_name: "age".to_string(),
                    rust_name: "age".to_string(),
                    rust_type: RustFieldType::I32,
                    optional: false,
                    vec: false,
                    target_namespace: None,
                },
            ],
            target_namespace,
            comment: Some("A person".to_string()),
        }
    }

    #[test]
    fn can_write_a_simple_type_to_rust() {
        const EXPECTED: &str = r"/// A person
pub type Person = String;
";
        let mut writer = Vec::new();
        let props = prep_simple_props(None);
        let rust_type = RustType::Simple(props);
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    fn prep_simple_props(target_namespace: Option<Rc<Namespace>>) -> Box<SimpleProps> {
        SimpleProps {
            xml_name: "Person".to_string(),
            rust_type: RustFieldType::String,
            target_namespace,
            restrictions: None,
            comment: Some("A person".to_string()),
        }
        .into()
    }
}
