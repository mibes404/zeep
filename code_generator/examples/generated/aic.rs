use soap_client::envelop;
use soap_client::soap::Header;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub mod messages {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupPhoneTypesRequest", default)]
    pub struct LookupPhoneTypesRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupPhoneTypes,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLinkGroupsRequest", default)]
    pub struct LookupLinkGroupsRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupLinkGroups,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLRMIdsResponse", default)]
    pub struct LookupLRMIdsResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupLRMIdsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "CreateRequest", default)]
    pub struct CreateRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::Create,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "CreateResponse", default)]
    pub struct CreateResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::CreateResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "UpdateRequest", default)]
    pub struct UpdateRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::Update,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupSitesRequest", default)]
    pub struct LookupSitesRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupSites,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupDomainsRequest", default)]
    pub struct LookupDomainsRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupDomains,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupDomainsResponse", default)]
    pub struct LookupDomainsResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupDomainsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "GetResponse", default)]
    pub struct GetResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::GetResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "DeleteResponse", default)]
    pub struct DeleteResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::DeleteResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLRMIdsRequest", default)]
    pub struct LookupLRMIdsRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupLRMIds,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLinkGroupsResponse", default)]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupLinkGroupsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "UpdateResponse", default)]
    pub struct UpdateResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::UpdateResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "GetRequest", default)]
    pub struct GetRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::Get,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupAgentIdsResponse", default)]
    pub struct LookupAgentIdsResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupAgentIdsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupPhoneTypesResponse", default)]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupPhoneTypesResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupAgentIdsRequest", default)]
    pub struct LookupAgentIdsRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupAgentIds,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupSitesResponse", default)]
    pub struct LookupSitesResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupSitesResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupWorkgroupsResponse", default)]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupWorkgroupsResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AicServiceFault", default)]
    pub struct AicServiceFault {
        #[yaserde(rename = "fault", default)]
        pub fault: types::Fault,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "DeleteRequest", default)]
    pub struct DeleteRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::Delete,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupWorkgroupsRequest", default)]
    pub struct LookupWorkgroupsRequest {
        #[yaserde(rename = "parameters", default)]
        pub parameters: types::LookupWorkgroups,
    }
}

pub mod bindings {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;

    pub struct AicAgentAdminSoapBinding {}

    impl ports::AicAgentAdmin for AicAgentAdminSoapBinding {
        fn get(
            &self,
            get_request: ports::GetRequest,
        ) -> Result<ports::GetResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn update(
            &self,
            update_request: ports::UpdateRequest,
        ) -> Result<ports::UpdateResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn delete(
            &self,
            delete_request: ports::DeleteRequest,
        ) -> Result<ports::DeleteResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_agent_ids(
            &self,
            lookup_agent_ids_request: ports::LookupAgentIdsRequest,
        ) -> Result<ports::LookupAgentIdsResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_lrm_ids(
            &self,
            lookup_lrm_ids_request: ports::LookupLRMIdsRequest,
        ) -> Result<ports::LookupLRMIdsResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_workgroups(
            &self,
            lookup_workgroups_request: ports::LookupWorkgroupsRequest,
        ) -> Result<ports::LookupWorkgroupsResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_domains(
            &self,
            lookup_domains_request: ports::LookupDomainsRequest,
        ) -> Result<ports::LookupDomainsResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_link_groups(
            &self,
            lookup_link_groups_request: ports::LookupLinkGroupsRequest,
        ) -> Result<ports::LookupLinkGroupsResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_phone_types(
            &self,
            lookup_phone_types_request: ports::LookupPhoneTypesRequest,
        ) -> Result<ports::LookupPhoneTypesResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn lookup_sites(
            &self,
            lookup_sites_request: ports::LookupSitesRequest,
        ) -> Result<ports::LookupSitesResponse, ports::AicServiceFault> {
            unimplemented!();
        }
        fn create(
            &self,
            create_request: ports::CreateRequest,
        ) -> Result<ports::CreateResponse, ports::AicServiceFault> {
            unimplemented!();
        }
    }

    impl Default for AicAgentAdminSoapBinding {
        fn default() -> Self {
            AicAgentAdminSoapBinding {}
        }
    }
}

