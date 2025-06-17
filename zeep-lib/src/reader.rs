//! This module contains the `XmlReader` struct which is responsible for reading the XML and
//! converting it to a `RustDocument` The entry point is the `read_xml` method which takes a
//! `FilesToRead` struct and returns a `RustDocument` The `Files` struct is used to hold the
//! XML content and the `FilesToRead` struct is used to hold the `Files` and the starting file
//! to read from.
//!
//! The `RustDocument` (and the included elements) implement the `WriteXml` trait which is used
//! to write the Rust code to a writer.
//!
//! The `read_input_file_and_xsd_files_at_path` helper function can be used to easily construct
//! the `FilesToRead` struct from a file path.

use crate::{
    error::{WriterError, WriterResult},
    model::{
        TryFromNode,
        doc::RustDocument,
        node::RustNode,
        soap::{binding::SoapBinding, message::SoapMessage, port::SoapPort, service::SoapService},
    },
};
use roxmltree::Node;
use std::{collections::HashMap, fmt::Display, io, sync::atomic::AtomicBool};

pub const WELL_KNOWN_NAMESPACES: &[&str] = &[
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/2001/XMLSchema",
    "http://www.w3.org/2001/XMLSchema-instance",
    "http://www.w3.org/2007/XMLSchema-versioning",
];

/// The `WriteXml` trait is used to write the Rust code to a writer, like a file or a buffer.
pub trait WriteXml<W>
where
    W: io::Write,
{
    /// Generate XML from the Rust type
    ///
    /// # Errors
    /// Returns an error if the XML could not be written to the writer
    fn write_xml(&self, writer: &mut W) -> WriterResult<()>;
}

/// The `XmlReader` struct is responsible for reading the XML and converting it to a `RustDocument`
pub struct XmlReader;

/// The `FileContent` struct is used to hold the XML content and a flag to indicate if the file
/// has been processed or not.
struct FileContent {
    xml: String,
    processed: AtomicBool,
}

impl FileContent {
    #[must_use]
    fn new(xml: String) -> Self {
        FileContent {
            xml,
            processed: AtomicBool::new(false),
        }
    }
}

type Schemalocation = String;

/// The `Files` struct is used to hold the XML content
pub struct Files {
    map: HashMap<Schemalocation, FileContent>,
}

impl Files {
    pub fn new<F, X>(file_name: F, xml: X) -> Self
    where
        F: Display,
        X: Display,
    {
        Files {
            map: HashMap::from([(file_name.to_string(), FileContent::new(xml.to_string()))]),
        }
    }

    pub fn add<F, X>(&mut self, file_name: F, xml: X)
    where
        F: Display,
        X: Display,
    {
        let Files { map, .. } = self;
        map.insert(file_name.to_string(), FileContent::new(xml.to_string()));
    }
}

/// The `FilesToRead` struct is used to hold the `Files` and the starting file to read from.
pub struct FilesToRead {
    start_with_file: String,
    files: Files,
}

impl FilesToRead {
    pub fn new(start_with_file: impl Display, files: Files) -> Self {
        FilesToRead {
            start_with_file: start_with_file.to_string(),
            files,
        }
    }

    fn inner(&self) -> (&FileContent, &str, &Files) {
        let FilesToRead { start_with_file, files } = self;
        let content = files.map.get(start_with_file).unwrap();
        (content, start_with_file, files)
    }
}

impl XmlReader {
    /// Read the XML and convert it to a `RustDocument`
    ///
    /// # Errors
    /// Returns an error if the XSD/WSDL is invalid
    pub fn read_xml(files_to_read: &FilesToRead) -> WriterResult<RustDocument> {
        let (content, start_with_file, files) = files_to_read.inner();
        Self::read_xml_internal(content, start_with_file, files)
    }

    #[cfg(test)]
    pub(crate) fn read_xml_from_file(file_name: &str, xml: &str) -> WriterResult<RustDocument> {
        let files = Files::new(file_name, xml);
        let files_to_read = FilesToRead::new(file_name, files);
        Self::read_xml(&files_to_read)
    }

    fn read_xml_internal(file: &FileContent, file_name: &str, files: &Files) -> WriterResult<RustDocument> {
        if file.processed.load(std::sync::atomic::Ordering::SeqCst) {
            let rust_doc = RustDocument::empty();
            return Ok(rust_doc);
        }

        let xml = &file.xml;
        let doc = roxmltree::Document::parse(xml)
            .map_err(|e| WriterError::new(format!("Unable to parse file {file_name}: {e}")))?;
        let mut rust_doc = RustDocument::init(&doc);

        for child in doc.root().children() {
            Self::read(child, files, &mut rust_doc)?;
        }

        file.processed.store(true, std::sync::atomic::Ordering::SeqCst);

        Ok(rust_doc)
    }

