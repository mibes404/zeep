//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes may be lost during subsequent runs of the code generator.
//!
use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            pub mod ports {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

}


    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct Header {}
    pub mod types {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

pub type SecureStore = XmlSecureStore;

pub type User = XmlUser;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "users", default)]
pub struct Users {
	#[yaserde(prefix = "tns", rename = "secureStore", default)]
	pub secure_store: Vec<XmlSecureStore>,
	#[yaserde(prefix = "tns", rename = "user", default)]
	pub user: Vec<XmlUser>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "UserProvisionRules", default)]
pub struct UserProvisionRules {
	#[yaserde(prefix = "tns", rename = "UserProvisionRuleName", default)]
	pub user_provision_rule_name: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "roles", default)]
pub struct Roles {
	#[yaserde(prefix = "tns", rename = "role", default)]
	pub role: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "ownedContactLists", default)]
pub struct OwnedContactLists {
	#[yaserde(prefix = "tns", rename = "contactList", default)]
	pub contact_list: Vec<XmlContactList>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "ownedContacts", default)]
pub struct OwnedContacts {
	#[yaserde(prefix = "tns", rename = "contact", default)]
	pub contact: Vec<XmlContact>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlUser", default)]
pub struct XmlUser {
	#[yaserde(prefix = "tns", rename = "UserOrganizationDetails", default)]
	pub user_organization_details: Vec<UserOrganizationDetailsType>,
	#[yaserde(prefix = "tns", rename = "authenticationType", default)]
	pub authentication_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "description", default)]
	pub description: String,
	#[yaserde(prefix = "tns", rename = "displayName", default)]
	pub display_name: String,
	#[yaserde(prefix = "tns", rename = "displayNameAscii", default)]
	pub display_name_ascii: String,
	#[yaserde(prefix = "tns", rename = "dn", default)]
	pub dn: String,
	#[yaserde(prefix = "tns", rename = "isDuplicatedLoginAllowed", default)]
	pub is_duplicated_login_allowed: bool,
	#[yaserde(prefix = "tns", rename = "isEnabled", default)]
	pub is_enabled: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "isVirtualUser", default)]
	pub is_virtual_user: bool,
	#[yaserde(prefix = "tns", rename = "givenName", default)]
	pub given_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
	pub given_name_ascii: Vec<String>,
	#[yaserde(prefix = "tns", rename = "honorific", default)]
	pub honorific: String,
	#[yaserde(prefix = "tns", rename = "employeeNo", default)]
	pub employee_no: Vec<String>,
	#[yaserde(prefix = "tns", rename = "department", default)]
	pub department: Vec<String>,
	#[yaserde(prefix = "tns", rename = "organization", default)]
	pub organization: Vec<String>,
	#[yaserde(prefix = "tns", rename = "middleName", default)]
	pub middle_name: String,
	#[yaserde(prefix = "tns", rename = "managerName", default)]
	pub manager_name: String,
	#[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
	pub preferred_given_name: String,
	#[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
	pub preferred_language: String,
	#[yaserde(prefix = "tns", rename = "source", default)]
	pub source: Vec<String>,
	#[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
	pub source_user_key: Vec<String>,
	#[yaserde(prefix = "tns", rename = "status", default)]
	pub status: String,
	#[yaserde(prefix = "tns", rename = "suffix", default)]
	pub suffix: String,
	#[yaserde(prefix = "tns", rename = "surname", default)]
	pub surname: Vec<String>,
	#[yaserde(prefix = "tns", rename = "surnameAscii", default)]
	pub surname_ascii: Vec<String>,
	#[yaserde(prefix = "tns", rename = "timeZone", default)]
	pub time_zone: String,
	#[yaserde(prefix = "tns", rename = "title", default)]
	pub title: String,
	#[yaserde(prefix = "tns", rename = "userName", default)]
	pub user_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "userPassword", default)]
	pub user_password: String,
	#[yaserde(prefix = "tns", rename = "commPassword", default)]
	pub comm_password: String,
	#[yaserde(prefix = "tns", rename = "userType", default)]
	pub user_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "localizedNames", default)]
	pub localized_names: Vec<XmLocalizedNames>,
	#[yaserde(prefix = "tns", rename = "address", default)]
	pub address: Vec<XmlAddress>,
	#[yaserde(prefix = "tns", rename = "securityIdentity", default)]
	pub security_identity: Vec<XmlSecurityIdentity>,
	#[yaserde(prefix = "tns", rename = "presenceUserDefault", default)]
	pub presence_user_default: XmlPresUserDefaultType,
	#[yaserde(prefix = "tns", rename = "presenceUserACL", default)]
	pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
	#[yaserde(prefix = "tns", rename = "presenceUserCLDefault", default)]
	pub presence_user_cl_default: Vec<XmlPresUserCLDefaultType>,
	#[yaserde(prefix = "tns", rename = "commProfileSet", default)]
	pub comm_profile_set: Vec<XmlCommProfileSetType>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlSecurityIdentity", default)]
