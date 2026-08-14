use yaserde_derive::{YaDeserialize, YaSerialize};

fn empty_string() -> String {
    String::new()
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "Person")]
struct Person {
    #[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "name")]
    pub name: String,
    #[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "age")]
    pub age: i32,
}

#[test]
fn can_write_a_struct_type_with_namespace_to_rust() {
    let rust_type = Person {
        name: "John".to_string(),
        age: 42,
    };
    let xml = yaserde::ser::to_string(&rust_type).unwrap();

    let expected = r#"<?xml version="1.0" encoding="UTF-8"?><ex:Person xmlns:ex="http://example.com"><ex:name>John</ex:name><ex:age>42</ex:age></ex:Person>"#;
    assert_eq!(xml, expected);
}

// Mirrors the `#[yaserde(text = true, default = "__yaserde_default_string")]`
// pattern the generator emits for XSD simple-type text wrappers (see
// `write_type_alias` in `model/structures/writer.rs`). Some SOAP services
// send self-closing elements like `<foobar />`
// for optional-content fields; xml-rs emits no `Characters` event for those,
// so without an explicit default, deserializing a *present but empty*
// element fails with a spurious "required field" error.
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
struct TextWrapper {
    #[yaserde(text = true, default = "empty_string")]
    pub value: String,
}

#[test]
fn can_deserialize_self_closing_element_into_text_wrapper() {
    let parsed: TextWrapper = yaserde::de::from_str("<TextWrapper/>").unwrap();
    assert_eq!(parsed.value, "");
}
