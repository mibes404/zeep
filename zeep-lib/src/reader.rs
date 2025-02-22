use crate::{
    error::{WriterError, WriterResult},
    model::{
        TryFromNode,
        doc::RustDocument,
        node::RustNode,
        soap::{binding::SoapBinding, message::SoapMessage, port::SoapPort},
    },
};
use roxmltree::Node;
use std::{collections::HashMap, fmt::Display, io, rc::Rc, sync::atomic::AtomicBool};

const WELL_KNOWN_NAMESPACES: &[&str] = &[
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/2001/XMLSchema",
    "http://www.w3.org/2001/XMLSchema-instance",
    "http://www.w3.org/2007/XMLSchema-versioning",
];

pub trait WriteXml<W>
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()>;
}

pub struct XmlReader;

pub struct FileContent {
    xml: String,
    processed: AtomicBool,
}

impl FileContent {
    #[must_use]
    pub fn new(xml: String) -> Self {
        FileContent {
            xml,
            processed: AtomicBool::new(false),
        }
    }
}

pub type Schemalocation = String;

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
        }

        Ok(())
    }

    fn read_soap_types_schema<'n>(
        files: &Files,
        doc: &mut RustDocument,
        child: Node<'n, 'n>,
    ) -> Result<(), WriterError> {
        let schema = child
            .children()
            .find(|n| n.tag_name().name() == "schema")
            .ok_or(WriterError::SchemaNotFound)?;
        Self::read_xsd(schema, files, doc)?;
        Ok(())
    }

    fn read_xsd<'n>(node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<()> {
        for child in node.children() {
            if child.tag_name().name() == "import" {
                doc.nodes.extend(Self::process_import(child, files)?);
                continue;
            }

            if let Ok(child_node) = RustNode::try_from_node(child, doc) {
                doc.nodes.push(child_node.into());
            }
        }

        Ok(())
    }

    fn process_import(node: Node, files: &Files) -> WriterResult<Vec<Rc<RustNode>>> {
        let namespace = node.attribute("namespace").ok_or(WriterError::NamespaceMissing)?;

        if WELL_KNOWN_NAMESPACES.contains(&namespace) {
            return Ok(vec![]);
        }

        let Some(schema_location) = node.attribute("schemaLocation") else {
            return Ok(vec![]);
        };

        let file = files
            .map
            .get(schema_location)
            .ok_or_else(|| WriterError::ImportNotFound(schema_location.to_string()))?;

        if file.processed.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(vec![]);
        }

        let rust_doc = Self::read_xml_internal(file, schema_location, files)?;
        Ok(rust_doc.nodes)
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
            comment,
        } = &**props;

        assert_eq!(xml_name, "InstalledAppType");
        assert_eq!(fields.len(), 14);
        let id_field = fields.first().unwrap();
        assert_eq!(id_field.xml_name, "Id");
        assert_eq!(id_field.rust_type, RustFieldType::String);
        assert!(id_field.optional);
        assert!(!id_field.vec);

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
            fields,
            xml_name,
            target_namespace,
            comment,
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
            restrictions,
            comment,
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
        assert_eq!(rust_doc.namespace_references.len(), 2, "Expected two namespaces");

        let nodes = &rust_doc.nodes;
        assert_eq!(nodes.len(), 4);

        let type_node = nodes.get(3).expect("Expected a third node");
        let RustType::Complex(props) = &type_node.rust_type else {
            panic!()
        };

        let ComplexProps {
            fields,
            xml_name,
            target_namespace,
            comment,
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
        let ComplexProps {
            xml_name,
            fields,
            target_namespace,
            comment,
        } = &**props;

        // check the amount of fields, it should include the fields from the base type
        assert_eq!(fields.len(), 1);
    }
}
