//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!

#![allow(dead_code)]
#![allow(unused_imports)]
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

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupPhoneTypesRequest", default)]
    pub struct LookupPhoneTypesRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupPhoneTypes,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupLinkGroupsRequest", default)]
    pub struct LookupLinkGroupsRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupLinkGroups,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupLRMIdsResponse", default)]
    pub struct LookupLRMIdsResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupLRMIdsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "CreateRequest", default)]
    pub struct CreateRequest {
        #[yaserde(flatten)]
        pub parameters: types::Create,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "CreateResponse", default)]
    pub struct CreateResponse {
        #[yaserde(flatten)]
        pub parameters: types::CreateResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UpdateRequest", default)]
    pub struct UpdateRequest {
        #[yaserde(flatten)]
        pub parameters: types::Update,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupSitesRequest", default)]
    pub struct LookupSitesRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupSites,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupDomainsRequest", default)]
    pub struct LookupDomainsRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupDomains,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupDomainsResponse", default)]
    pub struct LookupDomainsResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupDomainsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetResponse", default)]
    pub struct GetResponse {
        #[yaserde(flatten)]
        pub parameters: types::GetResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "DeleteResponse", default)]
    pub struct DeleteResponse {
        #[yaserde(flatten)]
        pub parameters: types::DeleteResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupLRMIdsRequest", default)]
    pub struct LookupLRMIdsRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupLRMIds,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupLinkGroupsResponse", default)]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupLinkGroupsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UpdateResponse", default)]
    pub struct UpdateResponse {
        #[yaserde(flatten)]
        pub parameters: types::UpdateResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetRequest", default)]
    pub struct GetRequest {
        #[yaserde(flatten)]
        pub parameters: types::Get,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupAgentIdsResponse", default)]
    pub struct LookupAgentIdsResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupAgentIdsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupPhoneTypesResponse", default)]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupPhoneTypesResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupAgentIdsRequest", default)]
    pub struct LookupAgentIdsRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupAgentIds,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupSitesResponse", default)]
    pub struct LookupSitesResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupSitesResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupWorkgroupsResponse", default)]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(flatten)]
        pub parameters: types::LookupWorkgroupsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "AicServiceFault", default)]
    pub struct AicServiceFault {
        #[yaserde(flatten)]
        pub fault: types::Fault,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "DeleteRequest", default)]
    pub struct DeleteRequest {
        #[yaserde(flatten)]
        pub parameters: types::Delete,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "LookupWorkgroupsRequest", default)]
    pub struct LookupWorkgroupsRequest {
        #[yaserde(flatten)]
        pub parameters: types::LookupWorkgroups,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

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

    pub type GetRequest = messages::GetRequest;
    pub type GetResponse = messages::GetResponse;
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
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "Get",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Get {
        #[yaserde(prefix = "tns", rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetResponse {
        #[yaserde(prefix = "tns", rename = "GetReturn", default)]
        pub get_return: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentAdvocateInfo",
        default
    )]
    pub struct AgentAdvocateInfo {
        #[yaserde(prefix = "tns", rename = "LRMID", default)]
        pub lrmid: Option<String>,
        #[yaserde(prefix = "tns", rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(prefix = "tns", rename = "telephonyLinkGroup", default)]
        pub telephony_link_group: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "ArrayOf_xsd_string",
        default
    )]
    pub struct ArrayOfXsdString {
        #[yaserde(prefix = "tns", rename = "item", default)]
        pub item: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentBasicProfile",
        default
    )]
    pub struct AgentBasicProfile {
        #[yaserde(prefix = "tns", rename = "domain", default)]
        pub domain: Option<String>,
        #[yaserde(prefix = "tns", rename = "employeeId", default)]
        pub employee_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "externalAgent", default)]
        pub external_agent: bool,
        #[yaserde(prefix = "tns", rename = "firstName", default)]
        pub first_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "lastName", default)]
        pub last_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "outOfOffice", default)]
        pub out_of_office: bool,
        #[yaserde(prefix = "tns", rename = "preferredName", default)]
        pub preferred_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "site", default)]
        pub site: Option<String>,
        #[yaserde(prefix = "tns", rename = "softwareAgent", default)]
        pub software_agent: bool,
        #[yaserde(prefix = "tns", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "tns", rename = "userAddressable", default)]
        pub user_addressable: bool,
        #[yaserde(prefix = "tns", rename = "workgroups", default)]
        pub workgroups: Option<ArrayOfXsdString>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentChatChannel",
        default
    )]
    pub struct AgentChatChannel {
        #[yaserde(prefix = "tns", rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(prefix = "tns", rename = "taskCeiling", default)]
        pub task_ceiling: i16,
        #[yaserde(prefix = "tns", rename = "taskLoad", default)]
        pub task_load: i16,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentEmailChannel",
        default
    )]
    pub struct AgentEmailChannel {
        #[yaserde(prefix = "tns", rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(prefix = "tns", rename = "fromAddress", default)]
        pub from_address: Option<String>,
        #[yaserde(prefix = "tns", rename = "showFullHeader", default)]
        pub show_full_header: bool,
        #[yaserde(prefix = "tns", rename = "taskCeiling", default)]
        pub task_ceiling: i16,
        #[yaserde(prefix = "tns", rename = "taskLoad", default)]
        pub task_load: i16,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentAddressInfo",
        default
    )]
    pub struct AgentAddressInfo {
        #[yaserde(prefix = "tns", rename = "POBox", default)]
        pub po_box: Option<String>,
        #[yaserde(prefix = "tns", rename = "address1", default)]
        pub address_1: Option<String>,
        #[yaserde(prefix = "tns", rename = "address2", default)]
        pub address_2: Option<String>,
        #[yaserde(prefix = "tns", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "tns", rename = "city", default)]
        pub city: Option<String>,
        #[yaserde(prefix = "tns", rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(prefix = "tns", rename = "countryOrRegion", default)]
        pub country_or_region: Option<String>,
        #[yaserde(prefix = "tns", rename = "mailStop", default)]
        pub mail_stop: Option<String>,
        #[yaserde(prefix = "tns", rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(prefix = "tns", rename = "zipOrPostalCode", default)]
        pub zip_or_postal_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentAddress",
        default
    )]
    pub struct AgentAddress {
        #[yaserde(prefix = "tns", rename = "home", default)]
        pub home: Option<AgentAddressInfo>,
        #[yaserde(prefix = "tns", rename = "office", default)]
        pub office: Option<AgentAddressInfo>,
        #[yaserde(prefix = "tns", rename = "other", default)]
        pub other: Option<AgentAddressInfo>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentEmail",
        default
    )]
    pub struct AgentEmail {
        #[yaserde(prefix = "tns", rename = "internal", default)]
        pub internal: Option<String>,
        #[yaserde(prefix = "tns", rename = "mobileDevice", default)]
        pub mobile_device: Option<String>,
        #[yaserde(prefix = "tns", rename = "personal", default)]
        pub personal: Option<String>,
        #[yaserde(prefix = "tns", rename = "primary", default)]
        pub primary: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentPhoneInfo",
        default
    )]
    pub struct AgentPhoneInfo {
        #[yaserde(prefix = "tns", rename = "extension", default)]
        pub extension: Option<String>,
        #[yaserde(prefix = "tns", rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentPhone",
        default
    )]
    pub struct AgentPhone {
        #[yaserde(prefix = "tns", rename = "fax", default)]
        pub fax: Option<AgentPhoneInfo>,
        #[yaserde(prefix = "tns", rename = "home", default)]
        pub home: Option<AgentPhoneInfo>,
        #[yaserde(prefix = "tns", rename = "mobile", default)]
        pub mobile: Option<AgentPhoneInfo>,
        #[yaserde(prefix = "tns", rename = "pager", default)]
        pub pager: Option<AgentPhoneInfo>,
        #[yaserde(prefix = "tns", rename = "primary", default)]
        pub primary: Option<AgentPhoneInfo>,
        #[yaserde(prefix = "tns", rename = "secondary", default)]
        pub secondary: Option<AgentPhoneInfo>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentExtendedProfile",
        default
    )]
    pub struct AgentExtendedProfile {
        #[yaserde(prefix = "tns", rename = "address", default)]
        pub address: Option<AgentAddress>,
        #[yaserde(prefix = "tns", rename = "email", default)]
        pub email: Option<AgentEmail>,
        #[yaserde(prefix = "tns", rename = "phone", default)]
        pub phone: Option<AgentPhone>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentSecurity",
        default
    )]
    pub struct AgentSecurity {
        #[yaserde(prefix = "tns", rename = "disableLogin", default)]
        pub disable_login: bool,
        #[yaserde(prefix = "tns", rename = "forcePwdChange", default)]
        pub force_pwd_change: bool,
        #[yaserde(prefix = "tns", rename = "password", default)]
        pub password: Option<String>,
        #[yaserde(prefix = "tns", rename = "roleAdmin", default)]
        pub role_admin: bool,
        #[yaserde(prefix = "tns", rename = "roleAgent", default)]
        pub role_agent: bool,
        #[yaserde(prefix = "tns", rename = "roleClerk", default)]
        pub role_clerk: bool,
        #[yaserde(prefix = "tns", rename = "roleEditor", default)]
        pub role_editor: bool,
        #[yaserde(prefix = "tns", rename = "roleOperator", default)]
        pub role_operator: bool,
        #[yaserde(prefix = "tns", rename = "rolePostmaster", default)]
        pub role_postmaster: bool,
        #[yaserde(prefix = "tns", rename = "roleSupervisor", default)]
        pub role_supervisor: bool,
        #[yaserde(prefix = "tns", rename = "roleSupport", default)]
        pub role_support: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentTaskLoad",
        default
    )]
    pub struct AgentTaskLoad {
        #[yaserde(prefix = "tns", rename = "taskCeiling", default)]
        pub task_ceiling: i16,
        #[yaserde(prefix = "tns", rename = "taskLoad", default)]
        pub task_load: i16,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AgentVoiceChannel",
        default
    )]
    pub struct AgentVoiceChannel {
        #[yaserde(prefix = "tns", rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(prefix = "tns", rename = "equipment", default)]
        pub equipment: Option<String>,
        #[yaserde(prefix = "tns", rename = "password", default)]
        pub password: Option<String>,
        #[yaserde(prefix = "tns", rename = "phoneId", default)]
        pub phone_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "phoneType", default)]
        pub phone_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "queue", default)]
        pub queue: Option<String>,
        #[yaserde(prefix = "tns", rename = "taskCeiling", default)]
        pub task_ceiling: i16,
        #[yaserde(prefix = "tns", rename = "taskLoad", default)]
        pub task_load: i16,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "Agent",
        default
    )]
    pub struct Agent {
        #[yaserde(prefix = "tns", rename = "advocateInfo", default)]
        pub advocate_info: Option<AgentAdvocateInfo>,
        #[yaserde(prefix = "tns", rename = "basicProfile", default)]
        pub basic_profile: Option<AgentBasicProfile>,
        #[yaserde(prefix = "tns", rename = "chatChannel", default)]
        pub chat_channel: Option<AgentChatChannel>,
        #[yaserde(prefix = "tns", rename = "emailChannel", default)]
        pub email_channel: Option<AgentEmailChannel>,
        #[yaserde(prefix = "tns", rename = "extendedProfile", default)]
        pub extended_profile: Option<AgentExtendedProfile>,
        #[yaserde(prefix = "tns", rename = "loginId", default)]
        pub login_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "security", default)]
        pub security: Option<AgentSecurity>,
        #[yaserde(prefix = "tns", rename = "taskLoad", default)]
        pub task_load: Option<AgentTaskLoad>,
        #[yaserde(prefix = "tns", rename = "voiceChannel", default)]
        pub voice_channel: Option<AgentVoiceChannel>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        root = "AicServiceFault",
        default
    )]
    pub struct AicServiceFault {}

    pub type Fault = AicServiceFault;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "Update",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Update {
        #[yaserde(prefix = "tns", rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "UpdateResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct UpdateResponse {
        #[yaserde(prefix = "tns", rename = "UpdateReturn", default)]
        pub update_return: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "Delete",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Delete {
        #[yaserde(prefix = "tns", rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "DeleteResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct DeleteResponse {
        #[yaserde(prefix = "tns", rename = "DeleteReturn", default)]
        pub delete_return: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupAgentIds",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupAgentIds {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupAgentIdsResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupAgentIdsResponse {
        #[yaserde(prefix = "tns", rename = "LookupAgentIdsReturn", default)]
        pub lookup_agent_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupLRMIds",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupLRMIds {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupLRMIdsResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupLRMIdsResponse {
        #[yaserde(prefix = "tns", rename = "LookupLRMIdsReturn", default)]
        pub lookup_lrm_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupWorkgroups",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupWorkgroups {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupWorkgroupsResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(prefix = "tns", rename = "LookupWorkgroupsReturn", default)]
        pub lookup_workgroups_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupDomains",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupDomains {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupDomainsResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupDomainsResponse {
        #[yaserde(prefix = "tns", rename = "LookupDomainsReturn", default)]
        pub lookup_domains_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupLinkGroups",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupLinkGroups {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupLinkGroupsResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(prefix = "tns", rename = "LookupLinkGroupsReturn", default)]
        pub lookup_link_groups_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupPhoneTypes",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupPhoneTypes {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupPhoneTypesResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(prefix = "tns", rename = "LookupPhoneTypesReturn", default)]
        pub lookup_phone_types_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupSites",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupSites {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "LookupSitesResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct LookupSitesResponse {
        #[yaserde(prefix = "tns", rename = "LookupSitesReturn", default)]
        pub lookup_sites_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "Create",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Create {
        #[yaserde(prefix = "tns", rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "CreateResponse",
        default,
        namespace = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "tns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct CreateResponse {
        #[yaserde(prefix = "tns", rename = "CreateReturn", default)]
        pub create_return: bool,
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: GetResponseSoapEnvelope = from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: UpdateResponseSoapEnvelope = from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: DeleteResponseSoapEnvelope = from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupAgentIdsResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupLRMIdsResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupWorkgroupsResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupDomainsResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupLinkGroupsResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupPhoneTypesResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: LookupSitesResponseSoapEnvelope =
                from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
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
                xmlns: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
            });

            let (status, response) = self.send_soap_request(&__request, "").await;

            let r: CreateResponseSoapEnvelope = from_str(&response).expect("can not unmarshal");
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl Default for AicAgentAdminSoapBinding {
        fn default() -> Self {
            AicAgentAdminSoapBinding {
                client: reqwest::Client::new(),
                url: "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl AicAgentAdminSoapBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            AicAgentAdminSoapBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetRequest {
        #[yaserde(rename = "Get", default)]
        pub body: ports::GetRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetRequestSoapEnvelope {
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
        pub body: SoapGetRequest,
    }

    impl GetRequestSoapEnvelope {
        pub fn new(body: SoapGetRequest) -> Self {
            GetRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::GetResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetResponseSoapEnvelope {
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
        pub body: SoapGetResponse,
    }

    impl GetResponseSoapEnvelope {
        pub fn new(body: SoapGetResponse) -> Self {
            GetResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "Update", default)]
        pub body: ports::UpdateRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateRequestSoapEnvelope {
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
        pub body: SoapUpdateRequest,
    }

    impl UpdateRequestSoapEnvelope {
        pub fn new(body: SoapUpdateRequest) -> Self {
            UpdateRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::UpdateResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateResponseSoapEnvelope {
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
        pub body: SoapUpdateResponse,
    }

    impl UpdateResponseSoapEnvelope {
        pub fn new(body: SoapUpdateResponse) -> Self {
            UpdateResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "Delete", default)]
        pub body: ports::DeleteRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteRequestSoapEnvelope {
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
        pub body: SoapDeleteRequest,
    }

    impl DeleteRequestSoapEnvelope {
        pub fn new(body: SoapDeleteRequest) -> Self {
            DeleteRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::DeleteResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteResponseSoapEnvelope {
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
        pub body: SoapDeleteResponse,
    }

    impl DeleteResponseSoapEnvelope {
        pub fn new(body: SoapDeleteResponse) -> Self {
            DeleteResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupAgentIds", default)]
        pub body: ports::LookupAgentIdsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupAgentIdsRequestSoapEnvelope {
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
        pub body: SoapLookupAgentIdsRequest,
    }

    impl LookupAgentIdsRequestSoapEnvelope {
        pub fn new(body: SoapLookupAgentIdsRequest) -> Self {
            LookupAgentIdsRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupAgentIdsResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupAgentIdsResponseSoapEnvelope {
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
        pub body: SoapLookupAgentIdsResponse,
    }

    impl LookupAgentIdsResponseSoapEnvelope {
        pub fn new(body: SoapLookupAgentIdsResponse) -> Self {
            LookupAgentIdsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupLRMIds", default)]
        pub body: ports::LookupLRMIdsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLRMIdsRequestSoapEnvelope {
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
        pub body: SoapLookupLRMIdsRequest,
    }

    impl LookupLRMIdsRequestSoapEnvelope {
        pub fn new(body: SoapLookupLRMIdsRequest) -> Self {
            LookupLRMIdsRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupLRMIdsResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLRMIdsResponseSoapEnvelope {
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
        pub body: SoapLookupLRMIdsResponse,
    }

    impl LookupLRMIdsResponseSoapEnvelope {
        pub fn new(body: SoapLookupLRMIdsResponse) -> Self {
            LookupLRMIdsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupWorkgroups", default)]
        pub body: ports::LookupWorkgroupsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupWorkgroupsRequestSoapEnvelope {
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
        pub body: SoapLookupWorkgroupsRequest,
    }

    impl LookupWorkgroupsRequestSoapEnvelope {
        pub fn new(body: SoapLookupWorkgroupsRequest) -> Self {
            LookupWorkgroupsRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupWorkgroupsResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupWorkgroupsResponseSoapEnvelope {
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
        pub body: SoapLookupWorkgroupsResponse,
    }

    impl LookupWorkgroupsResponseSoapEnvelope {
        pub fn new(body: SoapLookupWorkgroupsResponse) -> Self {
            LookupWorkgroupsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupDomains", default)]
        pub body: ports::LookupDomainsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupDomainsRequestSoapEnvelope {
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
        pub body: SoapLookupDomainsRequest,
    }

    impl LookupDomainsRequestSoapEnvelope {
        pub fn new(body: SoapLookupDomainsRequest) -> Self {
            LookupDomainsRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupDomainsResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupDomainsResponseSoapEnvelope {
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
        pub body: SoapLookupDomainsResponse,
    }

    impl LookupDomainsResponseSoapEnvelope {
        pub fn new(body: SoapLookupDomainsResponse) -> Self {
            LookupDomainsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupLinkGroups", default)]
        pub body: ports::LookupLinkGroupsRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLinkGroupsRequestSoapEnvelope {
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
        pub body: SoapLookupLinkGroupsRequest,
    }

    impl LookupLinkGroupsRequestSoapEnvelope {
        pub fn new(body: SoapLookupLinkGroupsRequest) -> Self {
            LookupLinkGroupsRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupLinkGroupsResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupLinkGroupsResponseSoapEnvelope {
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
        pub body: SoapLookupLinkGroupsResponse,
    }

    impl LookupLinkGroupsResponseSoapEnvelope {
        pub fn new(body: SoapLookupLinkGroupsResponse) -> Self {
            LookupLinkGroupsResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupPhoneTypes", default)]
        pub body: ports::LookupPhoneTypesRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupPhoneTypesRequestSoapEnvelope {
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
        pub body: SoapLookupPhoneTypesRequest,
    }

    impl LookupPhoneTypesRequestSoapEnvelope {
        pub fn new(body: SoapLookupPhoneTypesRequest) -> Self {
            LookupPhoneTypesRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupPhoneTypesResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupPhoneTypesResponseSoapEnvelope {
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
        pub body: SoapLookupPhoneTypesResponse,
    }

    impl LookupPhoneTypesResponseSoapEnvelope {
        pub fn new(body: SoapLookupPhoneTypesResponse) -> Self {
            LookupPhoneTypesResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "LookupSites", default)]
        pub body: ports::LookupSitesRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupSitesRequestSoapEnvelope {
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
        pub body: SoapLookupSitesRequest,
    }

    impl LookupSitesRequestSoapEnvelope {
        pub fn new(body: SoapLookupSitesRequest) -> Self {
            LookupSitesRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::LookupSitesResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct LookupSitesResponseSoapEnvelope {
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
        pub body: SoapLookupSitesResponse,
    }

    impl LookupSitesResponseSoapEnvelope {
        pub fn new(body: SoapLookupSitesResponse) -> Self {
            LookupSitesResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        #[yaserde(rename = "Create", default)]
        pub body: ports::CreateRequest,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateRequestSoapEnvelope {
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
        pub body: SoapCreateRequest,
    }

    impl CreateRequestSoapEnvelope {
        pub fn new(body: SoapCreateRequest) -> Self {
            CreateRequestSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
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
        pub body: ports::CreateResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<ports::SoapAicServiceFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateResponseSoapEnvelope {
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
        pub body: SoapCreateResponse,
    }

    impl CreateResponseSoapEnvelope {
        pub fn new(body: SoapCreateResponse) -> Self {
            CreateResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from(
                    "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string(),
                ),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}
