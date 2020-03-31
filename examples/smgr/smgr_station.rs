//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.0.2
//!
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
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "users",
        default
    )]
    pub struct Users {
        #[yaserde(prefix = "ns1", rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(prefix = "ns1", rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "UserProvisionRules",
        default
    )]
    pub struct UserProvisionRules {
        #[yaserde(prefix = "ns1", rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "roles",
        default
    )]
    pub struct Roles {
        #[yaserde(prefix = "ns1", rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "ownedContactLists",
        default
    )]
    pub struct OwnedContactLists {
        #[yaserde(prefix = "ns1", rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "ownedContacts",
        default
    )]
    pub struct OwnedContacts {
        #[yaserde(prefix = "ns1", rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlUser",
        default
    )]
    pub struct XmlUser {
        #[yaserde(prefix = "ns1", rename = "UserOrganizationDetails", default)]
        pub user_organization_details: Option<UserOrganizationDetailsType>,
        #[yaserde(prefix = "ns1", rename = "UserProvisionRules", default)]
        pub user_provision_rules: Option<UserProvisionRules>,
        #[yaserde(prefix = "ns1", rename = "authenticationType", default)]
        pub authentication_type: String,
        #[yaserde(prefix = "ns1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "ns1", rename = "displayName", default)]
        pub display_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "displayNameAscii", default)]
        pub display_name_ascii: Option<String>,
        #[yaserde(prefix = "ns1", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "ns1", rename = "isDuplicatedLoginAllowed", default)]
        pub is_duplicated_login_allowed: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "isEnabled", default)]
        pub is_enabled: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "isVirtualUser", default)]
        pub is_virtual_user: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "ns1", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "ns1", rename = "honorific", default)]
        pub honorific: Option<String>,
        #[yaserde(prefix = "ns1", rename = "loginName", default)]
        pub login_name: String,
        #[yaserde(prefix = "ns1", rename = "newLoginName", default)]
        pub new_login_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "employeeNo", default)]
        pub employee_no: Option<String>,
        #[yaserde(prefix = "ns1", rename = "department", default)]
        pub department: Option<String>,
        #[yaserde(prefix = "ns1", rename = "organization", default)]
        pub organization: Option<String>,
        #[yaserde(prefix = "ns1", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "managerName", default)]
        pub manager_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "ns1", rename = "source", default)]
        pub source: Option<String>,
        #[yaserde(prefix = "ns1", rename = "sourceUserKey", default)]
        pub source_user_key: Option<String>,
        #[yaserde(prefix = "ns1", rename = "status", default)]
        pub status: Option<String>,
        #[yaserde(prefix = "ns1", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "ns1", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "ns1", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "ns1", rename = "timeZone", default)]
        pub time_zone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "ns1", rename = "userName", default)]
        pub user_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "userPassword", default)]
        pub user_password: Option<String>,
        #[yaserde(prefix = "ns1", rename = "commPassword", default)]
        pub comm_password: Option<String>,
        #[yaserde(prefix = "ns1", rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(prefix = "ns1", rename = "roles", default)]
        pub roles: Option<Roles>,
        #[yaserde(prefix = "ns1", rename = "localizedNames", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(prefix = "ns1", rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(prefix = "ns1", rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(prefix = "ns1", rename = "ownedContactLists", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(prefix = "ns1", rename = "ownedContacts", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(prefix = "ns1", rename = "presenceUserDefault", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(prefix = "ns1", rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(prefix = "ns1", rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(prefix = "ns1", rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlSecurityIdentity",
        default
    )]
    pub struct XmlSecurityIdentity {
        #[yaserde(prefix = "ns1", rename = "identity", default)]
        pub identity: String,
        #[yaserde(prefix = "ns1", rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(prefix = "ns1", rename = "type", default)]
        pub rs_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresInfoTypeAccessType",
        default
    )]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(prefix = "ns1", rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(prefix = "ns1", rename = "access", default)]
        pub access: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresACRuleType",
        default
    )]
    pub struct XmlPresACRuleType {
        #[yaserde(prefix = "ns1", rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserDefaultType",
        default
    )]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserCLDefaultType",
        default
    )]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresUserACLEntryType",
        default
    )]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlPresInfoTypeType",
        default
    )]
    pub struct XmlPresInfoTypeType {
        #[yaserde(prefix = "ns1", rename = "label", default)]
        pub label: String,
        #[yaserde(prefix = "ns1", rename = "filter", default)]
        pub filter: String,
        #[yaserde(prefix = "ns1", rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlContactList",
        default
    )]
    pub struct XmlContactList {
        #[yaserde(prefix = "ns1", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "ns1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "ns1", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "ns1", rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(prefix = "ns1", rename = "contactListType", default)]
        pub contact_list_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlContactListMember",
        default
    )]
    pub struct XmlContactListMember {
        #[yaserde(prefix = "ns1", rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(prefix = "ns1", rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(prefix = "ns1", rename = "speedDialEntry", default)]
        pub speed_dial_entry: Option<u64>,
        #[yaserde(prefix = "ns1", rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: bool,
        #[yaserde(prefix = "ns1", rename = "label", default)]
        pub label: Option<String>,
        #[yaserde(prefix = "ns1", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "ns1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "ns1", rename = "priorityLevel", default)]
        pub priority_level: Option<u64>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlContactAddress",
        default
    )]
    pub struct XmlContactAddress {
        #[yaserde(prefix = "ns1", rename = "address", default)]
        pub address: String,
        #[yaserde(prefix = "ns1", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "ns1", rename = "contactCategory", default)]
        pub contact_category: String,
        #[yaserde(prefix = "ns1", rename = "contactType", default)]
        pub contact_type: String,
        #[yaserde(prefix = "ns1", rename = "label", default)]
        pub label: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlAddress",
        default
    )]
    pub struct XmlAddress {
        #[yaserde(prefix = "ns1", rename = "addressType", default)]
        pub address_type: String,
        #[yaserde(prefix = "ns1", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "ns1", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "ns1", rename = "localityName", default)]
        pub locality_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "postalCode", default)]
        pub postal_code: Option<String>,
        #[yaserde(prefix = "ns1", rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(prefix = "ns1", rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(prefix = "ns1", rename = "country", default)]
        pub country: Option<String>,
        #[yaserde(prefix = "ns1", rename = "street", default)]
        pub street: Option<String>,
        #[yaserde(prefix = "ns1", rename = "businessphone", default)]
        pub businessphone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "otherbusinessphone", default)]
        pub otherbusinessphone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "fax", default)]
        pub fax: Option<String>,
        #[yaserde(prefix = "ns1", rename = "homephone", default)]
        pub homephone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "otherhomephone", default)]
        pub otherhomephone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "mobilephone", default)]
        pub mobilephone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "othermobilephone", default)]
        pub othermobilephone: Option<String>,
        #[yaserde(prefix = "ns1", rename = "pager", default)]
        pub pager: Option<String>,
        #[yaserde(prefix = "ns1", rename = "pager2", default)]
        pub pager_2: Option<String>,
        #[yaserde(prefix = "ns1", rename = "postalAddress", default)]
        pub postal_address: Option<String>,
        #[yaserde(prefix = "ns1", rename = "isPrivate", default)]
        pub is_private: Option<bool>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlContact",
        default
    )]
    pub struct XmlContact {
        #[yaserde(prefix = "ns1", rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(prefix = "ns1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "ns1", rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(prefix = "ns1", rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(prefix = "ns1", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "ns1", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "ns1", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "ns1", rename = "initials", default)]
        pub initials: Option<String>,
        #[yaserde(prefix = "ns1", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "ns1", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "ns1", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "ns1", rename = "source", default)]
        pub source: String,
        #[yaserde(prefix = "ns1", rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(prefix = "ns1", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "ns1", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "ns1", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "ns1", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "ns1", rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(prefix = "ns1", rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlHandle",
        default
    )]
    pub struct XmlHandle {
        #[yaserde(prefix = "ns1", rename = "handleName", default)]
        pub handle_name: String,
        #[yaserde(prefix = "ns1", rename = "handleType", default)]
        pub handle_type: String,
        #[yaserde(prefix = "ns1", rename = "handleSubType", default)]
        pub handle_sub_type: Option<String>,
        #[yaserde(prefix = "ns1", rename = "domainName", default)]
        pub domain_name: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlCommProfileType",
        default
    )]
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "ns1", rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(prefix = "ns1", rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(prefix = "ns1", rename = "jobId", default)]
        pub job_id: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "handleList",
        default
    )]
    pub struct HandleList {
        #[yaserde(prefix = "ns1", rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "commProfileList",
        default
    )]
    pub struct CommProfileList {
        #[yaserde(prefix = "ns1", rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlCommProfileSetType",
        default
    )]
    pub struct XmlCommProfileSetType {
        #[yaserde(prefix = "ns1", rename = "commProfileSetName", default)]
        pub comm_profile_set_name: String,
        #[yaserde(prefix = "ns1", rename = "isPrimary", default)]
        pub is_primary: bool,
        #[yaserde(prefix = "ns1", rename = "handleList", default)]
        pub handle_list: Option<HandleList>,
        #[yaserde(prefix = "ns1", rename = "commProfileList", default)]
        pub comm_profile_list: Option<CommProfileList>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "ForgeinCommProfileType",
        default
    )]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "ns1", rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<u64>,
        #[yaserde(prefix = "ns1", rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(prefix = "ns1", rename = "serviceData", default)]
        pub service_data: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlSecureStore",
        default
    )]
    pub struct XmlSecureStore {
        #[yaserde(prefix = "ns1", rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(prefix = "ns1", rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmlLocalizedName",
        default
    )]
    pub struct XmlLocalizedName {
        #[yaserde(prefix = "ns1", rename = "locale", default)]
        pub locale: String,
        #[yaserde(prefix = "ns1", rename = "name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "xmLocalizedNames",
        default
    )]
    pub struct XmLocalizedNames {
        #[yaserde(prefix = "ns1", rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "tenant",
        default
    )]
    pub struct Tenant {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import",
        rename = "UserOrganizationDetailsType",
        default
    )]
    pub struct UserOrganizationDetailsType {
        #[yaserde(prefix = "ns1", rename = "tenant", default)]
        pub tenant: Tenant,
        #[yaserde(prefix = "ns1", rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: Option<String>,
        #[yaserde(prefix = "ns1", rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: Option<String>,
        #[yaserde(prefix = "ns1", rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlStationProfile",
        default
    )]
    pub struct XmlStationProfile {
        #[yaserde(prefix = "tns", rename = "cmName", default)]
        pub cm_name: String,
        #[yaserde(prefix = "tns", rename = "prefHandleId", default)]
        pub pref_handle_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "useExistingExtension", default)]
        pub use_existing_extension: Option<bool>,
        #[yaserde(prefix = "tns", rename = "extensionRange", default)]
        pub extension_range: Option<String>,
        #[yaserde(prefix = "tns", rename = "extension", default)]
        pub extension: String,
        #[yaserde(prefix = "tns", rename = "template", default)]
        pub template: Option<String>,
        #[yaserde(prefix = "tns", rename = "setType", default)]
        pub set_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "securityCode", default)]
        pub security_code: Option<String>,
        #[yaserde(prefix = "tns", rename = "port", default)]
        pub port: Option<String>,
        #[yaserde(prefix = "tns", rename = "deleteOnUnassign", default)]
        pub delete_on_unassign: Option<bool>,
        #[yaserde(prefix = "tns", rename = "overRideEndpointName", default)]
        pub over_ride_endpoint_name: Option<bool>,
        #[yaserde(prefix = "tns", rename = "dualRegistration", default)]
        pub dual_registration: Option<bool>,
        #[yaserde(prefix = "tns", rename = "enhCallrInfodisplay", default)]
        pub enh_callr_infodisplay: Option<bool>,
        #[yaserde(prefix = "tns", rename = "lockMessages", default)]
        pub lock_messages: Option<bool>,
        #[yaserde(prefix = "tns", rename = "coveragePath1", default)]
        pub coverage_path_1: Option<String>,
        #[yaserde(prefix = "tns", rename = "coveragePath2", default)]
        pub coverage_path_2: Option<String>,
        #[yaserde(prefix = "tns", rename = "huntToStation", default)]
        pub hunt_to_station: Option<String>,
        #[yaserde(prefix = "tns", rename = "tn", default)]
        pub tn: Option<u64>,
        #[yaserde(prefix = "tns", rename = "cor", default)]
        pub cor: Option<u64>,
        #[yaserde(prefix = "tns", rename = "cos", default)]
        pub cos: Option<u64>,
        #[yaserde(prefix = "tns", rename = "xmobileType", default)]
        pub xmobile_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "mappingMode", default)]
        pub mapping_mode: Option<String>,
        #[yaserde(prefix = "tns", rename = "configurationSet", default)]
        pub configuration_set: Option<String>,
        #[yaserde(prefix = "tns", rename = "mobilityTrunkGroup", default)]
        pub mobility_trunk_group: Option<String>,
        #[yaserde(prefix = "tns", rename = "dialPrefix", default)]
        pub dial_prefix: Option<String>,
        #[yaserde(prefix = "tns", rename = "cellPhoneNumber", default)]
        pub cell_phone_number: Option<String>,
        #[yaserde(prefix = "tns", rename = "musicSource", default)]
        pub music_source: Option<u64>,
        #[yaserde(prefix = "tns", rename = "tests", default)]
        pub tests: Option<bool>,
        #[yaserde(prefix = "tns", rename = "dataModule", default)]
        pub data_module: Option<bool>,
        #[yaserde(prefix = "tns", rename = "speakerphone", default)]
        pub speakerphone: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayLanguage", default)]
        pub display_language: Option<String>,
        #[yaserde(prefix = "tns", rename = "personalizedRingingPattern", default)]
        pub personalized_ringing_pattern: Option<u64>,
        #[yaserde(prefix = "tns", rename = "messageLampExt", default)]
        pub message_lamp_ext: Option<String>,
        #[yaserde(prefix = "tns", rename = "muteButtonEnabled", default)]
        pub mute_button_enabled: Option<bool>,
        #[yaserde(prefix = "tns", rename = "mediaComplexExt", default)]
        pub media_complex_ext: Option<String>,
        #[yaserde(prefix = "tns", rename = "ipSoftphone", default)]
        pub ip_softphone: Option<bool>,
        #[yaserde(prefix = "tns", rename = "survivableGkNodeName", default)]
        pub survivable_gk_node_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "survivableCOR", default)]
        pub survivable_cor: Option<String>,
        #[yaserde(prefix = "tns", rename = "survivableTrunkDest", default)]
        pub survivable_trunk_dest: Option<bool>,
        #[yaserde(prefix = "tns", rename = "voiceMailNumber", default)]
        pub voice_mail_number: Option<String>,
        #[yaserde(prefix = "tns", rename = "offPremisesStation", default)]
        pub off_premises_station: Option<bool>,
        #[yaserde(prefix = "tns", rename = "dataOption", default)]
        pub data_option: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayModule", default)]
        pub display_module: Option<bool>,
        #[yaserde(prefix = "tns", rename = "messageWaitingIndicator", default)]
        pub message_waiting_indicator: Option<String>,
        #[yaserde(prefix = "tns", rename = "remoteOfficePhone", default)]
        pub remote_office_phone: Option<bool>,
        #[yaserde(prefix = "tns", rename = "lwcReception", default)]
        pub lwc_reception: Option<String>,
        #[yaserde(prefix = "tns", rename = "lwcActivation", default)]
        pub lwc_activation: Option<bool>,
        #[yaserde(prefix = "tns", rename = "lwcLogExternalCalls", default)]
        pub lwc_log_external_calls: Option<bool>,
        #[yaserde(prefix = "tns", rename = "cdrPrivacy", default)]
        pub cdr_privacy: Option<bool>,
        #[yaserde(prefix = "tns", rename = "redirectNotification", default)]
        pub redirect_notification: Option<bool>,
        #[yaserde(prefix = "tns", rename = "perButtonRingControl", default)]
        pub per_button_ring_control: Option<bool>,
        #[yaserde(prefix = "tns", rename = "bridgedCallAlerting", default)]
        pub bridged_call_alerting: Option<bool>,
        #[yaserde(prefix = "tns", rename = "bridgedIdleLinePreference", default)]
        pub bridged_idle_line_preference: Option<bool>,
        #[yaserde(prefix = "tns", rename = "confTransOnPrimaryAppearance", default)]
        pub conf_trans_on_primary_appearance: Option<bool>,
        #[yaserde(prefix = "tns", rename = "customizableLabels", default)]
        pub customizable_labels: Option<bool>,
        #[yaserde(prefix = "tns", rename = "expansionModule", default)]
        pub expansion_module: Option<bool>,
        #[yaserde(prefix = "tns", rename = "ipVideoSoftphone", default)]
        pub ip_video_softphone: Option<bool>,
        #[yaserde(prefix = "tns", rename = "activeStationRinging", default)]
        pub active_station_ringing: Option<String>,
        #[yaserde(prefix = "tns", rename = "idleActiveRinging", default)]
        pub idle_active_ringing: Option<String>,
        #[yaserde(prefix = "tns", rename = "switchhookFlash", default)]
        pub switchhook_flash: Option<bool>,
        #[yaserde(prefix = "tns", rename = "ignoreRotaryDigits", default)]
        pub ignore_rotary_digits: Option<bool>,
        #[yaserde(prefix = "tns", rename = "h320Conversion", default)]
        pub h_320_conversion: Option<bool>,
        #[yaserde(prefix = "tns", rename = "serviceLinkMode", default)]
        pub service_link_mode: Option<String>,
        #[yaserde(prefix = "tns", rename = "multimediaMode", default)]
        pub multimedia_mode: Option<String>,
        #[yaserde(prefix = "tns", rename = "mwiServedUserType", default)]
        pub mwi_served_user_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "audixName", default)]
        pub audix_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "automaticMoves", default)]
        pub automatic_moves: Option<String>,
        #[yaserde(prefix = "tns", rename = "remoteSoftphoneEmergencyCalls", default)]
        pub remote_softphone_emergency_calls: Option<String>,
        #[yaserde(prefix = "tns", rename = "emergencyLocationExt", default)]
        pub emergency_location_ext: Option<String>,
        #[yaserde(prefix = "tns", rename = "alwaysUse", default)]
        pub always_use: Option<bool>,
        #[yaserde(prefix = "tns", rename = "precedenceCallWaiting", default)]
        pub precedence_call_waiting: Option<bool>,
        #[yaserde(prefix = "tns", rename = "autoSelectAnyIdleAppearance", default)]
        pub auto_select_any_idle_appearance: Option<bool>,
        #[yaserde(prefix = "tns", rename = "coverageMsgRetrieval", default)]
        pub coverage_msg_retrieval: Option<bool>,
        #[yaserde(prefix = "tns", rename = "autoAnswer", default)]
        pub auto_answer: Option<String>,
        #[yaserde(prefix = "tns", rename = "dataRestriction", default)]
        pub data_restriction: Option<bool>,
        #[yaserde(prefix = "tns", rename = "idleAppearancePreference", default)]
        pub idle_appearance_preference: Option<bool>,
        #[yaserde(prefix = "tns", rename = "callWaitingIndication", default)]
        pub call_waiting_indication: Option<bool>,
        #[yaserde(prefix = "tns", rename = "attCallWaitingIndication", default)]
        pub att_call_waiting_indication: Option<bool>,
        #[yaserde(prefix = "tns", rename = "distinctiveAudibleAlert", default)]
        pub distinctive_audible_alert: Option<bool>,
        #[yaserde(prefix = "tns", rename = "restrictLastAppearance", default)]
        pub restrict_last_appearance: Option<bool>,
        #[yaserde(prefix = "tns", rename = "adjunctSupervision", default)]
        pub adjunct_supervision: Option<bool>,
        #[yaserde(prefix = "tns", rename = "perStationCpnSendCallingNumber", default)]
        pub per_station_cpn_send_calling_number: Option<String>,
        #[yaserde(prefix = "tns", rename = "busyAutoCallbackWithoutFlash", default)]
        pub busy_auto_callback_without_flash: Option<bool>,
        #[yaserde(prefix = "tns", rename = "audibleMessageWaiting", default)]
        pub audible_message_waiting: Option<bool>,
        #[yaserde(prefix = "tns", rename = "extendedLocalCalls", default)]
        pub extended_local_calls: Option<bool>,
        #[yaserde(prefix = "tns", rename = "imsFeatureSequencing", default)]
        pub ims_feature_sequencing: Option<bool>,
        #[yaserde(prefix = "tns", rename = "displayClientRedirection", default)]
        pub display_client_redirection: Option<bool>,
        #[yaserde(prefix = "tns", rename = "selectLastUsedAppearance", default)]
        pub select_last_used_appearance: Option<bool>,
        #[yaserde(prefix = "tns", rename = "coverageAfterForwarding", default)]
        pub coverage_after_forwarding: Option<String>,
        #[yaserde(prefix = "tns", rename = "directIpIpAudioConnections", default)]
        pub direct_ip_ip_audio_connections: Option<bool>,
        #[yaserde(prefix = "tns", rename = "ipAudioHairpinning", default)]
        pub ip_audio_hairpinning: Option<bool>,
        #[yaserde(prefix = "tns", rename = "primeAppearancePreference", default)]
        pub prime_appearance_preference: Option<String>,
        #[yaserde(prefix = "tns", rename = "stationSiteData", default)]
        pub station_site_data: Option<XmlStationSiteData>,
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
        pub station_data_module: Option<XmlStationDataModule>,
        #[yaserde(prefix = "tns", rename = "hotLineData", default)]
        pub hot_line_data: Option<XmlStationHotLineData>,
        #[yaserde(prefix = "tns", rename = "nativeName", default)]
        pub native_name: Option<XmlNativeNameData>,
        #[yaserde(prefix = "tns", rename = "buttonModules", default)]
        pub button_modules: Option<u64>,
        #[yaserde(prefix = "tns", rename = "unconditionalInternalDest", default)]
        pub unconditional_internal_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "unconditionalInternalActive", default)]
        pub unconditional_internal_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "unconditionalExternalDest", default)]
        pub unconditional_external_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "unconditionalExternalActive", default)]
        pub unconditional_external_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "busyInternalDest", default)]
        pub busy_internal_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "busyInternalActive", default)]
        pub busy_internal_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "busyExternalDest", default)]
        pub busy_external_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "busyExternalActive", default)]
        pub busy_external_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "noReplyInternalDest", default)]
        pub no_reply_internal_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "noReplyInternalActive", default)]
        pub no_reply_internal_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "noReplyExternalDest", default)]
        pub no_reply_external_dest: Option<String>,
        #[yaserde(prefix = "tns", rename = "noReplyExternalActive", default)]
        pub no_reply_external_active: Option<bool>,
        #[yaserde(prefix = "tns", rename = "sacCfOverride", default)]
        pub sac_cf_override: Option<String>,
        #[yaserde(prefix = "tns", rename = "lossGroup", default)]
        pub loss_group: Option<u64>,
        #[yaserde(prefix = "tns", rename = "timeOfDayLockTable", default)]
        pub time_of_day_lock_table: Option<String>,
        #[yaserde(prefix = "tns", rename = "emuLoginAllowed", default)]
        pub emu_login_allowed: Option<bool>,
        #[yaserde(prefix = "tns", rename = "ec500State", default)]
        pub ec_500_state: Option<String>,
        #[yaserde(prefix = "tns", rename = "muteOnOffHookInSCMode", default)]
        pub mute_on_off_hook_in_sc_mode: Option<bool>,
        #[yaserde(prefix = "tns", rename = "type3pccEnabled", default)]
        pub type_3pcc_enabled: Option<String>,
        #[yaserde(prefix = "tns", rename = "calculateRoutePattern", default)]
        pub calculate_route_pattern: Option<bool>,
        #[yaserde(prefix = "tns", rename = "sipTrunk", default)]
        pub sip_trunk: Option<String>,
        #[yaserde(prefix = "tns", rename = "enableReachStaDomainControl", default)]
        pub enable_reach_sta_domain_control: Option<String>,
        #[yaserde(prefix = "tns", rename = "multimediaEarlyAnswer", default)]
        pub multimedia_early_answer: Option<bool>,
        #[yaserde(prefix = "tns", rename = "bridgedApprOrigRestr", default)]
        pub bridged_appr_orig_restr: Option<bool>,
        #[yaserde(prefix = "tns", rename = "callApprDispFormat", default)]
        pub call_appr_disp_format: Option<String>,
        #[yaserde(prefix = "tns", rename = "ipPhoneGroupId", default)]
        pub ip_phone_group_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "xoipEndPointType", default)]
        pub xoip_end_point_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "xid", default)]
        pub xid: Option<bool>,
        #[yaserde(prefix = "tns", rename = "stepClearing", default)]
        pub step_clearing: Option<bool>,
        #[yaserde(prefix = "tns", rename = "fixedTei", default)]
        pub fixed_tei: Option<bool>,
        #[yaserde(prefix = "tns", rename = "tei", default)]
        pub tei: Option<String>,
        #[yaserde(prefix = "tns", rename = "countryProtocol", default)]
        pub country_protocol: Option<String>,
        #[yaserde(prefix = "tns", rename = "endptInit", default)]
        pub endpt_init: Option<bool>,
        #[yaserde(prefix = "tns", rename = "spid", default)]
        pub spid: Option<String>,
        #[yaserde(prefix = "tns", rename = "endptId", default)]
        pub endpt_id: Option<String>,
        #[yaserde(prefix = "tns", rename = "isMCTSignalling", default)]
        pub is_mct_signalling: Option<bool>,
        #[yaserde(prefix = "tns", rename = "isShortCallingPartyDisplay", default)]
        pub is_short_calling_party_display: Option<bool>,
        #[yaserde(prefix = "tns", rename = "passageWay", default)]
        pub passage_way: Option<bool>,
        #[yaserde(prefix = "tns", rename = "dtmfOverIp", default)]
        pub dtmf_over_ip: Option<String>,
        #[yaserde(prefix = "tns", rename = "location", default)]
        pub location: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayCallerId", default)]
        pub display_caller_id: Option<bool>,
        #[yaserde(prefix = "tns", rename = "callerIdMsgWaitingIndication", default)]
        pub caller_id_msg_waiting_indication: Option<bool>,
        #[yaserde(prefix = "tns", rename = "recallRotaryDigit", default)]
        pub recall_rotary_digit: Option<bool>,
        #[yaserde(prefix = "tns", rename = "profileSettingsData", default)]
        pub profile_settings_data: Option<XmlProfileSettings>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlStationSiteData",
        default
    )]
    pub struct XmlStationSiteData {
        #[yaserde(prefix = "tns", rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(prefix = "tns", rename = "jack", default)]
        pub jack: Option<String>,
        #[yaserde(prefix = "tns", rename = "cable", default)]
        pub cable: Option<String>,
        #[yaserde(prefix = "tns", rename = "floor", default)]
        pub floor: Option<String>,
        #[yaserde(prefix = "tns", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "tns", rename = "headset", default)]
        pub headset: Option<bool>,
        #[yaserde(prefix = "tns", rename = "speaker", default)]
        pub speaker: Option<bool>,
        #[yaserde(prefix = "tns", rename = "mounting", default)]
        pub mounting: Option<String>,
        #[yaserde(prefix = "tns", rename = "cordLength", default)]
        pub cord_length: Option<u64>,
        #[yaserde(prefix = "tns", rename = "setColor", default)]
        pub set_color: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlStationAbbreviatedDialingData",
        default
    )]
    pub struct XmlStationAbbreviatedDialingData {
        #[yaserde(prefix = "tns", rename = "listType", default)]
        pub list_type: String,
        #[yaserde(prefix = "tns", rename = "number", default)]
        pub number: u64,
        #[yaserde(prefix = "tns", rename = "listId", default)]
        pub list_id: Option<u64>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlButtonData",
        default
    )]
    pub struct XmlButtonData {
        #[yaserde(prefix = "tns", rename = "number", default)]
        pub number: u64,
        #[yaserde(prefix = "tns", rename = "type", default)]
        pub rs_type: String,
        #[yaserde(prefix = "tns", rename = "data1", default)]
        pub data_1: Option<String>,
        #[yaserde(prefix = "tns", rename = "data2", default)]
        pub data_2: Option<String>,
        #[yaserde(prefix = "tns", rename = "data3", default)]
        pub data_3: Option<String>,
        #[yaserde(prefix = "tns", rename = "data4", default)]
        pub data_4: Option<String>,
        #[yaserde(prefix = "tns", rename = "data5", default)]
        pub data_5: Option<String>,
        #[yaserde(prefix = "tns", rename = "data6", default)]
        pub data_6: Option<String>,
        #[yaserde(prefix = "tns", rename = "isFavorite", default)]
        pub is_favorite: Option<bool>,
        #[yaserde(prefix = "tns", rename = "buttonLabel", default)]
        pub button_label: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlStationDataModule",
        default
    )]
    pub struct XmlStationDataModule {
        #[yaserde(prefix = "tns", rename = "dataExtension", default)]
        pub data_extension: String,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: Option<String>,
        #[yaserde(prefix = "tns", rename = "cor", default)]
        pub cor: u64,
        #[yaserde(prefix = "tns", rename = "cos", default)]
        pub cos: u64,
        #[yaserde(prefix = "tns", rename = "itc", default)]
        pub itc: String,
        #[yaserde(prefix = "tns", rename = "tn", default)]
        pub tn: u64,
        #[yaserde(prefix = "tns", rename = "listType", default)]
        pub list_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "listId", default)]
        pub list_id: Option<u64>,
        #[yaserde(prefix = "tns", rename = "specialDialingOption", default)]
        pub special_dialing_option: Option<String>,
        #[yaserde(prefix = "tns", rename = "specialDialingAbbrDialCode", default)]
        pub special_dialing_abbr_dial_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlStationHotLineData",
        default
    )]
    pub struct XmlStationHotLineData {
        #[yaserde(prefix = "tns", rename = "hotLineDestAbbrevList", default)]
        pub hot_line_dest_abbrev_list: Option<u64>,
        #[yaserde(prefix = "tns", rename = "hotLineAbbrevDialCode", default)]
        pub hot_line_abbrev_dial_code: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlNativeNameData",
        default
    )]
    pub struct XmlNativeNameData {
        #[yaserde(prefix = "tns", rename = "locale", default)]
        pub locale: Option<String>,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import_csm_cm",
        rename = "xmlProfileSettings",
        default
    )]
    pub struct XmlProfileSettings {
        #[yaserde(prefix = "tns", rename = "phoneScreenCalling", default)]
        pub phone_screen_calling: Option<String>,
        #[yaserde(prefix = "tns", rename = "profileRedial", default)]
        pub profile_redial: Option<String>,
        #[yaserde(prefix = "tns", rename = "dialingOption", default)]
        pub dialing_option: Option<String>,
        #[yaserde(prefix = "tns", rename = "headsetSignaling", default)]
        pub headset_signaling: Option<String>,
        #[yaserde(prefix = "tns", rename = "audioPath", default)]
        pub audio_path: Option<String>,
        #[yaserde(prefix = "tns", rename = "buttonClicks", default)]
        pub button_clicks: Option<String>,
        #[yaserde(prefix = "tns", rename = "phoneScreen", default)]
        pub phone_screen: Option<String>,
        #[yaserde(prefix = "tns", rename = "backgroundLogo", default)]
        pub background_logo: Option<String>,
        #[yaserde(prefix = "tns", rename = "personalizedRinging", default)]
        pub personalized_ringing: Option<String>,
        #[yaserde(prefix = "tns", rename = "callPickUpIndication", default)]
        pub call_pick_up_indication: Option<String>,
        #[yaserde(prefix = "tns", rename = "touchPanel", default)]
        pub touch_panel: Option<String>,
        #[yaserde(prefix = "tns", rename = "language", default)]
        pub language: Option<String>,
        #[yaserde(prefix = "tns", rename = "userPreferredLanguage", default)]
        pub user_preferred_language: Option<String>,
        #[yaserde(prefix = "tns", rename = "languageFileInUse", default)]
        pub language_file_in_use: Option<String>,
        #[yaserde(prefix = "tns", rename = "timeFormat", default)]
        pub time_format: Option<String>,
        #[yaserde(prefix = "tns", rename = "awayTimer", default)]
        pub away_timer: Option<String>,
        #[yaserde(prefix = "tns", rename = "awayTimerValue", default)]
        pub away_timer_value: Option<u64>,
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
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