pub struct XmlSecurityIdentity {
	#[yaserde(prefix = "tns", rename = "identity", default)]
	pub identity: Vec<String>,
	#[yaserde(prefix = "tns", rename = "realm", default)]
	pub realm: String,
	#[yaserde(prefix = "tns", rename = "type", default)]
	pub rs_type: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresInfoTypeAccessType", default)]
pub struct XmlPresInfoTypeAccessType {
	#[yaserde(prefix = "tns", rename = "infoType", default)]
	pub info_type: Vec<XmlPresInfoTypeType>,
	#[yaserde(prefix = "tns", rename = "access", default)]
	pub access: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresACRuleType", default)]
pub struct XmlPresACRuleType {
	#[yaserde(prefix = "tns", rename = "infoTypeAccess", default)]
	pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresUserDefaultType", default)]
pub struct XmlPresUserDefaultType {
	#[yaserde(flatten)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresUserCLDefaultType", default)]
pub struct XmlPresUserCLDefaultType {
	#[yaserde(flatten)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresUserACLEntryType", default)]
pub struct XmlPresUserACLEntryType {
	#[yaserde(flatten)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlPresInfoTypeType", default)]
pub struct XmlPresInfoTypeType {
	#[yaserde(prefix = "tns", rename = "label", default)]
	pub label: Vec<String>,
	#[yaserde(prefix = "tns", rename = "filter", default)]
	pub filter: Vec<String>,
	#[yaserde(prefix = "tns", rename = "specFlags", default)]
	pub spec_flags: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlContactList", default)]
pub struct XmlContactList {
	#[yaserde(prefix = "tns", rename = "name", default)]
	pub name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "description", default)]
	pub description: String,
	#[yaserde(prefix = "tns", rename = "isPublic", default)]
	pub is_public: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "members", default)]
	pub members: Vec<XmlContactListMember>,
	#[yaserde(prefix = "tns", rename = "contactListType", default)]
	pub contact_list_type: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlContactListMember", default)]
pub struct XmlContactListMember {
	#[yaserde(prefix = "tns", rename = "isFavorite", default)]
	pub is_favorite: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "isSpeedDial", default)]
	pub is_speed_dial: bool,
	#[yaserde(prefix = "tns", rename = "speedDialEntry", default)]
	pub speed_dial_entry: u64,
	#[yaserde(prefix = "tns", rename = "isPresenceBuddy", default)]
	pub is_presence_buddy: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "label", default)]
	pub label: String,
	#[yaserde(prefix = "tns", rename = "altLabel", default)]
	pub alt_label: String,
	#[yaserde(prefix = "tns", rename = "description", default)]
	pub description: String,
	#[yaserde(prefix = "tns", rename = "priorityLevel", default)]
	pub priority_level: u64,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlContactAddress", default)]
pub struct XmlContactAddress {
	#[yaserde(prefix = "tns", rename = "address", default)]
	pub address: Vec<String>,
	#[yaserde(prefix = "tns", rename = "altLabel", default)]
	pub alt_label: String,
	#[yaserde(prefix = "tns", rename = "contactCategory", default)]
	pub contact_category: Vec<String>,
	#[yaserde(prefix = "tns", rename = "contactType", default)]
	pub contact_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "label", default)]
	pub label: String,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlAddress", default)]
