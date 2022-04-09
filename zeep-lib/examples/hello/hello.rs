//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.4
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SayHelloResponse")]
    pub struct SayHelloResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::SayHelloResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SayHello")]
    pub struct SayHello {
        #[yaserde(flatten, default)]
        pub parameters: types::SayHello,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SayHello",
        namespace = "tns: http://learnwebservices.com/services/hello",
        prefix = "tns"
    )]
    pub struct SayHello {
        #[yaserde(rename = "HelloRequest", prefix = "tns", default)]
        pub hello_request: HelloRequest,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "helloRequest",
        namespace = "tns: http://learnwebservices.com/services/hello",
        prefix = "tns"
    )]
    pub struct HelloRequest {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SayHelloResponse",
        namespace = "tns: http://learnwebservices.com/services/hello",
        prefix = "tns"
    )]
    pub struct SayHelloResponse {
        #[yaserde(rename = "HelloResponse", prefix = "tns", default)]
        pub hello_response: HelloResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "helloResponse",
        namespace = "tns: http://learnwebservices.com/services/hello",
        prefix = "tns"
    )]
    pub struct HelloResponse {
        #[yaserde(rename = "Message", prefix = "tns", default)]
        pub message: String,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type SayHello = messages::SayHello;

    pub type SayHelloResponse = messages::SayHelloResponse;

    #[async_trait]
    pub trait HelloEndpoint {
        async fn say_hello(
            &self,
            say_hello: SayHello,
        ) -> Result<SayHelloResponse, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl HelloEndpointServiceSoapBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
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
        rename = "Envelope",
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
                tnsattr: Option::Some("http://learnwebservices.com/services/hello".to_string()),
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
        rename = "Envelope",
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
                tnsattr: Option::Some("http://learnwebservices.com/services/hello".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
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
    pub struct HelloEndpointServiceSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::HelloEndpoint for HelloEndpointServiceSoapBinding {
        async fn say_hello(
            &self,
            say_hello: ports::SayHello,
        ) -> Result<ports::SayHelloResponse, Option<SoapFault>> {
            let __request = SayHelloSoapEnvelope::new(SoapSayHello {
                body: say_hello,
                xmlns: Option::Some("http://learnwebservices.com/services/hello".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: SayHelloResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub struct HelloEndpointService {}
    impl HelloEndpointService {
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::HelloEndpointServiceSoapBinding {
            bindings::HelloEndpointServiceSoapBinding::new(
                "http://www.learnwebservices.com/services/hello",
                credentials,
            )
        }
    }
}