pub mod types {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "Get", default)]
    pub struct Get {
        #[yaserde(rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "GetResponse", default)]
    pub struct GetResponse {
        #[yaserde(rename = "GetReturn", default)]
        pub get_return: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentAdvocateInfo", default)]
    pub struct AgentAdvocateInfo {
        #[yaserde(rename = "LRMID", default)]
        pub lrmid: Option<String>,
        #[yaserde(rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(rename = "telephonyLinkGroup", default)]
        pub telephony_link_group: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ArrayOf_xsd_string", default)]
    pub struct ArrayOfXsdString {
        #[yaserde(rename = "item", default)]
        pub item: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentBasicProfile", default)]
    pub struct AgentBasicProfile {
        #[yaserde(rename = "domain", default)]
        pub domain: Option<String>,
        #[yaserde(rename = "employeeId", default)]
        pub employee_id: Option<String>,
        #[yaserde(rename = "externalAgent", default)]
        pub external_agent: bool,
        #[yaserde(rename = "firstName", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "lastName", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "outOfOffice", default)]
        pub out_of_office: bool,
        #[yaserde(rename = "preferredName", default)]
        pub preferred_name: Option<String>,
        #[yaserde(rename = "site", default)]
        pub site: Option<String>,
        #[yaserde(rename = "softwareAgent", default)]
        pub software_agent: bool,
        #[yaserde(rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(rename = "userAddressable", default)]
        pub user_addressable: bool,
        #[yaserde(rename = "workgroups", default)]
        pub workgroups: Option<ArrayOfXsdString>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentChatChannel", default)]
    pub struct AgentChatChannel {
        #[yaserde(rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[yaserde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentEmailChannel", default)]
    pub struct AgentEmailChannel {
        #[yaserde(rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(rename = "fromAddress", default)]
        pub from_address: Option<String>,
        #[yaserde(rename = "showFullHeader", default)]
        pub show_full_header: bool,
        #[yaserde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[yaserde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentAddressInfo", default)]
    pub struct AgentAddressInfo {
        #[yaserde(rename = "POBox", default)]
        pub po_box: Option<String>,
        #[yaserde(rename = "address1", default)]
        pub address_1: Option<String>,
        #[yaserde(rename = "address2", default)]
        pub address_2: Option<String>,
        #[yaserde(rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(rename = "city", default)]
        pub city: Option<String>,
        #[yaserde(rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(rename = "countryOrRegion", default)]
        pub country_or_region: Option<String>,
        #[yaserde(rename = "mailStop", default)]
        pub mail_stop: Option<String>,
        #[yaserde(rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(rename = "zipOrPostalCode", default)]
        pub zip_or_postal_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentAddress", default)]
    pub struct AgentAddress {
        #[yaserde(rename = "home", default)]
        pub home: Option<AgentAddressInfo>,
        #[yaserde(rename = "office", default)]
        pub office: Option<AgentAddressInfo>,
        #[yaserde(rename = "other", default)]
        pub other: Option<AgentAddressInfo>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentEmail", default)]
    pub struct AgentEmail {
        #[yaserde(rename = "internal", default)]
        pub internal: Option<String>,
        #[yaserde(rename = "mobileDevice", default)]
        pub mobile_device: Option<String>,
        #[yaserde(rename = "personal", default)]
        pub personal: Option<String>,
        #[yaserde(rename = "primary", default)]
        pub primary: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentPhoneInfo", default)]
    pub struct AgentPhoneInfo {
        #[yaserde(rename = "extension", default)]
        pub extension: Option<String>,
        #[yaserde(rename = "phoneNumber", default)]
        pub phone_number: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentPhone", default)]
    pub struct AgentPhone {
        #[yaserde(rename = "fax", default)]
        pub fax: Option<AgentPhoneInfo>,
        #[yaserde(rename = "home", default)]
        pub home: Option<AgentPhoneInfo>,
        #[yaserde(rename = "mobile", default)]
        pub mobile: Option<AgentPhoneInfo>,
        #[yaserde(rename = "pager", default)]
        pub pager: Option<AgentPhoneInfo>,
        #[yaserde(rename = "primary", default)]
        pub primary: Option<AgentPhoneInfo>,
        #[yaserde(rename = "secondary", default)]
        pub secondary: Option<AgentPhoneInfo>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentExtendedProfile", default)]
    pub struct AgentExtendedProfile {
        #[yaserde(rename = "address", default)]
        pub address: Option<AgentAddress>,
        #[yaserde(rename = "email", default)]
        pub email: Option<AgentEmail>,
        #[yaserde(rename = "phone", default)]
        pub phone: Option<AgentPhone>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentSecurity", default)]
    pub struct AgentSecurity {
        #[yaserde(rename = "disableLogin", default)]
        pub disable_login: bool,
        #[yaserde(rename = "forcePwdChange", default)]
        pub force_pwd_change: bool,
        #[yaserde(rename = "password", default)]
        pub password: Option<String>,
        #[yaserde(rename = "roleAdmin", default)]
        pub role_admin: bool,
        #[yaserde(rename = "roleAgent", default)]
        pub role_agent: bool,
        #[yaserde(rename = "roleClerk", default)]
        pub role_clerk: bool,
        #[yaserde(rename = "roleEditor", default)]
        pub role_editor: bool,
        #[yaserde(rename = "roleOperator", default)]
        pub role_operator: bool,
        #[yaserde(rename = "rolePostmaster", default)]
        pub role_postmaster: bool,
        #[yaserde(rename = "roleSupervisor", default)]
        pub role_supervisor: bool,
        #[yaserde(rename = "roleSupport", default)]
        pub role_support: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentTaskLoad", default)]
    pub struct AgentTaskLoad {
        #[yaserde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[yaserde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AgentVoiceChannel", default)]
    pub struct AgentVoiceChannel {
        #[yaserde(rename = "enabled", default)]
        pub enabled: bool,
        #[yaserde(rename = "equipment", default)]
        pub equipment: Option<String>,
        #[yaserde(rename = "password", default)]
        pub password: Option<String>,
        #[yaserde(rename = "phoneId", default)]
        pub phone_id: Option<String>,
        #[yaserde(rename = "phoneType", default)]
        pub phone_type: Option<String>,
        #[yaserde(rename = "queue", default)]
        pub queue: Option<String>,
        #[yaserde(rename = "taskCeiling", default)]
        pub task_ceiling: u8,
        #[yaserde(rename = "taskLoad", default)]
        pub task_load: u8,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "Agent", default)]
    pub struct Agent {
        #[yaserde(rename = "advocateInfo", default)]
        pub advocate_info: Option<AgentAdvocateInfo>,
        #[yaserde(rename = "basicProfile", default)]
        pub basic_profile: Option<AgentBasicProfile>,
        #[yaserde(rename = "chatChannel", default)]
        pub chat_channel: Option<AgentChatChannel>,
        #[yaserde(rename = "emailChannel", default)]
        pub email_channel: Option<AgentEmailChannel>,
        #[yaserde(rename = "extendedProfile", default)]
        pub extended_profile: Option<AgentExtendedProfile>,
        #[yaserde(rename = "loginId", default)]
        pub login_id: Option<String>,
        #[yaserde(rename = "security", default)]
        pub security: Option<AgentSecurity>,
        #[yaserde(rename = "taskLoad", default)]
        pub task_load: Option<AgentTaskLoad>,
        #[yaserde(rename = "voiceChannel", default)]
        pub voice_channel: Option<AgentVoiceChannel>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "AicServiceFault", default)]
    pub struct AicServiceFault {}

    pub type Fault = AicServiceFault;

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "Update", default)]
    pub struct Update {
        #[yaserde(rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "UpdateResponse", default)]
    pub struct UpdateResponse {
        #[yaserde(rename = "UpdateReturn", default)]
        pub update_return: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "Delete", default)]
    pub struct Delete {
        #[yaserde(rename = "loginId", default)]
        pub login_id: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "DeleteResponse", default)]
    pub struct DeleteResponse {
        #[yaserde(rename = "DeleteReturn", default)]
        pub delete_return: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupAgentIds", default)]
    pub struct LookupAgentIds {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupAgentIdsResponse", default)]
    pub struct LookupAgentIdsResponse {
        #[yaserde(rename = "LookupAgentIdsReturn", default)]
        pub lookup_agent_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLRMIds", default)]
    pub struct LookupLRMIds {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLRMIdsResponse", default)]
    pub struct LookupLRMIdsResponse {
        #[yaserde(rename = "LookupLRMIdsReturn", default)]
        pub lookup_lrm_ids_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupWorkgroups", default)]
    pub struct LookupWorkgroups {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupWorkgroupsResponse", default)]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(rename = "LookupWorkgroupsReturn", default)]
        pub lookup_workgroups_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupDomains", default)]
    pub struct LookupDomains {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupDomainsResponse", default)]
    pub struct LookupDomainsResponse {
        #[yaserde(rename = "LookupDomainsReturn", default)]
        pub lookup_domains_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLinkGroups", default)]
    pub struct LookupLinkGroups {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupLinkGroupsResponse", default)]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(rename = "LookupLinkGroupsReturn", default)]
        pub lookup_link_groups_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupPhoneTypes", default)]
    pub struct LookupPhoneTypes {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupPhoneTypesResponse", default)]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(rename = "LookupPhoneTypesReturn", default)]
        pub lookup_phone_types_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupSites", default)]
    pub struct LookupSites {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "LookupSitesResponse", default)]
    pub struct LookupSitesResponse {
        #[yaserde(rename = "LookupSitesReturn", default)]
        pub lookup_sites_return: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "Create", default)]
    pub struct Create {
        #[yaserde(rename = "agent", default)]
        pub agent: Agent,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "CreateResponse", default)]
    pub struct CreateResponse {
        #[yaserde(rename = "CreateReturn", default)]
        pub create_return: bool,
    }
}

pub mod ports {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;

    pub trait AicAgentAdmin {
        fn get(&self, get_request: GetRequest) -> Result<GetResponse, AicServiceFault>;
        fn update(&self, update_request: UpdateRequest) -> Result<UpdateResponse, AicServiceFault>;
        fn delete(&self, delete_request: DeleteRequest) -> Result<DeleteResponse, AicServiceFault>;
        fn lookup_agent_ids(
            &self,
            lookup_agent_ids_request: LookupAgentIdsRequest,
        ) -> Result<LookupAgentIdsResponse, AicServiceFault>;
        fn lookup_lrm_ids(
            &self,
            lookup_lrm_ids_request: LookupLRMIdsRequest,
        ) -> Result<LookupLRMIdsResponse, AicServiceFault>;
        fn lookup_workgroups(
            &self,
            lookup_workgroups_request: LookupWorkgroupsRequest,
        ) -> Result<LookupWorkgroupsResponse, AicServiceFault>;
        fn lookup_domains(
            &self,
            lookup_domains_request: LookupDomainsRequest,
        ) -> Result<LookupDomainsResponse, AicServiceFault>;
        fn lookup_link_groups(
            &self,
            lookup_link_groups_request: LookupLinkGroupsRequest,
        ) -> Result<LookupLinkGroupsResponse, AicServiceFault>;
        fn lookup_phone_types(
            &self,
            lookup_phone_types_request: LookupPhoneTypesRequest,
        ) -> Result<LookupPhoneTypesResponse, AicServiceFault>;
        fn lookup_sites(
            &self,
            lookup_sites_request: LookupSitesRequest,
        ) -> Result<LookupSitesResponse, AicServiceFault>;
        fn create(&self, create_request: CreateRequest) -> Result<CreateResponse, AicServiceFault>;
    }

    pub type GetRequest = messages::GetRequest;
    envelop!(GetRequestSoapEnvelope, GetRequest);

    pub type GetResponse = messages::GetResponse;
    pub type AicServiceFault = messages::AicServiceFault;
    pub type UpdateRequest = messages::UpdateRequest;
    envelop!(UpdateRequestSoapEnvelope, UpdateRequest);

    pub type UpdateResponse = messages::UpdateResponse;
    pub type DeleteRequest = messages::DeleteRequest;
    envelop!(DeleteRequestSoapEnvelope, DeleteRequest);

    pub type DeleteResponse = messages::DeleteResponse;
    pub type LookupAgentIdsRequest = messages::LookupAgentIdsRequest;
    envelop!(LookupAgentIdsRequestSoapEnvelope, LookupAgentIdsRequest);

    pub type LookupAgentIdsResponse = messages::LookupAgentIdsResponse;
    pub type LookupLRMIdsRequest = messages::LookupLRMIdsRequest;
    envelop!(LookupLRMIdsRequestSoapEnvelope, LookupLRMIdsRequest);

    pub type LookupLRMIdsResponse = messages::LookupLRMIdsResponse;
    pub type LookupWorkgroupsRequest = messages::LookupWorkgroupsRequest;
    envelop!(LookupWorkgroupsRequestSoapEnvelope, LookupWorkgroupsRequest);

    pub type LookupWorkgroupsResponse = messages::LookupWorkgroupsResponse;
    pub type LookupDomainsRequest = messages::LookupDomainsRequest;
    envelop!(LookupDomainsRequestSoapEnvelope, LookupDomainsRequest);

    pub type LookupDomainsResponse = messages::LookupDomainsResponse;
    pub type LookupLinkGroupsRequest = messages::LookupLinkGroupsRequest;
    envelop!(LookupLinkGroupsRequestSoapEnvelope, LookupLinkGroupsRequest);

    pub type LookupLinkGroupsResponse = messages::LookupLinkGroupsResponse;
    pub type LookupPhoneTypesRequest = messages::LookupPhoneTypesRequest;
    envelop!(LookupPhoneTypesRequestSoapEnvelope, LookupPhoneTypesRequest);

    pub type LookupPhoneTypesResponse = messages::LookupPhoneTypesResponse;
    pub type LookupSitesRequest = messages::LookupSitesRequest;
    envelop!(LookupSitesRequestSoapEnvelope, LookupSitesRequest);

    pub type LookupSitesResponse = messages::LookupSitesResponse;
    pub type CreateRequest = messages::CreateRequest;
    envelop!(CreateRequestSoapEnvelope, CreateRequest);

    pub type CreateResponse = messages::CreateResponse;
}