pub struct XmlAddress {
	#[yaserde(prefix = "tns", rename = "addressType", default)]
	pub address_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "name", default)]
	pub name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "building", default)]
	pub building: String,
	#[yaserde(prefix = "tns", rename = "localityName", default)]
	pub locality_name: String,
	#[yaserde(prefix = "tns", rename = "postalCode", default)]
	pub postal_code: String,
	#[yaserde(prefix = "tns", rename = "room", default)]
	pub room: String,
	#[yaserde(prefix = "tns", rename = "stateOrProvince", default)]
	pub state_or_province: String,
	#[yaserde(prefix = "tns", rename = "country", default)]
	pub country: String,
	#[yaserde(prefix = "tns", rename = "street", default)]
	pub street: String,
	#[yaserde(prefix = "tns", rename = "businessphone", default)]
	pub businessphone: String,
	#[yaserde(prefix = "tns", rename = "otherbusinessphone", default)]
	pub otherbusinessphone: String,
	#[yaserde(prefix = "tns", rename = "fax", default)]
	pub fax: String,
	#[yaserde(prefix = "tns", rename = "homephone", default)]
	pub homephone: String,
	#[yaserde(prefix = "tns", rename = "otherhomephone", default)]
	pub otherhomephone: String,
	#[yaserde(prefix = "tns", rename = "mobilephone", default)]
	pub mobilephone: String,
	#[yaserde(prefix = "tns", rename = "othermobilephone", default)]
	pub othermobilephone: String,
	#[yaserde(prefix = "tns", rename = "pager", default)]
	pub pager: String,
	#[yaserde(prefix = "tns", rename = "pager2", default)]
	pub pager_2: String,
	#[yaserde(prefix = "tns", rename = "isPrivate", default)]
	pub is_private: bool,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlContact", default)]
pub struct XmlContact {
	#[yaserde(prefix = "tns", rename = "company", default)]
	pub company: String,
	#[yaserde(prefix = "tns", rename = "description", default)]
	pub description: String,
	#[yaserde(prefix = "tns", rename = "displayName", default)]
	pub display_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "displayNameAscii", default)]
	pub display_name_ascii: Vec<String>,
	#[yaserde(prefix = "tns", rename = "dn", default)]
	pub dn: String,
	#[yaserde(prefix = "tns", rename = "givenName", default)]
	pub given_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
	pub given_name_ascii: Vec<String>,
	#[yaserde(prefix = "tns", rename = "initials", default)]
	pub initials: String,
	#[yaserde(prefix = "tns", rename = "middleName", default)]
	pub middle_name: String,
	#[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
	pub preferred_given_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
	pub preferred_language: String,
	#[yaserde(prefix = "tns", rename = "isPublic", default)]
	pub is_public: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "source", default)]
	pub source: Vec<String>,
	#[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
	pub source_user_key: Vec<String>,
	#[yaserde(prefix = "tns", rename = "suffix", default)]
	pub suffix: String,
	#[yaserde(prefix = "tns", rename = "surname", default)]
	pub surname: Vec<String>,
	#[yaserde(prefix = "tns", rename = "surnameAscii", default)]
	pub surname_ascii: Vec<String>,
	#[yaserde(prefix = "tns", rename = "title", default)]
	pub title: String,
	#[yaserde(prefix = "tns", rename = "ContactAddress", default)]
	pub contact_address: Vec<XmlContactAddress>,
	#[yaserde(prefix = "tns", rename = "addresses", default)]
	pub addresses: Vec<XmlAddress>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlHandle", default)]
pub struct XmlHandle {
	#[yaserde(prefix = "tns", rename = "handleName", default)]
	pub handle_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "handleType", default)]
	pub handle_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "handleSubType", default)]
	pub handle_sub_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "domainName", default)]
	pub domain_name: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlCommProfileType", default)]
pub struct XmlCommProfileType {
	#[yaserde(prefix = "tns", rename = "commProfileType", default)]
	pub comm_profile_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "commProfileSubType", default)]
	pub comm_profile_sub_type: Vec<String>,
	#[yaserde(prefix = "tns", rename = "jobId", default)]
	pub job_id: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "handleList", default)]
