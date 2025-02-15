use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "Person")]
struct Person {
    #[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "name")]
    pub name: String,
    #[yaserde(prefix = "ex", namespaces = {"ex" = "http://example.com"}, rename = "age")]
    pub age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
