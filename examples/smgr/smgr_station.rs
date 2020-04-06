//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!

            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod types {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            pub type SecureStore = XmlSecureStore;

pub type User = XmlUser;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "users",
	namespace = "tns: http://xml.avaya.com/schema/import",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct Users {
	#[yaserde(rename = "secureStore", default)]
	pub secure_store: Option<XmlSecureStore>, 
	#[yaserde(rename = "user", default)]
	pub user: Option<XmlUser>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "UserProvisionRules",
)]
pub struct UserProvisionRules {
	#[yaserde(rename = "UserProvisionRuleName", default)]
	pub user_provision_rule_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "roles",
)]
pub struct Roles {
	#[yaserde(rename = "role", default)]
	pub role: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "ownedContactLists",
)]
pub struct OwnedContactLists {
	#[yaserde(rename = "contactList", default)]
	pub contact_list: XmlContactList, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "ownedContacts",
)]
pub struct OwnedContacts {
	#[yaserde(rename = "contact", default)]
	pub contact: XmlContact, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlUser",
)]
pub struct XmlUser {
	#[yaserde(rename = "UserOrganizationDetails", default)]
	pub user_organization_details: Option<UserOrganizationDetailsType>, 
	#[yaserde(rename = "UserProvisionRules", default)]
	pub user_provision_rules: Option<UserProvisionRules>, 
	#[yaserde(rename = "authenticationType", default)]
	pub authentication_type: String, 
	#[yaserde(rename = "description", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "displayName", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "displayNameAscii", default)]
	pub display_name_ascii: Option<String>, 
	#[yaserde(rename = "dn", default)]
	pub dn: Option<String>, 
	#[yaserde(rename = "isDuplicatedLoginAllowed", default)]
	pub is_duplicated_login_allowed: Option<bool>, 
	#[yaserde(rename = "isEnabled", default)]
	pub is_enabled: Option<bool>, 
	#[yaserde(rename = "isVirtualUser", default)]
	pub is_virtual_user: Option<bool>, 
	#[yaserde(rename = "givenName", default)]
	pub given_name: String, 
	#[yaserde(rename = "givenNameAscii", default)]
	pub given_name_ascii: Option<String>, 
	#[yaserde(rename = "honorific", default)]
	pub honorific: Option<String>, 
	#[yaserde(rename = "loginName", default)]
	pub login_name: String, 
	#[yaserde(rename = "newLoginName", default)]
	pub new_login_name: Option<String>, 
	#[yaserde(rename = "employeeNo", default)]
	pub employee_no: Option<String>, 
	#[yaserde(rename = "department", default)]
	pub department: Option<String>, 
	#[yaserde(rename = "organization", default)]
	pub organization: Option<String>, 
	#[yaserde(rename = "middleName", default)]
	pub middle_name: Option<String>, 
	#[yaserde(rename = "managerName", default)]
	pub manager_name: Option<String>, 
	#[yaserde(rename = "preferredGivenName", default)]
	pub preferred_given_name: Option<String>, 
	#[yaserde(rename = "preferredLanguage", default)]
	pub preferred_language: Option<String>, 
	#[yaserde(rename = "source", default)]
	pub source: Option<String>, 
	#[yaserde(rename = "sourceUserKey", default)]
	pub source_user_key: Option<String>, 
	#[yaserde(rename = "status", default)]
	pub status: Option<String>, 
	#[yaserde(rename = "suffix", default)]
	pub suffix: Option<String>, 
	#[yaserde(rename = "surname", default)]
	pub surname: String, 
	#[yaserde(rename = "surnameAscii", default)]
	pub surname_ascii: Option<String>, 
	#[yaserde(rename = "timeZone", default)]
	pub time_zone: Option<String>, 
	#[yaserde(rename = "title", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "userName", default)]
	pub user_name: Option<String>, 
	#[yaserde(rename = "userPassword", default)]
	pub user_password: Option<String>, 
	#[yaserde(rename = "commPassword", default)]
	pub comm_password: Option<String>, 
	#[yaserde(rename = "userType", default)]
	pub user_type: Option<String>, 
	#[yaserde(rename = "roles", default)]
	pub roles: Option<Roles>, 
	#[yaserde(rename = "localizedNames", default)]
	pub localized_names: Option<XmLocalizedNames>, 
	#[yaserde(rename = "address", default)]
	pub address: Option<XmlAddress>, 
	#[yaserde(rename = "securityIdentity", default)]
	pub security_identity: Option<XmlSecurityIdentity>, 
	#[yaserde(rename = "ownedContactLists", default)]
	pub owned_contact_lists: Option<OwnedContactLists>, 
	#[yaserde(rename = "ownedContacts", default)]
	pub owned_contacts: Option<OwnedContacts>, 
	#[yaserde(rename = "presenceUserDefault", default)]
	pub presence_user_default: Option<XmlPresUserDefaultType>, 
	#[yaserde(rename = "presenceUserACL", default)]
	pub presence_user_acl: Option<XmlPresUserACLEntryType>, 
	#[yaserde(rename = "presenceUserCLDefault", default)]
	pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>, 
	#[yaserde(rename = "commProfileSet", default)]
	pub comm_profile_set: Option<XmlCommProfileSetType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlSecurityIdentity",
)]
pub struct XmlSecurityIdentity {
	#[yaserde(rename = "identity", default)]
	pub identity: String, 
	#[yaserde(rename = "realm", default)]
	pub realm: Option<String>, 
	#[yaserde(rename = "type", default)]
	pub rs_type: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresInfoTypeAccessType",
)]
pub struct XmlPresInfoTypeAccessType {
	#[yaserde(rename = "infoType", default)]
	pub info_type: XmlPresInfoTypeType, 
	#[yaserde(rename = "access", default)]
	pub access: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresACRuleType",
)]
pub struct XmlPresACRuleType {
	#[yaserde(rename = "infoTypeAccess", default)]
	pub info_type_access: Option<XmlPresInfoTypeAccessType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresUserDefaultType",
)]
pub struct XmlPresUserDefaultType {
	#[yaserde(flatten, default)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresUserCLDefaultType",
)]
pub struct XmlPresUserCLDefaultType {
	#[yaserde(flatten, default)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresUserACLEntryType",
)]
pub struct XmlPresUserACLEntryType {
	#[yaserde(flatten, default)]
	pub xml_pres_ac_rule_type: XmlPresACRuleType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlPresInfoTypeType",
)]
pub struct XmlPresInfoTypeType {
	#[yaserde(rename = "label", default)]
	pub label: String, 
	#[yaserde(rename = "filter", default)]
	pub filter: String, 
	#[yaserde(rename = "specFlags", default)]
	pub spec_flags: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlContactList",
)]
pub struct XmlContactList {
	#[yaserde(rename = "name", default)]
	pub name: String, 
	#[yaserde(rename = "description", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "isPublic", default)]
	pub is_public: bool, 
	#[yaserde(rename = "members", default)]
	pub members: Option<XmlContactListMember>, 
	#[yaserde(rename = "contactListType", default)]
	pub contact_list_type: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlContactListMember",
)]
pub struct XmlContactListMember {
	#[yaserde(rename = "isFavorite", default)]
	pub is_favorite: bool, 
	#[yaserde(rename = "isSpeedDial", default)]
	pub is_speed_dial: bool, 
	#[yaserde(rename = "speedDialEntry", default)]
	pub speed_dial_entry: Option<i32>, 
	#[yaserde(rename = "isPresenceBuddy", default)]
	pub is_presence_buddy: bool, 
	#[yaserde(rename = "label", default)]
	pub label: Option<String>, 
	#[yaserde(rename = "altLabel", default)]
	pub alt_label: Option<String>, 
	#[yaserde(rename = "description", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "priorityLevel", default)]
	pub priority_level: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlContactAddress",
)]
pub struct XmlContactAddress {
	#[yaserde(rename = "address", default)]
	pub address: String, 
	#[yaserde(rename = "altLabel", default)]
	pub alt_label: Option<String>, 
	#[yaserde(rename = "contactCategory", default)]
	pub contact_category: String, 
	#[yaserde(rename = "contactType", default)]
	pub contact_type: String, 
	#[yaserde(rename = "label", default)]
	pub label: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlAddress",
)]
pub struct XmlAddress {
	#[yaserde(rename = "addressType", default)]
	pub address_type: String, 
	#[yaserde(rename = "name", default)]
	pub name: String, 
	#[yaserde(rename = "building", default)]
	pub building: Option<String>, 
	#[yaserde(rename = "localityName", default)]
	pub locality_name: Option<String>, 
	#[yaserde(rename = "postalCode", default)]
	pub postal_code: Option<String>, 
	#[yaserde(rename = "room", default)]
	pub room: Option<String>, 
	#[yaserde(rename = "stateOrProvince", default)]
	pub state_or_province: Option<String>, 
	#[yaserde(rename = "country", default)]
	pub country: Option<String>, 
	#[yaserde(rename = "street", default)]
	pub street: Option<String>, 
	#[yaserde(rename = "businessphone", default)]
	pub businessphone: Option<String>, 
	#[yaserde(rename = "otherbusinessphone", default)]
	pub otherbusinessphone: Option<String>, 
	#[yaserde(rename = "fax", default)]
	pub fax: Option<String>, 
	#[yaserde(rename = "homephone", default)]
	pub homephone: Option<String>, 
	#[yaserde(rename = "otherhomephone", default)]
	pub otherhomephone: Option<String>, 
	#[yaserde(rename = "mobilephone", default)]
	pub mobilephone: Option<String>, 
	#[yaserde(rename = "othermobilephone", default)]
	pub othermobilephone: Option<String>, 
	#[yaserde(rename = "pager", default)]
	pub pager: Option<String>, 
	#[yaserde(rename = "pager2", default)]
	pub pager_2: Option<String>, 
	#[yaserde(rename = "postalAddress", default)]
	pub postal_address: Option<String>, 
	#[yaserde(rename = "isPrivate", default)]
	pub is_private: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlContact",
)]
pub struct XmlContact {
	#[yaserde(rename = "company", default)]
	pub company: Option<String>, 
	#[yaserde(rename = "description", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "displayName", default)]
	pub display_name: String, 
	#[yaserde(rename = "displayNameAscii", default)]
	pub display_name_ascii: String, 
	#[yaserde(rename = "dn", default)]
	pub dn: Option<String>, 
	#[yaserde(rename = "givenName", default)]
	pub given_name: String, 
	#[yaserde(rename = "givenNameAscii", default)]
	pub given_name_ascii: Option<String>, 
	#[yaserde(rename = "initials", default)]
	pub initials: Option<String>, 
	#[yaserde(rename = "middleName", default)]
	pub middle_name: Option<String>, 
	#[yaserde(rename = "preferredGivenName", default)]
	pub preferred_given_name: Option<String>, 
	#[yaserde(rename = "preferredLanguage", default)]
	pub preferred_language: Option<String>, 
	#[yaserde(rename = "isPublic", default)]
	pub is_public: bool, 
	#[yaserde(rename = "source", default)]
	pub source: String, 
	#[yaserde(rename = "sourceUserKey", default)]
	pub source_user_key: String, 
	#[yaserde(rename = "suffix", default)]
	pub suffix: Option<String>, 
	#[yaserde(rename = "surname", default)]
	pub surname: String, 
	#[yaserde(rename = "surnameAscii", default)]
	pub surname_ascii: Option<String>, 
	#[yaserde(rename = "title", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "ContactAddress", default)]
	pub contact_address: Option<XmlContactAddress>, 
	#[yaserde(rename = "addresses", default)]
	pub addresses: Option<XmlAddress>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlHandle",
)]
pub struct XmlHandle {
	#[yaserde(rename = "handleName", default)]
	pub handle_name: String, 
	#[yaserde(rename = "handleType", default)]
	pub handle_type: String, 
	#[yaserde(rename = "handleSubType", default)]
	pub handle_sub_type: Option<String>, 
	#[yaserde(rename = "domainName", default)]
	pub domain_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlCommProfileType",
)]
pub struct XmlCommProfileType {
	#[yaserde(rename = "commProfileType", default)]
	pub comm_profile_type: String, 
	#[yaserde(rename = "commProfileSubType", default)]
	pub comm_profile_sub_type: Option<String>, 
	#[yaserde(rename = "jobId", default)]
	pub job_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "handleList",
)]
pub struct HandleList {
	#[yaserde(rename = "handle", default)]
	pub handle: XmlHandle, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "commProfileList",
)]
pub struct CommProfileList {
	#[yaserde(rename = "commProfile", default)]
	pub comm_profile: XmlCommProfileType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlCommProfileSetType",
)]
pub struct XmlCommProfileSetType {
	#[yaserde(rename = "commProfileSetName", default)]
	pub comm_profile_set_name: String, 
	#[yaserde(rename = "isPrimary", default)]
	pub is_primary: bool, 
	#[yaserde(rename = "handleList", default)]
	pub handle_list: Option<HandleList>, 
	#[yaserde(rename = "commProfileList", default)]
	pub comm_profile_list: Option<CommProfileList>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "ForgeinCommProfileType",
)]
pub struct ForgeinCommProfileType {
	#[yaserde(flatten, default)]
	pub xml_comm_profile_type: XmlCommProfileType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "csEncryptionKeyId", default)]
	pub cs_encryption_key_id: Option<i64>, 
	#[yaserde(rename = "servicePassword", default)]
	pub service_password: Option<String>, 
	#[yaserde(rename = "serviceData", default)]
	pub service_data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlSecureStore",
)]
pub struct XmlSecureStore {
	#[yaserde(rename = "secureStoreData", default)]
	pub secure_store_data: String, 
	#[yaserde(rename = "passwordEncrypted", default)]
	pub password_encrypted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlLocalizedName",
)]
pub struct XmlLocalizedName {
	#[yaserde(rename = "locale", default)]
	pub locale: String, 
	#[yaserde(rename = "name", default)]
	pub name: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmLocalizedNames",
)]
pub struct XmLocalizedNames {
	#[yaserde(rename = "localizedName", default)]
	pub localized_name: Option<XmlLocalizedName>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "tenant",
)]
pub struct Tenant {
#[yaserde(rename="name", attribute)]
pub name: String,
#[yaserde(rename="createTenantIfNotAlreadyPresent", attribute)]
pub create_tenant_if_not_already_present: bool,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "UserOrganizationDetailsType",
)]
pub struct UserOrganizationDetailsType {
	#[yaserde(rename = "tenant", default)]
	pub tenant: Tenant, 
	#[yaserde(rename = "organizationUnitLevelOne", default)]
	pub organization_unit_level_one: Option<String>, 
	#[yaserde(rename = "organizationUnitLevelTwo", default)]
	pub organization_unit_level_two: Option<String>, 
	#[yaserde(rename = "organizationUnitLevelThree", default)]
	pub organization_unit_level_three: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlStationProfile",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlStationProfile {
	#[yaserde(flatten, default)]
	pub xml_comm_profile_type: XmlCommProfileType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "cmName", prefix = "ns2", default)]
	pub cm_name: String, 
	#[yaserde(rename = "prefHandleId", prefix = "ns2", default)]
	pub pref_handle_id: Option<String>, 
	#[yaserde(rename = "useExistingExtension", prefix = "ns2", default)]
	pub use_existing_extension: Option<bool>, 
	#[yaserde(rename = "extensionRange", prefix = "ns2", default)]
	pub extension_range: Option<String>, 
	#[yaserde(rename = "extension", prefix = "ns2", default)]
	pub extension: String, 
	#[yaserde(rename = "template", prefix = "ns2", default)]
	pub template: Option<String>, 
	#[yaserde(rename = "setType", prefix = "ns2", default)]
	pub set_type: Option<String>, 
	#[yaserde(rename = "securityCode", prefix = "ns2", default)]
	pub security_code: Option<String>, 
	#[yaserde(rename = "port", prefix = "ns2", default)]
	pub port: Option<String>, 
	#[yaserde(rename = "deleteOnUnassign", prefix = "ns2", default)]
	pub delete_on_unassign: Option<bool>, 
	#[yaserde(rename = "overRideEndpointName", prefix = "ns2", default)]
	pub over_ride_endpoint_name: Option<bool>, 
	#[yaserde(rename = "dualRegistration", prefix = "ns2", default)]
	pub dual_registration: Option<bool>, 
	#[yaserde(rename = "enhCallrInfodisplay", prefix = "ns2", default)]
	pub enh_callr_infodisplay: Option<bool>, 
	#[yaserde(rename = "lockMessages", prefix = "ns2", default)]
	pub lock_messages: Option<bool>, 
	#[yaserde(rename = "coveragePath1", prefix = "ns2", default)]
	pub coverage_path_1: Option<String>, 
	#[yaserde(rename = "coveragePath2", prefix = "ns2", default)]
	pub coverage_path_2: Option<String>, 
	#[yaserde(rename = "huntToStation", prefix = "ns2", default)]
	pub hunt_to_station: Option<String>, 
	#[yaserde(rename = "tn", prefix = "ns2", default)]
	pub tn: Option<i32>, 
	#[yaserde(rename = "cor", prefix = "ns2", default)]
	pub cor: Option<i32>, 
	#[yaserde(rename = "cos", prefix = "ns2", default)]
	pub cos: Option<i32>, 
	#[yaserde(rename = "xmobileType", prefix = "ns2", default)]
	pub xmobile_type: Option<String>, 
	#[yaserde(rename = "mappingMode", prefix = "ns2", default)]
	pub mapping_mode: Option<String>, 
	#[yaserde(rename = "configurationSet", prefix = "ns2", default)]
	pub configuration_set: Option<String>, 
	#[yaserde(rename = "mobilityTrunkGroup", prefix = "ns2", default)]
	pub mobility_trunk_group: Option<String>, 
	#[yaserde(rename = "dialPrefix", prefix = "ns2", default)]
	pub dial_prefix: Option<String>, 
	#[yaserde(rename = "cellPhoneNumber", prefix = "ns2", default)]
	pub cell_phone_number: Option<String>, 
	#[yaserde(rename = "musicSource", prefix = "ns2", default)]
	pub music_source: Option<i32>, 
	#[yaserde(rename = "tests", prefix = "ns2", default)]
	pub tests: Option<bool>, 
	#[yaserde(rename = "dataModule", prefix = "ns2", default)]
	pub data_module: Option<bool>, 
	#[yaserde(rename = "speakerphone", prefix = "ns2", default)]
	pub speakerphone: Option<String>, 
	#[yaserde(rename = "displayLanguage", prefix = "ns2", default)]
	pub display_language: Option<String>, 
	#[yaserde(rename = "personalizedRingingPattern", prefix = "ns2", default)]
	pub personalized_ringing_pattern: Option<i32>, 
	#[yaserde(rename = "messageLampExt", prefix = "ns2", default)]
	pub message_lamp_ext: Option<String>, 
	#[yaserde(rename = "muteButtonEnabled", prefix = "ns2", default)]
	pub mute_button_enabled: Option<bool>, 
	#[yaserde(rename = "mediaComplexExt", prefix = "ns2", default)]
	pub media_complex_ext: Option<String>, 
	#[yaserde(rename = "ipSoftphone", prefix = "ns2", default)]
	pub ip_softphone: Option<bool>, 
	#[yaserde(rename = "survivableGkNodeName", prefix = "ns2", default)]
	pub survivable_gk_node_name: Option<String>, 
	#[yaserde(rename = "survivableCOR", prefix = "ns2", default)]
	pub survivable_cor: Option<String>, 
	#[yaserde(rename = "survivableTrunkDest", prefix = "ns2", default)]
	pub survivable_trunk_dest: Option<bool>, 
	#[yaserde(rename = "voiceMailNumber", prefix = "ns2", default)]
	pub voice_mail_number: Option<String>, 
	#[yaserde(rename = "offPremisesStation", prefix = "ns2", default)]
	pub off_premises_station: Option<bool>, 
	#[yaserde(rename = "dataOption", prefix = "ns2", default)]
	pub data_option: Option<String>, 
	#[yaserde(rename = "displayModule", prefix = "ns2", default)]
	pub display_module: Option<bool>, 
	#[yaserde(rename = "messageWaitingIndicator", prefix = "ns2", default)]
	pub message_waiting_indicator: Option<String>, 
	#[yaserde(rename = "remoteOfficePhone", prefix = "ns2", default)]
	pub remote_office_phone: Option<bool>, 
	#[yaserde(rename = "lwcReception", prefix = "ns2", default)]
	pub lwc_reception: Option<String>, 
	#[yaserde(rename = "lwcActivation", prefix = "ns2", default)]
	pub lwc_activation: Option<bool>, 
	#[yaserde(rename = "lwcLogExternalCalls", prefix = "ns2", default)]
	pub lwc_log_external_calls: Option<bool>, 
	#[yaserde(rename = "cdrPrivacy", prefix = "ns2", default)]
	pub cdr_privacy: Option<bool>, 
	#[yaserde(rename = "redirectNotification", prefix = "ns2", default)]
	pub redirect_notification: Option<bool>, 
	#[yaserde(rename = "perButtonRingControl", prefix = "ns2", default)]
	pub per_button_ring_control: Option<bool>, 
	#[yaserde(rename = "bridgedCallAlerting", prefix = "ns2", default)]
	pub bridged_call_alerting: Option<bool>, 
	#[yaserde(rename = "bridgedIdleLinePreference", prefix = "ns2", default)]
	pub bridged_idle_line_preference: Option<bool>, 
	#[yaserde(rename = "confTransOnPrimaryAppearance", prefix = "ns2", default)]
	pub conf_trans_on_primary_appearance: Option<bool>, 
	#[yaserde(rename = "customizableLabels", prefix = "ns2", default)]
	pub customizable_labels: Option<bool>, 
	#[yaserde(rename = "expansionModule", prefix = "ns2", default)]
	pub expansion_module: Option<bool>, 
	#[yaserde(rename = "ipVideoSoftphone", prefix = "ns2", default)]
	pub ip_video_softphone: Option<bool>, 
	#[yaserde(rename = "activeStationRinging", prefix = "ns2", default)]
	pub active_station_ringing: Option<String>, 
	#[yaserde(rename = "idleActiveRinging", prefix = "ns2", default)]
	pub idle_active_ringing: Option<String>, 
	#[yaserde(rename = "switchhookFlash", prefix = "ns2", default)]
	pub switchhook_flash: Option<bool>, 
	#[yaserde(rename = "ignoreRotaryDigits", prefix = "ns2", default)]
	pub ignore_rotary_digits: Option<bool>, 
	#[yaserde(rename = "h320Conversion", prefix = "ns2", default)]
	pub h_320_conversion: Option<bool>, 
	#[yaserde(rename = "serviceLinkMode", prefix = "ns2", default)]
	pub service_link_mode: Option<String>, 
	#[yaserde(rename = "multimediaMode", prefix = "ns2", default)]
	pub multimedia_mode: Option<String>, 
	#[yaserde(rename = "mwiServedUserType", prefix = "ns2", default)]
	pub mwi_served_user_type: Option<String>, 
	#[yaserde(rename = "audixName", prefix = "ns2", default)]
	pub audix_name: Option<String>, 
	#[yaserde(rename = "automaticMoves", prefix = "ns2", default)]
	pub automatic_moves: Option<String>, 
	#[yaserde(rename = "remoteSoftphoneEmergencyCalls", prefix = "ns2", default)]
	pub remote_softphone_emergency_calls: Option<String>, 
	#[yaserde(rename = "emergencyLocationExt", prefix = "ns2", default)]
	pub emergency_location_ext: Option<String>, 
	#[yaserde(rename = "alwaysUse", prefix = "ns2", default)]
	pub always_use: Option<bool>, 
	#[yaserde(rename = "precedenceCallWaiting", prefix = "ns2", default)]
	pub precedence_call_waiting: Option<bool>, 
	#[yaserde(rename = "autoSelectAnyIdleAppearance", prefix = "ns2", default)]
	pub auto_select_any_idle_appearance: Option<bool>, 
	#[yaserde(rename = "coverageMsgRetrieval", prefix = "ns2", default)]
	pub coverage_msg_retrieval: Option<bool>, 
	#[yaserde(rename = "autoAnswer", prefix = "ns2", default)]
	pub auto_answer: Option<String>, 
	#[yaserde(rename = "dataRestriction", prefix = "ns2", default)]
	pub data_restriction: Option<bool>, 
	#[yaserde(rename = "idleAppearancePreference", prefix = "ns2", default)]
	pub idle_appearance_preference: Option<bool>, 
	#[yaserde(rename = "callWaitingIndication", prefix = "ns2", default)]
	pub call_waiting_indication: Option<bool>, 
	#[yaserde(rename = "attCallWaitingIndication", prefix = "ns2", default)]
	pub att_call_waiting_indication: Option<bool>, 
	#[yaserde(rename = "distinctiveAudibleAlert", prefix = "ns2", default)]
	pub distinctive_audible_alert: Option<bool>, 
	#[yaserde(rename = "restrictLastAppearance", prefix = "ns2", default)]
	pub restrict_last_appearance: Option<bool>, 
	#[yaserde(rename = "adjunctSupervision", prefix = "ns2", default)]
	pub adjunct_supervision: Option<bool>, 
	#[yaserde(rename = "perStationCpnSendCallingNumber", prefix = "ns2", default)]
	pub per_station_cpn_send_calling_number: Option<String>, 
	#[yaserde(rename = "busyAutoCallbackWithoutFlash", prefix = "ns2", default)]
	pub busy_auto_callback_without_flash: Option<bool>, 
	#[yaserde(rename = "audibleMessageWaiting", prefix = "ns2", default)]
	pub audible_message_waiting: Option<bool>, 
	#[yaserde(rename = "extendedLocalCalls", prefix = "ns2", default)]
	pub extended_local_calls: Option<bool>, 
	#[yaserde(rename = "imsFeatureSequencing", prefix = "ns2", default)]
	pub ims_feature_sequencing: Option<bool>, 
	#[yaserde(rename = "displayClientRedirection", prefix = "ns2", default)]
	pub display_client_redirection: Option<bool>, 
	#[yaserde(rename = "selectLastUsedAppearance", prefix = "ns2", default)]
	pub select_last_used_appearance: Option<bool>, 
	#[yaserde(rename = "coverageAfterForwarding", prefix = "ns2", default)]
	pub coverage_after_forwarding: Option<String>, 
	#[yaserde(rename = "directIpIpAudioConnections", prefix = "ns2", default)]
	pub direct_ip_ip_audio_connections: Option<bool>, 
	#[yaserde(rename = "ipAudioHairpinning", prefix = "ns2", default)]
	pub ip_audio_hairpinning: Option<bool>, 
	#[yaserde(rename = "primeAppearancePreference", prefix = "ns2", default)]
	pub prime_appearance_preference: Option<String>, 
	#[yaserde(rename = "stationSiteData", prefix = "ns2", default)]
	pub station_site_data: Option<XmlStationSiteData>, 
	#[yaserde(rename = "abbrList", prefix = "ns2", default)]
	pub abbr_list: Option<XmlStationAbbreviatedDialingData>, 
	#[yaserde(rename = "buttons", prefix = "ns2", default)]
	pub buttons: Option<XmlButtonData>, 
	#[yaserde(rename = "featureButtons", prefix = "ns2", default)]
	pub feature_buttons: Option<XmlButtonData>, 
	#[yaserde(rename = "expansionModuleButtons", prefix = "ns2", default)]
	pub expansion_module_buttons: Option<XmlButtonData>, 
	#[yaserde(rename = "softKeys", prefix = "ns2", default)]
	pub soft_keys: Option<XmlButtonData>, 
	#[yaserde(rename = "displayButtons", prefix = "ns2", default)]
	pub display_buttons: Option<XmlButtonData>, 
	#[yaserde(rename = "stationDataModule", prefix = "ns2", default)]
	pub station_data_module: Option<XmlStationDataModule>, 
	#[yaserde(rename = "hotLineData", prefix = "ns2", default)]
	pub hot_line_data: Option<XmlStationHotLineData>, 
	#[yaserde(rename = "nativeName", prefix = "ns2", default)]
	pub native_name: Option<XmlNativeNameData>, 
	#[yaserde(rename = "buttonModules", prefix = "ns2", default)]
	pub button_modules: Option<i32>, 
	#[yaserde(rename = "unconditionalInternalDest", prefix = "ns2", default)]
	pub unconditional_internal_dest: Option<String>, 
	#[yaserde(rename = "unconditionalInternalActive", prefix = "ns2", default)]
	pub unconditional_internal_active: Option<bool>, 
	#[yaserde(rename = "unconditionalExternalDest", prefix = "ns2", default)]
	pub unconditional_external_dest: Option<String>, 
	#[yaserde(rename = "unconditionalExternalActive", prefix = "ns2", default)]
	pub unconditional_external_active: Option<bool>, 
	#[yaserde(rename = "busyInternalDest", prefix = "ns2", default)]
	pub busy_internal_dest: Option<String>, 
	#[yaserde(rename = "busyInternalActive", prefix = "ns2", default)]
	pub busy_internal_active: Option<bool>, 
	#[yaserde(rename = "busyExternalDest", prefix = "ns2", default)]
	pub busy_external_dest: Option<String>, 
	#[yaserde(rename = "busyExternalActive", prefix = "ns2", default)]
	pub busy_external_active: Option<bool>, 
	#[yaserde(rename = "noReplyInternalDest", prefix = "ns2", default)]
	pub no_reply_internal_dest: Option<String>, 
	#[yaserde(rename = "noReplyInternalActive", prefix = "ns2", default)]
	pub no_reply_internal_active: Option<bool>, 
	#[yaserde(rename = "noReplyExternalDest", prefix = "ns2", default)]
	pub no_reply_external_dest: Option<String>, 
	#[yaserde(rename = "noReplyExternalActive", prefix = "ns2", default)]
	pub no_reply_external_active: Option<bool>, 
	#[yaserde(rename = "sacCfOverride", prefix = "ns2", default)]
	pub sac_cf_override: Option<String>, 
	#[yaserde(rename = "lossGroup", prefix = "ns2", default)]
	pub loss_group: Option<i32>, 
	#[yaserde(rename = "timeOfDayLockTable", prefix = "ns2", default)]
	pub time_of_day_lock_table: Option<String>, 
	#[yaserde(rename = "emuLoginAllowed", prefix = "ns2", default)]
	pub emu_login_allowed: Option<bool>, 
	#[yaserde(rename = "ec500State", prefix = "ns2", default)]
	pub ec_500_state: Option<String>, 
	#[yaserde(rename = "muteOnOffHookInSCMode", prefix = "ns2", default)]
	pub mute_on_off_hook_in_sc_mode: Option<bool>, 
	#[yaserde(rename = "type3pccEnabled", prefix = "ns2", default)]
	pub type_3pcc_enabled: Option<String>, 
	#[yaserde(rename = "calculateRoutePattern", prefix = "ns2", default)]
	pub calculate_route_pattern: Option<bool>, 
	#[yaserde(rename = "sipTrunk", prefix = "ns2", default)]
	pub sip_trunk: Option<String>, 
	#[yaserde(rename = "enableReachStaDomainControl", prefix = "ns2", default)]
	pub enable_reach_sta_domain_control: Option<String>, 
	#[yaserde(rename = "multimediaEarlyAnswer", prefix = "ns2", default)]
	pub multimedia_early_answer: Option<bool>, 
	#[yaserde(rename = "bridgedApprOrigRestr", prefix = "ns2", default)]
	pub bridged_appr_orig_restr: Option<bool>, 
	#[yaserde(rename = "callApprDispFormat", prefix = "ns2", default)]
	pub call_appr_disp_format: Option<String>, 
	#[yaserde(rename = "ipPhoneGroupId", prefix = "ns2", default)]
	pub ip_phone_group_id: Option<String>, 
	#[yaserde(rename = "xoipEndPointType", prefix = "ns2", default)]
	pub xoip_end_point_type: Option<String>, 
	#[yaserde(rename = "xid", prefix = "ns2", default)]
	pub xid: Option<bool>, 
	#[yaserde(rename = "stepClearing", prefix = "ns2", default)]
	pub step_clearing: Option<bool>, 
	#[yaserde(rename = "fixedTei", prefix = "ns2", default)]
	pub fixed_tei: Option<bool>, 
	#[yaserde(rename = "tei", prefix = "ns2", default)]
	pub tei: Option<String>, 
	#[yaserde(rename = "countryProtocol", prefix = "ns2", default)]
	pub country_protocol: Option<String>, 
	#[yaserde(rename = "endptInit", prefix = "ns2", default)]
	pub endpt_init: Option<bool>, 
	#[yaserde(rename = "spid", prefix = "ns2", default)]
	pub spid: Option<String>, 
	#[yaserde(rename = "endptId", prefix = "ns2", default)]
	pub endpt_id: Option<String>, 
	#[yaserde(rename = "isMCTSignalling", prefix = "ns2", default)]
	pub is_mct_signalling: Option<bool>, 
	#[yaserde(rename = "isShortCallingPartyDisplay", prefix = "ns2", default)]
	pub is_short_calling_party_display: Option<bool>, 
	#[yaserde(rename = "passageWay", prefix = "ns2", default)]
	pub passage_way: Option<bool>, 
	#[yaserde(rename = "dtmfOverIp", prefix = "ns2", default)]
	pub dtmf_over_ip: Option<String>, 
	#[yaserde(rename = "location", prefix = "ns2", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "displayCallerId", prefix = "ns2", default)]
	pub display_caller_id: Option<bool>, 
	#[yaserde(rename = "callerIdMsgWaitingIndication", prefix = "ns2", default)]
	pub caller_id_msg_waiting_indication: Option<bool>, 
	#[yaserde(rename = "recallRotaryDigit", prefix = "ns2", default)]
	pub recall_rotary_digit: Option<bool>, 
	#[yaserde(rename = "profileSettingsData", prefix = "ns2", default)]
	pub profile_settings_data: Option<XmlProfileSettings>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlStationSiteData",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlStationSiteData {
	#[yaserde(rename = "room", prefix = "ns2", default)]
	pub room: Option<String>, 
	#[yaserde(rename = "jack", prefix = "ns2", default)]
	pub jack: Option<String>, 
	#[yaserde(rename = "cable", prefix = "ns2", default)]
	pub cable: Option<String>, 
	#[yaserde(rename = "floor", prefix = "ns2", default)]
	pub floor: Option<String>, 
	#[yaserde(rename = "building", prefix = "ns2", default)]
	pub building: Option<String>, 
	#[yaserde(rename = "headset", prefix = "ns2", default)]
	pub headset: Option<bool>, 
	#[yaserde(rename = "speaker", prefix = "ns2", default)]
	pub speaker: Option<bool>, 
	#[yaserde(rename = "mounting", prefix = "ns2", default)]
	pub mounting: Option<String>, 
	#[yaserde(rename = "cordLength", prefix = "ns2", default)]
	pub cord_length: Option<i32>, 
	#[yaserde(rename = "setColor", prefix = "ns2", default)]
	pub set_color: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlStationAbbreviatedDialingData",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlStationAbbreviatedDialingData {
	#[yaserde(rename = "listType", prefix = "ns2", default)]
	pub list_type: String, 
	#[yaserde(rename = "number", prefix = "ns2", default)]
	pub number: i32, 
	#[yaserde(rename = "listId", prefix = "ns2", default)]
	pub list_id: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlButtonData",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlButtonData {
	#[yaserde(rename = "number", prefix = "ns2", default)]
	pub number: i32, 
	#[yaserde(rename = "type", prefix = "ns2", default)]
	pub rs_type: String, 
	#[yaserde(rename = "data1", prefix = "ns2", default)]
	pub data_1: Option<String>, 
	#[yaserde(rename = "data2", prefix = "ns2", default)]
	pub data_2: Option<String>, 
	#[yaserde(rename = "data3", prefix = "ns2", default)]
	pub data_3: Option<String>, 
	#[yaserde(rename = "data4", prefix = "ns2", default)]
	pub data_4: Option<String>, 
	#[yaserde(rename = "data5", prefix = "ns2", default)]
	pub data_5: Option<String>, 
	#[yaserde(rename = "data6", prefix = "ns2", default)]
	pub data_6: Option<String>, 
	#[yaserde(rename = "isFavorite", prefix = "ns2", default)]
	pub is_favorite: Option<bool>, 
	#[yaserde(rename = "buttonLabel", prefix = "ns2", default)]
	pub button_label: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlStationDataModule",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlStationDataModule {
	#[yaserde(rename = "dataExtension", prefix = "ns2", default)]
	pub data_extension: String, 
	#[yaserde(rename = "name", prefix = "ns2", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "cor", prefix = "ns2", default)]
	pub cor: i32, 
	#[yaserde(rename = "cos", prefix = "ns2", default)]
	pub cos: i32, 
	#[yaserde(rename = "itc", prefix = "ns2", default)]
	pub itc: String, 
	#[yaserde(rename = "tn", prefix = "ns2", default)]
	pub tn: i32, 
	#[yaserde(rename = "listType", prefix = "ns2", default)]
	pub list_type: Option<String>, 
	#[yaserde(rename = "listId", prefix = "ns2", default)]
	pub list_id: Option<i32>, 
	#[yaserde(rename = "specialDialingOption", prefix = "ns2", default)]
	pub special_dialing_option: Option<String>, 
	#[yaserde(rename = "specialDialingAbbrDialCode", prefix = "ns2", default)]
	pub special_dialing_abbr_dial_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlStationHotLineData",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlStationHotLineData {
	#[yaserde(rename = "hotLineDestAbbrevList", prefix = "ns2", default)]
	pub hot_line_dest_abbrev_list: Option<i32>, 
	#[yaserde(rename = "hotLineAbbrevDialCode", prefix = "ns2", default)]
	pub hot_line_abbrev_dial_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlNativeNameData",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlNativeNameData {
	#[yaserde(rename = "locale", prefix = "ns2", default)]
	pub locale: Option<String>, 
	#[yaserde(rename = "name", prefix = "ns2", default)]
	pub name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	root = "xmlProfileSettings",
	namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
	prefix = "ns2",
)]
pub struct XmlProfileSettings {
	#[yaserde(rename = "phoneScreenCalling", prefix = "ns2", default)]
	pub phone_screen_calling: Option<String>, 
	#[yaserde(rename = "profileRedial", prefix = "ns2", default)]
	pub profile_redial: Option<String>, 
	#[yaserde(rename = "dialingOption", prefix = "ns2", default)]
	pub dialing_option: Option<String>, 
	#[yaserde(rename = "headsetSignaling", prefix = "ns2", default)]
	pub headset_signaling: Option<String>, 
	#[yaserde(rename = "audioPath", prefix = "ns2", default)]
	pub audio_path: Option<String>, 
	#[yaserde(rename = "buttonClicks", prefix = "ns2", default)]
	pub button_clicks: Option<String>, 
	#[yaserde(rename = "phoneScreen", prefix = "ns2", default)]
	pub phone_screen: Option<String>, 
	#[yaserde(rename = "backgroundLogo", prefix = "ns2", default)]
	pub background_logo: Option<String>, 
	#[yaserde(rename = "personalizedRinging", prefix = "ns2", default)]
	pub personalized_ringing: Option<String>, 
	#[yaserde(rename = "callPickUpIndication", prefix = "ns2", default)]
	pub call_pick_up_indication: Option<String>, 
	#[yaserde(rename = "touchPanel", prefix = "ns2", default)]
	pub touch_panel: Option<String>, 
	#[yaserde(rename = "language", prefix = "ns2", default)]
	pub language: Option<String>, 
	#[yaserde(rename = "userPreferredLanguage", prefix = "ns2", default)]
	pub user_preferred_language: Option<String>, 
	#[yaserde(rename = "languageFileInUse", prefix = "ns2", default)]
	pub language_file_in_use: Option<String>, 
	#[yaserde(rename = "timeFormat", prefix = "ns2", default)]
	pub time_format: Option<String>, 
	#[yaserde(rename = "awayTimer", prefix = "ns2", default)]
	pub away_timer: Option<String>, 
	#[yaserde(rename = "awayTimerValue", prefix = "ns2", default)]
	pub away_timer_value: Option<i32>, 
}
}

pub mod ports {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod bindings {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod services {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

