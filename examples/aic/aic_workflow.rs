//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";

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

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        rename = "Execute",
        default
    )]
    pub struct Execute {
        #[yaserde(prefix = "tns", rename = "flowName", default)]
        pub flow_name: String,
        #[yaserde(prefix = "tns", rename = "input", default)]
        pub input: Vec<AicCouple>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        rename = "AicCouple",
        default
    )]
    pub struct AicCouple {
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: Option<String>,
        #[yaserde(prefix = "tns", rename = "value", default)]
        pub value: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        rename = "ExecuteResponse",
        default
    )]
    pub struct ExecuteResponse {
        #[yaserde(prefix = "tns", rename = "ExecuteReturn", default)]
        pub execute_return: Vec<AicCouple>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71",
        rename = "AicServiceFault",
        default
    )]
    pub struct AicServiceFault {}

    pub type Fault = AicServiceFault;
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[async_trait]
    pub trait AicWorkflow {
        async fn execute(
            &mut self,
            execute_request: ExecuteRequest,
        ) -> Result<ExecuteResponse, Option<SoapAicServiceFault>>;
    }

    pub type ExecuteRequest = messages::ExecuteRequest;
    pub type ExecuteResponse = messages::ExecuteResponse;
    pub type AicServiceFault = messages::AicServiceFault;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Fault",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SoapAicServiceFault {
        #[yaserde(rename = "faultcode", default)]
        pub fault_code: Option<String>,
        #[yaserde(rename = "faultstring", default)]
        pub fault_string: Option<String>,
        #[yaserde(rename = "AicServiceFault", default)]
        pub detail: Option<AicServiceFault>,
    }
}

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ExecuteRequest", default)]
    pub struct ExecuteRequest {
        #[yaserde(flatten)]
        pub parameters: types::Execute,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AicServiceFault", default)]
    pub struct AicServiceFault {
        #[yaserde(flatten)]
        pub fault: types::Fault,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ExecuteResponse", default)]
    pub struct ExecuteResponse {
        #[yaserde(flatten)]
        pub parameters: types::ExecuteResponse,
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl AicWorkflowSoapBinding {
        async fn send_soap_request<T: YaSerialize>(
            &mut self,
            request: &T,
            action: &str,
        ) -> (reqwest::StatusCode, String) {
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
                    Option::from(credentials.1.to_string()),
                );
            }
            let res = req.send().await.expect("can not send request");
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            (status, txt)
        }
    }
    pub struct AicWorkflowSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::AicWorkflow for AicWorkflowSoapBinding {
        async fn execute(
            &mut self,
            execute_request: ports::ExecuteRequest,
        ) -> Result<ports::ExecuteResponse, Option<ports::SoapAicServiceFault>> {
            let __request = ExecuteRequestSoapEnvelope::new(SoapExecuteRequest {
                body: execute_request,
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: ExecuteResponseSoapEnvelope = from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl Default for AicWorkflowSoapBinding {
        fn default() -> Self {
            AicWorkflowSoapBinding {
                client: reqwest::Client::new(),
                url: "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl AicWorkflowSoapBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            AicWorkflowSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapExecuteRequest {
        #[yaserde(rename = "Execute", default)]
        pub body: ports::ExecuteRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ExecuteRequestSoapEnvelope {
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
        pub body: SoapExecuteRequest,
    }

    impl ExecuteRequestSoapEnvelope {
        pub fn new(body: SoapExecuteRequest) -> Self {
            ExecuteRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapExecuteResponse {
        #[yaserde(rename = "ExecuteResponse", default)]
        pub body: ports::ExecuteResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ExecuteResponseSoapEnvelope {
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
        pub body: SoapExecuteResponse,
    }

    impl ExecuteResponseSoapEnvelope {
        pub fn new(body: SoapExecuteResponse) -> Self {
            ExecuteResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://xml.avaya.com/ws/WorkFlow/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}
