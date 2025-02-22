//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.2.0
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]
#![allow(unused_qualifications)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_71 {
    use super::*;
    pub type Get = String;
    pub type GetResponse = String;
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAdvocateInfo")]
    pub struct AgentAdvocateInfo {
        #[yaserde(prefix = "71", rename = "LRMID")]
        pub lrmid: String,
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "telephonyLinkGroup")]
        pub telephony_link_group: String,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "ArrayOf_xsd_string")]
    pub struct ArrayOfXsdString {
        #[yaserde(prefix = "71", rename = "item")]
        pub item: Vec<String>,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentBasicProfile")]
    pub struct AgentBasicProfile {
        #[yaserde(prefix = "71", rename = "domain")]
        pub domain: String,
        #[yaserde(prefix = "71", rename = "employeeId")]
        pub employee_id: String,
        #[yaserde(prefix = "71", rename = "externalAgent")]
        pub external_agent: bool,
        #[yaserde(prefix = "71", rename = "firstName")]
        pub first_name: String,
        #[yaserde(prefix = "71", rename = "lastName")]
        pub last_name: String,
        #[yaserde(prefix = "71", rename = "middleName")]
        pub middle_name: String,
        #[yaserde(prefix = "71", rename = "outOfOffice")]
        pub out_of_office: bool,
        #[yaserde(prefix = "71", rename = "preferredName")]
        pub preferred_name: String,
        #[yaserde(prefix = "71", rename = "site")]
        pub site: String,
        #[yaserde(prefix = "71", rename = "softwareAgent")]
        pub software_agent: bool,
        #[yaserde(prefix = "71", rename = "title")]
        pub title: String,
        #[yaserde(prefix = "71", rename = "userAddressable")]
        pub user_addressable: bool,
        #[yaserde(prefix = "71", rename = "workgroups")]
        pub workgroups: mod_71::ArrayOfXsdString,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentChatChannel")]
    pub struct AgentChatChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentEmailChannel")]
    pub struct AgentEmailChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "fromAddress")]
        pub from_address: String,
        #[yaserde(prefix = "71", rename = "showFullHeader")]
        pub show_full_header: bool,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAddressInfo")]
    pub struct AgentAddressInfo {
        #[yaserde(prefix = "71", rename = "POBox")]
        pub po_box: String,
        #[yaserde(prefix = "71", rename = "address1")]
        pub address_1: String,
        #[yaserde(prefix = "71", rename = "address2")]
        pub address_2: String,
        #[yaserde(prefix = "71", rename = "building")]
        pub building: String,
        #[yaserde(prefix = "71", rename = "city")]
        pub city: String,
        #[yaserde(prefix = "71", rename = "company")]
        pub company: String,
        #[yaserde(prefix = "71", rename = "countryOrRegion")]
        pub country_or_region: String,
        #[yaserde(prefix = "71", rename = "mailStop")]
        pub mail_stop: String,
        #[yaserde(prefix = "71", rename = "stateOrProvince")]
        pub state_or_province: String,
        #[yaserde(prefix = "71", rename = "zipOrPostalCode")]
        pub zip_or_postal_code: String,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAddress")]
    pub struct AgentAddress {
        #[yaserde(prefix = "71", rename = "home")]
        pub home: mod_71::AgentAddressInfo,
        #[yaserde(prefix = "71", rename = "office")]
        pub office: mod_71::AgentAddressInfo,
        #[yaserde(prefix = "71", rename = "other")]
        pub other: mod_71::AgentAddressInfo,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentEmail")]
    pub struct AgentEmail {
        #[yaserde(prefix = "71", rename = "internal")]
        pub internal: String,
        #[yaserde(prefix = "71", rename = "mobileDevice")]
        pub mobile_device: String,
        #[yaserde(prefix = "71", rename = "personal")]
        pub personal: String,
        #[yaserde(prefix = "71", rename = "primary")]
        pub primary: String,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentPhoneInfo")]
    pub struct AgentPhoneInfo {
        #[yaserde(prefix = "71", rename = "extension")]
        pub extension: String,
        #[yaserde(prefix = "71", rename = "phoneNumber")]
        pub phone_number: String,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentPhone")]
    pub struct AgentPhone {
        #[yaserde(prefix = "71", rename = "fax")]
        pub fax: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "home")]
        pub home: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "mobile")]
        pub mobile: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "pager")]
        pub pager: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "primary")]
        pub primary: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "secondary")]
        pub secondary: mod_71::AgentPhoneInfo,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentExtendedProfile")]
    pub struct AgentExtendedProfile {
        #[yaserde(prefix = "71", rename = "address")]
        pub address: mod_71::AgentAddress,
        #[yaserde(prefix = "71", rename = "email")]
        pub email: mod_71::AgentEmail,
        #[yaserde(prefix = "71", rename = "phone")]
        pub phone: mod_71::AgentPhone,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentSecurity")]
    pub struct AgentSecurity {
        #[yaserde(prefix = "71", rename = "disableLogin")]
        pub disable_login: bool,
        #[yaserde(prefix = "71", rename = "forcePwdChange")]
        pub force_pwd_change: bool,
        #[yaserde(prefix = "71", rename = "password")]
        pub password: String,
        #[yaserde(prefix = "71", rename = "roleAdmin")]
        pub role_admin: bool,
        #[yaserde(prefix = "71", rename = "roleAgent")]
        pub role_agent: bool,
        #[yaserde(prefix = "71", rename = "roleClerk")]
        pub role_clerk: bool,
        #[yaserde(prefix = "71", rename = "roleEditor")]
        pub role_editor: bool,
        #[yaserde(prefix = "71", rename = "roleOperator")]
        pub role_operator: bool,
        #[yaserde(prefix = "71", rename = "rolePostmaster")]
        pub role_postmaster: bool,
        #[yaserde(prefix = "71", rename = "roleSupervisor")]
        pub role_supervisor: bool,
        #[yaserde(prefix = "71", rename = "roleSupport")]
        pub role_support: bool,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentTaskLoad")]
    pub struct AgentTaskLoad {
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentVoiceChannel")]
    pub struct AgentVoiceChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "equipment")]
        pub equipment: String,
        #[yaserde(prefix = "71", rename = "password")]
        pub password: String,
        #[yaserde(prefix = "71", rename = "phoneId")]
        pub phone_id: String,
        #[yaserde(prefix = "71", rename = "phoneType")]
        pub phone_type: String,
        #[yaserde(prefix = "71", rename = "queue")]
        pub queue: String,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Agent")]
    pub struct Agent {
        #[yaserde(prefix = "71", rename = "advocateInfo")]
        pub advocate_info: mod_71::AgentAdvocateInfo,
        #[yaserde(prefix = "71", rename = "basicProfile")]
        pub basic_profile: mod_71::AgentBasicProfile,
        #[yaserde(prefix = "71", rename = "chatChannel")]
        pub chat_channel: mod_71::AgentChatChannel,
        #[yaserde(prefix = "71", rename = "emailChannel")]
        pub email_channel: mod_71::AgentEmailChannel,
        #[yaserde(prefix = "71", rename = "extendedProfile")]
        pub extended_profile: mod_71::AgentExtendedProfile,
        #[yaserde(prefix = "71", rename = "loginId")]
        pub login_id: String,
        #[yaserde(prefix = "71", rename = "security")]
        pub security: mod_71::AgentSecurity,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: mod_71::AgentTaskLoad,
        #[yaserde(prefix = "71", rename = "voiceChannel")]
        pub voice_channel: mod_71::AgentVoiceChannel,
    }
    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AicServiceFault")]
    pub struct AicServiceFault {}
    pub type Fault = mod_71::AicServiceFault;
    pub type Update = String;
    pub type UpdateResponse = String;
    pub type Delete = String;
    pub type DeleteResponse = String;
    pub type LookupAgentIds = String;
    pub type LookupAgentIdsResponse = String;
    pub type LookupLRMIds = String;
    pub type LookupLRMIdsResponse = String;
    pub type LookupWorkgroups = String;
    pub type LookupWorkgroupsResponse = String;
    pub type LookupDomains = String;
    pub type LookupDomainsResponse = String;
    pub type LookupLinkGroups = String;
    pub type LookupLinkGroupsResponse = String;
    pub type LookupPhoneTypes = String;
    pub type LookupPhoneTypesResponse = String;
    pub type LookupSites = String;
    pub type LookupSitesResponse = String;
    pub type Create = String;
    pub type CreateResponse = String;
}

