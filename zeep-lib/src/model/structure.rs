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
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct SimpleProps {
    pub xml_name: String,
    pub rust_type: RustFieldType,
    pub target_namespace: Option<Rc<TargetNamespace>>,
    pub restrictions: Option<Restrictions>,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Restrictions {
    pub min_inclusive: Option<String>,
    pub max_inclusive: Option<String>,
    pub min_exclusive: Option<String>,
    pub max_exclusive: Option<String>,
    pub total_digits: Option<String>,
    pub fraction_digits: Option<String>,
    pub length: Option<String>,
    pub min_length: Option<String>,
    pub max_length: Option<String>,
    pub enumeration: Option<Vec<String>>,
    pub white_space: Option<String>,
    pub pattern: Option<String>,
    pub acceptable_union_types: Option<Vec<RustFieldType>>,
    pub acceptable_list_type: Option<RustFieldType>,
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
            let comment = parse_comment(node);
            let fields = node
                .children()
                .filter(Node::is_element)
                .map(|n| Field::try_from_node(n, doc))
                .collect::<WriterResult<Vec<Field>>>()?;

            let struct_props = StructProps {
                xml_name: element_name.to_string(),
                fields,
                target_namespace: doc.current_target_namespace.clone(),
                comment,
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

impl<'n> TryFromNode<'n> for SimpleProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        // check for documentation
        let comment = parse_comment(node);

        // check if the node has restriction child
        if let Some(restriction) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "restriction")
        {
            let rust_type = restriction
                .attribute("base")
                .map(RustFieldType::from)
                .ok_or_else(|| WriterError::AttributeMissing("base".to_string()))?;

            let restrictions = build_restrictions(restriction);

            return Ok(SimpleProps {
                xml_name,
                rust_type,
                target_namespace: doc.current_target_namespace.clone(),
                restrictions: Some(restrictions),
                comment,
            });
        }

        // see if this is a list
        if let Some(list) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "list")
        {
            return build_simple_list_type(doc, xml_name, list, comment);
        }

        // check if this is a union type
        if let Some(list) = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "union")
        {
            return build_simple_union_type(doc, xml_name, list, comment);
        }

        // unsupported type
        Err(WriterError::UnsupportedXsdType(xml_name))
    }
}

fn build_simple_union_type<'n>(
    doc: &mut RustDocument,
    xml_name: String,
    list: Node<'n, 'n>,
    comment: Option<String>,
) -> WriterResult<SimpleProps> {
    let rust_type = RustFieldType::String;
    if let Some(member_types) = list.attribute("memberTypes") {
        // split the member types and try to convert them to RustFieldTypes
        let member_types = member_types
            .split_whitespace()
            .map(RustFieldType::from)
            .collect::<Vec<RustFieldType>>();

        let restrictions = Some(Restrictions {
            acceptable_union_types: Some(member_types),
            ..Restrictions::default()
        });

        return Ok(SimpleProps {
            xml_name,
            rust_type,
            target_namespace: doc.current_target_namespace.clone(),
            restrictions,
            comment,
        });
    }

    let simple_types = list
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "simpleType");
    let member_types = simple_types
        .map(|n| {
            n.attribute("base")
                .map(RustFieldType::from)
                .ok_or_else(|| WriterError::AttributeMissing("base".to_string()))
        })
        .collect::<WriterResult<Vec<RustFieldType>>>()?;
    let restrictions = Some(Restrictions {
        acceptable_union_types: Some(member_types),
        ..Restrictions::default()
    });

    Ok(SimpleProps {
        xml_name,
        rust_type,
        target_namespace: doc.current_target_namespace.clone(),
        restrictions,
        comment,
    })
}

fn build_simple_list_type<'n>(
    doc: &mut RustDocument,
    xml_name: String,
    list: Node<'n, 'n>,
    comment: Option<String>,
) -> WriterResult<SimpleProps> {
    let rust_type = RustFieldType::String;
    if let Some(item_type) = list.attribute("itemType").map(RustFieldType::from) {
        let restrictions = Some(Restrictions {
            acceptable_list_type: Some(item_type),
            ..Restrictions::default()
        });

        return Ok(SimpleProps {
            xml_name,
            rust_type,
            target_namespace: doc.current_target_namespace.clone(),
            restrictions,
            comment,
        });
    }

    let simple_type = list
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "simpleType")
        .ok_or_else(|| WriterError::UnsupportedXsdType("list".to_string()))?;
    let restriction = simple_type
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "restriction")
        .ok_or_else(|| WriterError::UnsupportedXsdType("list".to_string()))?;
    let base = restriction
        .attribute("base")
        .map(RustFieldType::from)
        .ok_or_else(|| WriterError::AttributeMissing("base".to_string()))?;
    let mut restrictions = Restrictions {
        acceptable_list_type: Some(base),
        ..Restrictions::default()
    };
    let enumeration = restriction
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "enumeration");
    for enum_i in enumeration {
        if let Some(value) = enum_i.attribute("value") {
            if let Some(enumeration) = &mut restrictions.enumeration {
                enumeration.push(value.to_string());
            }
        }
    }

    Ok(SimpleProps {
        xml_name,
        rust_type,
        target_namespace: doc.current_target_namespace.clone(),
        restrictions: Some(restrictions),
        comment,
    })
}

fn build_restrictions<'n>(restriction: Node<'n, 'n>) -> Restrictions {
    // get the restrictions
    let mut restrictions = Restrictions::default();

    if let Some(min_inclusive) = restriction.attribute("minInclusive") {
        restrictions.min_inclusive = Some(min_inclusive.to_string());
    }

    if let Some(max_inclusive) = restriction.attribute("maxInclusive") {
        restrictions.max_inclusive = Some(max_inclusive.to_string());
    }

    if let Some(min_exclusive) = restriction.attribute("minExclusive") {
        restrictions.min_exclusive = Some(min_exclusive.to_string());
    }

    if let Some(max_exclusive) = restriction.attribute("maxExclusive") {
        restrictions.max_exclusive = Some(max_exclusive.to_string());
    }

    if let Some(total_digits) = restriction.attribute("totalDigits") {
        restrictions.total_digits = Some(total_digits.to_string());
    }

    if let Some(fraction_digits) = restriction.attribute("fractionDigits") {
        restrictions.fraction_digits = Some(fraction_digits.to_string());
    }

    if let Some(length) = restriction.attribute("length") {
        restrictions.length = Some(length.to_string());
    }

    if let Some(min_length) = restriction.attribute("minLength") {
        restrictions.min_length = Some(min_length.to_string());
    }

    if let Some(max_length) = restriction.attribute("maxLength") {
        restrictions.max_length = Some(max_length.to_string());
    }

    if let Some(white_space) = restriction.attribute("whiteSpace") {
        restrictions.white_space = Some(white_space.to_string());
    }

    if let Some(pattern) = restriction.attribute("pattern") {
        restrictions.pattern = Some(pattern.to_string());
    }

    let enumeration = restriction
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "enumeration")
        .map(|n| n.attribute("value").unwrap().to_string())
        .collect::<Vec<String>>();

    if !enumeration.is_empty() {
        restrictions.enumeration = Some(enumeration);
    }

    restrictions
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
                comment,
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
                restrictions: _,
                comment: _,
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
            comment: None,
        }
    }
}