    fn read<'n>(node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<()> {
        if !node.is_element() {
            return Ok(());
        }

        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
        }

        match node.tag_name().name() {
            "definitions" => Self::read_wsdl(node, files, doc)?,
            "schema" => Self::read_xsd(node, files, doc)?,
            _ => return Ok(()),
        }

        Ok(())
    }

    fn read_wsdl<'n>(node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<()> {
        for child in node.children() {
            let node_name = child.tag_name().name();
            // first read the types as if it were an XSD
            if node_name == "types" {
                Self::read_soap_types_schema(files, doc, child)?;
            }

            // read soap messages
            if node_name == "message" {
                let message = SoapMessage::try_from_node(child, doc)?;
                doc.soap_messages.push(message.into());
            }

            // read soap ports
            if node_name == "portType" {
                let port = SoapPort::try_from_node(child, doc)?;
                doc.soap_ports.push(port.into());
            }

            // read soap bindings
            if node_name == "binding" {
                let binding = SoapBinding::try_from_node(child, doc)?;
                doc.soap_bindings.push(binding.into());
            }

            // read soap services
            if node_name == "service" {
                let service = SoapService::try_from_node(child, doc)?;
                doc.soap_services.push(service);
            }
        }

        Ok(())
    }

    fn read_soap_types_schema<'n>(
        files: &Files,
        doc: &mut RustDocument,
        child: Node<'n, 'n>,
    ) -> Result<(), WriterError> {
        let mut any = false;
        for node in child.children() {
            if node.tag_name().name() == "schema" {
                Self::read_xsd(node, files, doc)?;
                any = true;
            }
        }
        if any {
            Ok(())
        } else {
            Err(WriterError::SchemaNotFound)
        }
    }

    fn read_xsd<'n>(node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<()> {
        for child in node.children() {
            if child.tag_name().name() == "import" {
                doc.extend(Self::process_import(child, files)?);
                continue;
            }

            if let Ok(child_node) = RustNode::try_from_node(child, doc) {
                doc.nodes.push(child_node.into());
            }
        }

        Ok(())
    }

    fn process_import(node: Node, files: &Files) -> WriterResult<RustDocument> {
        let namespace = node.attribute("namespace").ok_or(WriterError::NamespaceMissing)?;

        if WELL_KNOWN_NAMESPACES.contains(&namespace) {
            return Ok(RustDocument::empty());
        }

        let Some(schema_location) = node.attribute("schemaLocation") else {
            return Ok(RustDocument::empty());
        };

        let file = files
            .map
            .get(schema_location)
            .ok_or_else(|| WriterError::ImportNotFound(schema_location.to_string()))?;

        if file.processed.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(RustDocument::empty());
        }

        let rust_doc = Self::read_xml_internal(file, schema_location, files)?;
        Ok(rust_doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        field::RustFieldType,
        structures::{RustType, complex::ComplexProps, simple::SimpleProps},
    };

    #[test]
    fn can_read_a_simple_xsd() {
        const XSD: &str = include_str!("../test-data/single-complex.xsd");
        let files = Files::new("types.xsd", XSD);
        let (file_name, file) = files.map.get_key_value("types.xsd").unwrap();
        let nodes = XmlReader::read_xml_internal(file, file_name, &files).unwrap().nodes;
        assert_eq!(nodes.len(), 1);
        let node = nodes.first().unwrap();
        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        let ComplexProps {
            fields,
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "InstalledAppType");
        assert_eq!(fields.len(), 14);
        let id_field = fields.first().unwrap();
        assert_eq!(id_field.xml_name, "Id");
        assert_eq!(id_field.rust_type, RustFieldType::String);
        assert!(id_field.is_optional);
        assert!(!id_field.is_vec);

        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(target_namespace.as_ref().unwrap().abbreviation, "typ");
    }

    #[test]
    fn can_read_nested_target_namespace() {
        const XSD_MESSAGES: &str = include_str!("../test-data/single-simple-with-nested-tns.xsd");
        const XSD_TYPES: &str = include_str!("../test-data/single-complex.xsd");
        let mut files = Files::new("messages.xsd", XSD_MESSAGES);
        files.add("types.xsd", XSD_TYPES);

        let (file_name, file) = files.map.get_key_value("messages.xsd").unwrap();
        let nodes = XmlReader::read_xml_internal(file, file_name, &files).unwrap().nodes;
        assert_eq!(nodes.len(), 2);

        let type_node = nodes.first().unwrap();
        let RustType::Complex(props) = &type_node.rust_type else {
            panic!()
        };

        let ComplexProps {
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "InstalledAppType");
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );

        let message_node = nodes.last().unwrap();
        let RustType::Simple(props) = &message_node.rust_type else {
            panic!()
        };
        let SimpleProps {
            xml_name,
            rust_type,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "ResponseCodeType");
        assert_eq!(*rust_type, RustFieldType::String);
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/messages"
        );
    }

    #[test]
    fn can_read_elements_with_extensions() {
        const XSD_TYPES: &str = include_str!("../test-data/extensions.xsd");
        let files = Files::new("types.xsd", XSD_TYPES);

        let (file_name, file) = files.map.get_key_value("types.xsd").unwrap();
        let rust_doc = XmlReader::read_xml_internal(file, file_name, &files).unwrap();

        // check that we found the two namespaces
        assert_eq!(rust_doc.namespaces.len(), 2, "Expected two namespaces");

        let nodes = &rust_doc.nodes;
        assert_eq!(nodes.len(), 5);

        let type_node = nodes.get(4).expect("Expected a fourth node");
        let RustType::Complex(props) = &type_node.rust_type else {
            panic!()
        };

        let ComplexProps {
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "ItemChangeDescriptionType");
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );

        let message_node = nodes.last().unwrap();
        let RustType::Complex(props) = &message_node.rust_type else {
            panic!()
        };
        let ComplexProps { fields, .. } = &**props;

        // check the number of fields, it should include the fields from the base type
        assert_eq!(fields.len(), 4);

        // check the field names
        let field_names: Vec<&str> = fields.iter().map(|f| f.xml_name.as_str()).collect();
        assert_eq!(field_names, ["Path", "Source", "Target", "Drive"]);

        // the Target and Drive should be optional
        let target_field = fields.iter().find(|f| f.xml_name == "Target").unwrap();
        assert!(target_field.is_optional);

        let drive_field = fields.iter().find(|f| f.xml_name == "Drive").unwrap();
        assert!(drive_field.is_optional);
    }

    #[test]
    fn can_parse_forward_pointing_base_type() {
        const XSD_TYPES: &str = include_str!("../test-data/forward-pointing-type.xsd");
        let files = Files::new("types.xsd", XSD_TYPES);

        let (file_name, file) = files.map.get_key_value("types.xsd").unwrap();
        let rust_doc = XmlReader::read_xml_internal(file, file_name, &files).unwrap();
        assert_eq!(rust_doc.nodes.len(), 2);

        // check node name
        let node = rust_doc.nodes.first().unwrap();
        assert_eq!(node.rust_type.xml_name(), Some("AddDelegateType"));
    }

    #[test]
    fn can_parse_groups() {
        const XSD_TYPES: &str = include_str!("../test-data/use-of-groups.xsd");
        let files = Files::new("types.xsd", XSD_TYPES);

        let (file_name, file) = files.map.get_key_value("types.xsd").unwrap();
        let rust_doc = XmlReader::read_xml_internal(file, file_name, &files).unwrap();
        assert_eq!(rust_doc.nodes.len(), 3);

        // check node name
        let node = rust_doc.nodes.last().unwrap();
        assert_eq!(node.rust_type.xml_name(), Some("TimeChangeType"));
    }

    #[test]
    fn should_ensure_that_we_have_namespaces_on_all_types() {
        const XSD_MESSAGES: &str = include_str!("../test-data/exchange/messages.xsd");
        const XSD_TYPES: &str = include_str!("../test-data/exchange/types.xsd");
        let mut files = Files::new("messages.xsd", XSD_MESSAGES);
        files.add("types.xsd", XSD_TYPES);

        let (file_name, file) = files.map.get_key_value("messages.xsd").unwrap();
        let nodes = XmlReader::read_xml_internal(file, file_name, &files).unwrap().nodes;
        assert_eq!(nodes.len(), 1457);

        // get the GetUserAvailabilityRequestType
        let node = nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("GetUserAvailabilityRequestType"))
            .expect("Expected GetUserAvailabilityRequestType");

        // the node should have 4 fields. TimeZone, FreeBusyViewOptions and SuggestionsViewOptions should have the "typ" prefix
        // the MailboxDataArray should have the "mes" prefix

        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        assert_eq!(props.fields.len(), 4);

        // get the TimeZone field
        let time_zone = props.fields.first().expect("Expected a TimeZone field");
        assert_eq!(time_zone.xml_name, "TimeZone");
        assert_eq!(time_zone.target_namespace.as_ref().unwrap().abbreviation, "typ");

        // get the MailboxDataArray field
        let mailbox_data_array = props.fields.get(1).expect("Expected a MailboxDataArray field");
        assert_eq!(mailbox_data_array.xml_name, "MailboxDataArray");
        assert_eq!(
            mailbox_data_array.target_namespace.as_ref().unwrap().abbreviation,
            "mes"
        );

        // get the FreeBusyViewOptions field
        let free_busy_view_options = props.fields.get(2).expect("Expected a FreeBusyViewOptions field");
        assert_eq!(free_busy_view_options.xml_name, "FreeBusyViewOptions");
        assert_eq!(
            free_busy_view_options.target_namespace.as_ref().unwrap().abbreviation,
            "typ"
        );

        // get the SuggestionsViewOptions field
        let suggestions_view_options = props.fields.last().expect("Expected a SuggestionsViewOptions field");
        assert_eq!(suggestions_view_options.xml_name, "SuggestionsViewOptions");
        assert_eq!(
            suggestions_view_options.target_namespace.as_ref().unwrap().abbreviation,
            "typ"
        );
    }
}
