//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.11
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupPhoneTypesRequest")]
    pub struct LookupPhoneTypesRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupPhoneTypes,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupLinkGroupsRequest")]
    pub struct LookupLinkGroupsRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupLinkGroups,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupLRMIdsResponse")]
    pub struct LookupLRMIdsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupLRMIdsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateRequest")]
    pub struct CreateRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::Create,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateResponse")]
    pub struct CreateResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::CreateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateRequest")]
    pub struct UpdateRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::Update,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupSitesRequest")]
    pub struct LookupSitesRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupSites,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupDomainsRequest")]
    pub struct LookupDomainsRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupDomains,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupDomainsResponse")]
    pub struct LookupDomainsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupDomainsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetResponse")]
    pub struct GetResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::GetResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "DeleteResponse")]
    pub struct DeleteResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::DeleteResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupLRMIdsRequest")]
    pub struct LookupLRMIdsRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupLRMIds,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupLinkGroupsResponse")]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupLinkGroupsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateResponse")]
    pub struct UpdateResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetRequest")]
    pub struct GetRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::Get,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupAgentIdsResponse")]
    pub struct LookupAgentIdsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupAgentIdsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupPhoneTypesResponse")]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupPhoneTypesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupAgentIdsRequest")]
    pub struct LookupAgentIdsRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupAgentIds,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupSitesResponse")]
    pub struct LookupSitesResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupSitesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupWorkgroupsResponse")]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupWorkgroupsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AicServiceFault")]
    pub struct AicServiceFault {
        #[yaserde(flatten, default)]
        pub fault: types::Fault,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "DeleteRequest")]
    pub struct DeleteRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::Delete,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "LookupWorkgroupsRequest")]
    pub struct LookupWorkgroupsRequest {
        #[yaserde(flatten, default)]
        pub parameters: types::LookupWorkgroups,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Get",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Get {
        #[yaserde(rename = "loginId", prefix = "tns", default)]
        pub login_id: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetResponse {
        #[yaserde(rename = "GetReturn", prefix = "tns", default)]
        pub get_return: Agent,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentAdvocateInfo",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentAdvocateInfo {
        #[yaserde(rename = "LRMID", prefix = "tns", default)]
        pub lrmid: Option<String>,
        #[yaserde(rename = "enabled", prefix = "tns", default)]
        pub enabled: bool,
        #[yaserde(rename = "telephonyLinkGroup", prefix = "tns", default)]
        pub telephony_link_group: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOf_xsd_string",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct ArrayOfXsdString {
        #[yaserde(rename = "item", prefix = "tns", default)]
        pub item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentBasicProfile",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentBasicProfile {
        #[yaserde(rename = "domain", prefix = "tns", default)]
        pub domain: Option<String>,
        #[yaserde(rename = "employeeId", prefix = "tns", default)]
        pub employee_id: Option<String>,
        #[yaserde(rename = "externalAgent", prefix = "tns", default)]
        pub external_agent: bool,
        #[yaserde(rename = "firstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "lastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "middleName", prefix = "tns", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "outOfOffice", prefix = "tns", default)]
        pub out_of_office: bool,
        #[yaserde(rename = "preferredName", prefix = "tns", default)]
        pub preferred_name: Option<String>,
        #[yaserde(rename = "site", prefix = "tns", default)]
        pub site: Option<String>,
        #[yaserde(rename = "softwareAgent", prefix = "tns", default)]
        pub software_agent: bool,
        #[yaserde(rename = "title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "userAddressable", prefix = "tns", default)]
        pub user_addressable: bool,
        #[yaserde(rename = "workgroups", prefix = "tns", default)]
        pub workgroups: Option<ArrayOfXsdString>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentChatChannel",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentChatChannel {
        #[yaserde(rename = "enabled", prefix = "tns", default)]
        pub enabled: bool,
        #[yaserde(rename = "taskCeiling", prefix = "tns", default)]
        pub task_ceiling: i16,
        #[yaserde(rename = "taskLoad", prefix = "tns", default)]
        pub task_load: i16,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentEmailChannel",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentEmailChannel {
        #[yaserde(rename = "enabled", prefix = "tns", default)]
        pub enabled: bool,
        #[yaserde(rename = "fromAddress", prefix = "tns", default)]
        pub from_address: Option<String>,
        #[yaserde(rename = "showFullHeader", prefix = "tns", default)]
        pub show_full_header: bool,
        #[yaserde(rename = "taskCeiling", prefix = "tns", default)]
        pub task_ceiling: i16,
        #[yaserde(rename = "taskLoad", prefix = "tns", default)]
        pub task_load: i16,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentAddressInfo",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentAddressInfo {
        #[yaserde(rename = "POBox", prefix = "tns", default)]
        pub po_box: Option<String>,
        #[yaserde(rename = "address1", prefix = "tns", default)]
        pub address_1: Option<String>,
        #[yaserde(rename = "address2", prefix = "tns", default)]
        pub address_2: Option<String>,
        #[yaserde(rename = "building", prefix = "tns", default)]
        pub building: Option<String>,
        #[yaserde(rename = "city", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "company", prefix = "tns", default)]
        pub company: Option<String>,
        #[yaserde(rename = "countryOrRegion", prefix = "tns", default)]
        pub country_or_region: Option<String>,
        #[yaserde(rename = "mailStop", prefix = "tns", default)]
        pub mail_stop: Option<String>,
        #[yaserde(rename = "stateOrProvince", prefix = "tns", default)]
        pub state_or_province: Option<String>,
        #[yaserde(rename = "zipOrPostalCode", prefix = "tns", default)]
        pub zip_or_postal_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentAddress",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentAddress {
        #[yaserde(rename = "home", prefix = "tns", default)]
        pub home: Option<AgentAddressInfo>,
        #[yaserde(rename = "office", prefix = "tns", default)]
        pub office: Option<AgentAddressInfo>,
        #[yaserde(rename = "other", prefix = "tns", default)]
        pub other: Option<AgentAddressInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentEmail",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentEmail {
        #[yaserde(rename = "internal", prefix = "tns", default)]
        pub internal: Option<String>,
        #[yaserde(rename = "mobileDevice", prefix = "tns", default)]
        pub mobile_device: Option<String>,
        #[yaserde(rename = "personal", prefix = "tns", default)]
        pub personal: Option<String>,
        #[yaserde(rename = "primary", prefix = "tns", default)]
        pub primary: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentPhoneInfo",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentPhoneInfo {
        #[yaserde(rename = "extension", prefix = "tns", default)]
        pub extension: Option<String>,
        #[yaserde(rename = "phoneNumber", prefix = "tns", default)]
        pub phone_number: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentPhone",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentPhone {
        #[yaserde(rename = "fax", prefix = "tns", default)]
        pub fax: Option<AgentPhoneInfo>,
        #[yaserde(rename = "home", prefix = "tns", default)]
        pub home: Option<AgentPhoneInfo>,
        #[yaserde(rename = "mobile", prefix = "tns", default)]
        pub mobile: Option<AgentPhoneInfo>,
        #[yaserde(rename = "pager", prefix = "tns", default)]
        pub pager: Option<AgentPhoneInfo>,
        #[yaserde(rename = "primary", prefix = "tns", default)]
        pub primary: Option<AgentPhoneInfo>,
        #[yaserde(rename = "secondary", prefix = "tns", default)]
        pub secondary: Option<AgentPhoneInfo>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentExtendedProfile",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentExtendedProfile {
        #[yaserde(rename = "address", prefix = "tns", default)]
        pub address: Option<AgentAddress>,
        #[yaserde(rename = "email", prefix = "tns", default)]
        pub email: Option<AgentEmail>,
        #[yaserde(rename = "phone", prefix = "tns", default)]
        pub phone: Option<AgentPhone>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentSecurity",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentSecurity {
        #[yaserde(rename = "disableLogin", prefix = "tns", default)]
        pub disable_login: bool,
        #[yaserde(rename = "forcePwdChange", prefix = "tns", default)]
        pub force_pwd_change: bool,
        #[yaserde(rename = "password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "roleAdmin", prefix = "tns", default)]
        pub role_admin: bool,
        #[yaserde(rename = "roleAgent", prefix = "tns", default)]
        pub role_agent: bool,
        #[yaserde(rename = "roleClerk", prefix = "tns", default)]
        pub role_clerk: bool,
        #[yaserde(rename = "roleEditor", prefix = "tns", default)]
        pub role_editor: bool,
        #[yaserde(rename = "roleOperator", prefix = "tns", default)]
        pub role_operator: bool,
        #[yaserde(rename = "rolePostmaster", prefix = "tns", default)]
        pub role_postmaster: bool,
        #[yaserde(rename = "roleSupervisor", prefix = "tns", default)]
        pub role_supervisor: bool,
        #[yaserde(rename = "roleSupport", prefix = "tns", default)]
        pub role_support: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentTaskLoad",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentTaskLoad {
        #[yaserde(rename = "taskCeiling", prefix = "tns", default)]
        pub task_ceiling: i16,
        #[yaserde(rename = "taskLoad", prefix = "tns", default)]
        pub task_load: i16,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AgentVoiceChannel",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AgentVoiceChannel {
        #[yaserde(rename = "enabled", prefix = "tns", default)]
        pub enabled: bool,
        #[yaserde(rename = "equipment", prefix = "tns", default)]
        pub equipment: Option<String>,
        #[yaserde(rename = "password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "phoneId", prefix = "tns", default)]
        pub phone_id: Option<String>,
        #[yaserde(rename = "phoneType", prefix = "tns", default)]
        pub phone_type: Option<String>,
        #[yaserde(rename = "queue", prefix = "tns", default)]
        pub queue: Option<String>,
        #[yaserde(rename = "taskCeiling", prefix = "tns", default)]
        pub task_ceiling: i16,
        #[yaserde(rename = "taskLoad", prefix = "tns", default)]
        pub task_load: i16,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Agent",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct Agent {
        #[yaserde(rename = "advocateInfo", prefix = "tns", default)]
        pub advocate_info: Option<AgentAdvocateInfo>,
        #[yaserde(rename = "basicProfile", prefix = "tns", default)]
        pub basic_profile: Option<AgentBasicProfile>,
        #[yaserde(rename = "chatChannel", prefix = "tns", default)]
        pub chat_channel: Option<AgentChatChannel>,
        #[yaserde(rename = "emailChannel", prefix = "tns", default)]
        pub email_channel: Option<AgentEmailChannel>,
        #[yaserde(rename = "extendedProfile", prefix = "tns", default)]
        pub extended_profile: Option<AgentExtendedProfile>,
        #[yaserde(rename = "loginId", prefix = "tns", default)]
        pub login_id: Option<String>,
        #[yaserde(rename = "security", prefix = "tns", default)]
        pub security: Option<AgentSecurity>,
        #[yaserde(rename = "taskLoad", prefix = "tns", default)]
        pub task_load: Option<AgentTaskLoad>,
        #[yaserde(rename = "voiceChannel", prefix = "tns", default)]
        pub voice_channel: Option<AgentVoiceChannel>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AicServiceFault",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        prefix = "tns"
    )]
    pub struct AicServiceFault {}
    pub type Fault = AicServiceFault;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Update",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Update {
        #[yaserde(rename = "agent", prefix = "tns", default)]
        pub agent: Agent,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateResponse {
        #[yaserde(rename = "UpdateReturn", prefix = "tns", default)]
        pub update_return: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Delete",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Delete {
        #[yaserde(rename = "loginId", prefix = "tns", default)]
        pub login_id: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeleteResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteResponse {
        #[yaserde(rename = "DeleteReturn", prefix = "tns", default)]
        pub delete_return: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupAgentIds",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupAgentIds {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupAgentIdsResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupAgentIdsResponse {
        #[yaserde(rename = "LookupAgentIdsReturn", prefix = "tns", default)]
        pub lookup_agent_ids_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupLRMIds",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupLRMIds {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupLRMIdsResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupLRMIdsResponse {
        #[yaserde(rename = "LookupLRMIdsReturn", prefix = "tns", default)]
        pub lookup_lrm_ids_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupWorkgroups",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupWorkgroups {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupWorkgroupsResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(rename = "LookupWorkgroupsReturn", prefix = "tns", default)]
        pub lookup_workgroups_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupDomains",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupDomains {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupDomainsResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupDomainsResponse {
        #[yaserde(rename = "LookupDomainsReturn", prefix = "tns", default)]
        pub lookup_domains_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupLinkGroups",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupLinkGroups {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupLinkGroupsResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(rename = "LookupLinkGroupsReturn", prefix = "tns", default)]
        pub lookup_link_groups_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupPhoneTypes",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupPhoneTypes {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupPhoneTypesResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(rename = "LookupPhoneTypesReturn", prefix = "tns", default)]
        pub lookup_phone_types_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupSites",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupSites {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "LookupSitesResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct LookupSitesResponse {
        #[yaserde(rename = "LookupSitesReturn", prefix = "tns", default)]
        pub lookup_sites_return: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Create",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct Create {
        #[yaserde(rename = "agent", prefix = "tns", default)]
        pub agent: Agent,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateResponse",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CreateResponse {
        #[yaserde(rename = "CreateReturn", prefix = "tns", default)]
        pub create_return: bool,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Fault",
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
    pub type GetRequest = messages::GetRequest;

    pub type GetResponse = messages::GetResponse;

    pub type AicServiceFault = messages::AicServiceFault;

    pub type UpdateRequest = messages::UpdateRequest;

    pub type UpdateResponse = messages::UpdateResponse;

    pub type DeleteRequest = messages::DeleteRequest;

    pub type DeleteResponse = messages::DeleteResponse;

    pub type LookupAgentIdsRequest = messages::LookupAgentIdsRequest;

    pub type LookupAgentIdsResponse = messages::LookupAgentIdsResponse;

    pub type LookupLRMIdsRequest = messages::LookupLRMIdsRequest;

    pub type LookupLRMIdsResponse = messages::LookupLRMIdsResponse;

    pub type LookupWorkgroupsRequest = messages::LookupWorkgroupsRequest;

    pub type LookupWorkgroupsResponse = messages::LookupWorkgroupsResponse;

    pub type LookupDomainsRequest = messages::LookupDomainsRequest;

    pub type LookupDomainsResponse = messages::LookupDomainsResponse;

    pub type LookupLinkGroupsRequest = messages::LookupLinkGroupsRequest;

    pub type LookupLinkGroupsResponse = messages::LookupLinkGroupsResponse;

    pub type LookupPhoneTypesRequest = messages::LookupPhoneTypesRequest;

    pub type LookupPhoneTypesResponse = messages::LookupPhoneTypesResponse;

    pub type LookupSitesRequest = messages::LookupSitesRequest;

    pub type LookupSitesResponse = messages::LookupSitesResponse;

    pub type CreateRequest = messages::CreateRequest;

    pub type CreateResponse = messages::CreateResponse;

    #[async_trait]
    pub trait AicAgentAdmin {
        async fn get(
            &self,
            get_request: GetRequest,
        ) -> Result<GetResponse, Option<SoapAicServiceFault>>;
        async fn update(
            &self,
            update_request: UpdateRequest,
        ) -> Result<UpdateResponse, Option<SoapAicServiceFault>>;
        async fn delete(
            &self,
            delete_request: DeleteRequest,
        ) -> Result<DeleteResponse, Option<SoapAicServiceFault>>;
        async fn lookup_agent_ids(
            &self,
            lookup_agent_ids_request: LookupAgentIdsRequest,
        ) -> Result<LookupAgentIdsResponse, Option<SoapAicServiceFault>>;
        async fn lookup_lrm_ids(
            &self,
            lookup_lrm_ids_request: LookupLRMIdsRequest,
        ) -> Result<LookupLRMIdsResponse, Option<SoapAicServiceFault>>;
        async fn lookup_workgroups(
            &self,
            lookup_workgroups_request: LookupWorkgroupsRequest,
        ) -> Result<LookupWorkgroupsResponse, Option<SoapAicServiceFault>>;
        async fn lookup_domains(
            &self,
            lookup_domains_request: LookupDomainsRequest,
        ) -> Result<LookupDomainsResponse, Option<SoapAicServiceFault>>;
        async fn lookup_link_groups(
            &self,
            lookup_link_groups_request: LookupLinkGroupsRequest,
        ) -> Result<LookupLinkGroupsResponse, Option<SoapAicServiceFault>>;
        async fn lookup_phone_types(
            &self,
            lookup_phone_types_request: LookupPhoneTypesRequest,
        ) -> Result<LookupPhoneTypesResponse, Option<SoapAicServiceFault>>;
        async fn lookup_sites(
            &self,
            lookup_sites_request: LookupSitesRequest,
        ) -> Result<LookupSitesResponse, Option<SoapAicServiceFault>>;
        async fn create(
            &self,
            create_request: CreateRequest,
        ) -> Result<CreateResponse, Option<SoapAicServiceFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl AicAgentAdminSoapBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetRequest {
        #[yaserde(rename = "tns:Get", default)]
        pub body: ports::GetRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetRequest,
    }

    impl GetRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetRequest) -> Self {
            GetRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetResponse {
        #[yaserde(rename = "GetResponse", default)]
        pub body: Option<ports::GetResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetResponse,
    }

    impl GetResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetResponse) -> Self {
            GetResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateRequest {
        #[yaserde(rename = "tns:Update", default)]
        pub body: ports::UpdateRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateRequest,
    }

    impl UpdateRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateRequest) -> Self {
            UpdateRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateResponse {
        #[yaserde(rename = "UpdateResponse", default)]
        pub body: Option<ports::UpdateResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateResponse,
    }

    impl UpdateResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateResponse) -> Self {
            UpdateResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapDeleteRequest {
        #[yaserde(rename = "tns:Delete", default)]
        pub body: ports::DeleteRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapDeleteRequest,
    }

    impl DeleteRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapDeleteRequest) -> Self {
            DeleteRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapDeleteResponse {
        #[yaserde(rename = "DeleteResponse", default)]
        pub body: Option<ports::DeleteResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapDeleteResponse,
    }

    impl DeleteResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapDeleteResponse) -> Self {
            DeleteResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupAgentIdsRequest {
        #[yaserde(rename = "tns:LookupAgentIds", default)]
        pub body: ports::LookupAgentIdsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupAgentIdsRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupAgentIdsRequest,
    }

    impl LookupAgentIdsRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupAgentIdsRequest) -> Self {
            LookupAgentIdsRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupAgentIdsResponse {
        #[yaserde(rename = "LookupAgentIdsResponse", default)]
        pub body: Option<ports::LookupAgentIdsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupAgentIdsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupAgentIdsResponse,
    }

    impl LookupAgentIdsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupAgentIdsResponse) -> Self {
            LookupAgentIdsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupLRMIdsRequest {
        #[yaserde(rename = "tns:LookupLRMIds", default)]
        pub body: ports::LookupLRMIdsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLRMIdsRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupLRMIdsRequest,
    }

    impl LookupLRMIdsRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupLRMIdsRequest) -> Self {
            LookupLRMIdsRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupLRMIdsResponse {
        #[yaserde(rename = "LookupLRMIdsResponse", default)]
        pub body: Option<ports::LookupLRMIdsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLRMIdsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupLRMIdsResponse,
    }

    impl LookupLRMIdsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupLRMIdsResponse) -> Self {
            LookupLRMIdsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupWorkgroupsRequest {
        #[yaserde(rename = "tns:LookupWorkgroups", default)]
        pub body: ports::LookupWorkgroupsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupWorkgroupsRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupWorkgroupsRequest,
    }

    impl LookupWorkgroupsRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupWorkgroupsRequest) -> Self {
            LookupWorkgroupsRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupWorkgroupsResponse {
        #[yaserde(rename = "LookupWorkgroupsResponse", default)]
        pub body: Option<ports::LookupWorkgroupsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupWorkgroupsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupWorkgroupsResponse,
    }

    impl LookupWorkgroupsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupWorkgroupsResponse) -> Self {
            LookupWorkgroupsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupDomainsRequest {
        #[yaserde(rename = "tns:LookupDomains", default)]
        pub body: ports::LookupDomainsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupDomainsRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupDomainsRequest,
    }

    impl LookupDomainsRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupDomainsRequest) -> Self {
            LookupDomainsRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupDomainsResponse {
        #[yaserde(rename = "LookupDomainsResponse", default)]
        pub body: Option<ports::LookupDomainsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupDomainsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupDomainsResponse,
    }

    impl LookupDomainsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupDomainsResponse) -> Self {
            LookupDomainsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupLinkGroupsRequest {
        #[yaserde(rename = "tns:LookupLinkGroups", default)]
        pub body: ports::LookupLinkGroupsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLinkGroupsRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupLinkGroupsRequest,
    }

    impl LookupLinkGroupsRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupLinkGroupsRequest) -> Self {
            LookupLinkGroupsRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupLinkGroupsResponse {
        #[yaserde(rename = "LookupLinkGroupsResponse", default)]
        pub body: Option<ports::LookupLinkGroupsResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLinkGroupsResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupLinkGroupsResponse,
    }

    impl LookupLinkGroupsResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupLinkGroupsResponse) -> Self {
            LookupLinkGroupsResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupPhoneTypesRequest {
        #[yaserde(rename = "tns:LookupPhoneTypes", default)]
        pub body: ports::LookupPhoneTypesRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupPhoneTypesRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupPhoneTypesRequest,
    }

    impl LookupPhoneTypesRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupPhoneTypesRequest) -> Self {
            LookupPhoneTypesRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupPhoneTypesResponse {
        #[yaserde(rename = "LookupPhoneTypesResponse", default)]
        pub body: Option<ports::LookupPhoneTypesResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupPhoneTypesResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupPhoneTypesResponse,
    }

    impl LookupPhoneTypesResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupPhoneTypesResponse) -> Self {
            LookupPhoneTypesResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupSitesRequest {
        #[yaserde(rename = "tns:LookupSites", default)]
        pub body: ports::LookupSitesRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupSitesRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupSitesRequest,
    }

    impl LookupSitesRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupSitesRequest) -> Self {
            LookupSitesRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapLookupSitesResponse {
        #[yaserde(rename = "LookupSitesResponse", default)]
        pub body: Option<ports::LookupSitesResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupSitesResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapLookupSitesResponse,
    }

    impl LookupSitesResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapLookupSitesResponse) -> Self {
            LookupSitesResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateRequest {
        #[yaserde(rename = "tns:Create", default)]
        pub body: ports::CreateRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateRequestSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateRequest,
    }

    impl CreateRequestSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateRequest) -> Self {
            CreateRequestSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateResponse {
        #[yaserde(rename = "CreateResponse", default)]
        pub body: Option<ports::CreateResponse>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateResponse,
    }

    impl CreateResponseSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateResponse) -> Self {
            CreateResponseSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for AicAgentAdminSoapBinding {
        fn default() -> Self {
            AicAgentAdminSoapBinding {
                client: reqwest::Client::new(),
                url: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                credentials: None,
            }
        }
    }
    impl AicAgentAdminSoapBinding {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            AicAgentAdminSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct AicAgentAdminSoapBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::AicAgentAdmin for AicAgentAdminSoapBinding {
        async fn get(
            &self,
            get_request: ports::GetRequest,
        ) -> Result<ports::GetResponse, Option<ports::SoapAicServiceFault>> {
            let __request = GetRequestSoapEnvelope::new(SoapGetRequest {
                body: get_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: GetResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn update(
            &self,
            update_request: ports::UpdateRequest,
        ) -> Result<ports::UpdateResponse, Option<ports::SoapAicServiceFault>> {
            let __request = UpdateRequestSoapEnvelope::new(SoapUpdateRequest {
                body: update_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: UpdateResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn delete(
            &self,
            delete_request: ports::DeleteRequest,
        ) -> Result<ports::DeleteResponse, Option<ports::SoapAicServiceFault>> {
            let __request = DeleteRequestSoapEnvelope::new(SoapDeleteRequest {
                body: delete_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: DeleteResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_agent_ids(
            &self,
            lookup_agent_ids_request: ports::LookupAgentIdsRequest,
        ) -> Result<ports::LookupAgentIdsResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupAgentIdsRequestSoapEnvelope::new(SoapLookupAgentIdsRequest {
                body: lookup_agent_ids_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupAgentIdsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_lrm_ids(
            &self,
            lookup_lrm_ids_request: ports::LookupLRMIdsRequest,
        ) -> Result<ports::LookupLRMIdsResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupLRMIdsRequestSoapEnvelope::new(SoapLookupLRMIdsRequest {
                body: lookup_lrm_ids_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupLRMIdsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_workgroups(
            &self,
            lookup_workgroups_request: ports::LookupWorkgroupsRequest,
        ) -> Result<ports::LookupWorkgroupsResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupWorkgroupsRequestSoapEnvelope::new(SoapLookupWorkgroupsRequest {
                body: lookup_workgroups_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupWorkgroupsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_domains(
            &self,
            lookup_domains_request: ports::LookupDomainsRequest,
        ) -> Result<ports::LookupDomainsResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupDomainsRequestSoapEnvelope::new(SoapLookupDomainsRequest {
                body: lookup_domains_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupDomainsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_link_groups(
            &self,
            lookup_link_groups_request: ports::LookupLinkGroupsRequest,
        ) -> Result<ports::LookupLinkGroupsResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupLinkGroupsRequestSoapEnvelope::new(SoapLookupLinkGroupsRequest {
                body: lookup_link_groups_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupLinkGroupsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_phone_types(
            &self,
            lookup_phone_types_request: ports::LookupPhoneTypesRequest,
        ) -> Result<ports::LookupPhoneTypesResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupPhoneTypesRequestSoapEnvelope::new(SoapLookupPhoneTypesRequest {
                body: lookup_phone_types_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupPhoneTypesResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn lookup_sites(
            &self,
            lookup_sites_request: ports::LookupSitesRequest,
        ) -> Result<ports::LookupSitesResponse, Option<ports::SoapAicServiceFault>> {
            let __request = LookupSitesRequestSoapEnvelope::new(SoapLookupSitesRequest {
                body: lookup_sites_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: LookupSitesResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
        async fn create(
            &self,
            create_request: ports::CreateRequest,
        ) -> Result<ports::CreateResponse, Option<ports::SoapAicServiceFault>> {
            let __request = CreateRequestSoapEnvelope::new(SoapCreateRequest {
                body: create_request,
                xmlns: Some("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: CreateResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    /**
    Service to Administor Agent in Avaya Interaction Center
     */
    pub struct AicAgentAdminService {}
    impl AicAgentAdminService {
        #[must_use]
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::AicAgentAdminSoapBinding {
            Self::new_client_with_url(
                "http://aiccore.avayacloud.com:9800/webservices/services/AicAgentAdmin",
                credentials,
            )
        }

        #[must_use]
        pub fn new_client_with_url(
            url: &str,
            credentials: Option<(String, String)>,
        ) -> bindings::AicAgentAdminSoapBinding {
            bindings::AicAgentAdminSoapBinding::new(url, credentials)
        }
    }
}

pub mod multiref {
    //! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
    //! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
    //! Needs `xml-rs` and `yaserde` as dependencies.

    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(
            reader: &mut yaserde::de::Deserializer<R>,
        ) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Rc::new(RefCell::new(inner)),
            })
        }
    }

    impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.as_ref().borrow().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<
            (
                Vec<xml::attribute::OwnedAttribute>,
                xml::namespace::Namespace,
            ),
            String,
        > {
            self.inner
                .as_ref()
                .borrow()
                .serialize_attributes(attributes, namespace)
        }
    }

    impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self {
                inner: Rc::default(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.as_ref().borrow().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Rc<RefCell<T>>;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
