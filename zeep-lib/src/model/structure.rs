use super::{field::RustFieldType, TryFromNode};
use crate::{
    error::{WriterError, WriterResult},
    model::{doc::RustDocument, field::Field, TargetNamespace, WriteNode},
};
use roxmltree::Node;
use std::{io, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct StructProps {
    pub xml_name: String,
    pub fields: Vec<Field>,
    pub target_namespace: Option<Rc<TargetNamespace>>,
}

#[derive(Debug, PartialEq)]
pub struct SimpleProps {
    pub xml_name: String,
    pub rust_type: RustFieldType,
    pub target_namespace: Option<Rc<TargetNamespace>>,
}

#[derive(Debug, PartialEq)]
pub enum RustType {
    Ignore,
    Struct(StructProps),
    Simple(SimpleProps),
}

impl<'n> TryFromNode<'n> for StructProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        fn read_sequence_node(element_name: &str, node: Node, doc: &mut RustDocument) -> WriterResult<StructProps> {
            let fields = node
                .children()
                .filter(Node::is_element)
                .map(|n| Field::try_from_node(n, doc))
                .collect::<WriterResult<Vec<Field>>>()?;

            let struct_props = StructProps {
                xml_name: element_name.to_string(),
                fields,
                target_namespace: doc.current_target_namespace.clone(),
            };

            Ok(struct_props)
        }

        let element_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?;

        for n in node.children().filter(Node::is_element) {
            if n.tag_name().name() == "sequence" {
                return read_sequence_node(element_name, n, doc);
            }
        }

        Err(WriterError::UnsupportedXsdType(element_name.to_string()))
    }
}

impl<'n> TryFromNode<'n> for SimpleProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        // check if the node has restriction child
        let rust_type = if let Some(restriction) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "restriction")
        {
            restriction
                .attribute("base")
                .map(RustFieldType::from)
                .ok_or_else(|| WriterError::AttributeMissing("base".to_string()))?
        } else {
            // see if this is a list
            if let Some(list) = node
                .children()
                .find(|n| n.is_element() && n.tag_name().name() == "list")
            {
                // list can have itemType attribute or a nested simpleType
                if let Some(item_type) = list.attribute("itemType").map(RustFieldType::from) {
                    item_type
                } else {
                    let simple_type = list
                        .children()
                        .find(|n| n.is_element() && n.tag_name().name() == "simpleType")
                        .ok_or_else(|| WriterError::UnsupportedXsdType("list".to_string()))?;

                    return Self::try_from_node(simple_type, doc);
                }
            } else {
                // check if this is a union type
                if let Some(list) = node
                    .children()
                    .find(|n| n.is_element() && n.tag_name().name() == "union")
                {
                    // todo: at the moment we don't support union types, so we just include the first type
                    let simple_type = list
                        .children()
                        .find(|n| n.is_element() && n.tag_name().name() == "simpleType")
                        .ok_or_else(|| WriterError::UnsupportedXsdType("list".to_string()))?;

                    return Self::try_from_node(simple_type, doc);
                }

                return Err(WriterError::UnsupportedXsdType(xml_name));
            }
        };

        Ok(SimpleProps {
            xml_name,
            rust_type,
            target_namespace: doc.current_target_namespace.clone(),
        })
    }
}

impl<W> WriteNode<W> for RustType
where
    W: io::Write,
{
    fn write_node(&self, writer: &mut W) -> WriterResult<()> {
        match self {
            RustType::Ignore => Ok(()),
            RustType::Struct(StructProps {
                xml_name,
                fields,
                target_namespace,
            }) => {
                writeln!(writer, "#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]")?;
                if let Some(tns) = &target_namespace {
                    let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
                    writeln!(
                        writer,
                        "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
                        tns.abbreviation, namespaces, xml_name
                    )?;
                }
                writeln!(writer, "struct {xml_name} {{")?;
                for field in fields {
                    field.write_node(writer)?;
                }

                write!(writer, "}}")?;
                Ok(())
            }
            RustType::Simple(SimpleProps {
                xml_name,
                rust_type,
                target_namespace: _,
            }) => {
                // for now write this as a type alias; we may want to change this to a newtype
                // in the future
                writeln!(writer, "type {xml_name} = {rust_type};")?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::field::RustFieldType;

    #[test]
    fn can_write_a_struct_type_to_rust() {
        let mut writer = Vec::new();
        let props = prep_struct_props(None);
        let rust_type = RustType::Struct(props);
        rust_type.write_node(&mut writer).unwrap();

        let expected = r#"#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
struct Person {
    #[yaserde(rename = "name")]
    pub name: String,
    #[yaserde(rename = "age")]
    pub age: i32,
}"#;
        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    #[test]
    fn can_write_a_struct_type_with_namespace_to_rust() {
        let mut writer = Vec::new();
        let props = prep_struct_props(Some(Rc::new(TargetNamespace {
            namespace: "http://example.com".to_string(),
            abbreviation: "ex".to_string(),
        })));
        let rust_type = RustType::Struct(props);
        rust_type.write_node(&mut writer).unwrap();

        let expected = r#"#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "Person")]
struct Person {
    #[yaserde(rename = "name")]
    pub name: String,
    #[yaserde(rename = "age")]
    pub age: i32,
}"#;
        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    fn prep_struct_props(target_namespace: Option<Rc<TargetNamespace>>) -> StructProps {
        StructProps {
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
        }
    }
}