/* Delete */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Delete")]
    pub delete: mod_71::Delete,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "DeleteResponse")]
    pub delete_response: mod_71::DeleteResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteOutputEnvelopeBody,
}

/* LookupAgentIds */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupAgentIds")]
    pub lookup_agent_ids: mod_71::LookupAgentIds,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupAgentIdsInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupAgentIdsResponse")]
    pub lookup_agent_ids_response: mod_71::LookupAgentIdsResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupAgentIdsOutputEnvelopeBody,
}

/* Get */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Get")]
    pub get: mod_71::Get,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "GetResponse")]
    pub get_response: mod_71::GetResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetOutputEnvelopeBody,
}

/* LookupWorkgroups */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupWorkgroups")]
    pub lookup_workgroups: mod_71::LookupWorkgroups,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupWorkgroupsInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupWorkgroupsResponse")]
    pub lookup_workgroups_response: mod_71::LookupWorkgroupsResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupWorkgroupsOutputEnvelopeBody,
}

/* LookupPhoneTypes */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupPhoneTypes")]
    pub lookup_phone_types: mod_71::LookupPhoneTypes,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupPhoneTypesInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupPhoneTypesResponse")]
    pub lookup_phone_types_response: mod_71::LookupPhoneTypesResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupPhoneTypesOutputEnvelopeBody,
}

