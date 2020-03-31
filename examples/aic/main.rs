use crate::aic_agent::bindings::{
    AicAgentAdminSoapBinding, CreateRequestSoapEnvelope, LookupAgentIdsRequestSoapEnvelope,
    SoapCreateRequest, SoapLookupAgentIdsRequest, SoapLookupAgentIdsResponse,
};
use crate::aic_agent::messages::{GetRequest, UpdateRequest};
use crate::aic_agent::ports::{
    AicAgentAdmin, CreateRequest, LookupAgentIdsRequest, LookupAgentIdsResponse,
};
use crate::aic_agent::types::{
    Agent, AgentAdvocateInfo, AgentBasicProfile, AgentChatChannel, Create, Get, LookupAgentIds,
    Update,
};
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod aic_agent;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

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
        xmlns: None,
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
        xmlns: None,
    });

    let body = to_string(&list_request).expect("failed to generate xml");
    println!("-------");
    println!("{}", body);

    let aic = AicAgentAdminSoapBinding::new(
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

    let claire = aic
        .get(GetRequest {
            parameters: Get {
                login_id: "claire".to_string(),
            },
        })
        .await
        .expect("can not get claire");

    println!("{:?}", claire);

    let not_claire = aic
        .get(GetRequest {
            parameters: Get {
                login_id: "not_claire".to_string(),
            },
        })
        .await;

    if let Err(Some(err)) = not_claire {
        println!("{:?}", err.fault_string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aic_agent::bindings::LookupAgentIdsResponseSoapEnvelope;
    use crate::aic_agent::types;

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

        let t2 = LookupAgentIdsResponseSoapEnvelope::new(SoapLookupAgentIdsResponse {
            body: t,
            fault: Option::None,
        });

        let b3 = to_string(&t2).expect("failed to generate xml");
        println!("{:?}", b3);

        let b: LookupAgentIdsResponse = from_str(&body).expect("can not unmarshal");
        let r: LookupAgentIdsResponseSoapEnvelope = from_str(&xml).expect("can not unmarshal");

        println!("{:?}", r);
    }
}
