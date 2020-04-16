//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.1
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
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
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type User = XmlUserDelete;

    pub type DeleteType = XmlDeleteType;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "deleteUsers",
        namespace = "tns: http://xml.avaya.com/schema/bulkdelete",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteUsers {
        #[yaserde(rename = "deleteType", default)]
        pub delete_type: XmlDeleteType,
        #[yaserde(rename = "user", default)]
        pub user: Vec<XmlUserDelete>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlUserDelete")]
    pub struct XmlUserDelete {
        #[yaserde(rename = "loginName", default)]
        pub login_name: String,
        #[yaserde(rename = "id", default)]
        pub id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlDeleteType")]
    pub struct XmlDeleteType {
        #[yaserde(text, default)]
        pub body: String,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}
