//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes may be lost during subsequent runs of the code generator.
//!
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "SayHelloResponse", default)]
    pub struct SayHelloResponse {
        #[yaserde(flatten)]
        pub parameters: types::SayHelloResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "SayHello", default)]
    pub struct SayHello {
        #[yaserde(flatten)]
        pub parameters: types::SayHello,
    }
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
pub struct Header {}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    root = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[async_trait]
    pub trait HelloEndpoint {
        async fn say_hello(
            &mut self,
            say_hello: SayHello,
        ) -> Result<SayHelloResponse, Option<SoapFault>>;
    }

    pub type SayHello = messages::SayHello;
    pub type SayHelloResponse = messages::SayHelloResponse;
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct HelloEndpointServiceSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::HelloEndpoint for HelloEndpointServiceSoapBinding {
        async fn say_hello(
            &mut self,
            say_hello: ports::SayHello,
        ) -> Result<ports::SayHelloResponse, Option<SoapFault>> {
            let __request = SayHelloSoapEnvelope::new(SoapSayHello {
                body: say_hello,
                xmlns: Option::from("http://learnwebservices.com/services/hello".to_string()),
            });

            let body = to_string(&__request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);

            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", "");

            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::from(credentials.1.to_string()),
                );
            }

            let res = req.send().await.expect("can not send request");

            let status = res.status();
            debug!("SOAP Status: {}", status);

            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);

            let r: SayHelloResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl Default for HelloEndpointServiceSoapBinding {
        fn default() -> Self {
            HelloEndpointServiceSoapBinding {
                client: reqwest::Client::new(),
                url: "http://learnwebservices.com/services/hello".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl HelloEndpointServiceSoapBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            HelloEndpointServiceSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSayHello {
        #[yaserde(rename = "SayHello", default)]
        pub body: ports::SayHello,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SayHelloSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSayHello,
    }

    impl SayHelloSoapEnvelope {
        pub fn new(body: SoapSayHello) -> Self {
            SayHelloSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://learnwebservices.com/services/hello".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSayHelloResponse {
        #[yaserde(rename = "SayHelloResponse", default)]
        pub body: ports::SayHelloResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SayHelloResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSayHelloResponse,
    }

    impl SayHelloResponseSoapEnvelope {
        pub fn new(body: SoapSayHelloResponse) -> Self {
            SayHelloResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://learnwebservices.com/services/hello".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/hello",
        rename = "SayHello",
        default
    )]
    pub struct SayHello {
        #[yaserde(prefix = "tns", rename = "HelloRequest", default)]
        pub hello_request: HelloRequest,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/hello",
        rename = "helloRequest",
        default
    )]
    pub struct HelloRequest {
        #[yaserde(prefix = "tns", rename = "Name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/hello",
        rename = "SayHelloResponse",
        default
    )]
    pub struct SayHelloResponse {
        #[yaserde(prefix = "tns", rename = "HelloResponse", default)]
        pub hello_response: HelloResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://learnwebservices.com/services/hello",
        rename = "helloResponse",
        default
    )]
    pub struct HelloResponse {
        #[yaserde(prefix = "tns", rename = "Message", default)]
        pub message: String,
    }
}
