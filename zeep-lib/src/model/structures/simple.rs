use super::{
    Namespace, Node, Rc, RustDocument, RustFieldType, TryFromNode, WriterError, WriterResult, parse_comment,
    restrictions::{Restrictions, build_restrictions},
};
use crate::model::{field::as_rust_type, node::collect_namespaces_on_node};

#[derive(Debug, PartialEq)]
pub struct SimpleProps {
    pub xml_name: String,
    pub rust_type: RustFieldType,
    pub target_namespace: Option<Rc<Namespace>>,
    pub restrictions: Option<Restrictions>,
    pub comment: Option<String>,
}

impl<'n> TryFromNode<'n> for SimpleProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        // check if the node has an attribute that starts with xmlns
        collect_namespaces_on_node(node, doc);

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
                .map(|b| as_rust_type(b, doc))
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
            .map(|m| as_rust_type(m, doc))
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
                .map(|b| as_rust_type(b, doc))
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
    if let Some(item_type) = list.attribute("itemType").map(|i| as_rust_type(i, doc)) {
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
        .map(|b| as_rust_type(b, doc))
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
