//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.1
//!

#![allow(dead_code)]
#![allow(unused_imports)]
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
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type SecureStore = XmlSecureStore;

    pub type User = XmlUser;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "users",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "nsi1"
    )]
    pub struct Users {
        #[yaserde(rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(rename = "user", default)]
        pub user: Vec<XmlUser>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UserProvisionRules")]
    pub struct UserProvisionRules {
        #[yaserde(rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "roles")]
    pub struct Roles {
        #[yaserde(rename = "role", default)]
        pub role: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ownedContactLists")]
    pub struct OwnedContactLists {
        #[yaserde(rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ownedContacts")]
    pub struct OwnedContacts {
        #[yaserde(rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlUser")]
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
        pub user_type: Vec<String>,
        #[yaserde(rename = "roles", default)]
        pub roles: Option<Roles>,
        #[yaserde(rename = "localizedNames", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(rename = "ownedContactLists", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(rename = "ownedContacts", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(rename = "presenceUserDefault", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlSecurityIdentity")]
    pub struct XmlSecurityIdentity {
        #[yaserde(rename = "identity", default)]
        pub identity: String,
        #[yaserde(rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(rename = "type", default)]
        pub rs_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresInfoTypeAccessType")]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(rename = "access", default)]
        pub access: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresACRuleType")]
    pub struct XmlPresACRuleType {
        #[yaserde(rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresUserDefaultType")]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresUserCLDefaultType")]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresUserACLEntryType")]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "watcherLoginName", default)]
        pub watcher_login_name: Option<String>,
        #[yaserde(rename = "watcherDisplayName", default)]
        pub watcher_display_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlPresInfoTypeType")]
    pub struct XmlPresInfoTypeType {
        #[yaserde(rename = "label", default)]
        pub label: String,
        #[yaserde(rename = "filter", default)]
        pub filter: String,
        #[yaserde(rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlContactList")]
    pub struct XmlContactList {
        #[yaserde(rename = "name", default)]
        pub name: String,
        #[yaserde(rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(rename = "contactListType", default)]
        pub contact_list_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlContactListMember")]
    pub struct XmlContactListMember {
        #[yaserde(rename = "memberContact", default)]
        pub member_contact: Option<String>,
        #[yaserde(rename = "speedDialContactAddress", default)]
        pub speed_dial_contact_address: Option<XmlContactAddress>,
        #[yaserde(rename = "memberUser", default)]
        pub member_user: Option<String>,
        #[yaserde(rename = "speedDialHandle", default)]
        pub speed_dial_handle: Option<XmlHandle>,
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
    #[yaserde(rename = "xmlContactAddress")]
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
    #[yaserde(rename = "xmlAddress")]
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
    #[yaserde(rename = "xmlContact")]
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
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlHandle")]
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
    #[yaserde(rename = "xmlCommProfileType")]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "handleList")]
    pub struct HandleList {
        #[yaserde(rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "commProfileList")]
    pub struct CommProfileList {
        #[yaserde(rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlCommProfileSetType")]
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
    #[yaserde(rename = "ForgeinCommProfileType")]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten, default)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<i64>,
        #[yaserde(rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(rename = "serviceData", default)]
        pub service_data: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlSecureStore")]
    pub struct XmlSecureStore {
        #[yaserde(rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmlLocalizedName")]
    pub struct XmlLocalizedName {
        #[yaserde(rename = "locale", default)]
        pub locale: String,
        #[yaserde(rename = "name", default)]
        pub name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "xmLocalizedNames")]
    pub struct XmLocalizedNames {
        #[yaserde(rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "tenant")]
    pub struct Tenant {
        #[yaserde(rename = "name", attribute)]
        pub name: String,
        #[yaserde(rename = "createTenantIfNotAlreadyPresent", attribute)]
        pub create_tenant_if_not_already_present: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UserOrganizationDetailsType")]
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
        rename = "xmlStationProfile",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlStationProfile {
        #[yaserde(rename = "cmName", prefix = "csm", default)]
        pub cm_name: String,
        #[yaserde(rename = "prefHandleId", prefix = "csm", default)]
        pub pref_handle_id: Option<String>,
        #[yaserde(rename = "useExistingExtension", prefix = "csm", default)]
        pub use_existing_extension: Option<bool>,
        #[yaserde(rename = "extensionRange", prefix = "csm", default)]
        pub extension_range: Option<String>,
        #[yaserde(rename = "extension", prefix = "csm", default)]
        pub extension: String,
        #[yaserde(rename = "template", prefix = "csm", default)]
        pub template: Option<String>,
        #[yaserde(rename = "setType", prefix = "csm", default)]
        pub set_type: Option<String>,
        #[yaserde(rename = "securityCode", prefix = "csm", default)]
        pub security_code: Option<String>,
        #[yaserde(rename = "port", prefix = "csm", default)]
        pub port: Option<String>,
        #[yaserde(rename = "deleteOnUnassign", prefix = "csm", default)]
        pub delete_on_unassign: Option<bool>,
        #[yaserde(rename = "overRideEndpointName", prefix = "csm", default)]
        pub over_ride_endpoint_name: Option<bool>,
        #[yaserde(rename = "dualRegistration", prefix = "csm", default)]
        pub dual_registration: Option<bool>,
        #[yaserde(rename = "enhCallrInfodisplay", prefix = "csm", default)]
        pub enh_callr_infodisplay: Option<bool>,
        #[yaserde(rename = "lockMessages", prefix = "csm", default)]
        pub lock_messages: Option<bool>,
        #[yaserde(rename = "coveragePath1", prefix = "csm", default)]
        pub coverage_path_1: Option<String>,
        #[yaserde(rename = "coveragePath2", prefix = "csm", default)]
        pub coverage_path_2: Option<String>,
        #[yaserde(rename = "huntToStation", prefix = "csm", default)]
        pub hunt_to_station: Option<String>,
        #[yaserde(rename = "tn", prefix = "csm", default)]
        pub tn: Option<i32>,
        #[yaserde(rename = "cor", prefix = "csm", default)]
        pub cor: Option<i32>,
        #[yaserde(rename = "cos", prefix = "csm", default)]
        pub cos: Option<i32>,
        #[yaserde(rename = "xmobileType", prefix = "csm", default)]
        pub xmobile_type: Option<String>,
        #[yaserde(rename = "mappingMode", prefix = "csm", default)]
        pub mapping_mode: Option<String>,
        #[yaserde(rename = "configurationSet", prefix = "csm", default)]
        pub configuration_set: Option<String>,
        #[yaserde(rename = "mobilityTrunkGroup", prefix = "csm", default)]
        pub mobility_trunk_group: Option<String>,
        #[yaserde(rename = "dialPrefix", prefix = "csm", default)]
        pub dial_prefix: Option<String>,
        #[yaserde(rename = "cellPhoneNumber", prefix = "csm", default)]
        pub cell_phone_number: Option<String>,
        #[yaserde(rename = "musicSource", prefix = "csm", default)]
        pub music_source: Option<i32>,
        #[yaserde(rename = "tests", prefix = "csm", default)]
        pub tests: Option<bool>,
        #[yaserde(rename = "dataModule", prefix = "csm", default)]
        pub data_module: Option<bool>,
        #[yaserde(rename = "speakerphone", prefix = "csm", default)]
        pub speakerphone: Option<String>,
        #[yaserde(rename = "displayLanguage", prefix = "csm", default)]
        pub display_language: Option<String>,
        #[yaserde(rename = "personalizedRingingPattern", prefix = "csm", default)]
        pub personalized_ringing_pattern: Option<i32>,
        #[yaserde(rename = "messageLampExt", prefix = "csm", default)]
        pub message_lamp_ext: Option<String>,
        #[yaserde(rename = "muteButtonEnabled", prefix = "csm", default)]
        pub mute_button_enabled: Option<bool>,
        #[yaserde(rename = "mediaComplexExt", prefix = "csm", default)]
        pub media_complex_ext: Option<String>,
        #[yaserde(rename = "ipSoftphone", prefix = "csm", default)]
        pub ip_softphone: Option<bool>,
        #[yaserde(rename = "survivableGkNodeName", prefix = "csm", default)]
        pub survivable_gk_node_name: Option<String>,
        #[yaserde(rename = "survivableCOR", prefix = "csm", default)]
        pub survivable_cor: Option<String>,
        #[yaserde(rename = "survivableTrunkDest", prefix = "csm", default)]
        pub survivable_trunk_dest: Option<bool>,
        #[yaserde(rename = "voiceMailNumber", prefix = "csm", default)]
        pub voice_mail_number: Option<String>,
        #[yaserde(rename = "offPremisesStation", prefix = "csm", default)]
        pub off_premises_station: Option<bool>,
        #[yaserde(rename = "dataOption", prefix = "csm", default)]
        pub data_option: Option<String>,
        #[yaserde(rename = "displayModule", prefix = "csm", default)]
        pub display_module: Option<bool>,
        #[yaserde(rename = "messageWaitingIndicator", prefix = "csm", default)]
        pub message_waiting_indicator: Option<String>,
        #[yaserde(rename = "remoteOfficePhone", prefix = "csm", default)]
        pub remote_office_phone: Option<bool>,
        #[yaserde(rename = "lwcReception", prefix = "csm", default)]
        pub lwc_reception: Option<String>,
        #[yaserde(rename = "lwcActivation", prefix = "csm", default)]
        pub lwc_activation: Option<bool>,
        #[yaserde(rename = "lwcLogExternalCalls", prefix = "csm", default)]
        pub lwc_log_external_calls: Option<bool>,
        #[yaserde(rename = "cdrPrivacy", prefix = "csm", default)]
        pub cdr_privacy: Option<bool>,
        #[yaserde(rename = "redirectNotification", prefix = "csm", default)]
        pub redirect_notification: Option<bool>,
        #[yaserde(rename = "perButtonRingControl", prefix = "csm", default)]
        pub per_button_ring_control: Option<bool>,
        #[yaserde(rename = "bridgedCallAlerting", prefix = "csm", default)]
        pub bridged_call_alerting: Option<bool>,
        #[yaserde(rename = "bridgedIdleLinePreference", prefix = "csm", default)]
        pub bridged_idle_line_preference: Option<bool>,
        #[yaserde(rename = "confTransOnPrimaryAppearance", prefix = "csm", default)]
        pub conf_trans_on_primary_appearance: Option<bool>,
        #[yaserde(rename = "customizableLabels", prefix = "csm", default)]
        pub customizable_labels: Option<bool>,
        #[yaserde(rename = "expansionModule", prefix = "csm", default)]
        pub expansion_module: Option<bool>,
        #[yaserde(rename = "ipVideoSoftphone", prefix = "csm", default)]
        pub ip_video_softphone: Option<bool>,
        #[yaserde(rename = "activeStationRinging", prefix = "csm", default)]
        pub active_station_ringing: Option<String>,
        #[yaserde(rename = "idleActiveRinging", prefix = "csm", default)]
        pub idle_active_ringing: Option<String>,
        #[yaserde(rename = "switchhookFlash", prefix = "csm", default)]
        pub switchhook_flash: Option<bool>,
        #[yaserde(rename = "ignoreRotaryDigits", prefix = "csm", default)]
        pub ignore_rotary_digits: Option<bool>,
        #[yaserde(rename = "h320Conversion", prefix = "csm", default)]
        pub h_320_conversion: Option<bool>,
        #[yaserde(rename = "serviceLinkMode", prefix = "csm", default)]
        pub service_link_mode: Option<String>,
        #[yaserde(rename = "multimediaMode", prefix = "csm", default)]
        pub multimedia_mode: Option<String>,
        #[yaserde(rename = "mwiServedUserType", prefix = "csm", default)]
        pub mwi_served_user_type: Option<String>,
        #[yaserde(rename = "audixName", prefix = "csm", default)]
        pub audix_name: Option<String>,
        #[yaserde(rename = "automaticMoves", prefix = "csm", default)]
        pub automatic_moves: Option<String>,
        #[yaserde(rename = "remoteSoftphoneEmergencyCalls", prefix = "csm", default)]
        pub remote_softphone_emergency_calls: Option<String>,
        #[yaserde(rename = "emergencyLocationExt", prefix = "csm", default)]
        pub emergency_location_ext: Option<String>,
        #[yaserde(rename = "alwaysUse", prefix = "csm", default)]
        pub always_use: Option<bool>,
        #[yaserde(rename = "precedenceCallWaiting", prefix = "csm", default)]
        pub precedence_call_waiting: Option<bool>,
        #[yaserde(rename = "autoSelectAnyIdleAppearance", prefix = "csm", default)]
        pub auto_select_any_idle_appearance: Option<bool>,
        #[yaserde(rename = "coverageMsgRetrieval", prefix = "csm", default)]
        pub coverage_msg_retrieval: Option<bool>,
        #[yaserde(rename = "autoAnswer", prefix = "csm", default)]
        pub auto_answer: Option<String>,
        #[yaserde(rename = "dataRestriction", prefix = "csm", default)]
        pub data_restriction: Option<bool>,
        #[yaserde(rename = "idleAppearancePreference", prefix = "csm", default)]
        pub idle_appearance_preference: Option<bool>,
        #[yaserde(rename = "callWaitingIndication", prefix = "csm", default)]
        pub call_waiting_indication: Option<bool>,
        #[yaserde(rename = "attCallWaitingIndication", prefix = "csm", default)]
        pub att_call_waiting_indication: Option<bool>,
        #[yaserde(rename = "distinctiveAudibleAlert", prefix = "csm", default)]
        pub distinctive_audible_alert: Option<bool>,
        #[yaserde(rename = "restrictLastAppearance", prefix = "csm", default)]
        pub restrict_last_appearance: Option<bool>,
        #[yaserde(rename = "adjunctSupervision", prefix = "csm", default)]
        pub adjunct_supervision: Option<bool>,
        #[yaserde(rename = "perStationCpnSendCallingNumber", prefix = "csm", default)]
        pub per_station_cpn_send_calling_number: Option<String>,
        #[yaserde(rename = "busyAutoCallbackWithoutFlash", prefix = "csm", default)]
        pub busy_auto_callback_without_flash: Option<bool>,
        #[yaserde(rename = "audibleMessageWaiting", prefix = "csm", default)]
        pub audible_message_waiting: Option<bool>,
        #[yaserde(rename = "extendedLocalCalls", prefix = "csm", default)]
        pub extended_local_calls: Option<bool>,
        #[yaserde(rename = "imsFeatureSequencing", prefix = "csm", default)]
        pub ims_feature_sequencing: Option<bool>,
        #[yaserde(rename = "displayClientRedirection", prefix = "csm", default)]
        pub display_client_redirection: Option<bool>,
        #[yaserde(rename = "selectLastUsedAppearance", prefix = "csm", default)]
        pub select_last_used_appearance: Option<bool>,
        #[yaserde(rename = "coverageAfterForwarding", prefix = "csm", default)]
        pub coverage_after_forwarding: Option<String>,
        #[yaserde(rename = "directIpIpAudioConnections", prefix = "csm", default)]
        pub direct_ip_ip_audio_connections: Option<bool>,
        #[yaserde(rename = "ipAudioHairpinning", prefix = "csm", default)]
        pub ip_audio_hairpinning: Option<bool>,
        #[yaserde(rename = "primeAppearancePreference", prefix = "csm", default)]
        pub prime_appearance_preference: Option<String>,
        #[yaserde(rename = "stationSiteData", prefix = "csm", default)]
        pub station_site_data: Option<XmlStationSiteData>,
        #[yaserde(rename = "abbrList", prefix = "csm", default)]
        pub abbr_list: Vec<XmlStationAbbreviatedDialingData>,
        #[yaserde(rename = "buttons", prefix = "csm", default)]
        pub buttons: Vec<XmlButtonData>,
        #[yaserde(rename = "featureButtons", prefix = "csm", default)]
        pub feature_buttons: Vec<XmlButtonData>,
        #[yaserde(rename = "expansionModuleButtons", prefix = "csm", default)]
        pub expansion_module_buttons: Vec<XmlButtonData>,
        #[yaserde(rename = "softKeys", prefix = "csm", default)]
        pub soft_keys: Vec<XmlButtonData>,
        #[yaserde(rename = "displayButtons", prefix = "csm", default)]
        pub display_buttons: Vec<XmlButtonData>,
        #[yaserde(rename = "stationDataModule", prefix = "csm", default)]
        pub station_data_module: Option<XmlStationDataModule>,
        #[yaserde(rename = "hotLineData", prefix = "csm", default)]
        pub hot_line_data: Option<XmlStationHotLineData>,
        #[yaserde(rename = "nativeName", prefix = "csm", default)]
        pub native_name: Option<XmlNativeNameData>,
        #[yaserde(rename = "buttonModules", prefix = "csm", default)]
        pub button_modules: Option<i32>,
        #[yaserde(rename = "unconditionalInternalDest", prefix = "csm", default)]
        pub unconditional_internal_dest: Option<String>,
        #[yaserde(rename = "unconditionalInternalActive", prefix = "csm", default)]
        pub unconditional_internal_active: Option<bool>,
        #[yaserde(rename = "unconditionalExternalDest", prefix = "csm", default)]
        pub unconditional_external_dest: Option<String>,
        #[yaserde(rename = "unconditionalExternalActive", prefix = "csm", default)]
        pub unconditional_external_active: Option<bool>,
        #[yaserde(rename = "busyInternalDest", prefix = "csm", default)]
        pub busy_internal_dest: Option<String>,
        #[yaserde(rename = "busyInternalActive", prefix = "csm", default)]
        pub busy_internal_active: Option<bool>,
        #[yaserde(rename = "busyExternalDest", prefix = "csm", default)]
        pub busy_external_dest: Option<String>,
        #[yaserde(rename = "busyExternalActive", prefix = "csm", default)]
        pub busy_external_active: Option<bool>,
        #[yaserde(rename = "noReplyInternalDest", prefix = "csm", default)]
        pub no_reply_internal_dest: Option<String>,
        #[yaserde(rename = "noReplyInternalActive", prefix = "csm", default)]
        pub no_reply_internal_active: Option<bool>,
        #[yaserde(rename = "noReplyExternalDest", prefix = "csm", default)]
        pub no_reply_external_dest: Option<String>,
        #[yaserde(rename = "noReplyExternalActive", prefix = "csm", default)]
        pub no_reply_external_active: Option<bool>,
        #[yaserde(rename = "sacCfOverride", prefix = "csm", default)]
        pub sac_cf_override: Option<String>,
        #[yaserde(rename = "lossGroup", prefix = "csm", default)]
        pub loss_group: Option<i32>,
        #[yaserde(rename = "timeOfDayLockTable", prefix = "csm", default)]
        pub time_of_day_lock_table: Option<String>,
        #[yaserde(rename = "emuLoginAllowed", prefix = "csm", default)]
        pub emu_login_allowed: Option<bool>,
        #[yaserde(rename = "ec500State", prefix = "csm", default)]
        pub ec_500_state: Option<String>,
        #[yaserde(rename = "muteOnOffHookInSCMode", prefix = "csm", default)]
        pub mute_on_off_hook_in_sc_mode: Option<bool>,
        #[yaserde(rename = "type3pccEnabled", prefix = "csm", default)]
        pub type_3pcc_enabled: Option<String>,
        #[yaserde(rename = "calculateRoutePattern", prefix = "csm", default)]
        pub calculate_route_pattern: Option<bool>,
        #[yaserde(rename = "sipTrunk", prefix = "csm", default)]
        pub sip_trunk: Option<String>,
        #[yaserde(rename = "enableReachStaDomainControl", prefix = "csm", default)]
        pub enable_reach_sta_domain_control: Option<String>,
        #[yaserde(rename = "multimediaEarlyAnswer", prefix = "csm", default)]
        pub multimedia_early_answer: Option<bool>,
        #[yaserde(rename = "bridgedApprOrigRestr", prefix = "csm", default)]
        pub bridged_appr_orig_restr: Option<bool>,
        #[yaserde(rename = "callApprDispFormat", prefix = "csm", default)]
        pub call_appr_disp_format: Option<String>,
        #[yaserde(rename = "ipPhoneGroupId", prefix = "csm", default)]
        pub ip_phone_group_id: Option<String>,
        #[yaserde(rename = "xoipEndPointType", prefix = "csm", default)]
        pub xoip_end_point_type: Option<String>,
        #[yaserde(rename = "xid", prefix = "csm", default)]
        pub xid: Option<bool>,
        #[yaserde(rename = "stepClearing", prefix = "csm", default)]
        pub step_clearing: Option<bool>,
        #[yaserde(rename = "fixedTei", prefix = "csm", default)]
        pub fixed_tei: Option<bool>,
        #[yaserde(rename = "tei", prefix = "csm", default)]
        pub tei: Option<String>,
        #[yaserde(rename = "countryProtocol", prefix = "csm", default)]
        pub country_protocol: Option<String>,
        #[yaserde(rename = "endptInit", prefix = "csm", default)]
        pub endpt_init: Option<bool>,
        #[yaserde(rename = "spid", prefix = "csm", default)]
        pub spid: Option<String>,
        #[yaserde(rename = "endptId", prefix = "csm", default)]
        pub endpt_id: Option<String>,
        #[yaserde(rename = "isMCTSignalling", prefix = "csm", default)]
        pub is_mct_signalling: Option<bool>,
        #[yaserde(rename = "isShortCallingPartyDisplay", prefix = "csm", default)]
        pub is_short_calling_party_display: Option<bool>,
        #[yaserde(rename = "passageWay", prefix = "csm", default)]
        pub passage_way: Option<bool>,
        #[yaserde(rename = "dtmfOverIp", prefix = "csm", default)]
        pub dtmf_over_ip: Option<String>,
        #[yaserde(rename = "location", prefix = "csm", default)]
        pub location: Option<String>,
        #[yaserde(rename = "displayCallerId", prefix = "csm", default)]
        pub display_caller_id: Option<bool>,
        #[yaserde(rename = "callerIdMsgWaitingIndication", prefix = "csm", default)]
        pub caller_id_msg_waiting_indication: Option<bool>,
        #[yaserde(rename = "recallRotaryDigit", prefix = "csm", default)]
        pub recall_rotary_digit: Option<bool>,
        #[yaserde(rename = "profileSettingsData", prefix = "csm", default)]
        pub profile_settings_data: Option<XmlProfileSettings>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlStationSiteData",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlStationSiteData {
        #[yaserde(rename = "room", prefix = "csm", default)]
        pub room: Option<String>,
        #[yaserde(rename = "jack", prefix = "csm", default)]
        pub jack: Option<String>,
        #[yaserde(rename = "cable", prefix = "csm", default)]
        pub cable: Option<String>,
        #[yaserde(rename = "floor", prefix = "csm", default)]
        pub floor: Option<String>,
        #[yaserde(rename = "building", prefix = "csm", default)]
        pub building: Option<String>,
        #[yaserde(rename = "headset", prefix = "csm", default)]
        pub headset: Option<bool>,
        #[yaserde(rename = "speaker", prefix = "csm", default)]
        pub speaker: Option<bool>,
        #[yaserde(rename = "mounting", prefix = "csm", default)]
        pub mounting: Option<String>,
        #[yaserde(rename = "cordLength", prefix = "csm", default)]
        pub cord_length: Option<i32>,
        #[yaserde(rename = "setColor", prefix = "csm", default)]
        pub set_color: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlStationAbbreviatedDialingData",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlStationAbbreviatedDialingData {
        #[yaserde(rename = "listType", prefix = "csm", default)]
        pub list_type: String,
        #[yaserde(rename = "number", prefix = "csm", default)]
        pub number: i32,
        #[yaserde(rename = "listId", prefix = "csm", default)]
        pub list_id: Option<i32>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlButtonData",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlButtonData {
        #[yaserde(rename = "number", prefix = "csm", default)]
        pub number: i32,
        #[yaserde(rename = "type", prefix = "csm", default)]
        pub rs_type: String,
        #[yaserde(rename = "data1", prefix = "csm", default)]
        pub data_1: Option<String>,
        #[yaserde(rename = "data2", prefix = "csm", default)]
        pub data_2: Option<String>,
        #[yaserde(rename = "data3", prefix = "csm", default)]
        pub data_3: Option<String>,
        #[yaserde(rename = "data4", prefix = "csm", default)]
        pub data_4: Option<String>,
        #[yaserde(rename = "data5", prefix = "csm", default)]
        pub data_5: Option<String>,
        #[yaserde(rename = "data6", prefix = "csm", default)]
        pub data_6: Option<String>,
        #[yaserde(rename = "isFavorite", prefix = "csm", default)]
        pub is_favorite: Option<bool>,
        #[yaserde(rename = "buttonLabel", prefix = "csm", default)]
        pub button_label: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlStationDataModule",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlStationDataModule {
        #[yaserde(rename = "dataExtension", prefix = "csm", default)]
        pub data_extension: String,
        #[yaserde(rename = "name", prefix = "csm", default)]
        pub name: Option<String>,
        #[yaserde(rename = "cor", prefix = "csm", default)]
        pub cor: i32,
        #[yaserde(rename = "cos", prefix = "csm", default)]
        pub cos: i32,
        #[yaserde(rename = "itc", prefix = "csm", default)]
        pub itc: String,
        #[yaserde(rename = "tn", prefix = "csm", default)]
        pub tn: i32,
        #[yaserde(rename = "listType", prefix = "csm", default)]
        pub list_type: Option<String>,
        #[yaserde(rename = "listId", prefix = "csm", default)]
        pub list_id: Option<i32>,
        #[yaserde(rename = "specialDialingOption", prefix = "csm", default)]
        pub special_dialing_option: Option<String>,
        #[yaserde(rename = "specialDialingAbbrDialCode", prefix = "csm", default)]
        pub special_dialing_abbr_dial_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlStationHotLineData",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlStationHotLineData {
        #[yaserde(rename = "hotLineDestAbbrevList", prefix = "csm", default)]
        pub hot_line_dest_abbrev_list: Option<i32>,
        #[yaserde(rename = "hotLineAbbrevDialCode", prefix = "csm", default)]
        pub hot_line_abbrev_dial_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlNativeNameData",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlNativeNameData {
        #[yaserde(rename = "locale", prefix = "csm", default)]
        pub locale: Option<String>,
        #[yaserde(rename = "name", prefix = "csm", default)]
        pub name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "xmlProfileSettings",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        prefix = "csm"
    )]
    pub struct XmlProfileSettings {
        #[yaserde(rename = "phoneScreenCalling", prefix = "csm", default)]
        pub phone_screen_calling: Option<String>,
        #[yaserde(rename = "profileRedial", prefix = "csm", default)]
        pub profile_redial: Option<String>,
        #[yaserde(rename = "dialingOption", prefix = "csm", default)]
        pub dialing_option: Option<String>,
        #[yaserde(rename = "headsetSignaling", prefix = "csm", default)]
        pub headset_signaling: Option<String>,
        #[yaserde(rename = "audioPath", prefix = "csm", default)]
        pub audio_path: Option<String>,
        #[yaserde(rename = "buttonClicks", prefix = "csm", default)]
        pub button_clicks: Option<String>,
        #[yaserde(rename = "phoneScreen", prefix = "csm", default)]
        pub phone_screen: Option<String>,
        #[yaserde(rename = "backgroundLogo", prefix = "csm", default)]
        pub background_logo: Option<String>,
        #[yaserde(rename = "personalizedRinging", prefix = "csm", default)]
        pub personalized_ringing: Option<String>,
        #[yaserde(rename = "callPickUpIndication", prefix = "csm", default)]
        pub call_pick_up_indication: Option<String>,
        #[yaserde(rename = "touchPanel", prefix = "csm", default)]
        pub touch_panel: Option<String>,
        #[yaserde(rename = "language", prefix = "csm", default)]
        pub language: Option<String>,
        #[yaserde(rename = "userPreferredLanguage", prefix = "csm", default)]
        pub user_preferred_language: Option<String>,
        #[yaserde(rename = "languageFileInUse", prefix = "csm", default)]
        pub language_file_in_use: Option<String>,
        #[yaserde(rename = "timeFormat", prefix = "csm", default)]
        pub time_format: Option<String>,
        #[yaserde(rename = "awayTimer", prefix = "csm", default)]
        pub away_timer: Option<String>,
        #[yaserde(rename = "awayTimerValue", prefix = "csm", default)]
        pub away_timer_value: Option<i32>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}
