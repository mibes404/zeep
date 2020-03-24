use std::io::{Read, Write};
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

const SOAP_ENVELOPE: &str = "http://www.w3.org/2003/05/soap-envelope/";
pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";

#[macro_export]
macro_rules! envelop {
    ($id: ident, $type: ty) => {
        #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            root = "Envelope",
            namespace = "soap: http://www.w3.org/2003/05/soap-envelope/",
            prefix = "soap"
        )]
        pub struct $id {
            #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "Header", prefix = "soap")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soap")]
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
            header: Option::None,
            body: Message {},
        };

        let xml = to_string(&e).expect("can not convert to XML");
        println!("{}", xml);
    }
}
