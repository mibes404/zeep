use std::io::{Read, Write};
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

const SOAP_ENVELOPE: &str = "http://schemas.xmlsoap.org/soap/envelope/";
pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";

#[macro_export]
macro_rules! envelop {
    ($id: ident, $type: ty) => {
        #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            root = "Envelope",
            namespace = "SOAP-ENV: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "SOAP-ENV"
        )]
        pub struct $id {
            #[yaserde(rename = "encodingStyle", prefix = "SOAP-ENV", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: String,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: String,
            #[yaserde(rename = "Header", prefix = "SOAP-ENV")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "SOAP-ENV")]
            pub body: $type,
        }
    };
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Message {}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Header {}

envelop!(MessageEnvelope, Message);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope() {
        let e = MessageEnvelope {
            encoding_style: SOAP_ENCODING.to_string(),
            tnsattr: "".to_string(),
            urnattr: None,
            xsiattr: "".to_string(),
            header: Option::None,
            body: Message {},
        };

        let xml = to_string(&e).expect("can not convert to XML");
        println!("{}", xml);
    }
}
