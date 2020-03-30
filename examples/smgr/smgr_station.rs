//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod bindings {
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

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "users",
        default
    )]
    pub struct Users {
        #[yaserde(prefix = "tns", rename = "secureStore", default)]
        pub secure_store: XmlSecureStore,
        #[yaserde(prefix = "tns", rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "UserProvisionRules",
        default
    )]
    pub struct UserProvisionRules {
        #[yaserde(prefix = "tns", rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "roles",
        default
    )]
    pub struct Roles {
        #[yaserde(prefix = "tns", rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "ownedContactLists",
        default
    )]
    pub struct OwnedContactLists {
        #[yaserde(prefix = "tns", rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "ownedContacts",
        default
    )]
    pub struct OwnedContacts {
        #[yaserde(prefix = "tns", rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlUser",
        default
    )]
    pub struct XmlUser {
        #[yaserde(prefix = "tns", rename = "UserOrganizationDetails", default)]
        pub user_organization_details: UserOrganizationDetailsType,
        #[yaserde(prefix = "tns", rename = "authenticationType", default)]
        pub authentication_type: String,
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
        pub is_enabled: bool,
        #[yaserde(prefix = "tns", rename = "isVirtualUser", default)]
        pub is_virtual_user: bool,
        #[yaserde(prefix = "tns", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
        pub given_name_ascii: String,
        #[yaserde(prefix = "tns", rename = "honorific", default)]
        pub honorific: String,
        #[yaserde(prefix = "tns", rename = "employeeNo", default)]
        pub employee_no: String,
        #[yaserde(prefix = "tns", rename = "department", default)]
        pub department: String,
        #[yaserde(prefix = "tns", rename = "organization", default)]
        pub organization: String,
        #[yaserde(prefix = "tns", rename = "middleName", default)]
        pub middle_name: String,
        #[yaserde(prefix = "tns", rename = "managerName", default)]
        pub manager_name: String,
        #[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
        pub preferred_given_name: String,
        #[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
        pub preferred_language: String,
        #[yaserde(prefix = "tns", rename = "source", default)]
        pub source: String,
        #[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(prefix = "tns", rename = "status", default)]
        pub status: String,
        #[yaserde(prefix = "tns", rename = "suffix", default)]
        pub suffix: String,
        #[yaserde(prefix = "tns", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "tns", rename = "surnameAscii", default)]
        pub surname_ascii: String,
        #[yaserde(prefix = "tns", rename = "timeZone", default)]
        pub time_zone: String,
        #[yaserde(prefix = "tns", rename = "title", default)]
        pub title: String,
        #[yaserde(prefix = "tns", rename = "userName", default)]
        pub user_name: String,
        #[yaserde(prefix = "tns", rename = "userPassword", default)]
        pub user_password: String,
        #[yaserde(prefix = "tns", rename = "commPassword", default)]
        pub comm_password: String,
        #[yaserde(prefix = "tns", rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(prefix = "tns", rename = "localizedNames", default)]
        pub localized_names: XmLocalizedNames,
        #[yaserde(prefix = "tns", rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(prefix = "tns", rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(prefix = "tns", rename = "presenceUserDefault", default)]
        pub presence_user_default: XmlPresUserDefaultType,
        #[yaserde(prefix = "tns", rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(prefix = "tns", rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: XmlPresUserCLDefaultType,
        #[yaserde(prefix = "tns", rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlSecurityIdentity",
        default
    )]
    pub struct XmlSecurityIdentity {
        #[yaserde(prefix = "tns", rename = "identity", default)]
        pub identity: String,
        #[yaserde(prefix = "tns", rename = "realm", default)]
        pub realm: String,
        #[yaserde(prefix = "tns", rename = "type", default)]
        pub rs_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresInfoTypeAccessType",
        default
    )]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(prefix = "tns", rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(prefix = "tns", rename = "access", default)]
        pub access: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresACRuleType",
        default
    )]
    pub struct XmlPresACRuleType {
        #[yaserde(prefix = "tns", rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserDefaultType",
        default
    )]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserCLDefaultType",
        default
    )]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserACLEntryType",
        default
    )]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlPresInfoTypeType",
        default
    )]
    pub struct XmlPresInfoTypeType {
        #[yaserde(prefix = "tns", rename = "label", default)]
        pub label: String,
        #[yaserde(prefix = "tns", rename = "filter", default)]
        pub filter: String,
        #[yaserde(prefix = "tns", rename = "specFlags", default)]
        pub spec_flags: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlContactList",
        default
    )]
    pub struct XmlContactList {
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: String,
        #[yaserde(prefix = "tns", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "tns", rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(prefix = "tns", rename = "contactListType", default)]
        pub contact_list_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlContactListMember",
        default
    )]
    pub struct XmlContactListMember {
        #[yaserde(prefix = "tns", rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(prefix = "tns", rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(prefix = "tns", rename = "speedDialEntry", default)]
        pub speed_dial_entry: u64,
        #[yaserde(prefix = "tns", rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: bool,
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
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlContactAddress",
        default
    )]
    pub struct XmlContactAddress {
        #[yaserde(prefix = "tns", rename = "address", default)]
        pub address: String,
        #[yaserde(prefix = "tns", rename = "altLabel", default)]
        pub alt_label: String,
        #[yaserde(prefix = "tns", rename = "contactCategory", default)]
        pub contact_category: String,
        #[yaserde(prefix = "tns", rename = "contactType", default)]
        pub contact_type: String,
        #[yaserde(prefix = "tns", rename = "label", default)]
        pub label: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlAddress",
        default
    )]
    pub struct XmlAddress {
        #[yaserde(prefix = "tns", rename = "addressType", default)]
        pub address_type: String,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
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
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlContact",
        default
    )]
    pub struct XmlContact {
        #[yaserde(prefix = "tns", rename = "company", default)]
        pub company: String,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: String,
        #[yaserde(prefix = "tns", rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(prefix = "tns", rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(prefix = "tns", rename = "dn", default)]
        pub dn: String,
        #[yaserde(prefix = "tns", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
        pub given_name_ascii: String,
        #[yaserde(prefix = "tns", rename = "initials", default)]
        pub initials: String,
        #[yaserde(prefix = "tns", rename = "middleName", default)]
        pub middle_name: String,
        #[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
        pub preferred_given_name: String,
        #[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
        pub preferred_language: String,
        #[yaserde(prefix = "tns", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "tns", rename = "source", default)]
        pub source: String,
        #[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(prefix = "tns", rename = "suffix", default)]
        pub suffix: String,
        #[yaserde(prefix = "tns", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "tns", rename = "surnameAscii", default)]
        pub surname_ascii: String,
        #[yaserde(prefix = "tns", rename = "title", default)]
        pub title: String,
        #[yaserde(prefix = "tns", rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(prefix = "tns", rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlHandle",
        default
    )]
    pub struct XmlHandle {
        #[yaserde(prefix = "tns", rename = "handleName", default)]
        pub handle_name: String,
        #[yaserde(prefix = "tns", rename = "handleType", default)]
        pub handle_type: String,
        #[yaserde(prefix = "tns", rename = "handleSubType", default)]
        pub handle_sub_type: String,
        #[yaserde(prefix = "tns", rename = "domainName", default)]
        pub domain_name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlCommProfileType",
        default
    )]
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "tns", rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(prefix = "tns", rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: String,
        #[yaserde(prefix = "tns", rename = "jobId", default)]
        pub job_id: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "handleList",
        default
    )]
    pub struct HandleList {
        #[yaserde(prefix = "tns", rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "commProfileList",
        default
    )]
    pub struct CommProfileList {
        #[yaserde(prefix = "tns", rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlCommProfileSetType",
        default
    )]
    pub struct XmlCommProfileSetType {
        #[yaserde(prefix = "tns", rename = "commProfileSetName", default)]
        pub comm_profile_set_name: String,
        #[yaserde(prefix = "tns", rename = "isPrimary", default)]
        pub is_primary: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "ForgeinCommProfileType",
        default
    )]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "tns", rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: u64,
        #[yaserde(prefix = "tns", rename = "servicePassword", default)]
        pub service_password: String,
        #[yaserde(prefix = "tns", rename = "serviceData", default)]
        pub service_data: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlSecureStore",
        default
    )]
    pub struct XmlSecureStore {
        #[yaserde(prefix = "tns", rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(prefix = "tns", rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlLocalizedName",
        default
    )]
    pub struct XmlLocalizedName {
        #[yaserde(prefix = "tns", rename = "locale", default)]
        pub locale: String,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmLocalizedNames",
        default
    )]
    pub struct XmLocalizedNames {
        #[yaserde(prefix = "tns", rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "tenant",
        default
    )]
    pub struct Tenant {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "UserOrganizationDetailsType",
        default
    )]
    pub struct UserOrganizationDetailsType {
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: String,
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: String,
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlStationProfile",
        default
    )]
    pub struct XmlStationProfile {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "tns", rename = "cmName", default)]
        pub cm_name: String,
        #[yaserde(prefix = "tns", rename = "prefHandleId", default)]
        pub pref_handle_id: String,
        #[yaserde(prefix = "tns", rename = "useExistingExtension", default)]
        pub use_existing_extension: bool,
        #[yaserde(prefix = "tns", rename = "template", default)]
        pub template: String,
        #[yaserde(prefix = "tns", rename = "setType", default)]
        pub set_type: String,
        #[yaserde(prefix = "tns", rename = "port", default)]
        pub port: String,
        #[yaserde(prefix = "tns", rename = "deleteOnUnassign", default)]
        pub delete_on_unassign: bool,
        #[yaserde(prefix = "tns", rename = "overRideEndpointName", default)]
        pub over_ride_endpoint_name: bool,
        #[yaserde(prefix = "tns", rename = "dualRegistration", default)]
        pub dual_registration: bool,
        #[yaserde(prefix = "tns", rename = "enhCallrInfodisplay", default)]
        pub enh_callr_infodisplay: bool,
        #[yaserde(prefix = "tns", rename = "lockMessages", default)]
        pub lock_messages: bool,
        #[yaserde(prefix = "tns", rename = "huntToStation", default)]
        pub hunt_to_station: String,
        #[yaserde(prefix = "tns", rename = "tests", default)]
        pub tests: bool,
        #[yaserde(prefix = "tns", rename = "dataModule", default)]
        pub data_module: bool,
        #[yaserde(prefix = "tns", rename = "muteButtonEnabled", default)]
        pub mute_button_enabled: bool,
        #[yaserde(prefix = "tns", rename = "ipSoftphone", default)]
        pub ip_softphone: bool,
        #[yaserde(prefix = "tns", rename = "survivableGkNodeName", default)]
        pub survivable_gk_node_name: String,
        #[yaserde(prefix = "tns", rename = "survivableTrunkDest", default)]
        pub survivable_trunk_dest: bool,
        #[yaserde(prefix = "tns", rename = "offPremisesStation", default)]
        pub off_premises_station: bool,
        #[yaserde(prefix = "tns", rename = "displayModule", default)]
        pub display_module: bool,
        #[yaserde(prefix = "tns", rename = "remoteOfficePhone", default)]
        pub remote_office_phone: bool,
        #[yaserde(prefix = "tns", rename = "lwcActivation", default)]
        pub lwc_activation: bool,
        #[yaserde(prefix = "tns", rename = "lwcLogExternalCalls", default)]
        pub lwc_log_external_calls: bool,
        #[yaserde(prefix = "tns", rename = "cdrPrivacy", default)]
        pub cdr_privacy: bool,
        #[yaserde(prefix = "tns", rename = "redirectNotification", default)]
        pub redirect_notification: bool,
        #[yaserde(prefix = "tns", rename = "perButtonRingControl", default)]
        pub per_button_ring_control: bool,
        #[yaserde(prefix = "tns", rename = "bridgedCallAlerting", default)]
        pub bridged_call_alerting: bool,
        #[yaserde(prefix = "tns", rename = "bridgedIdleLinePreference", default)]
        pub bridged_idle_line_preference: bool,
        #[yaserde(prefix = "tns", rename = "confTransOnPrimaryAppearance", default)]
        pub conf_trans_on_primary_appearance: bool,
        #[yaserde(prefix = "tns", rename = "customizableLabels", default)]
        pub customizable_labels: bool,
        #[yaserde(prefix = "tns", rename = "expansionModule", default)]
        pub expansion_module: bool,
        #[yaserde(prefix = "tns", rename = "ipVideoSoftphone", default)]
        pub ip_video_softphone: bool,
        #[yaserde(prefix = "tns", rename = "idleActiveRinging", default)]
        pub idle_active_ringing: String,
        #[yaserde(prefix = "tns", rename = "switchhookFlash", default)]
        pub switchhook_flash: bool,
        #[yaserde(prefix = "tns", rename = "ignoreRotaryDigits", default)]
        pub ignore_rotary_digits: bool,
        #[yaserde(prefix = "tns", rename = "h320Conversion", default)]
        pub h_320_conversion: bool,
        #[yaserde(prefix = "tns", rename = "audixName", default)]
        pub audix_name: String,
        #[yaserde(prefix = "tns", rename = "alwaysUse", default)]
        pub always_use: bool,
        #[yaserde(prefix = "tns", rename = "precedenceCallWaiting", default)]
        pub precedence_call_waiting: bool,
        #[yaserde(prefix = "tns", rename = "autoSelectAnyIdleAppearance", default)]
        pub auto_select_any_idle_appearance: bool,
        #[yaserde(prefix = "tns", rename = "coverageMsgRetrieval", default)]
        pub coverage_msg_retrieval: bool,
        #[yaserde(prefix = "tns", rename = "dataRestriction", default)]
        pub data_restriction: bool,
        #[yaserde(prefix = "tns", rename = "idleAppearancePreference", default)]
        pub idle_appearance_preference: bool,
        #[yaserde(prefix = "tns", rename = "callWaitingIndication", default)]
        pub call_waiting_indication: bool,
        #[yaserde(prefix = "tns", rename = "attCallWaitingIndication", default)]
        pub att_call_waiting_indication: bool,
        #[yaserde(prefix = "tns", rename = "distinctiveAudibleAlert", default)]
        pub distinctive_audible_alert: bool,
        #[yaserde(prefix = "tns", rename = "restrictLastAppearance", default)]
        pub restrict_last_appearance: bool,
        #[yaserde(prefix = "tns", rename = "adjunctSupervision", default)]
        pub adjunct_supervision: bool,
        #[yaserde(prefix = "tns", rename = "busyAutoCallbackWithoutFlash", default)]
        pub busy_auto_callback_without_flash: bool,
        #[yaserde(prefix = "tns", rename = "audibleMessageWaiting", default)]
        pub audible_message_waiting: bool,
        #[yaserde(prefix = "tns", rename = "extendedLocalCalls", default)]
        pub extended_local_calls: bool,
        #[yaserde(prefix = "tns", rename = "imsFeatureSequencing", default)]
        pub ims_feature_sequencing: bool,
        #[yaserde(prefix = "tns", rename = "displayClientRedirection", default)]
        pub display_client_redirection: bool,
        #[yaserde(prefix = "tns", rename = "selectLastUsedAppearance", default)]
        pub select_last_used_appearance: bool,
        #[yaserde(prefix = "tns", rename = "coverageAfterForwarding", default)]
        pub coverage_after_forwarding: String,
        #[yaserde(prefix = "tns", rename = "directIpIpAudioConnections", default)]
        pub direct_ip_ip_audio_connections: bool,
        #[yaserde(prefix = "tns", rename = "ipAudioHairpinning", default)]
        pub ip_audio_hairpinning: bool,
        #[yaserde(prefix = "tns", rename = "primeAppearancePreference", default)]
        pub prime_appearance_preference: String,
        #[yaserde(prefix = "tns", rename = "stationSiteData", default)]
        pub station_site_data: XmlStationSiteData,
        #[yaserde(prefix = "tns", rename = "abbrList", default)]
        pub abbr_list: Vec<XmlStationAbbreviatedDialingData>,
        #[yaserde(prefix = "tns", rename = "buttons", default)]
        pub buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "tns", rename = "featureButtons", default)]
        pub feature_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "tns", rename = "expansionModuleButtons", default)]
        pub expansion_module_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "tns", rename = "softKeys", default)]
        pub soft_keys: Vec<XmlButtonData>,
        #[yaserde(prefix = "tns", rename = "displayButtons", default)]
        pub display_buttons: Vec<XmlButtonData>,
        #[yaserde(prefix = "tns", rename = "stationDataModule", default)]
        pub station_data_module: XmlStationDataModule,
        #[yaserde(prefix = "tns", rename = "hotLineData", default)]
        pub hot_line_data: XmlStationHotLineData,
        #[yaserde(prefix = "tns", rename = "nativeName", default)]
        pub native_name: XmlNativeNameData,
        #[yaserde(prefix = "tns", rename = "unconditionalInternalActive", default)]
        pub unconditional_internal_active: bool,
        #[yaserde(prefix = "tns", rename = "unconditionalExternalActive", default)]
        pub unconditional_external_active: bool,
        #[yaserde(prefix = "tns", rename = "busyInternalActive", default)]
        pub busy_internal_active: bool,
        #[yaserde(prefix = "tns", rename = "busyExternalActive", default)]
        pub busy_external_active: bool,
        #[yaserde(prefix = "tns", rename = "noReplyInternalActive", default)]
        pub no_reply_internal_active: bool,
        #[yaserde(prefix = "tns", rename = "noReplyExternalActive", default)]
        pub no_reply_external_active: bool,
        #[yaserde(prefix = "tns", rename = "emuLoginAllowed", default)]
        pub emu_login_allowed: bool,
        #[yaserde(prefix = "tns", rename = "muteOnOffHookInSCMode", default)]
        pub mute_on_off_hook_in_sc_mode: bool,
        #[yaserde(prefix = "tns", rename = "calculateRoutePattern", default)]
        pub calculate_route_pattern: bool,
        #[yaserde(prefix = "tns", rename = "multimediaEarlyAnswer", default)]
        pub multimedia_early_answer: bool,
        #[yaserde(prefix = "tns", rename = "bridgedApprOrigRestr", default)]
        pub bridged_appr_orig_restr: bool,
        #[yaserde(prefix = "tns", rename = "xid", default)]
        pub xid: bool,
        #[yaserde(prefix = "tns", rename = "stepClearing", default)]
        pub step_clearing: bool,
        #[yaserde(prefix = "tns", rename = "fixedTei", default)]
        pub fixed_tei: bool,
        #[yaserde(prefix = "tns", rename = "endptInit", default)]
        pub endpt_init: bool,
        #[yaserde(prefix = "tns", rename = "isMCTSignalling", default)]
        pub is_mct_signalling: bool,
        #[yaserde(prefix = "tns", rename = "isShortCallingPartyDisplay", default)]
        pub is_short_calling_party_display: bool,
        #[yaserde(prefix = "tns", rename = "passageWay", default)]
        pub passage_way: bool,
        #[yaserde(prefix = "tns", rename = "displayCallerId", default)]
        pub display_caller_id: bool,
        #[yaserde(prefix = "tns", rename = "callerIdMsgWaitingIndication", default)]
        pub caller_id_msg_waiting_indication: bool,
        #[yaserde(prefix = "tns", rename = "recallRotaryDigit", default)]
        pub recall_rotary_digit: bool,
        #[yaserde(prefix = "tns", rename = "profileSettingsData", default)]
        pub profile_settings_data: XmlProfileSettings,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlStationSiteData",
        default
    )]
    pub struct XmlStationSiteData {
        #[yaserde(prefix = "tns", rename = "floor", default)]
        pub floor: String,
        #[yaserde(prefix = "tns", rename = "building", default)]
        pub building: String,
        #[yaserde(prefix = "tns", rename = "headset", default)]
        pub headset: bool,
        #[yaserde(prefix = "tns", rename = "speaker", default)]
        pub speaker: bool,
        #[yaserde(prefix = "tns", rename = "setColor", default)]
        pub set_color: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlStationAbbreviatedDialingData",
        default
    )]
    pub struct XmlStationAbbreviatedDialingData {
        #[yaserde(prefix = "tns", rename = "number", default)]
        pub number: u64,
        #[yaserde(prefix = "tns", rename = "listId", default)]
        pub list_id: u64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlButtonData",
        default
    )]
    pub struct XmlButtonData {
        #[yaserde(prefix = "tns", rename = "number", default)]
        pub number: u64,
        #[yaserde(prefix = "tns", rename = "type", default)]
        pub rs_type: String,
        #[yaserde(prefix = "tns", rename = "data1", default)]
        pub data_1: String,
        #[yaserde(prefix = "tns", rename = "data2", default)]
        pub data_2: String,
        #[yaserde(prefix = "tns", rename = "data3", default)]
        pub data_3: String,
        #[yaserde(prefix = "tns", rename = "data4", default)]
        pub data_4: String,
        #[yaserde(prefix = "tns", rename = "data5", default)]
        pub data_5: String,
        #[yaserde(prefix = "tns", rename = "data6", default)]
        pub data_6: String,
        #[yaserde(prefix = "tns", rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(prefix = "tns", rename = "buttonLabel", default)]
        pub button_label: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlStationDataModule",
        default
    )]
    pub struct XmlStationDataModule {
        #[yaserde(prefix = "tns", rename = "listId", default)]
        pub list_id: u64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlStationHotLineData",
        default
    )]
    pub struct XmlStationHotLineData {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlNativeNameData",
        default
    )]
    pub struct XmlNativeNameData {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        rename = "xmlProfileSettings",
        default
    )]
    pub struct XmlProfileSettings {
        #[yaserde(prefix = "tns", rename = "backgroundLogo", default)]
        pub background_logo: String,
        #[yaserde(prefix = "tns", rename = "personalizedRinging", default)]
        pub personalized_ringing: String,
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

pub mod messages {
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
