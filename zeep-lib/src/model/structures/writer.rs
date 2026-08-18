use super::{
    ComplexProps, ElementProps, ElementType, Namespace, Rc, RustFieldType, RustType, SimpleProps, WriteXml,
    WriterError, WriterResult, io, xml_name_to_rust_name,
};
use crate::model::{
    helpers::{write_check_restrictions_footer, write_check_restrictions_header},
    structures::restrictions::Restrictions,
};
use inflector::cases::pascalcase::to_pascal_case;
use std::collections::HashMap;

impl<W> WriteXml<W> for RustType
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        match self {
            RustType::Ignore => Ok(()),
            RustType::Complex(props) => write_complex_type(writer, props),
            RustType::Simple(props) => write_simple_type(writer, props),
            RustType::Element(props) => {
                let ElementProps { xml_name, element_type } = &**props;
                let rust_name = xml_name_to_rust_name(xml_name);

                match element_type {
                    ElementType::RustType(rust_type) => {
                        if let Some(segment) = rust_type.to_string().split(':').next_back()
                            && segment == rust_name
                        {
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

fn write_simple_type<W>(writer: &mut W, props: &SimpleProps) -> WriterResult<()>
where
    W: io::Write,
{
    let SimpleProps {
        xml_name,
        rust_type,
        target_namespace,
        restrictions,
        comment,
    } = &props;

    let rust_name = xml_name_to_rust_name(xml_name);
    if let Some(segment) = rust_type.to_string().split(':').next_back()
        && segment == rust_name
    {
        // NOOP
        return Ok(());
    }

    if let Some(comment) = comment {
        comment.split('\n').for_each(|line| {
            writeln!(writer, "/// {line}").unwrap();
        });
    }

    // A string-based restriction whose facets are a fixed set of enumeration
    // values maps naturally onto a Rust enum: the valid states become
    // unrepresentable at the type level, so no runtime restriction check is
    // needed. Union/list restrictions still carry a `String` value and stay as
    // structs.
    if let Some(values) = string_enum_values(rust_type, restrictions.as_ref()) {
        write_enum(writer, xml_name, &rust_name, target_namespace.as_ref(), values)?;
        return Ok(());
    }

    // for now, write this as a type alias; we may want to change this to a newtype
    // in the future
    write_type_alias(
        writer,
        xml_name,
        &rust_name,
        rust_type,
        target_namespace.as_ref(),
        restrictions.as_ref(),
    )?;
    Ok(())
}

/// Returns the enumeration values when `props` describes a plain string
/// restriction whose only relevant facet is `enumeration` (i.e. not a union or
/// list, which still need a `String`-backed struct).
fn string_enum_values<'r>(
    rust_type: &RustFieldType,
    restrictions: Option<&'r Restrictions>,
) -> Option<&'r [String]> {
    if !rust_type.is_string() {
        return None;
    }

    let restrictions = restrictions?;
    if restrictions.acceptable_union_types.is_some() || restrictions.acceptable_list_type.is_some() {
        return None;
    }

    restrictions
        .enumeration
        .as_deref()
        .filter(|values| !values.is_empty())
}

fn write_enum<W>(
    writer: &mut W,
    xml_name: &str,
    rust_name: &str,
    target_namespace: Option<&Rc<Namespace>>,
    values: &[String],
) -> WriterResult<()>
where
    W: io::Write,
{
    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    if let Some(tns) = target_namespace {
        let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
        writeln!(
            writer,
            "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
            tns.abbreviation, namespaces, xml_name
        )?;
    }
    writeln!(writer, "pub enum {rust_name} {{")?;

    // Distinct XML values can collapse onto the same Rust identifier (e.g. `a-b`
    // and `a.b` both pascal-case to `AB`); disambiguate with a numeric suffix so
    // every variant name stays unique. The original value is always preserved
    // verbatim in the `rename` attribute, so the wire format is unaffected.
    let mut used: HashMap<String, u32> = HashMap::new();
    for (idx, value) in values.iter().enumerate() {
        let base = enum_value_to_variant_name(value);
        let seen = used.entry(base.clone()).or_insert(0);
        let variant = if *seen == 0 { base.clone() } else { format!("{base}_{seen}") };
        *seen += 1;

        writeln!(writer, "    /// {value}")?;
        // `Default` is derived, so the first variant is the default. This is
        // arbitrary but mirrors how the struct form defaulted to an empty string.
        if idx == 0 {
            writeln!(writer, "    #[default]")?;
        }
        writeln!(writer, "    #[yaserde(rename = \"{value}\")]")?;
        writeln!(writer, "    {variant},")?;
    }
    writeln!(writer, "}}")?;

    // Valid by construction: nothing to check at runtime.
    write_check_restrictions_header(writer, rust_name, None)?;
    writeln!(writer, "    drop(restrictions);")?;
    writeln!(writer, "    Ok(())")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
}

/// Turns an XSD enumeration value into a valid Rust enum variant identifier.
///
/// The exact XML value is preserved separately via a `#[yaserde(rename = ...)]`
/// attribute, so this only needs to produce *a* legal identifier, not a
/// reversible one.
fn enum_value_to_variant_name(value: &str) -> String {
    let mut name = {
        let pascal = to_pascal_case(value);
        if pascal.is_empty() {
            // Nothing alphanumeric survived (e.g. `""` or `"-"`); synthesise a
            // stable identifier from the raw scalar values so distinct values
            // stay distinct.
            use std::fmt::Write as _;
            let mut synthesised = String::from("Value");
            for c in value.chars() {
                let _ = write!(synthesised, "{:x}", c as u32);
            }
            synthesised
        } else {
            pascal
        }
    };

    // Rust identifiers cannot start with a digit.
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        name.insert(0, '_');
    }

    // `Self` is reserved and cannot be used even as a raw identifier.
    if name == "Self" {
        name.push('_');
    }

    name
}

fn write_type_alias<W>(
    writer: &mut W,
    xml_name: &str,
    rust_name: &str,
    rust_type: &RustFieldType,
    target_namespace: Option<&Rc<Namespace>>,
    restrictions: Option<&Restrictions>,
) -> Result<(), WriterError>
where
    W: io::Write,
{
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
    if rust_type.is_string() {
        // `default` covers the case where the element is present but empty
        // (e.g. `<foo/>`)
        writeln!(writer, "    #[yaserde(text = true, default = \"__yaserde_default_string\")]")?;
        writeln!(writer, "    pub value: {rust_type}")?;
    } else if rust_type.is_other() {
        writeln!(writer, "    #[yaserde(flatten = true)]")?;
        writeln!(writer, "    pub value: {rust_type}")?;
    } else {
        // note: flatten is not supported for other types
        writeln!(writer, "    #[yaserde(text = true, default = \"__yaserde_default_string\")]")?;
        writeln!(writer, "    pub value: String")?;
    }
    writeln!(writer, "}}")?;

    // Write the restriction check
    write_check_restrictions_header(writer, rust_name, restrictions)?;
    writeln!(writer, "     self.value.check_restrictions(restrictions)")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
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
        comment.split('\n').for_each(|line| {
            writeln!(writer, "/// {line}").unwrap();
        });
    }

    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    if let Some(tns) = &target_namespace {
        // Collect all unique namespaces: the struct's own namespace plus any referenced by fields
        let mut namespaces = vec![(tns.abbreviation.as_str(), tns.namespace.as_str())];

        for field in fields {
            if let Some(field_ns) = &field.target_namespace {
                // Only add if it's different from the struct's namespace and not already added
                if field_ns.namespace != tns.namespace
                    && !namespaces.iter().any(|(_, ns)| *ns == field_ns.namespace.as_str())
                {
                    namespaces.push((field_ns.abbreviation.as_str(), field_ns.namespace.as_str()));
                }
            }
        }

        let namespaces_str = namespaces
            .iter()
            .map(|(abbr, ns)| format!("\"{abbr}\" = \"{ns}\""))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            writer,
            "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
            tns.abbreviation, namespaces_str, xml_name
        )?;
    }
    writeln!(writer, "pub struct {rust_name} {{")?;
    for field in fields {
        field.write_xml(writer)?;
    }
    writeln!(writer, "}}")?;

    // Write the restriction check
    write_check_restrictions_header(writer, &rust_name, None)?;
    for field in fields {
        let field_name = &field.rust_name;
        writeln!(
            writer,
            "     self.{field_name}.check_restrictions(restrictions.clone())?;"
        )?;
    }

    writeln!(writer, "    drop(restrictions);")?;
    writeln!(writer, "    Ok(())")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::field::{Field, RustFieldType};

    #[test]
    fn can_write_a_struct_type_to_rust() {
        let mut writer = Vec::new();
        let props = prep_struct_props(None);
        let rust_type = RustType::Complex(props.into());
        rust_type.write_xml(&mut writer).unwrap();

        let expected = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct Person {\n    #[yaserde(rename = \"name\")]\n    pub name: String,\n    #[yaserde(rename = \"age\")]\n    pub age: i32,\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.name.check_restrictions(restrictions.clone())?;\n     self.age.check_restrictions(restrictions.clone())?;\n    drop(restrictions);\n    Ok(())\n  }\n}\n";
        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    #[test]
    fn can_write_a_struct_type_with_namespace_to_rust() {
        const EXPECTED: &str = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n#[yaserde(prefix = \"ex\", namespaces = {\"ex\" = \"http://example.com\"}, rename = \"Person\")]\npub struct Person {\n    #[yaserde(rename = \"name\")]\n    pub name: String,\n    #[yaserde(rename = \"age\")]\n    pub age: i32,\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.name.check_restrictions(restrictions.clone())?;\n     self.age.check_restrictions(restrictions.clone())?;\n    drop(restrictions);\n    Ok(())\n  }\n}\n";

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
                    ..Default::default()
                },
                Field {
                    xml_name: "age".to_string(),
                    rust_name: "age".to_string(),
                    rust_type: RustFieldType::I32,
                    ..Default::default()
                },
            ],
            target_namespace,
            comment: Some("A person".to_string()),
        }
    }

    #[test]
    fn can_write_a_simple_type_to_rust() {
        const EXPECTED: &str = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct Person {\n    #[yaserde(text = true, default = \"__yaserde_default_string\")]\n    pub value: String\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.value.check_restrictions(restrictions)\n  }\n}\n";
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

    fn prep_enum_props(target_namespace: Option<Rc<Namespace>>, values: &[&str]) -> Box<SimpleProps> {
        SimpleProps {
            xml_name: "AlimentationEtatCodeType".to_string(),
            rust_type: RustFieldType::String,
            target_namespace,
            restrictions: Some(Restrictions {
                enumeration: Some(values.iter().map(ToString::to_string).collect()),
                ..Restrictions::default()
            }),
            comment: None,
        }
        .into()
    }

    #[test]
    fn a_string_enumeration_simple_type_is_written_as_an_enum() {
        const EXPECTED: &str = "#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub enum AlimentationEtatCodeType {\n    #[default]\n    #[yaserde(rename = \"ALIM\")]\n    Alim,\n    #[yaserde(rename = \"COUP\")]\n    Coup,\n}\nimpl restrictions::CheckRestrictions for AlimentationEtatCodeType {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n    drop(restrictions);\n    Ok(())\n  }\n}\n";
        let mut writer = Vec::new();
        let rust_type = RustType::Simple(prep_enum_props(None, &["ALIM", "COUP"]));
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    #[test]
    fn a_string_enumeration_simple_type_with_namespace_is_written_as_an_enum() {
        const EXPECTED: &str = "#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n#[yaserde(prefix = \"ex\", namespaces = {\"ex\" = \"http://example.com\"}, rename = \"AlimentationEtatCodeType\")]\npub enum AlimentationEtatCodeType {\n    #[default]\n    #[yaserde(rename = \"ALIM\")]\n    Alim,\n}\nimpl restrictions::CheckRestrictions for AlimentationEtatCodeType {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n    drop(restrictions);\n    Ok(())\n  }\n}\n";
        let mut writer = Vec::new();
        let props = prep_enum_props(
            Some(Rc::new(Namespace {
                namespace: "http://example.com".to_string(),
                abbreviation: "ex".to_string(),
                rust_mod_name: "mod_ex".to_string(),
            })),
            &["ALIM"],
        );
        let rust_type = RustType::Simple(props);
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    #[test]
    fn colliding_enum_values_get_unique_variant_names() {
        // `a-b` and `a.b` both pascal-case to `AB`; the second must be suffixed.
        let mut writer = Vec::new();
        let rust_type = RustType::Simple(prep_enum_props(None, &["a-b", "a.b"]));
        rust_type.write_xml(&mut writer).unwrap();
        let out = String::from_utf8(writer).unwrap();
        assert!(out.contains("#[yaserde(rename = \"a-b\")]\n    AB,\n"), "{out}");
        assert!(out.contains("#[yaserde(rename = \"a.b\")]\n    AB_1,\n"), "{out}");
    }

    #[test]
    fn union_and_list_restrictions_are_not_written_as_enums() {
        // A union that happens to carry enumeration values must remain a struct.
        let props = SimpleProps {
            xml_name: "SomeUnion".to_string(),
            rust_type: RustFieldType::String,
            target_namespace: None,
            restrictions: Some(Restrictions {
                enumeration: Some(vec!["A".to_string()]),
                acceptable_union_types: Some(vec![RustFieldType::String]),
                ..Restrictions::default()
            }),
            comment: None,
        };
        let mut writer = Vec::new();
        RustType::Simple(props.into()).write_xml(&mut writer).unwrap();
        let out = String::from_utf8(writer).unwrap();
        assert!(out.contains("pub struct SomeUnion"), "{out}");
        assert!(!out.contains("pub enum"), "{out}");
    }

    #[test]
    fn enum_variant_names_are_sanitised() {
        assert_eq!(enum_value_to_variant_name("ALIM"), "Alim");
        assert_eq!(enum_value_to_variant_name("Exchange2010_SP1"), "Exchange2010SP1");
        // leading digit is not a legal identifier start
        assert_eq!(enum_value_to_variant_name("3D"), "_3D");
        // `self`/`Self` are reserved even after pascal-casing
        assert_eq!(enum_value_to_variant_name("self"), "Self_");
        assert_eq!(enum_value_to_variant_name("Self"), "Self_");
        // no alphanumeric content survives -> synthesised, but still an identifier
        assert_eq!(enum_value_to_variant_name(""), "Value");
        assert_eq!(enum_value_to_variant_name("-"), "Value2d");
    }
}