pub struct HandleList {
	#[yaserde(prefix = "tns", rename = "handle", default)]
	pub handle: Vec<XmlHandle>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "commProfileList", default)]
pub struct CommProfileList {
	#[yaserde(prefix = "tns", rename = "commProfile", default)]
	pub comm_profile: Vec<XmlCommProfileType>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlCommProfileSetType", default)]
pub struct XmlCommProfileSetType {
	#[yaserde(prefix = "tns", rename = "commProfileSetName", default)]
	pub comm_profile_set_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "isPrimary", default)]
	pub is_primary: Vec<bool>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "ForgeinCommProfileType", default)]
pub struct ForgeinCommProfileType {
	#[yaserde(flatten)]
	pub xml_comm_profile_type: XmlCommProfileType,
	#[yaserde(prefix = "tns", rename = "csEncryptionKeyId", default)]
	pub cs_encryption_key_id: Vec<u64>,
	#[yaserde(prefix = "tns", rename = "servicePassword", default)]
	pub service_password: Vec<String>,
	#[yaserde(prefix = "tns", rename = "serviceData", default)]
	pub service_data: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlSecureStore", default)]
pub struct XmlSecureStore {
	#[yaserde(prefix = "tns", rename = "secureStoreData", default)]
	pub secure_store_data: Vec<String>,
	#[yaserde(prefix = "tns", rename = "passwordEncrypted", default)]
	pub password_encrypted: bool,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlLocalizedName", default)]
pub struct XmlLocalizedName {
	#[yaserde(prefix = "tns", rename = "locale", default)]
	pub locale: Vec<String>,
	#[yaserde(prefix = "tns", rename = "name", default)]
	pub name: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmLocalizedNames", default)]
pub struct XmLocalizedNames {
	#[yaserde(prefix = "tns", rename = "localizedName", default)]
	pub localized_name: Vec<XmlLocalizedName>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "tenant", default)]
pub struct Tenant {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "UserOrganizationDetailsType", default)]
pub struct UserOrganizationDetailsType {
	#[yaserde(prefix = "tns", rename = "organizationUnitLevelOne", default)]
	pub organization_unit_level_one: Vec<String>,
	#[yaserde(prefix = "tns", rename = "organizationUnitLevelTwo", default)]
	pub organization_unit_level_two: Vec<String>,
	#[yaserde(prefix = "tns", rename = "organizationUnitLevelThree", default)]
	pub organization_unit_level_three: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlAgentProfile", default)]
pub struct XmlAgentProfile {
	#[yaserde(flatten)]
	pub xml_comm_profile_type: XmlCommProfileType,
	#[yaserde(prefix = "tns", rename = "cmName", default)]
	pub cm_name: Vec<String>,
	#[yaserde(prefix = "tns", rename = "useExistingAgent", default)]
	pub use_existing_agent: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "template", default)]
	pub template: Vec<String>,
	#[yaserde(prefix = "tns", rename = "aas", default)]
	pub aas: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "audix", default)]
	pub audix: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "deleteOnUnassign", default)]
	pub delete_on_unassign: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "lwcLogExternalCalls", default)]
	pub lwc_log_external_calls: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "audixNameforMessaging", default)]
	pub audix_namefor_messaging: Vec<String>,
	#[yaserde(prefix = "tns", rename = "hearsServiceObservingTone", default)]
	pub hears_service_observing_tone: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "loginIDforISDNSIPDisplay", default)]
	pub login_i_dfor_isdnsip_display: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "serviceObjective", default)]
	pub service_objective: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "directAgentCallsFirst", default)]
	pub direct_agent_calls_first: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "localCallPreference", default)]
	pub local_call_preference: Vec<bool>,
	#[yaserde(prefix = "tns", rename = "skills", default)]
	pub skills: Vec<XmlAgentLoginIdSkillsData>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://xml.avaya.com/schema/import", rename = "xmlAgentLoginIdSkillsData", default)]
pub struct XmlAgentLoginIdSkillsData {
	#[yaserde(prefix = "tns", rename = "number", default)]
	pub number: Vec<String>,
}

}

pub mod messages {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

}

pub mod bindings {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;

}

