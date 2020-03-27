pub mod messages {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupPhoneTypesRequest", default)]
pub struct LookupPhoneTypesRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupPhoneTypes,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupLinkGroupsRequest", default)]
pub struct LookupLinkGroupsRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupLinkGroups,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupLRMIdsResponse", default)]
pub struct LookupLRMIdsResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupLRMIdsResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CreateRequest", default)]
pub struct CreateRequest {
	#[yaserde(flatten)]
	pub parameters: types::Create,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CreateResponse", default)]
pub struct CreateResponse {
	#[yaserde(flatten)]
	pub parameters: types::CreateResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UpdateRequest", default)]
pub struct UpdateRequest {
	#[yaserde(flatten)]
	pub parameters: types::Update,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupSitesRequest", default)]
pub struct LookupSitesRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupSites,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupDomainsRequest", default)]
pub struct LookupDomainsRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupDomains,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupDomainsResponse", default)]
pub struct LookupDomainsResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupDomainsResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetResponse", default)]
pub struct GetResponse {
	#[yaserde(flatten)]
	pub parameters: types::GetResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DeleteResponse", default)]
pub struct DeleteResponse {
	#[yaserde(flatten)]
	pub parameters: types::DeleteResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupLRMIdsRequest", default)]
pub struct LookupLRMIdsRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupLRMIds,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupLinkGroupsResponse", default)]
pub struct LookupLinkGroupsResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupLinkGroupsResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UpdateResponse", default)]
pub struct UpdateResponse {
	#[yaserde(flatten)]
	pub parameters: types::UpdateResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetRequest", default)]
pub struct GetRequest {
	#[yaserde(flatten)]
	pub parameters: types::Get,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupAgentIdsResponse", default)]
pub struct LookupAgentIdsResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupAgentIdsResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupPhoneTypesResponse", default)]
pub struct LookupPhoneTypesResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupPhoneTypesResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupAgentIdsRequest", default)]
pub struct LookupAgentIdsRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupAgentIds,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupSitesResponse", default)]
pub struct LookupSitesResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupSitesResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupWorkgroupsResponse", default)]
pub struct LookupWorkgroupsResponse {
	#[yaserde(flatten)]
	pub parameters: types::LookupWorkgroupsResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "AicServiceFault", default)]
pub struct AicServiceFault {
	#[yaserde(flatten)]
	pub fault: types::Fault,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DeleteRequest", default)]
pub struct DeleteRequest {
	#[yaserde(flatten)]
	pub parameters: types::Delete,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LookupWorkgroupsRequest", default)]
pub struct LookupWorkgroupsRequest {
	#[yaserde(flatten)]
	pub parameters: types::LookupWorkgroups,
}

}

pub mod types {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "Get", default)]
pub struct Get {
	#[yaserde(prefix = "ns", rename = "loginId", default)]
	pub login_id: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "GetResponse", default)]
pub struct GetResponse {
	#[yaserde(prefix = "ns", rename = "GetReturn", default)]
	pub get_return: Agent,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentAdvocateInfo", default)]
pub struct AgentAdvocateInfo {
	#[yaserde(prefix = "ns", rename = "LRMID", default)]
	pub lrmid: Option<String>,
	#[yaserde(prefix = "ns", rename = "enabled", default)]
	pub enabled: bool,
	#[yaserde(prefix = "ns", rename = "telephonyLinkGroup", default)]
	pub telephony_link_group: Option<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "ArrayOf_xsd_string", default)]
pub struct ArrayOfXsdString {
	#[yaserde(prefix = "ns", rename = "item", default)]
	pub item: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentBasicProfile", default)]
pub struct AgentBasicProfile {
	#[yaserde(prefix = "ns", rename = "domain", default)]
	pub domain: Option<String>,
	#[yaserde(prefix = "ns", rename = "employeeId", default)]
	pub employee_id: Option<String>,
	#[yaserde(prefix = "ns", rename = "externalAgent", default)]
	pub external_agent: bool,
	#[yaserde(prefix = "ns", rename = "firstName", default)]
	pub first_name: Option<String>,
	#[yaserde(prefix = "ns", rename = "lastName", default)]
	pub last_name: Option<String>,
	#[yaserde(prefix = "ns", rename = "middleName", default)]
	pub middle_name: Option<String>,
	#[yaserde(prefix = "ns", rename = "outOfOffice", default)]
	pub out_of_office: bool,
	#[yaserde(prefix = "ns", rename = "preferredName", default)]
	pub preferred_name: Option<String>,
	#[yaserde(prefix = "ns", rename = "site", default)]
	pub site: Option<String>,
	#[yaserde(prefix = "ns", rename = "softwareAgent", default)]
	pub software_agent: bool,
	#[yaserde(prefix = "ns", rename = "title", default)]
	pub title: Option<String>,
	#[yaserde(prefix = "ns", rename = "userAddressable", default)]
	pub user_addressable: bool,
	#[yaserde(prefix = "ns", rename = "workgroups", default)]
	pub workgroups: Option<ArrayOfXsdString>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentChatChannel", default)]
pub struct AgentChatChannel {
	#[yaserde(prefix = "ns", rename = "enabled", default)]
	pub enabled: bool,
	#[yaserde(prefix = "ns", rename = "taskCeiling", default)]
	pub task_ceiling: u8,
	#[yaserde(prefix = "ns", rename = "taskLoad", default)]
	pub task_load: u8,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentEmailChannel", default)]
pub struct AgentEmailChannel {
	#[yaserde(prefix = "ns", rename = "enabled", default)]
	pub enabled: bool,
	#[yaserde(prefix = "ns", rename = "fromAddress", default)]
	pub from_address: Option<String>,
	#[yaserde(prefix = "ns", rename = "showFullHeader", default)]
	pub show_full_header: bool,
	#[yaserde(prefix = "ns", rename = "taskCeiling", default)]
	pub task_ceiling: u8,
	#[yaserde(prefix = "ns", rename = "taskLoad", default)]
	pub task_load: u8,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentAddressInfo", default)]
pub struct AgentAddressInfo {
	#[yaserde(prefix = "ns", rename = "POBox", default)]
	pub po_box: Option<String>,
	#[yaserde(prefix = "ns", rename = "address1", default)]
	pub address_1: Option<String>,
	#[yaserde(prefix = "ns", rename = "address2", default)]
	pub address_2: Option<String>,
	#[yaserde(prefix = "ns", rename = "building", default)]
	pub building: Option<String>,
	#[yaserde(prefix = "ns", rename = "city", default)]
	pub city: Option<String>,
	#[yaserde(prefix = "ns", rename = "company", default)]
	pub company: Option<String>,
	#[yaserde(prefix = "ns", rename = "countryOrRegion", default)]
	pub country_or_region: Option<String>,
	#[yaserde(prefix = "ns", rename = "mailStop", default)]
	pub mail_stop: Option<String>,
	#[yaserde(prefix = "ns", rename = "stateOrProvince", default)]
	pub state_or_province: Option<String>,
	#[yaserde(prefix = "ns", rename = "zipOrPostalCode", default)]
	pub zip_or_postal_code: Option<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentAddress", default)]
pub struct AgentAddress {
	#[yaserde(prefix = "ns", rename = "home", default)]
	pub home: Option<AgentAddressInfo>,
	#[yaserde(prefix = "ns", rename = "office", default)]
	pub office: Option<AgentAddressInfo>,
	#[yaserde(prefix = "ns", rename = "other", default)]
	pub other: Option<AgentAddressInfo>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentEmail", default)]
pub struct AgentEmail {
	#[yaserde(prefix = "ns", rename = "internal", default)]
	pub internal: Option<String>,
	#[yaserde(prefix = "ns", rename = "mobileDevice", default)]
	pub mobile_device: Option<String>,
	#[yaserde(prefix = "ns", rename = "personal", default)]
	pub personal: Option<String>,
	#[yaserde(prefix = "ns", rename = "primary", default)]
	pub primary: Option<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentPhoneInfo", default)]
pub struct AgentPhoneInfo {
	#[yaserde(prefix = "ns", rename = "extension", default)]
	pub extension: Option<String>,
	#[yaserde(prefix = "ns", rename = "phoneNumber", default)]
	pub phone_number: Option<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentPhone", default)]
pub struct AgentPhone {
	#[yaserde(prefix = "ns", rename = "fax", default)]
	pub fax: Option<AgentPhoneInfo>,
	#[yaserde(prefix = "ns", rename = "home", default)]
	pub home: Option<AgentPhoneInfo>,
	#[yaserde(prefix = "ns", rename = "mobile", default)]
	pub mobile: Option<AgentPhoneInfo>,
	#[yaserde(prefix = "ns", rename = "pager", default)]
	pub pager: Option<AgentPhoneInfo>,
	#[yaserde(prefix = "ns", rename = "primary", default)]
	pub primary: Option<AgentPhoneInfo>,
	#[yaserde(prefix = "ns", rename = "secondary", default)]
	pub secondary: Option<AgentPhoneInfo>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentExtendedProfile", default)]
pub struct AgentExtendedProfile {
	#[yaserde(prefix = "ns", rename = "address", default)]
	pub address: Option<AgentAddress>,
	#[yaserde(prefix = "ns", rename = "email", default)]
	pub email: Option<AgentEmail>,
	#[yaserde(prefix = "ns", rename = "phone", default)]
	pub phone: Option<AgentPhone>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentSecurity", default)]
pub struct AgentSecurity {
	#[yaserde(prefix = "ns", rename = "disableLogin", default)]
	pub disable_login: bool,
	#[yaserde(prefix = "ns", rename = "forcePwdChange", default)]
	pub force_pwd_change: bool,
	#[yaserde(prefix = "ns", rename = "password", default)]
	pub password: Option<String>,
	#[yaserde(prefix = "ns", rename = "roleAdmin", default)]
	pub role_admin: bool,
	#[yaserde(prefix = "ns", rename = "roleAgent", default)]
	pub role_agent: bool,
	#[yaserde(prefix = "ns", rename = "roleClerk", default)]
	pub role_clerk: bool,
	#[yaserde(prefix = "ns", rename = "roleEditor", default)]
	pub role_editor: bool,
	#[yaserde(prefix = "ns", rename = "roleOperator", default)]
	pub role_operator: bool,
	#[yaserde(prefix = "ns", rename = "rolePostmaster", default)]
	pub role_postmaster: bool,
	#[yaserde(prefix = "ns", rename = "roleSupervisor", default)]
	pub role_supervisor: bool,
	#[yaserde(prefix = "ns", rename = "roleSupport", default)]
	pub role_support: bool,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentTaskLoad", default)]
pub struct AgentTaskLoad {
	#[yaserde(prefix = "ns", rename = "taskCeiling", default)]
	pub task_ceiling: u8,
	#[yaserde(prefix = "ns", rename = "taskLoad", default)]
	pub task_load: u8,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AgentVoiceChannel", default)]
pub struct AgentVoiceChannel {
	#[yaserde(prefix = "ns", rename = "enabled", default)]
	pub enabled: bool,
	#[yaserde(prefix = "ns", rename = "equipment", default)]
	pub equipment: Option<String>,
	#[yaserde(prefix = "ns", rename = "password", default)]
	pub password: Option<String>,
	#[yaserde(prefix = "ns", rename = "phoneId", default)]
	pub phone_id: Option<String>,
	#[yaserde(prefix = "ns", rename = "phoneType", default)]
	pub phone_type: Option<String>,
	#[yaserde(prefix = "ns", rename = "queue", default)]
	pub queue: Option<String>,
	#[yaserde(prefix = "ns", rename = "taskCeiling", default)]
	pub task_ceiling: u8,
	#[yaserde(prefix = "ns", rename = "taskLoad", default)]
	pub task_load: u8,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "Agent", default)]
pub struct Agent {
	#[yaserde(prefix = "ns", rename = "advocateInfo", default)]
	pub advocate_info: Option<AgentAdvocateInfo>,
	#[yaserde(prefix = "ns", rename = "basicProfile", default)]
	pub basic_profile: Option<AgentBasicProfile>,
	#[yaserde(prefix = "ns", rename = "chatChannel", default)]
	pub chat_channel: Option<AgentChatChannel>,
	#[yaserde(prefix = "ns", rename = "emailChannel", default)]
	pub email_channel: Option<AgentEmailChannel>,
	#[yaserde(prefix = "ns", rename = "extendedProfile", default)]
	pub extended_profile: Option<AgentExtendedProfile>,
	#[yaserde(prefix = "ns", rename = "loginId", default)]
	pub login_id: Option<String>,
	#[yaserde(prefix = "ns", rename = "security", default)]
	pub security: Option<AgentSecurity>,
	#[yaserde(prefix = "ns", rename = "taskLoad", default)]
	pub task_load: Option<AgentTaskLoad>,
	#[yaserde(prefix = "ns", rename = "voiceChannel", default)]
	pub voice_channel: Option<AgentVoiceChannel>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "AicServiceFault", default)]
pub struct AicServiceFault {
}

pub type Fault = AicServiceFault;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "Update", default)]
pub struct Update {
	#[yaserde(prefix = "ns", rename = "agent", default)]
	pub agent: Agent,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "UpdateResponse", default)]
pub struct UpdateResponse {
	#[yaserde(prefix = "ns", rename = "UpdateReturn", default)]
	pub update_return: bool,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "Delete", default)]
pub struct Delete {
	#[yaserde(prefix = "ns", rename = "loginId", default)]
	pub login_id: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "DeleteResponse", default)]
pub struct DeleteResponse {
	#[yaserde(prefix = "ns", rename = "DeleteReturn", default)]
	pub delete_return: bool,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupAgentIds", default)]
pub struct LookupAgentIds {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupAgentIdsResponse", default)]
pub struct LookupAgentIdsResponse {
	#[yaserde(prefix = "ns", rename = "LookupAgentIdsReturn", default)]
	pub lookup_agent_ids_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupLRMIds", default)]
pub struct LookupLRMIds {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupLRMIdsResponse", default)]
pub struct LookupLRMIdsResponse {
	#[yaserde(prefix = "ns", rename = "LookupLRMIdsReturn", default)]
	pub lookup_lrm_ids_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupWorkgroups", default)]
pub struct LookupWorkgroups {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupWorkgroupsResponse", default)]
pub struct LookupWorkgroupsResponse {
	#[yaserde(prefix = "ns", rename = "LookupWorkgroupsReturn", default)]
	pub lookup_workgroups_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupDomains", default)]
pub struct LookupDomains {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupDomainsResponse", default)]
pub struct LookupDomainsResponse {
	#[yaserde(prefix = "ns", rename = "LookupDomainsReturn", default)]
	pub lookup_domains_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupLinkGroups", default)]
pub struct LookupLinkGroups {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupLinkGroupsResponse", default)]
pub struct LookupLinkGroupsResponse {
	#[yaserde(prefix = "ns", rename = "LookupLinkGroupsReturn", default)]
	pub lookup_link_groups_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupPhoneTypes", default)]
pub struct LookupPhoneTypes {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupPhoneTypesResponse", default)]
pub struct LookupPhoneTypesResponse {
	#[yaserde(prefix = "ns", rename = "LookupPhoneTypesReturn", default)]
	pub lookup_phone_types_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupSites", default)]
pub struct LookupSites {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "LookupSitesResponse", default)]
pub struct LookupSitesResponse {
	#[yaserde(prefix = "ns", rename = "LookupSitesReturn", default)]
	pub lookup_sites_return: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "Create", default)]
pub struct Create {
	#[yaserde(prefix = "ns", rename = "agent", default)]
	pub agent: Agent,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71", rename = "CreateResponse", default)]
pub struct CreateResponse {
	#[yaserde(prefix = "ns", rename = "CreateReturn", default)]
	pub create_return: bool,
}

}

pub mod bindings {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

pub struct AicAgentAdminSoapBinding {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                
                #[async_trait]
                impl ports::AicAgentAdmin for AicAgentAdminSoapBinding {
                	async fn get (&mut self, get_request: ports::GetRequest) -> Result<ports::GetResponse, ports::AicServiceFault> {

        let __request = GetRequestSoapEnvelope::new(SoapGetRequest {
            body: get_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn update (&mut self, update_request: ports::UpdateRequest) -> Result<ports::UpdateResponse, ports::AicServiceFault> {

        let __request = UpdateRequestSoapEnvelope::new(SoapUpdateRequest {
            body: update_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: UpdateResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn delete (&mut self, delete_request: ports::DeleteRequest) -> Result<ports::DeleteResponse, ports::AicServiceFault> {

        let __request = DeleteRequestSoapEnvelope::new(SoapDeleteRequest {
            body: delete_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: DeleteResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_agent_ids (&mut self, lookup_agent_ids_request: ports::LookupAgentIdsRequest) -> Result<ports::LookupAgentIdsResponse, ports::AicServiceFault> {

        let __request = LookupAgentIdsRequestSoapEnvelope::new(SoapLookupAgentIdsRequest {
            body: lookup_agent_ids_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupAgentIdsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_lrm_ids (&mut self, lookup_lrm_ids_request: ports::LookupLRMIdsRequest) -> Result<ports::LookupLRMIdsResponse, ports::AicServiceFault> {

        let __request = LookupLRMIdsRequestSoapEnvelope::new(SoapLookupLRMIdsRequest {
            body: lookup_lrm_ids_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupLRMIdsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_workgroups (&mut self, lookup_workgroups_request: ports::LookupWorkgroupsRequest) -> Result<ports::LookupWorkgroupsResponse, ports::AicServiceFault> {

        let __request = LookupWorkgroupsRequestSoapEnvelope::new(SoapLookupWorkgroupsRequest {
            body: lookup_workgroups_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupWorkgroupsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_domains (&mut self, lookup_domains_request: ports::LookupDomainsRequest) -> Result<ports::LookupDomainsResponse, ports::AicServiceFault> {

        let __request = LookupDomainsRequestSoapEnvelope::new(SoapLookupDomainsRequest {
            body: lookup_domains_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupDomainsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_link_groups (&mut self, lookup_link_groups_request: ports::LookupLinkGroupsRequest) -> Result<ports::LookupLinkGroupsResponse, ports::AicServiceFault> {

        let __request = LookupLinkGroupsRequestSoapEnvelope::new(SoapLookupLinkGroupsRequest {
            body: lookup_link_groups_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupLinkGroupsResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_phone_types (&mut self, lookup_phone_types_request: ports::LookupPhoneTypesRequest) -> Result<ports::LookupPhoneTypesResponse, ports::AicServiceFault> {

        let __request = LookupPhoneTypesRequestSoapEnvelope::new(SoapLookupPhoneTypesRequest {
            body: lookup_phone_types_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupPhoneTypesResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn lookup_sites (&mut self, lookup_sites_request: ports::LookupSitesRequest) -> Result<ports::LookupSitesResponse, ports::AicServiceFault> {

        let __request = LookupSitesRequestSoapEnvelope::new(SoapLookupSitesRequest {
            body: lookup_sites_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: LookupSitesResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
	async fn create (&mut self, create_request: ports::CreateRequest) -> Result<ports::CreateResponse, ports::AicServiceFault> {

        let __request = CreateRequestSoapEnvelope::new(SoapCreateRequest {
            body: create_request,
            xmlns: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post(&self.url)
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: CreateResponseSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        Ok(r.body.body)}
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
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
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
                    tnsattr: Option::from("http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
}

pub mod ports {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

#[async_trait]
pub trait AicAgentAdmin {
	async fn get (&mut self, get_request: GetRequest) -> Result<GetResponse, AicServiceFault>;
	async fn update (&mut self, update_request: UpdateRequest) -> Result<UpdateResponse, AicServiceFault>;
	async fn delete (&mut self, delete_request: DeleteRequest) -> Result<DeleteResponse, AicServiceFault>;
	async fn lookup_agent_ids (&mut self, lookup_agent_ids_request: LookupAgentIdsRequest) -> Result<LookupAgentIdsResponse, AicServiceFault>;
	async fn lookup_lrm_ids (&mut self, lookup_lrm_ids_request: LookupLRMIdsRequest) -> Result<LookupLRMIdsResponse, AicServiceFault>;
	async fn lookup_workgroups (&mut self, lookup_workgroups_request: LookupWorkgroupsRequest) -> Result<LookupWorkgroupsResponse, AicServiceFault>;
	async fn lookup_domains (&mut self, lookup_domains_request: LookupDomainsRequest) -> Result<LookupDomainsResponse, AicServiceFault>;
	async fn lookup_link_groups (&mut self, lookup_link_groups_request: LookupLinkGroupsRequest) -> Result<LookupLinkGroupsResponse, AicServiceFault>;
	async fn lookup_phone_types (&mut self, lookup_phone_types_request: LookupPhoneTypesRequest) -> Result<LookupPhoneTypesResponse, AicServiceFault>;
	async fn lookup_sites (&mut self, lookup_sites_request: LookupSitesRequest) -> Result<LookupSitesResponse, AicServiceFault>;
	async fn create (&mut self, create_request: CreateRequest) -> Result<CreateResponse, AicServiceFault>;
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
}

use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct Header {}
    