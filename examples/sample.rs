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
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_tem {
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitRequest")]
struct celsiusToFahrenheitRequest {
    #[yaserde(rename = "TemperatureInCelsius")]
    pub temperature_in_celsius: f64,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitResponse")]
struct celsiusToFahrenheitResponse {
    #[yaserde(rename = "TemperatureInFahrenheit")]
    pub temperature_in_fahrenheit: f64,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusRequest")]
struct fahrenheitToCelsiusRequest {
    #[yaserde(rename = "TemperatureInFahrenheit")]
    pub temperature_in_fahrenheit: f64,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusResponse")]
struct fahrenheitToCelsiusResponse {
    #[yaserde(rename = "TemperatureInCelsius")]
    pub temperature_in_celsius: f64,
}
}

/* CelsiusToFahrenheit */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct CelsiusToFahrenheitInputEnvelope {
    #[yaserde(rename = "Body")]
    pub body: CelsiusToFahrenheitRequest,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct CelsiusToFahrenheitOutputEnvelope {
    #[yaserde(rename = "Body")]
    pub body: CelsiusToFahrenheitResponse,
}

/* FahrenheitToCelsius */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct FahrenheitToCelsiusInputEnvelope {
    #[yaserde(rename = "Body")]
    pub body: FahrenheitToCelsiusRequest,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct FahrenheitToCelsiusOutputEnvelope {
    #[yaserde(rename = "Body")]
    pub body: FahrenheitToCelsiusResponse,
}
pub struct TempConverterEndpointService {
    pub client: reqwest::Client,
    pub location: reqwest::Url,
    pub credentials: Option<(String, String)>,
}
impl TempConverterEndpointService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: reqwest::Url::parse("https://apps.learnwebservices.com/services/tempconverter").expect("invalid url"),
            credentials,
        }
    }
async fn celsius_to_fahrenheit(&self, req: CelsiusToFahrenheitInputEnvelope) -> error::SoapResult<CelsiusToFahrenheitOutputEnvelope> {
    helpers::send_soap_request(self.action, req, self.credentials).await
}
async fn fahrenheit_to_celsius(&self, req: FahrenheitToCelsiusInputEnvelope) -> error::SoapResult<FahrenheitToCelsiusOutputEnvelope> {
    helpers::send_soap_request(self.action, req, self.credentials).await
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
    use yaserde::{YaDeserialize, YaSerialize};

    async fn send_soap_request<YI, YO>(url: &str, credentials: Option<(String, String)>, req: YI) -> SoapResult<YO>
    where
        YI: YaSerialize,
        YO: YaDeserialize,
    {
        let client = Client::new();
        send_soap_request_using_client(client, url, credentials, req).await
    }

    async fn send_soap_request_using_client<YI, YO>(
        client: Client,
        url: &str,
        credentials: Option<(String, String)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize,
        YO: YaDeserialize,
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
