//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.2.0
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_tem {
    use super::{Write, YaDeserialize, YaSerialize};
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitRequest")]
    pub struct CelsiusToFahrenheitRequest {
        #[yaserde(prefix = "tem", rename = "TemperatureInCelsius")]
        pub temperature_in_celsius: f64,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitResponse")]
    pub struct CelsiusToFahrenheitResponse {
        #[yaserde(prefix = "tem", rename = "TemperatureInFahrenheit")]
        pub temperature_in_fahrenheit: f64,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusRequest")]
    pub struct FahrenheitToCelsiusRequest {
        #[yaserde(prefix = "tem", rename = "TemperatureInFahrenheit")]
        pub temperature_in_fahrenheit: f64,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusResponse")]
    pub struct FahrenheitToCelsiusResponse {
        #[yaserde(prefix = "tem", rename = "TemperatureInCelsius")]
        pub temperature_in_celsius: f64,
    }
}

/* FahrenheitToCelsius */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusInputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "FahrenheitToCelsiusRequest")]
    pub fahrenheit_to_celsius_request: mod_tem::FahrenheitToCelsiusRequest,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: FahrenheitToCelsiusInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusOutputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "FahrenheitToCelsiusResponse")]
    pub fahrenheit_to_celsius_response: mod_tem::FahrenheitToCelsiusResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: FahrenheitToCelsiusOutputEnvelopeBody,
}

/* CelsiusToFahrenheit */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitInputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "CelsiusToFahrenheitRequest")]
    pub celsius_to_fahrenheit_request: mod_tem::CelsiusToFahrenheitRequest,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CelsiusToFahrenheitInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitOutputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "CelsiusToFahrenheitResponse")]
    pub celsius_to_fahrenheit_response: mod_tem::CelsiusToFahrenheitResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CelsiusToFahrenheitOutputEnvelopeBody,
}
pub struct TempConverterEndpointService {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl TempConverterEndpointService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "https://apps.learnwebservices.com/services/tempconverter".to_string(),
            credentials,
        }
    }

    pub async fn fahrenheit_to_celsius(
        &self,
        req: FahrenheitToCelsiusInputEnvelope,
    ) -> error::SoapResult<FahrenheitToCelsiusOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn celsius_to_fahrenheit(
        &self,
        req: CelsiusToFahrenheitInputEnvelope,
    ) -> error::SoapResult<CelsiusToFahrenheitOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }
}
pub mod error {
    use std::error::Error;

    #[derive(Debug)]
    pub enum SoapError {
        YaserdeError(String),
        Http(reqwest::Error),
    }

    pub type SoapResult<T> = Result<T, SoapError>;

    impl std::fmt::Display for SoapError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SoapError::YaserdeError(e) => write!(f, "Yaserde error: {e}"),
                SoapError::Http(e) => write!(f, "HTTP error: {e}"),
            }
        }
    }

    impl Error for SoapError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                SoapError::YaserdeError(_) => None,
                SoapError::Http(e) => Some(e),
            }
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }

    impl From<reqwest::Error> for SoapError {
        fn from(e: reqwest::Error) -> Self {
            SoapError::Http(e)
        }
    }
}

mod helpers {
    use super::error::{SoapError, SoapResult};
    use reqwest::Client;
    use std::fmt;
    use yaserde::{YaDeserialize, YaSerialize};

    pub(super) async fn send_soap_request<YI, YO, U, P>(
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        let client = Client::new();
        send_soap_request_using_client(&client, url, credentials, req).await
    }

    pub(super) async fn send_soap_request_using_client<YI, YO, U, P>(
        client: &Client,
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        let body = yaserde::ser::to_string(&req).map_err(SoapError::YaserdeError)?;
        let mut req = client.post(url).body(body);
        if let Some((username, password)) = credentials {
            req = req.basic_auth(username, Some(password));
        }
        let res = req.send().await?;
        res.error_for_status_ref()?;
        let response_body = res.text().await?;
        let response = yaserde::de::from_str(&response_body).map_err(SoapError::YaserdeError)?;
        Ok(response)
    }
}

/// This module contains the `MultiRef` type which is a wrapper around `Arc<RwLock<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
/// Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
/// Needs `xml-rs`, `tokio` and `yaserde` as dependencies.
pub mod multi_ref {
    use std::{ops::Deref, sync::Arc};
    use tokio::sync::RwLock;
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Arc<RwLock<T>>,
    }

    impl<T: YaDeserialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Arc::new(RwLock::new(inner)),
            })
        }
    }

    impl<T: YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.blocking_write().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
            self.inner.blocking_read().serialize_attributes(attributes, namespace)
        }
    }

    impl<T: Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self { inner: Arc::default() }
        }
    }

    impl<T: Clone> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.blocking_read().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Arc<RwLock<T>>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
