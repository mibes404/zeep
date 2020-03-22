pub mod messages {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupPhoneTypesRequest", default)]
    pub struct LookupPhoneTypesRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupPhoneTypes,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLinkGroupsRequest", default)]
    pub struct LookupLinkGroupsRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupLinkGroups,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLRMIdsResponse", default)]
    pub struct LookupLRMIdsResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupLRMIdsResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "CreateRequest", default)]
    pub struct CreateRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::Create,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "CreateResponse", default)]
    pub struct CreateResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::CreateResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "UpdateRequest", default)]
    pub struct UpdateRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::Update,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupSitesRequest", default)]
    pub struct LookupSitesRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupSites,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupDomainsRequest", default)]
    pub struct LookupDomainsRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupDomains,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupDomainsResponse", default)]
    pub struct LookupDomainsResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupDomainsResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "GetResponse", default)]
    pub struct GetResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::GetResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "DeleteResponse", default)]
    pub struct DeleteResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::DeleteResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLRMIdsRequest", default)]
    pub struct LookupLRMIdsRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupLRMIds,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLinkGroupsResponse", default)]
    pub struct LookupLinkGroupsResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupLinkGroupsResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "UpdateResponse", default)]
    pub struct UpdateResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::UpdateResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "GetRequest", default)]
    pub struct GetRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::Get,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupAgentIdsResponse", default)]
    pub struct LookupAgentIdsResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupAgentIdsResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupPhoneTypesResponse", default)]
    pub struct LookupPhoneTypesResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupPhoneTypesResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupAgentIdsRequest", default)]
    pub struct LookupAgentIdsRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupAgentIds,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupSitesResponse", default)]
    pub struct LookupSitesResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupSitesResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupWorkgroupsResponse", default)]
    pub struct LookupWorkgroupsResponse {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupWorkgroupsResponse,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AicServiceFault", default)]
    pub struct AicServiceFault {
        #[serde(rename = "fault", default)]
        pub fault: types::Fault,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "DeleteRequest", default)]
    pub struct DeleteRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::Delete,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupWorkgroupsRequest", default)]
    pub struct LookupWorkgroupsRequest {
        #[serde(rename = "parameters", default)]
        pub parameters: types::LookupWorkgroups,
    }
}

pub mod ports {
    use serde::{Deserialize, Serialize};

    use super::*;

    pub struct AicAgentAdmin {}

    impl AicAgentAdmin {
        pub fn get(
            &self,
            get_request: messages::GetRequest,
        ) -> Result<messages::GetResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn update(
            &self,
            update_request: messages::UpdateRequest,
        ) -> Result<messages::UpdateResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn delete(
            &self,
            delete_request: messages::DeleteRequest,
        ) -> Result<messages::DeleteResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_agent_ids(
            &self,
            lookup_agent_ids_request: messages::LookupAgentIdsRequest,
        ) -> Result<messages::LookupAgentIdsResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_lrm_ids(
            &self,
            lookup_lrm_ids_request: messages::LookupLRMIdsRequest,
        ) -> Result<messages::LookupLRMIdsResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_workgroups(
            &self,
            lookup_workgroups_request: messages::LookupWorkgroupsRequest,
        ) -> Result<messages::LookupWorkgroupsResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_domains(
            &self,
            lookup_domains_request: messages::LookupDomainsRequest,
        ) -> Result<messages::LookupDomainsResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_link_groups(
            &self,
            lookup_link_groups_request: messages::LookupLinkGroupsRequest,
        ) -> Result<messages::LookupLinkGroupsResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_phone_types(
            &self,
            lookup_phone_types_request: messages::LookupPhoneTypesRequest,
        ) -> Result<messages::LookupPhoneTypesResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn lookup_sites(
            &self,
            lookup_sites_request: messages::LookupSitesRequest,
        ) -> Result<messages::LookupSitesResponse, messages::AicServiceFault> {
            unimplemented!()
        }
        pub fn create(
            &self,
            create_request: messages::CreateRequest,
        ) -> Result<messages::CreateResponse, messages::AicServiceFault> {
            unimplemented!()
        }
    }

