use crate::xsd::{Agent, AgentChatChannel, Create};

mod xsd;

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

    let xml = serde_xml_rs::to_string(&c).expect("can not convert to XML");
    println!("{}", xml);
}
