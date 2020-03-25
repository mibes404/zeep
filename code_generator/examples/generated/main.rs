use crate::aic::bindings::{
    AicAgentAdminSoapBinding, CreateRequestSoapEnvelope, LookupAgentIdsRequestSoapEnvelope,
    LookupAgentIdsResponseSoapEnvelope, SoapCreateRequest, SoapLookupAgentIdsRequest,
    SoapLookupAgentIdsResponse,
};
use crate::aic::ports::{
    AicAgentAdmin, CreateRequest, LookupAgentIdsRequest, LookupAgentIdsResponse,
};
use crate::aic::types::{Agent, AgentChatChannel, Create, LookupAgentIds};
use crate::smgr::types::XmlUser;
use soap_client::envelop;
use soap_client::soap::SOAP_ENCODING;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod aic;
mod smgr;

#[tokio::main]
async fn main() {
    let c = Create {
        agent: Agent {
            advocate_info: None,
            basic_profile: None,
            chat_channel: Some(AgentChatChannel {
                enabled: true,
                task_ceiling: 1,
                task_load: 1,
            }),
            email_channel: None,
            extended_profile: None,
            login_id: Some("mibes".to_string()),
            security: None,
            task_load: None,
            voice_channel: None,
        },
    };

    println!("{}", to_string(&c).expect("failed to generate xml"));

    let aaa = AicAgentAdminSoapBinding::default();

    // smgr

    let xml_user = XmlUser {
        user_organization_details: vec![],
        authentication_type: vec![],
        description: "".to_string(),
        display_name: "".to_string(),
        display_name_ascii: "".to_string(),
        dn: "".to_string(),
        is_duplicated_login_allowed: false,
        is_enabled: vec![],
        is_virtual_user: false,
        given_name: vec![],
        given_name_ascii: vec![],
        honorific: "".to_string(),
        employee_no: vec![],
        department: vec![],
        organization: vec![],
        middle_name: "".to_string(),
        manager_name: "".to_string(),
        preferred_given_name: "".to_string(),
        preferred_language: "".to_string(),
        source: vec![],
        source_user_key: vec![],
        status: "".to_string(),
        suffix: "".to_string(),
        surname: vec![],
        surname_ascii: vec![],
        time_zone: "".to_string(),
        title: "".to_string(),
        user_name: vec![],
        user_password: "".to_string(),
        comm_password: "".to_string(),
        user_type: vec![],
        localized_names: vec![],
        address: vec![],
        security_identity: vec![],
        presence_user_default: Default::default(),
        presence_user_acl: vec![],
        presence_user_cl_default: vec![],
        comm_profile_set: vec![],
    };

    println!("-------");
    println!("{}", to_string(&xml_user).expect("failed to generate xml"));

    // test SOAP
    let create_request = CreateRequestSoapEnvelope {
        encoding_style: SOAP_ENCODING.to_string(),
        tnsattr: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
        urnattr: Option::None,
        xsiattr: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
        header: None,
        body: SoapCreateRequest {
            body: CreateRequest {
                parameters: Create {
                    agent: Agent {
                        advocate_info: None,
                        basic_profile: None,
                        chat_channel: None,
                        email_channel: None,
                        extended_profile: None,
                        login_id: Option::from("mibes".to_string()),
                        security: None,
                        task_load: None,
                        voice_channel: None,
                    },
                },
            },
        },
    };

    println!("-------");
    println!(
        "{}",
        to_string(&create_request).expect("failed to generate xml")
    );

    let list_request = LookupAgentIdsRequestSoapEnvelope {
        encoding_style: SOAP_ENCODING.to_string(),
        tnsattr: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
        urnattr: Option::None,
        xsiattr: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
        header: None,
        body: SoapLookupAgentIdsRequest {
            body: LookupAgentIdsRequest {
                parameters: LookupAgentIds {},
            },
        },
    };

    let body = to_string(&list_request).expect("failed to generate xml");
    println!("-------");
    println!("{}", body);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/LookupAgentIds",
        )
        .basic_auth("Admin", Option::from("Avaya123$"))
        .send()
        .await
        .expect("failed to POST to AIC");

    let status = res.status();
    let txt = res.text().await.unwrap_or_default();

    let r: LookupAgentIdsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
    println!("{}: {:?}", status, r);
}
