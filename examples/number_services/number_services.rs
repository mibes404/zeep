//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.11
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
    #[yaserde(rename = "NumberToWordsSoapRequest")]
    pub struct NumberToWordsSoapRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::NumberToWords,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "NumberToWordsResponse")]
    pub struct NumberToWordsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::NumberToWordsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "NumberToDollarsSoapRequest")]
    pub struct NumberToDollarsSoapRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::NumberToDollars,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "NumberToDollarsResponse")]
    pub struct NumberToDollarsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::NumberToDollarsResponse,
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
        rename = "NumberToWords",
        namespace = "m: http://www.dataaccess.com/webservicesserver/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "m"
    )]
    pub struct NumberToWords {
        #[yaserde(rename = "ubiNum", prefix = "m", default)]
        pub ubi_num: u64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "NumberToWordsResponse",
        namespace = "m: http://www.dataaccess.com/webservicesserver/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "m"
    )]
    pub struct NumberToWordsResponse {
        #[yaserde(rename = "NumberToWordsResult", prefix = "m", default)]
        pub number_to_words_result: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "NumberToDollars",
        namespace = "m: http://www.dataaccess.com/webservicesserver/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "m"
    )]
    pub struct NumberToDollars {
        #[yaserde(rename = "dNum", prefix = "m", default)]
        pub d_num: f64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "NumberToDollarsResponse",
        namespace = "m: http://www.dataaccess.com/webservicesserver/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "m"
    )]
    pub struct NumberToDollarsResponse {
        #[yaserde(rename = "NumberToDollarsResult", prefix = "m", default)]
        pub number_to_dollars_result: String,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type NumberToWordsSoapRequest = messages::NumberToWordsSoapRequest;

    pub type NumberToWordsResponse = messages::NumberToWordsResponse;

    pub type NumberToDollarsSoapRequest = messages::NumberToDollarsSoapRequest;

    pub type NumberToDollarsResponse = messages::NumberToDollarsResponse;

    #[async_trait]
    pub trait NumberConversionSoapType {
        async fn number_to_words(
            &self,
            number_to_words_soap_request: NumberToWordsSoapRequest,
        ) -> Result<NumberToWordsResponse, Option<SoapFault>>;
        async fn number_to_dollars(
            &self,
            number_to_dollars_soap_request: NumberToDollarsSoapRequest,
        ) -> Result<NumberToDollarsResponse, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl NumberConversionSoapBinding {
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
    pub struct SoapNumberToWordsSoapRequest {
        #[yaserde(rename = "tns:NumberToWords", default)]
        pub body: ports::NumberToWordsSoapRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct NumberToWordsSoapRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "m", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapNumberToWordsSoapRequest,
    }

    impl NumberToWordsSoapRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapNumberToWordsSoapRequest) -> Self {
            NumberToWordsSoapRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapNumberToWordsResponse {
        #[yaserde(rename = "NumberToWordsResponse", default)]
        pub body: Option<ports::NumberToWordsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct NumberToWordsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "m", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapNumberToWordsResponse,
    }

    impl NumberToWordsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapNumberToWordsResponse) -> Self {
            NumberToWordsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapNumberToDollarsSoapRequest {
        #[yaserde(rename = "tns:NumberToDollars", default)]
        pub body: ports::NumberToDollarsSoapRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct NumberToDollarsSoapRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "m", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapNumberToDollarsSoapRequest,
    }

    impl NumberToDollarsSoapRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapNumberToDollarsSoapRequest) -> Self {
            NumberToDollarsSoapRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapNumberToDollarsResponse {
        #[yaserde(rename = "NumberToDollarsResponse", default)]
        pub body: Option<ports::NumberToDollarsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct NumberToDollarsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "m", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapNumberToDollarsResponse,
    }

    impl NumberToDollarsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapNumberToDollarsResponse) -> Self {
            NumberToDollarsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for NumberConversionSoapBinding {
        fn default() -> Self {
            NumberConversionSoapBinding {
                client: reqwest::Client::new(),
                url: "http://www.dataaccess.com/webservicesserver/".to_string(),
                credentials: None,
            }
        }
    }
    impl NumberConversionSoapBinding {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            NumberConversionSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct NumberConversionSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::NumberConversionSoapType for NumberConversionSoapBinding {
        async fn number_to_words(
            &self,
            number_to_words_soap_request: ports::NumberToWordsSoapRequest,
        ) -> Result<ports::NumberToWordsResponse, Option<SoapFault>> {
            let __request =
                NumberToWordsSoapRequestSoapEnvelope::new(SoapNumberToWordsSoapRequest {
                    body: number_to_words_soap_request,
                    xmlns: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: NumberToWordsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn number_to_dollars(
            &self,
            number_to_dollars_soap_request: ports::NumberToDollarsSoapRequest,
        ) -> Result<ports::NumberToDollarsResponse, Option<SoapFault>> {
            let __request =
                NumberToDollarsSoapRequestSoapEnvelope::new(SoapNumberToDollarsSoapRequest {
                    body: number_to_dollars_soap_request,
                    xmlns: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: NumberToDollarsResponseSoapEnvelope = from_str(&response).map_err(|err| {
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

    impl NumberConversionSoapBinding12 {
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
    impl Default for NumberConversionSoapBinding12 {
        fn default() -> Self {
            NumberConversionSoapBinding12 {
                client: reqwest::Client::new(),
                url: "http://www.dataaccess.com/webservicesserver/".to_string(),
                credentials: None,
            }
        }
    }
    impl NumberConversionSoapBinding12 {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            NumberConversionSoapBinding12 {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct NumberConversionSoapBinding12 {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::NumberConversionSoapType for NumberConversionSoapBinding12 {
        async fn number_to_words(
            &self,
            number_to_words_soap_request: ports::NumberToWordsSoapRequest,
        ) -> Result<ports::NumberToWordsResponse, Option<SoapFault>> {
            let __request =
                NumberToWordsSoapRequestSoapEnvelope::new(SoapNumberToWordsSoapRequest {
                    body: number_to_words_soap_request,
                    xmlns: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: NumberToWordsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn number_to_dollars(
            &self,
            number_to_dollars_soap_request: ports::NumberToDollarsSoapRequest,
        ) -> Result<ports::NumberToDollarsResponse, Option<SoapFault>> {
            let __request =
                NumberToDollarsSoapRequestSoapEnvelope::new(SoapNumberToDollarsSoapRequest {
                    body: number_to_dollars_soap_request,
                    xmlns: Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: NumberToDollarsResponseSoapEnvelope = from_str(&response).map_err(|err| {
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
    /** The Number Conversion Web Service, implemented with Visual DataFlex, provides functions that convert numbers into words or dollar amounts.
     */
    pub struct NumberConversion {}
    impl NumberConversion {
        #[must_use]
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::NumberConversionSoapBinding {
            Self::new_client_with_url(
                "https://www.dataaccess.com/webservicesserver/NumberConversion.wso",
                credentials,
            )
        }

        #[must_use]
        pub fn new_client_with_url(
            url: &str,
            credentials: Option<(String, String)>,
        ) -> bindings::NumberConversionSoapBinding {
            bindings::NumberConversionSoapBinding::new(url, credentials)
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
