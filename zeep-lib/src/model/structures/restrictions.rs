use super::{Node, RustFieldType};
use crate::{error::WriterResult, reader::WriteXml};
use std::io;

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

impl<W> WriteXml<W> for Restrictions
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        // Here we want to write the constructor for the helpers_content::restrictions::Restrictions
        // and write that to the fn write_xml(&self, writer: &mut W) -> WriterResult<()> {

        writeln!(writer, "Rc::new(restrictions::Restrictions {{")?;
        if let Some(min_inclusive) = &self.min_inclusive {
            writeln!(writer, "   min_inclusive: Some({min_inclusive}), ")?;
        }
        if let Some(max_inclusive) = &self.max_inclusive {
            writeln!(writer, "   max_inclusive: Some({max_inclusive}), ")?;
        }
        if let Some(min_exclusive) = &self.min_exclusive {
            writeln!(writer, "   min_exclusive: Some({min_exclusive}), ")?;
        }
        if let Some(max_exclusive) = &self.max_exclusive {
            writeln!(writer, "   max_exclusive: Some({max_exclusive}), ")?;
        }
        if let Some(length) = &self.length {
            writeln!(writer, "   length: Some({length}), ")?;
        }
        if let Some(min_length) = &self.min_length {
            writeln!(writer, "   min_length: Some({min_length}), ")?;
        }
        if let Some(max_length) = &self.max_length {
            writeln!(writer, "   max_length: Some({max_length}), ")?;
        }

        // add the enumeration
        if let Some(enumeration) = &self.enumeration {
            writeln!(writer, "   enumeration: Some(vec![")?;
            for value in enumeration {
                writeln!(writer, "      \"{value}\".to_string(),")?;
            }
            writeln!(writer, "   ]),")?;
        }

        writeln!(writer, "   ..Default::default()")?;
        writeln!(writer, "}})")?;

        Ok(())
    }
}

pub fn build_restrictions<'n>(restriction: Node<'n, 'n>) -> Restrictions {
    // get the restrictions
    let mut restrictions = Restrictions::default();

    get_restriction_from_attribute_or_node(restriction, &mut restrictions.min_inclusive, "minInclusive");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.max_inclusive, "maxInclusive");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.min_exclusive, "minExclusive");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.max_exclusive, "maxExclusive");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.total_digits, "totalDigits");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.fraction_digits, "fractionDigits");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.length, "length");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.min_length, "minLength");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.max_length, "maxLength");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.white_space, "whiteSpace");
    get_restriction_from_attribute_or_node(restriction, &mut restrictions.pattern, "pattern");

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

fn get_restriction_from_attribute_or_node(
    restriction: Node,
    target_field: &mut Option<String>,
    restriction_name: &str,
) {
    if let Some(value) = restriction.attribute(restriction_name) {
        *target_field = Some(value.to_string());
    } else if let Some(value_node) = restriction.children().find(|n| n.tag_name().name() == restriction_name) {
        if let Some(value) = value_node.attribute("value") {
            *target_field = Some(value.to_string());
        }
    }
}
