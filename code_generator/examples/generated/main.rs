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
    let create_request = CreateRequestSoapEnvelope::new(SoapCreateRequest {
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
    });

    println!("-------");
    println!(
        "{}",
        to_string(&create_request).expect("failed to generate xml")
    );

    let list_request = LookupAgentIdsRequestSoapEnvelope::new(SoapLookupAgentIdsRequest {
        body: LookupAgentIdsRequest {
            parameters: LookupAgentIds {},
        },
    });

    let body = to_string(&list_request).expect("failed to generate xml");
    println!("-------");
    println!("{}", body);

    let mut aic = AicAgentAdminSoapBinding::new(
        "http://localhost:9800/webservices/services/AicAgentAdmin",
        Option::from(("Admin".to_string(), "Avaya123$".to_string())),
    );

    let r = aic
        .lookup_agent_ids(LookupAgentIdsRequest {
            parameters: Default::default(),
        })
        .await
        .expect("can not lookup agents");

    println!("{:?}", r);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aic::types;

    #[test]
    fn test_unmarshal() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" 
    xmlns:xsd="http://www.w3.org/2001/XMLSchema" 
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <soapenv:Body>
        <LookupAgentIdsResponse xmlns="">
            <ns1:LookupAgentIdsReturn xmlns:ns1="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">Admin</ns1:LookupAgentIdsReturn>
            <ns2:LookupAgentIdsReturn xmlns:ns2="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">website</ns2:LookupAgentIdsReturn>
            <ns3:LookupAgentIdsReturn xmlns:ns3="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">icmbridge</ns3:LookupAgentIdsReturn>
            <ns4:LookupAgentIdsReturn xmlns:ns4="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">dcobridge1</ns4:LookupAgentIdsReturn>
            <ns5:LookupAgentIdsReturn xmlns:ns5="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">alice</ns5:LookupAgentIdsReturn>
            <ns6:LookupAgentIdsReturn xmlns:ns6="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">bob</ns6:LookupAgentIdsReturn>
            <ns7:LookupAgentIdsReturn xmlns:ns7="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">claire</ns7:LookupAgentIdsReturn>
        </LookupAgentIdsResponse>
    </soapenv:Body>
</soapenv:Envelope>"#;

        let body = r#"<LookupAgentIdsResponse xmlns="">
            <ns1:LookupAgentIdsReturn xmlns:ns1="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">Admin</ns1:LookupAgentIdsReturn>
            <ns2:LookupAgentIdsReturn xmlns:ns2="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">website</ns2:LookupAgentIdsReturn>
            <ns3:LookupAgentIdsReturn xmlns:ns3="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">icmbridge</ns3:LookupAgentIdsReturn>
            <ns4:LookupAgentIdsReturn xmlns:ns4="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">dcobridge1</ns4:LookupAgentIdsReturn>
            <ns5:LookupAgentIdsReturn xmlns:ns5="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">alice</ns5:LookupAgentIdsReturn>
            <ns6:LookupAgentIdsReturn xmlns:ns6="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">bob</ns6:LookupAgentIdsReturn>
            <ns7:LookupAgentIdsReturn xmlns:ns7="http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71">claire</ns7:LookupAgentIdsReturn>
        </LookupAgentIdsResponse>"#;

        let t = LookupAgentIdsResponse {
            parameters: types::LookupAgentIdsResponse {
                lookup_agent_ids_return: vec!["mibes".to_string(), "hmacias".to_string()],
            },
        };

        let b2 = to_string(&t).expect("failed to generate xml");
        println!("{:?}", b2);

        let t2 = LookupAgentIdsResponseSoapEnvelope::new(SoapLookupAgentIdsResponse { body: t });

        let b3 = to_string(&t2).expect("failed to generate xml");
        println!("{:?}", b3);

        let b: LookupAgentIdsResponse = from_str(&body).expect("can not unmarshal");
        let r: LookupAgentIdsResponseSoapEnvelope = from_str(&xml).expect("can not unmarshal");

        println!("{:?}", r);
    }
}
