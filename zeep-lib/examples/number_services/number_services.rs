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
    pub struct SoapNumberToWordsSoapRequest {
        #[yaserde(rename = "NumberToWords", default)]
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
        pub encoding_style: String,
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
        pub fn new(body: SoapNumberToWordsSoapRequest) -> Self {
            NumberToWordsSoapRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
        pub body: ports::NumberToWordsResponse,
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
        pub encoding_style: String,
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
        pub fn new(body: SoapNumberToWordsResponse) -> Self {
            NumberToWordsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapNumberToDollarsSoapRequest {
        #[yaserde(rename = "NumberToDollars", default)]
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
        pub encoding_style: String,
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
        pub fn new(body: SoapNumberToDollarsSoapRequest) -> Self {
            NumberToDollarsSoapRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
        pub body: ports::NumberToDollarsResponse,
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
        pub encoding_style: String,
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
        pub fn new(body: SoapNumberToDollarsResponse) -> Self {
            NumberToDollarsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
                credentials: Option::None,
            }
        }
    }
    impl NumberConversionSoapBinding {
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
                    xmlns: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
                Ok(r.body.body)
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
                    xmlns: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
                Ok(r.body.body)
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
    impl Default for NumberConversionSoapBinding12 {
        fn default() -> Self {
            NumberConversionSoapBinding12 {
                client: reqwest::Client::new(),
                url: "http://www.dataaccess.com/webservicesserver/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl NumberConversionSoapBinding12 {
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
                    xmlns: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
                Ok(r.body.body)
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
                    xmlns: Option::Some("http://www.dataaccess.com/webservicesserver/".to_string()),
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
    /** The Number Conversion Web Service, implemented with Visual DataFlex, provides functions that convert numbers into words or dollar amounts.
     */
    pub struct NumberConversion {}
    impl NumberConversion {
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::NumberConversionSoapBinding {
            bindings::NumberConversionSoapBinding::new(
                "https://www.dataaccess.com/webservicesserver/NumberConversion.wso",
                credentials,
            )
        }
    }
}
