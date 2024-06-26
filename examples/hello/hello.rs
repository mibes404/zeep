//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.10
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

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
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
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
        pub hello_response: types::HelloResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SayHello")]
    pub struct SayHello {
        #[yaserde(flatten, default)]
        pub hello_request: types::HelloRequest,
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
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
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
        #[yaserde(rename = "tns:HelloRequest", default)]
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
        pub encoding_style: Option<String>,
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
        #[must_use]
        pub fn new(body: SoapSayHello) -> Self {
            SayHelloSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://learnwebservices.com/services/hello".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSayHelloResponse {
        #[yaserde(rename = "HelloResponse", default)]
        pub body: Option<ports::SayHelloResponse>,
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
        pub encoding_style: Option<String>,
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
        #[must_use]
        pub fn new(body: SoapSayHelloResponse) -> Self {
            SayHelloResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://learnwebservices.com/services/hello".to_string()),
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
                credentials: None,
            }
        }
    }
    impl HelloEndpointServiceSoapBinding {
        #[must_use]
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
                xmlns: Some("http://learnwebservices.com/services/hello".to_string()),
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
                Ok(r.body.body.expect("missing body"))
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
        #[must_use]
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::HelloEndpointServiceSoapBinding {
            Self::new_client_with_url(
                "https://apps.learnwebservices.com:443/services/hello",
                credentials,
            )
        }

        #[must_use]
        pub fn new_client_with_url(
            url: &str,
            credentials: Option<(String, String)>,
        ) -> bindings::HelloEndpointServiceSoapBinding {
            bindings::HelloEndpointServiceSoapBinding::new(url, credentials)
        }
    }
}

pub mod multiref {
    //! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
    //! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
    //! Needs `xml-rs` and `yaserde` as dependencies.

    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(
            reader: &mut yaserde::de::Deserializer<R>,
        ) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Rc::new(RefCell::new(inner)),
            })
        }
    }

    impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.as_ref().borrow().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<
            (
                Vec<xml::attribute::OwnedAttribute>,
                xml::namespace::Namespace,
            ),
            String,
        > {
            self.inner
                .as_ref()
                .borrow()
                .serialize_attributes(attributes, namespace)
        }
    }

    impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self {
                inner: Rc::default(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.as_ref().borrow().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Rc<RefCell<T>>;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
