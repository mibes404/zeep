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
pub mod services {
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

pub mod ports {
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
        prefix = "nsi1",
        root = "users",
        default,
        namespace = "http://xml.avaya.com/schema/import",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct Users {
        #[yaserde(rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UserProvisionRules", default)]
    pub struct UserProvisionRules {
        #[yaserde(rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "roles", default)]
    pub struct Roles {
        #[yaserde(rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "ownedContactLists", default)]
    pub struct OwnedContactLists {
        #[yaserde(rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "ownedContacts", default)]
    pub struct OwnedContacts {
        #[yaserde(rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlUser", default)]
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
    #[yaserde(root = "xmlSecurityIdentity", default)]
    pub struct XmlSecurityIdentity {
        #[yaserde(rename = "identity", default)]
        pub identity: String,
        #[yaserde(rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(rename = "type", default)]
        pub rs_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresInfoTypeAccessType", default)]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(rename = "access", default)]
        pub access: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresACRuleType", default)]
    pub struct XmlPresACRuleType {
        #[yaserde(rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserDefaultType", default)]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlPresACRuleType
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserCLDefaultType", default)]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlPresACRuleType
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserACLEntryType", default)]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlPresACRuleType
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresInfoTypeType", default)]
    pub struct XmlPresInfoTypeType {
        #[yaserde(rename = "label", default)]
        pub label: String,
        #[yaserde(rename = "filter", default)]
        pub filter: String,
        #[yaserde(rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlContactList", default)]
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
    #[yaserde(root = "xmlContactListMember", default)]
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
    #[yaserde(root = "xmlContactAddress", default)]
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
    #[yaserde(root = "xmlAddress", default)]
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
    #[yaserde(root = "xmlContact", default)]
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
    #[yaserde(root = "xmlHandle", default)]
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
    #[yaserde(root = "xmlCommProfileType", default)]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "handleList", default)]
    pub struct HandleList {
        #[yaserde(rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "commProfileList", default)]
    pub struct CommProfileList {
        #[yaserde(rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlCommProfileSetType", default)]
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
    #[yaserde(root = "ForgeinCommProfileType", default)]
    pub struct ForgeinCommProfileType {
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String, // XmlCommProfileType
        #[yaserde(rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<i64>,
        #[yaserde(rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(rename = "serviceData", default)]
        pub service_data: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlSecureStore", default)]
    pub struct XmlSecureStore {
        #[yaserde(rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlLocalizedName", default)]
    pub struct XmlLocalizedName {
        #[yaserde(rename = "locale", default)]
        pub locale: String,
        #[yaserde(rename = "name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmLocalizedNames", default)]
    pub struct XmLocalizedNames {
        #[yaserde(rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "tenant", default)]
    pub struct Tenant {
        #[yaserde(rename = "name", attribute)]
        pub name: String,
        #[yaserde(rename = "createTenantIfNotAlreadyPresent", attribute)]
        pub create_tenant_if_not_already_present: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UserOrganizationDetailsType", default)]
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
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlStationProfile",
        default
    )]
    pub struct XmlStationProfile {
        #[yaserde(prefix = "ns2", rename = "cmName", default)]
        pub cm_name: String,
        #[yaserde(prefix = "ns2", rename = "prefHandleId", default)]
        pub pref_handle_id: Option<String>,
        #[yaserde(prefix = "ns2", rename = "useExistingExtension", default)]
        pub use_existing_extension: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "extensionRange", default)]
        pub extension_range: Option<String>,
        #[yaserde(prefix = "ns2", rename = "extension", default)]
        pub extension: String,
        #[yaserde(prefix = "ns2", rename = "template", default)]
        pub template: Option<String>,
        #[yaserde(prefix = "ns2", rename = "setType", default)]
        pub set_type: Option<String>,
        #[yaserde(prefix = "ns2", rename = "securityCode", default)]
        pub security_code: Option<String>,
        #[yaserde(prefix = "ns2", rename = "port", default)]
        pub port: Option<String>,
        #[yaserde(prefix = "ns2", rename = "deleteOnUnassign", default)]
        pub delete_on_unassign: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "overRideEndpointName", default)]
        pub over_ride_endpoint_name: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "dualRegistration", default)]
        pub dual_registration: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "enhCallrInfodisplay", default)]
        pub enh_callr_infodisplay: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "lockMessages", default)]
        pub lock_messages: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "coveragePath1", default)]
        pub coverage_path_1: Option<String>,
        #[yaserde(prefix = "ns2", rename = "coveragePath2", default)]
        pub coverage_path_2: Option<String>,
        #[yaserde(prefix = "ns2", rename = "huntToStation", default)]
        pub hunt_to_station: Option<String>,
        #[yaserde(prefix = "ns2", rename = "tn", default)]
        pub tn: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "cor", default)]
        pub cor: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "cos", default)]
        pub cos: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "xmobileType", default)]
        pub xmobile_type: Option<String>,
        #[yaserde(prefix = "ns2", rename = "mappingMode", default)]
        pub mapping_mode: Option<String>,
        #[yaserde(prefix = "ns2", rename = "configurationSet", default)]
        pub configuration_set: Option<String>,
        #[yaserde(prefix = "ns2", rename = "mobilityTrunkGroup", default)]
        pub mobility_trunk_group: Option<String>,
        #[yaserde(prefix = "ns2", rename = "dialPrefix", default)]
        pub dial_prefix: Option<String>,
        #[yaserde(prefix = "ns2", rename = "cellPhoneNumber", default)]
        pub cell_phone_number: Option<String>,
        #[yaserde(prefix = "ns2", rename = "musicSource", default)]
        pub music_source: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "tests", default)]
        pub tests: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "dataModule", default)]
        pub data_module: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "speakerphone", default)]
        pub speakerphone: Option<String>,
        #[yaserde(prefix = "ns2", rename = "displayLanguage", default)]
        pub display_language: Option<String>,
        #[yaserde(prefix = "ns2", rename = "personalizedRingingPattern", default)]
        pub personalized_ringing_pattern: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "messageLampExt", default)]
        pub message_lamp_ext: Option<String>,
        #[yaserde(prefix = "ns2", rename = "muteButtonEnabled", default)]
        pub mute_button_enabled: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "mediaComplexExt", default)]
        pub media_complex_ext: Option<String>,
        #[yaserde(prefix = "ns2", rename = "ipSoftphone", default)]
        pub ip_softphone: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "survivableGkNodeName", default)]
        pub survivable_gk_node_name: Option<String>,
        #[yaserde(prefix = "ns2", rename = "survivableCOR", default)]
        pub survivable_cor: Option<String>,
        #[yaserde(prefix = "ns2", rename = "survivableTrunkDest", default)]
        pub survivable_trunk_dest: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "voiceMailNumber", default)]
        pub voice_mail_number: Option<String>,
        #[yaserde(prefix = "ns2", rename = "offPremisesStation", default)]
        pub off_premises_station: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "dataOption", default)]
        pub data_option: Option<String>,
        #[yaserde(prefix = "ns2", rename = "displayModule", default)]
        pub display_module: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "messageWaitingIndicator", default)]
        pub message_waiting_indicator: Option<String>,
        #[yaserde(prefix = "ns2", rename = "remoteOfficePhone", default)]
        pub remote_office_phone: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "lwcReception", default)]
        pub lwc_reception: Option<String>,
        #[yaserde(prefix = "ns2", rename = "lwcActivation", default)]
        pub lwc_activation: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "lwcLogExternalCalls", default)]
        pub lwc_log_external_calls: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "cdrPrivacy", default)]
        pub cdr_privacy: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "redirectNotification", default)]
        pub redirect_notification: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "perButtonRingControl", default)]
        pub per_button_ring_control: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "bridgedCallAlerting", default)]
        pub bridged_call_alerting: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "bridgedIdleLinePreference", default)]
        pub bridged_idle_line_preference: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "confTransOnPrimaryAppearance", default)]
        pub conf_trans_on_primary_appearance: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "customizableLabels", default)]
        pub customizable_labels: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "expansionModule", default)]
        pub expansion_module: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "ipVideoSoftphone", default)]
        pub ip_video_softphone: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "activeStationRinging", default)]
        pub active_station_ringing: Option<String>,
        #[yaserde(prefix = "ns2", rename = "idleActiveRinging", default)]
        pub idle_active_ringing: Option<String>,
        #[yaserde(prefix = "ns2", rename = "switchhookFlash", default)]
        pub switchhook_flash: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "ignoreRotaryDigits", default)]
        pub ignore_rotary_digits: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "h320Conversion", default)]
        pub h_320_conversion: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "serviceLinkMode", default)]
        pub service_link_mode: Option<String>,
        #[yaserde(prefix = "ns2", rename = "multimediaMode", default)]
        pub multimedia_mode: Option<String>,
        #[yaserde(prefix = "ns2", rename = "mwiServedUserType", default)]
        pub mwi_served_user_type: Option<String>,
        #[yaserde(prefix = "ns2", rename = "audixName", default)]
        pub audix_name: Option<String>,
        #[yaserde(prefix = "ns2", rename = "automaticMoves", default)]
        pub automatic_moves: Option<String>,
        #[yaserde(prefix = "ns2", rename = "remoteSoftphoneEmergencyCalls", default)]
        pub remote_softphone_emergency_calls: Option<String>,
        #[yaserde(prefix = "ns2", rename = "emergencyLocationExt", default)]
        pub emergency_location_ext: Option<String>,
        #[yaserde(prefix = "ns2", rename = "alwaysUse", default)]
        pub always_use: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "precedenceCallWaiting", default)]
        pub precedence_call_waiting: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "autoSelectAnyIdleAppearance", default)]
        pub auto_select_any_idle_appearance: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "coverageMsgRetrieval", default)]
        pub coverage_msg_retrieval: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "autoAnswer", default)]
        pub auto_answer: Option<String>,
        #[yaserde(prefix = "ns2", rename = "dataRestriction", default)]
        pub data_restriction: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "idleAppearancePreference", default)]
        pub idle_appearance_preference: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "callWaitingIndication", default)]
        pub call_waiting_indication: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "attCallWaitingIndication", default)]
        pub att_call_waiting_indication: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "distinctiveAudibleAlert", default)]
        pub distinctive_audible_alert: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "restrictLastAppearance", default)]
        pub restrict_last_appearance: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "adjunctSupervision", default)]
        pub adjunct_supervision: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "perStationCpnSendCallingNumber", default)]
        pub per_station_cpn_send_calling_number: Option<String>,
        #[yaserde(prefix = "ns2", rename = "busyAutoCallbackWithoutFlash", default)]
        pub busy_auto_callback_without_flash: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "audibleMessageWaiting", default)]
        pub audible_message_waiting: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "extendedLocalCalls", default)]
        pub extended_local_calls: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "imsFeatureSequencing", default)]
        pub ims_feature_sequencing: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "displayClientRedirection", default)]
        pub display_client_redirection: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "selectLastUsedAppearance", default)]
        pub select_last_used_appearance: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "coverageAfterForwarding", default)]
        pub coverage_after_forwarding: Option<String>,
        #[yaserde(prefix = "ns2", rename = "directIpIpAudioConnections", default)]
        pub direct_ip_ip_audio_connections: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "ipAudioHairpinning", default)]
        pub ip_audio_hairpinning: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "primeAppearancePreference", default)]
        pub prime_appearance_preference: Option<String>,
        #[yaserde(prefix = "ns2", rename = "stationSiteData", default)]
        pub station_site_data: Option<XmlStationSiteData>,
        #[yaserde(prefix = "ns2", rename = "abbrList", default)]
        pub abbr_list: Vec<XmlStationAbbreviatedDialingData>,
        #[yaserde(prefix = "ns2", rename = "buttons", default)]
        pub buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "ns2", rename = "featureButtons", default)]
        pub feature_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "ns2", rename = "expansionModuleButtons", default)]
        pub expansion_module_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "ns2", rename = "softKeys", default)]
        pub soft_keys: Vec<XmlButtonData>,
        #[yaserde(prefix = "ns2", rename = "displayButtons", default)]
        pub display_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "ns2", rename = "stationDataModule", default)]
        pub station_data_module: Option<XmlStationDataModule>,
        #[yaserde(prefix = "ns2", rename = "hotLineData", default)]
        pub hot_line_data: Option<XmlStationHotLineData>,
        #[yaserde(prefix = "ns2", rename = "nativeName", default)]
        pub native_name: Option<XmlNativeNameData>,
        #[yaserde(prefix = "ns2", rename = "buttonModules", default)]
        pub button_modules: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "unconditionalInternalDest", default)]
        pub unconditional_internal_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "unconditionalInternalActive", default)]
        pub unconditional_internal_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "unconditionalExternalDest", default)]
        pub unconditional_external_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "unconditionalExternalActive", default)]
        pub unconditional_external_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "busyInternalDest", default)]
        pub busy_internal_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "busyInternalActive", default)]
        pub busy_internal_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "busyExternalDest", default)]
        pub busy_external_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "busyExternalActive", default)]
        pub busy_external_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "noReplyInternalDest", default)]
        pub no_reply_internal_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "noReplyInternalActive", default)]
        pub no_reply_internal_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "noReplyExternalDest", default)]
        pub no_reply_external_dest: Option<String>,
        #[yaserde(prefix = "ns2", rename = "noReplyExternalActive", default)]
        pub no_reply_external_active: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "sacCfOverride", default)]
        pub sac_cf_override: Option<String>,
        #[yaserde(prefix = "ns2", rename = "lossGroup", default)]
        pub loss_group: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "timeOfDayLockTable", default)]
        pub time_of_day_lock_table: Option<String>,
        #[yaserde(prefix = "ns2", rename = "emuLoginAllowed", default)]
        pub emu_login_allowed: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "ec500State", default)]
        pub ec_500_state: Option<String>,
        #[yaserde(prefix = "ns2", rename = "muteOnOffHookInSCMode", default)]
        pub mute_on_off_hook_in_sc_mode: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "type3pccEnabled", default)]
        pub type_3pcc_enabled: Option<String>,
        #[yaserde(prefix = "ns2", rename = "calculateRoutePattern", default)]
        pub calculate_route_pattern: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "sipTrunk", default)]
        pub sip_trunk: Option<String>,
        #[yaserde(prefix = "ns2", rename = "enableReachStaDomainControl", default)]
        pub enable_reach_sta_domain_control: Option<String>,
        #[yaserde(prefix = "ns2", rename = "multimediaEarlyAnswer", default)]
        pub multimedia_early_answer: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "bridgedApprOrigRestr", default)]
        pub bridged_appr_orig_restr: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "callApprDispFormat", default)]
        pub call_appr_disp_format: Option<String>,
        #[yaserde(prefix = "ns2", rename = "ipPhoneGroupId", default)]
        pub ip_phone_group_id: Option<String>,
        #[yaserde(prefix = "ns2", rename = "xoipEndPointType", default)]
        pub xoip_end_point_type: Option<String>,
        #[yaserde(prefix = "ns2", rename = "xid", default)]
        pub xid: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "stepClearing", default)]
        pub step_clearing: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "fixedTei", default)]
        pub fixed_tei: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "tei", default)]
        pub tei: Option<String>,
        #[yaserde(prefix = "ns2", rename = "countryProtocol", default)]
        pub country_protocol: Option<String>,
        #[yaserde(prefix = "ns2", rename = "endptInit", default)]
        pub endpt_init: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "spid", default)]
        pub spid: Option<String>,
        #[yaserde(prefix = "ns2", rename = "endptId", default)]
        pub endpt_id: Option<String>,
        #[yaserde(prefix = "ns2", rename = "isMCTSignalling", default)]
        pub is_mct_signalling: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "isShortCallingPartyDisplay", default)]
        pub is_short_calling_party_display: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "passageWay", default)]
        pub passage_way: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "dtmfOverIp", default)]
        pub dtmf_over_ip: Option<String>,
        #[yaserde(prefix = "ns2", rename = "location", default)]
        pub location: Option<String>,
        #[yaserde(prefix = "ns2", rename = "displayCallerId", default)]
        pub display_caller_id: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "callerIdMsgWaitingIndication", default)]
        pub caller_id_msg_waiting_indication: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "recallRotaryDigit", default)]
        pub recall_rotary_digit: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "profileSettingsData", default)]
        pub profile_settings_data: Option<XmlProfileSettings>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlStationSiteData",
        default
    )]
    pub struct XmlStationSiteData {
        #[yaserde(prefix = "ns2", rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(prefix = "ns2", rename = "jack", default)]
        pub jack: Option<String>,
        #[yaserde(prefix = "ns2", rename = "cable", default)]
        pub cable: Option<String>,
        #[yaserde(prefix = "ns2", rename = "floor", default)]
        pub floor: Option<String>,
        #[yaserde(prefix = "ns2", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "ns2", rename = "headset", default)]
        pub headset: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "speaker", default)]
        pub speaker: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "mounting", default)]
        pub mounting: Option<String>,
        #[yaserde(prefix = "ns2", rename = "cordLength", default)]
        pub cord_length: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "setColor", default)]
        pub set_color: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlStationAbbreviatedDialingData",
        default
    )]
    pub struct XmlStationAbbreviatedDialingData {
        #[yaserde(prefix = "ns2", rename = "listType", default)]
        pub list_type: String,
        #[yaserde(prefix = "ns2", rename = "number", default)]
        pub number: i32,
        #[yaserde(prefix = "ns2", rename = "listId", default)]
        pub list_id: Option<i32>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlButtonData",
        default
    )]
    pub struct XmlButtonData {
        #[yaserde(prefix = "ns2", rename = "number", default)]
        pub number: i32,
        #[yaserde(prefix = "ns2", rename = "type", default)]
        pub rs_type: String,
        #[yaserde(prefix = "ns2", rename = "data1", default)]
        pub data_1: Option<String>,
        #[yaserde(prefix = "ns2", rename = "data2", default)]
        pub data_2: Option<String>,
        #[yaserde(prefix = "ns2", rename = "data3", default)]
        pub data_3: Option<String>,
        #[yaserde(prefix = "ns2", rename = "data4", default)]
        pub data_4: Option<String>,
        #[yaserde(prefix = "ns2", rename = "data5", default)]
        pub data_5: Option<String>,
        #[yaserde(prefix = "ns2", rename = "data6", default)]
        pub data_6: Option<String>,
        #[yaserde(prefix = "ns2", rename = "isFavorite", default)]
        pub is_favorite: Option<bool>,
        #[yaserde(prefix = "ns2", rename = "buttonLabel", default)]
        pub button_label: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlStationDataModule",
        default
    )]
    pub struct XmlStationDataModule {
        #[yaserde(prefix = "ns2", rename = "dataExtension", default)]
        pub data_extension: String,
        #[yaserde(prefix = "ns2", rename = "name", default)]
        pub name: Option<String>,
        #[yaserde(prefix = "ns2", rename = "cor", default)]
        pub cor: i32,
        #[yaserde(prefix = "ns2", rename = "cos", default)]
        pub cos: i32,
        #[yaserde(prefix = "ns2", rename = "itc", default)]
        pub itc: String,
        #[yaserde(prefix = "ns2", rename = "tn", default)]
        pub tn: i32,
        #[yaserde(prefix = "ns2", rename = "listType", default)]
        pub list_type: Option<String>,
        #[yaserde(prefix = "ns2", rename = "listId", default)]
        pub list_id: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "specialDialingOption", default)]
        pub special_dialing_option: Option<String>,
        #[yaserde(prefix = "ns2", rename = "specialDialingAbbrDialCode", default)]
        pub special_dialing_abbr_dial_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlStationHotLineData",
        default
    )]
    pub struct XmlStationHotLineData {
        #[yaserde(prefix = "ns2", rename = "hotLineDestAbbrevList", default)]
        pub hot_line_dest_abbrev_list: Option<i32>,
        #[yaserde(prefix = "ns2", rename = "hotLineAbbrevDialCode", default)]
        pub hot_line_abbrev_dial_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlNativeNameData",
        default
    )]
    pub struct XmlNativeNameData {
        #[yaserde(prefix = "ns2", rename = "locale", default)]
        pub locale: Option<String>,
        #[yaserde(prefix = "ns2", rename = "name", default)]
        pub name: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns2",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        root = "xmlProfileSettings",
        default
    )]
    pub struct XmlProfileSettings {
        #[yaserde(prefix = "ns2", rename = "phoneScreenCalling", default)]
        pub phone_screen_calling: Option<String>,
        #[yaserde(prefix = "ns2", rename = "profileRedial", default)]
        pub profile_redial: Option<String>,
        #[yaserde(prefix = "ns2", rename = "dialingOption", default)]
        pub dialing_option: Option<String>,
        #[yaserde(prefix = "ns2", rename = "headsetSignaling", default)]
        pub headset_signaling: Option<String>,
        #[yaserde(prefix = "ns2", rename = "audioPath", default)]
        pub audio_path: Option<String>,
        #[yaserde(prefix = "ns2", rename = "buttonClicks", default)]
        pub button_clicks: Option<String>,
        #[yaserde(prefix = "ns2", rename = "phoneScreen", default)]
        pub phone_screen: Option<String>,
        #[yaserde(prefix = "ns2", rename = "backgroundLogo", default)]
        pub background_logo: Option<String>,
        #[yaserde(prefix = "ns2", rename = "personalizedRinging", default)]
        pub personalized_ringing: Option<String>,
        #[yaserde(prefix = "ns2", rename = "callPickUpIndication", default)]
        pub call_pick_up_indication: Option<String>,
        #[yaserde(prefix = "ns2", rename = "touchPanel", default)]
        pub touch_panel: Option<String>,
        #[yaserde(prefix = "ns2", rename = "language", default)]
        pub language: Option<String>,
        #[yaserde(prefix = "ns2", rename = "userPreferredLanguage", default)]
        pub user_preferred_language: Option<String>,
        #[yaserde(prefix = "ns2", rename = "languageFileInUse", default)]
        pub language_file_in_use: Option<String>,
        #[yaserde(prefix = "ns2", rename = "timeFormat", default)]
        pub time_format: Option<String>,
        #[yaserde(prefix = "ns2", rename = "awayTimer", default)]
        pub away_timer: Option<String>,
        #[yaserde(prefix = "ns2", rename = "awayTimerValue", default)]
        pub away_timer_value: Option<i32>,
    }
}

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

type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}
