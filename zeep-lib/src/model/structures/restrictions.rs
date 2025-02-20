use super::*;

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

pub fn build_restrictions<'n>(restriction: Node<'n, 'n>) -> Restrictions {
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
