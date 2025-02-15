use crate::{
    error::WriterResult,
    model::{field::Field, TargetNamespace, WriteNode},
};
use std::{io, io::Write, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct StructProps {
    pub xml_name: String,
    pub fields: Vec<Field>,
    pub target_namespace: Option<Rc<TargetNamespace>>,
}

#[derive(Debug, PartialEq)]
pub enum RustType {
    Ignore,
    Struct(StructProps),
}

impl<W> WriteNode<W> for RustType
where
    W: io::Write,
{
    fn write_node(&self, writer: &mut W) -> WriterResult<()> {
        match self {
            RustType::Ignore => Ok(()),
            RustType::Struct(props) => {
                writeln!(writer, "#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]")?;
                if let Some(tns) = &props.target_namespace {
                    let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
                    writeln!(
                        writer,
                        "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
                        tns.abbreviation, namespaces, props.xml_name
                    )?;
                }
                writeln!(writer, "struct {} {{", props.xml_name)?;
                for field in &props.fields {
                    field.write_node(writer)?;
                }

                write!(writer, "}}")?;
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
