use crate::aic::{Agent, AgentChatChannel, Create};
use crate::smgr::XmlUser;

mod aic;
mod smgr;

fn main() {
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

    let xml = quick_xml::se::to_string(&c).expect("can not convert to XML");
    println!("{}", xml);

    let smu = XmlUser {
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

    let xml = quick_xml::se::to_string(&smu).expect("can not convert to XML");
    println!("{}", xml);
}