/* LookupDomains */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupDomains")]
    pub lookup_domains: mod_71::LookupDomains,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupDomainsInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupDomainsResponse")]
    pub lookup_domains_response: mod_71::LookupDomainsResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupDomainsOutputEnvelopeBody,
}

/* LookupLRMIds */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLRMIds")]
    pub lookup_lrm_ids: mod_71::LookupLRMIds,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLRMIdsInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLRMIdsResponse")]
    pub lookup_lrm_ids_response: mod_71::LookupLRMIdsResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLRMIdsOutputEnvelopeBody,
}

/* LookupSites */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupSites")]
    pub lookup_sites: mod_71::LookupSites,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupSitesInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupSitesResponse")]
    pub lookup_sites_response: mod_71::LookupSitesResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupSitesOutputEnvelopeBody,
}

/* LookupLinkGroups */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLinkGroups")]
    pub lookup_link_groups: mod_71::LookupLinkGroups,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLinkGroupsInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLinkGroupsResponse")]
    pub lookup_link_groups_response: mod_71::LookupLinkGroupsResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLinkGroupsOutputEnvelopeBody,
}

/* Create */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Create")]
    pub create: mod_71::Create,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "CreateResponse")]
    pub create_response: mod_71::CreateResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateOutputEnvelopeBody,
}

/* Update */

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Update")]
    pub update: mod_71::Update,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateInputEnvelopeBody,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "UpdateResponse")]
    pub update_response: mod_71::UpdateResponse,
}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateOutputEnvelopeBody,
}
pub struct AicAgentAdminService {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl AicAgentAdminService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "http://aiccore.avayacloud.com:9800/webservices/services/AicAgentAdmin".to_string(),
            credentials,
        }
    }

    pub async fn delete(&self, req: DeleteInputEnvelope) -> error::SoapResult<DeleteOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_agent_ids(
        &self,
        req: LookupAgentIdsInputEnvelope,
    ) -> error::SoapResult<LookupAgentIdsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get(&self, req: GetInputEnvelope) -> error::SoapResult<GetOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_workgroups(
        &self,
        req: LookupWorkgroupsInputEnvelope,
    ) -> error::SoapResult<LookupWorkgroupsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_phone_types(
        &self,
        req: LookupPhoneTypesInputEnvelope,
    ) -> error::SoapResult<LookupPhoneTypesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_domains(
        &self,
        req: LookupDomainsInputEnvelope,
    ) -> error::SoapResult<LookupDomainsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_lrm_ids(
        &self,
        req: LookupLRMIdsInputEnvelope,
    ) -> error::SoapResult<LookupLRMIdsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_sites(&self, req: LookupSitesInputEnvelope) -> error::SoapResult<LookupSitesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_link_groups(
        &self,
        req: LookupLinkGroupsInputEnvelope,
    ) -> error::SoapResult<LookupLinkGroupsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn create(&self, req: CreateInputEnvelope) -> error::SoapResult<CreateOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update(&self, req: UpdateInputEnvelope) -> error::SoapResult<UpdateOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }
}
pub mod error {
    #![allow(dead_code)]

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
    #![allow(dead_code)]

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