    impl Default for AicAgentAdmin {
        fn default() -> Self {
            AicAgentAdmin {}
        }
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "Get", default)]
    pub struct Get {
        #[serde(rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "GetResponse", default)]
    pub struct GetResponse {
        #[serde(rename = "GetReturn", default)]
        pub get_return: Agent,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentAdvocateInfo", default)]
    pub struct AgentAdvocateInfo {
        #[serde(rename = "LRMID", default)]
        pub lrmid: Option<String>,
        #[serde(rename = "enabled", default)]
        pub enabled: bool,
        #[serde(rename = "telephonyLinkGroup", default)]
        pub telephony_link_group: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "ArrayOf_xsd_string", default)]
    pub struct ArrayOfXsdString {
        #[serde(rename = "item", default)]
        pub item: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentBasicProfile", default)]
    pub struct AgentBasicProfile {
        #[serde(rename = "domain", default)]
        pub domain: Option<String>,
        #[serde(rename = "employeeId", default)]
        pub employee_id: Option<String>,
        #[serde(rename = "externalAgent", default)]
        pub external_agent: bool,
        #[serde(rename = "firstName", default)]
        pub first_name: Option<String>,
        #[serde(rename = "lastName", default)]
        pub last_name: Option<String>,
        #[serde(rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[serde(rename = "outOfOffice", default)]
        pub out_of_office: bool,
        #[serde(rename = "preferredName", default)]
        pub preferred_name: Option<String>,
        #[serde(rename = "site", default)]
        pub site: Option<String>,
        #[serde(rename = "softwareAgent", default)]
        pub software_agent: bool,
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[serde(rename = "userAddressable", default)]
        pub user_addressable: bool,
        #[serde(rename = "workgroups", default)]
        pub workgroups: Option<ArrayOfXsdString>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentChatChannel", default)]
    pub struct AgentChatChannel {
        #[serde(rename = "enabled", default)]
        pub enabled: bool,
        #[serde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[serde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentEmailChannel", default)]
    pub struct AgentEmailChannel {
        #[serde(rename = "enabled", default)]
        pub enabled: bool,
        #[serde(rename = "fromAddress", default)]
        pub from_address: Option<String>,
        #[serde(rename = "showFullHeader", default)]
        pub show_full_header: bool,
        #[serde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[serde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentAddressInfo", default)]
    pub struct AgentAddressInfo {
        #[serde(rename = "POBox", default)]
        pub po_box: Option<String>,
        #[serde(rename = "address1", default)]
        pub address_1: Option<String>,
        #[serde(rename = "address2", default)]
        pub address_2: Option<String>,
        #[serde(rename = "building", default)]
        pub building: Option<String>,
        #[serde(rename = "city", default)]
        pub city: Option<String>,
        #[serde(rename = "company", default)]
        pub company: Option<String>,
        #[serde(rename = "countryOrRegion", default)]
        pub country_or_region: Option<String>,
        #[serde(rename = "mailStop", default)]
        pub mail_stop: Option<String>,
        #[serde(rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[serde(rename = "zipOrPostalCode", default)]
        pub zip_or_postal_code: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentAddress", default)]
    pub struct AgentAddress {
        #[serde(rename = "home", default)]
        pub home: Option<AgentAddressInfo>,
        #[serde(rename = "office", default)]
        pub office: Option<AgentAddressInfo>,
        #[serde(rename = "other", default)]
        pub other: Option<AgentAddressInfo>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentEmail", default)]
    pub struct AgentEmail {
        #[serde(rename = "internal", default)]
        pub internal: Option<String>,
        #[serde(rename = "mobileDevice", default)]
        pub mobile_device: Option<String>,
        #[serde(rename = "personal", default)]
        pub personal: Option<String>,
        #[serde(rename = "primary", default)]
        pub primary: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentPhoneInfo", default)]
    pub struct AgentPhoneInfo {
        #[serde(rename = "extension", default)]
        pub extension: Option<String>,
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentPhone", default)]
    pub struct AgentPhone {
        #[serde(rename = "fax", default)]
        pub fax: Option<AgentPhoneInfo>,
        #[serde(rename = "home", default)]
        pub home: Option<AgentPhoneInfo>,
        #[serde(rename = "mobile", default)]
        pub mobile: Option<AgentPhoneInfo>,
        #[serde(rename = "pager", default)]
        pub pager: Option<AgentPhoneInfo>,
        #[serde(rename = "primary", default)]
        pub primary: Option<AgentPhoneInfo>,
        #[serde(rename = "secondary", default)]
        pub secondary: Option<AgentPhoneInfo>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentExtendedProfile", default)]
    pub struct AgentExtendedProfile {
        #[serde(rename = "address", default)]
        pub address: Option<AgentAddress>,
        #[serde(rename = "email", default)]
        pub email: Option<AgentEmail>,
        #[serde(rename = "phone", default)]
        pub phone: Option<AgentPhone>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentSecurity", default)]
    pub struct AgentSecurity {
        #[serde(rename = "disableLogin", default)]
        pub disable_login: bool,
        #[serde(rename = "forcePwdChange", default)]
        pub force_pwd_change: bool,
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[serde(rename = "roleAdmin", default)]
        pub role_admin: bool,
        #[serde(rename = "roleAgent", default)]
        pub role_agent: bool,
        #[serde(rename = "roleClerk", default)]
        pub role_clerk: bool,
        #[serde(rename = "roleEditor", default)]
        pub role_editor: bool,
        #[serde(rename = "roleOperator", default)]
        pub role_operator: bool,
        #[serde(rename = "rolePostmaster", default)]
        pub role_postmaster: bool,
        #[serde(rename = "roleSupervisor", default)]
        pub role_supervisor: bool,
        #[serde(rename = "roleSupport", default)]
        pub role_support: bool,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentTaskLoad", default)]
    pub struct AgentTaskLoad {
        #[serde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[serde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AgentVoiceChannel", default)]
    pub struct AgentVoiceChannel {
        #[serde(rename = "enabled", default)]
        pub enabled: bool,
        #[serde(rename = "equipment", default)]
        pub equipment: Option<String>,
        #[serde(rename = "password", default)]
        pub password: Option<String>,
        #[serde(rename = "phoneId", default)]
        pub phone_id: Option<String>,
        #[serde(rename = "phoneType", default)]
        pub phone_type: Option<String>,
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
        #[serde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[serde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "Agent", default)]
    pub struct Agent {
        #[serde(rename = "advocateInfo", default)]
        pub advocate_info: Option<AgentAdvocateInfo>,
        #[serde(rename = "basicProfile", default)]
        pub basic_profile: Option<AgentBasicProfile>,
        #[serde(rename = "chatChannel", default)]
        pub chat_channel: Option<AgentChatChannel>,
        #[serde(rename = "emailChannel", default)]
        pub email_channel: Option<AgentEmailChannel>,
        #[serde(rename = "extendedProfile", default)]
        pub extended_profile: Option<AgentExtendedProfile>,
        #[serde(rename = "loginId", default)]
        pub login_id: Option<String>,
        #[serde(rename = "security", default)]
        pub security: Option<AgentSecurity>,
        #[serde(rename = "taskLoad", default)]
        pub task_load: Option<AgentTaskLoad>,
        #[serde(rename = "voiceChannel", default)]
        pub voice_channel: Option<AgentVoiceChannel>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "AicServiceFault", default)]
    pub struct AicServiceFault {}

    pub type Fault = AicServiceFault;

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "Update", default)]
    pub struct Update {
        #[serde(rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "UpdateResponse", default)]
    pub struct UpdateResponse {
        #[serde(rename = "UpdateReturn", default)]
        pub update_return: bool,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "Delete", default)]
    pub struct Delete {
        #[serde(rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "DeleteResponse", default)]
    pub struct DeleteResponse {
        #[serde(rename = "DeleteReturn", default)]
        pub delete_return: bool,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupAgentIds", default)]
    pub struct LookupAgentIds {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupAgentIdsResponse", default)]
    pub struct LookupAgentIdsResponse {
        #[serde(rename = "LookupAgentIdsReturn", default)]
        pub lookup_agent_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLRMIds", default)]
    pub struct LookupLRMIds {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLRMIdsResponse", default)]
    pub struct LookupLRMIdsResponse {
        #[serde(rename = "LookupLRMIdsReturn", default)]
        pub lookup_lrm_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupWorkgroups", default)]
    pub struct LookupWorkgroups {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupWorkgroupsResponse", default)]
    pub struct LookupWorkgroupsResponse {
        #[serde(rename = "LookupWorkgroupsReturn", default)]
        pub lookup_workgroups_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupDomains", default)]
    pub struct LookupDomains {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupDomainsResponse", default)]
    pub struct LookupDomainsResponse {
        #[serde(rename = "LookupDomainsReturn", default)]
        pub lookup_domains_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLinkGroups", default)]
    pub struct LookupLinkGroups {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupLinkGroupsResponse", default)]
    pub struct LookupLinkGroupsResponse {
        #[serde(rename = "LookupLinkGroupsReturn", default)]
        pub lookup_link_groups_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupPhoneTypes", default)]
    pub struct LookupPhoneTypes {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupPhoneTypesResponse", default)]
    pub struct LookupPhoneTypesResponse {
        #[serde(rename = "LookupPhoneTypesReturn", default)]
        pub lookup_phone_types_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupSites", default)]
    pub struct LookupSites {}

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "LookupSitesResponse", default)]
    pub struct LookupSitesResponse {
        #[serde(rename = "LookupSitesReturn", default)]
        pub lookup_sites_return: Vec<String>,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "Create", default)]
    pub struct Create {
        #[serde(rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename = "CreateResponse", default)]
    pub struct CreateResponse {
        #[serde(rename = "CreateReturn", default)]
        pub create_return: bool,
    }
}

use serde::{Deserialize, Serialize};
