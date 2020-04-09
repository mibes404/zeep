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
        root = "users",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "nsi1"
    )]
    pub struct Users {
        #[yaserde(rename = "secureStore", prefix = "nsi1", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(rename = "user", prefix = "nsi1", default)]
        pub user: Vec<XmlUser>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "UserProvisionRules",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct UserProvisionRules {
        #[yaserde(rename = "UserProvisionRuleName", prefix = "nsi1", default)]
        pub user_provision_rule_name: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "roles",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct Roles {
        #[yaserde(rename = "role", prefix = "nsi1", default)]
        pub role: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "ownedContactLists",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct OwnedContactLists {
        #[yaserde(rename = "contactList", prefix = "nsi1", default)]
        pub contact_list: XmlContactList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "ownedContacts",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct OwnedContacts {
        #[yaserde(rename = "contact", prefix = "nsi1", default)]
        pub contact: Vec<XmlContact>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlUser",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlUser {
        #[yaserde(rename = "UserOrganizationDetails", prefix = "nsi1", default)]
        pub user_organization_details: Option<UserOrganizationDetailsType>,
        #[yaserde(rename = "UserProvisionRules", prefix = "nsi1", default)]
        pub user_provision_rules: Option<UserProvisionRules>,
        #[yaserde(rename = "authenticationType", prefix = "nsi1", default)]
        pub authentication_type: String,
        #[yaserde(rename = "description", prefix = "nsi1", default)]
        pub description: Option<String>,
        #[yaserde(rename = "displayName", prefix = "nsi1", default)]
        pub display_name: Option<String>,
        #[yaserde(rename = "displayNameAscii", prefix = "nsi1", default)]
        pub display_name_ascii: Option<String>,
        #[yaserde(rename = "dn", prefix = "nsi1", default)]
        pub dn: Option<String>,
        #[yaserde(rename = "isDuplicatedLoginAllowed", prefix = "nsi1", default)]
        pub is_duplicated_login_allowed: Option<bool>,
        #[yaserde(rename = "isEnabled", prefix = "nsi1", default)]
        pub is_enabled: Option<bool>,
        #[yaserde(rename = "isVirtualUser", prefix = "nsi1", default)]
        pub is_virtual_user: Option<bool>,
        #[yaserde(rename = "givenName", prefix = "nsi1", default)]
        pub given_name: String,
        #[yaserde(rename = "givenNameAscii", prefix = "nsi1", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(rename = "honorific", prefix = "nsi1", default)]
        pub honorific: Option<String>,
        #[yaserde(rename = "loginName", prefix = "nsi1", default)]
        pub login_name: String,
        #[yaserde(rename = "newLoginName", prefix = "nsi1", default)]
        pub new_login_name: Option<String>,
        #[yaserde(rename = "employeeNo", prefix = "nsi1", default)]
        pub employee_no: Option<String>,
        #[yaserde(rename = "department", prefix = "nsi1", default)]
        pub department: Option<String>,
        #[yaserde(rename = "organization", prefix = "nsi1", default)]
        pub organization: Option<String>,
        #[yaserde(rename = "middleName", prefix = "nsi1", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "managerName", prefix = "nsi1", default)]
        pub manager_name: Option<String>,
        #[yaserde(rename = "preferredGivenName", prefix = "nsi1", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(rename = "preferredLanguage", prefix = "nsi1", default)]
        pub preferred_language: Option<String>,
        #[yaserde(rename = "source", prefix = "nsi1", default)]
        pub source: Option<String>,
        #[yaserde(rename = "sourceUserKey", prefix = "nsi1", default)]
        pub source_user_key: Option<String>,
        #[yaserde(rename = "status", prefix = "nsi1", default)]
        pub status: Option<String>,
        #[yaserde(rename = "suffix", prefix = "nsi1", default)]
        pub suffix: Option<String>,
        #[yaserde(rename = "surname", prefix = "nsi1", default)]
        pub surname: String,
        #[yaserde(rename = "surnameAscii", prefix = "nsi1", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(rename = "timeZone", prefix = "nsi1", default)]
        pub time_zone: Option<String>,
        #[yaserde(rename = "title", prefix = "nsi1", default)]
        pub title: Option<String>,
        #[yaserde(rename = "userName", prefix = "nsi1", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "userPassword", prefix = "nsi1", default)]
        pub user_password: Option<String>,
        #[yaserde(rename = "commPassword", prefix = "nsi1", default)]
        pub comm_password: Option<String>,
        #[yaserde(rename = "userType", prefix = "nsi1", default)]
        pub user_type: Vec<String>,
        #[yaserde(rename = "roles", prefix = "nsi1", default)]
        pub roles: Option<Roles>,
        #[yaserde(rename = "localizedNames", prefix = "nsi1", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(rename = "address", prefix = "nsi1", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(rename = "securityIdentity", prefix = "nsi1", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(rename = "ownedContactLists", prefix = "nsi1", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(rename = "ownedContacts", prefix = "nsi1", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(rename = "presenceUserDefault", prefix = "nsi1", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(rename = "presenceUserACL", prefix = "nsi1", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(rename = "presenceUserCLDefault", prefix = "nsi1", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(rename = "commProfileSet", prefix = "nsi1", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlSecurityIdentity",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlSecurityIdentity {
        #[yaserde(rename = "identity", prefix = "nsi1", default)]
        pub identity: String,
        #[yaserde(rename = "realm", prefix = "nsi1", default)]
        pub realm: Option<String>,
        #[yaserde(rename = "type", prefix = "nsi1", default)]
        pub rs_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresInfoTypeAccessType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(rename = "infoType", prefix = "nsi1", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(rename = "access", prefix = "nsi1", default)]
        pub access: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresACRuleType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresACRuleType {
        #[yaserde(rename = "infoTypeAccess", prefix = "nsi1", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresUserDefaultType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresUserCLDefaultType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresUserACLEntryType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "watcherLoginName", prefix = "nsi1", default)]
        pub watcher_login_name: Option<String>,
        #[yaserde(rename = "watcherDisplayName", prefix = "nsi1", default)]
        pub watcher_display_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlPresInfoTypeType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlPresInfoTypeType {
        #[yaserde(rename = "label", prefix = "nsi1", default)]
        pub label: String,
        #[yaserde(rename = "filter", prefix = "nsi1", default)]
        pub filter: String,
        #[yaserde(rename = "specFlags", prefix = "nsi1", default)]
        pub spec_flags: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlContactList",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlContactList {
        #[yaserde(rename = "name", prefix = "nsi1", default)]
        pub name: String,
        #[yaserde(rename = "description", prefix = "nsi1", default)]
        pub description: Option<String>,
        #[yaserde(rename = "isPublic", prefix = "nsi1", default)]
        pub is_public: bool,
        #[yaserde(rename = "members", prefix = "nsi1", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(rename = "contactListType", prefix = "nsi1", default)]
        pub contact_list_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlContactListMember",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlContactListMember {
        #[yaserde(rename = "memberContact", prefix = "nsi1", default)]
        pub member_contact: Option<String>,
        #[yaserde(rename = "speedDialContactAddress", prefix = "nsi1", default)]
        pub speed_dial_contact_address: Option<XmlContactAddress>,
        #[yaserde(rename = "memberUser", prefix = "nsi1", default)]
        pub member_user: Option<String>,
        #[yaserde(rename = "speedDialHandle", prefix = "nsi1", default)]
        pub speed_dial_handle: Option<XmlHandle>,
        #[yaserde(rename = "isFavorite", prefix = "nsi1", default)]
        pub is_favorite: bool,
        #[yaserde(rename = "isSpeedDial", prefix = "nsi1", default)]
        pub is_speed_dial: bool,
        #[yaserde(rename = "speedDialEntry", prefix = "nsi1", default)]
        pub speed_dial_entry: Option<i32>,
        #[yaserde(rename = "isPresenceBuddy", prefix = "nsi1", default)]
        pub is_presence_buddy: bool,
        #[yaserde(rename = "label", prefix = "nsi1", default)]
        pub label: Option<String>,
        #[yaserde(rename = "altLabel", prefix = "nsi1", default)]
        pub alt_label: Option<String>,
        #[yaserde(rename = "description", prefix = "nsi1", default)]
        pub description: Option<String>,
        #[yaserde(rename = "priorityLevel", prefix = "nsi1", default)]
        pub priority_level: Option<i32>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlContactAddress",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlContactAddress {
        #[yaserde(rename = "address", prefix = "nsi1", default)]
        pub address: String,
        #[yaserde(rename = "altLabel", prefix = "nsi1", default)]
        pub alt_label: Option<String>,
        #[yaserde(rename = "contactCategory", prefix = "nsi1", default)]
        pub contact_category: String,
        #[yaserde(rename = "contactType", prefix = "nsi1", default)]
        pub contact_type: String,
        #[yaserde(rename = "label", prefix = "nsi1", default)]
        pub label: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlAddress",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlAddress {
        #[yaserde(rename = "addressType", prefix = "nsi1", default)]
        pub address_type: String,
        #[yaserde(rename = "name", prefix = "nsi1", default)]
        pub name: String,
        #[yaserde(rename = "building", prefix = "nsi1", default)]
        pub building: Option<String>,
        #[yaserde(rename = "localityName", prefix = "nsi1", default)]
        pub locality_name: Option<String>,
        #[yaserde(rename = "postalCode", prefix = "nsi1", default)]
        pub postal_code: Option<String>,
        #[yaserde(rename = "room", prefix = "nsi1", default)]
        pub room: Option<String>,
        #[yaserde(rename = "stateOrProvince", prefix = "nsi1", default)]
        pub state_or_province: Option<String>,
        #[yaserde(rename = "country", prefix = "nsi1", default)]
        pub country: Option<String>,
        #[yaserde(rename = "street", prefix = "nsi1", default)]
        pub street: Option<String>,
        #[yaserde(rename = "businessphone", prefix = "nsi1", default)]
        pub businessphone: Option<String>,
        #[yaserde(rename = "otherbusinessphone", prefix = "nsi1", default)]
        pub otherbusinessphone: Option<String>,
        #[yaserde(rename = "fax", prefix = "nsi1", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "homephone", prefix = "nsi1", default)]
        pub homephone: Option<String>,
        #[yaserde(rename = "otherhomephone", prefix = "nsi1", default)]
        pub otherhomephone: Option<String>,
        #[yaserde(rename = "mobilephone", prefix = "nsi1", default)]
        pub mobilephone: Option<String>,
        #[yaserde(rename = "othermobilephone", prefix = "nsi1", default)]
        pub othermobilephone: Option<String>,
        #[yaserde(rename = "pager", prefix = "nsi1", default)]
        pub pager: Option<String>,
        #[yaserde(rename = "pager2", prefix = "nsi1", default)]
        pub pager_2: Option<String>,
        #[yaserde(rename = "postalAddress", prefix = "nsi1", default)]
        pub postal_address: Option<String>,
        #[yaserde(rename = "isPrivate", prefix = "nsi1", default)]
        pub is_private: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlContact",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlContact {
        #[yaserde(rename = "company", prefix = "nsi1", default)]
        pub company: Option<String>,
        #[yaserde(rename = "description", prefix = "nsi1", default)]
        pub description: Option<String>,
        #[yaserde(rename = "displayName", prefix = "nsi1", default)]
        pub display_name: String,
        #[yaserde(rename = "displayNameAscii", prefix = "nsi1", default)]
        pub display_name_ascii: String,
        #[yaserde(rename = "dn", prefix = "nsi1", default)]
        pub dn: Option<String>,
        #[yaserde(rename = "givenName", prefix = "nsi1", default)]
        pub given_name: String,
        #[yaserde(rename = "givenNameAscii", prefix = "nsi1", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(rename = "initials", prefix = "nsi1", default)]
        pub initials: Option<String>,
        #[yaserde(rename = "middleName", prefix = "nsi1", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "preferredGivenName", prefix = "nsi1", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(rename = "preferredLanguage", prefix = "nsi1", default)]
        pub preferred_language: Option<String>,
        #[yaserde(rename = "isPublic", prefix = "nsi1", default)]
        pub is_public: bool,
        #[yaserde(rename = "source", prefix = "nsi1", default)]
        pub source: String,
        #[yaserde(rename = "sourceUserKey", prefix = "nsi1", default)]
        pub source_user_key: String,
        #[yaserde(rename = "suffix", prefix = "nsi1", default)]
        pub suffix: Option<String>,
        #[yaserde(rename = "surname", prefix = "nsi1", default)]
        pub surname: String,
        #[yaserde(rename = "surnameAscii", prefix = "nsi1", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(rename = "title", prefix = "nsi1", default)]
        pub title: Option<String>,
        #[yaserde(rename = "ContactAddress", prefix = "nsi1", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(rename = "addresses", prefix = "nsi1", default)]
        pub addresses: Vec<XmlAddress>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlHandle",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlHandle {
        #[yaserde(rename = "handleName", prefix = "nsi1", default)]
        pub handle_name: String,
        #[yaserde(rename = "handleType", prefix = "nsi1", default)]
        pub handle_type: String,
        #[yaserde(rename = "handleSubType", prefix = "nsi1", default)]
        pub handle_sub_type: Option<String>,
        #[yaserde(rename = "domainName", prefix = "nsi1", default)]
        pub domain_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlCommProfileType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", prefix = "nsi1", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", prefix = "nsi1", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", prefix = "nsi1", default)]
        pub job_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "handleList",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct HandleList {
        #[yaserde(rename = "handle", prefix = "nsi1", default)]
        pub handle: Vec<XmlHandle>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "commProfileList",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct CommProfileList {
        #[yaserde(rename = "commProfile", prefix = "nsi1", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlCommProfileSetType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlCommProfileSetType {
        #[yaserde(rename = "commProfileSetName", prefix = "nsi1", default)]
        pub comm_profile_set_name: String,
        #[yaserde(rename = "isPrimary", prefix = "nsi1", default)]
        pub is_primary: bool,
        #[yaserde(rename = "handleList", prefix = "nsi1", default)]
        pub handle_list: Option<HandleList>,
        #[yaserde(rename = "commProfileList", prefix = "nsi1", default)]
        pub comm_profile_list: Option<CommProfileList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "ForgeinCommProfileType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct ForgeinCommProfileType {
        #[yaserde(rename = "csEncryptionKeyId", prefix = "nsi1", default)]
        pub cs_encryption_key_id: Option<i64>,
        #[yaserde(rename = "servicePassword", prefix = "nsi1", default)]
        pub service_password: Option<String>,
        #[yaserde(rename = "serviceData", prefix = "nsi1", default)]
        pub service_data: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlSecureStore",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlSecureStore {
        #[yaserde(rename = "secureStoreData", prefix = "nsi1", default)]
        pub secure_store_data: String,
        #[yaserde(rename = "passwordEncrypted", prefix = "nsi1", default)]
        pub password_encrypted: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlLocalizedName",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmlLocalizedName {
        #[yaserde(rename = "locale", prefix = "nsi1", default)]
        pub locale: String,
        #[yaserde(rename = "name", prefix = "nsi1", default)]
        pub name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmLocalizedNames",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct XmLocalizedNames {
        #[yaserde(rename = "localizedName", prefix = "nsi1", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "tenant",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct Tenant {
        #[yaserde(rename = "name", attribute)]
        pub name: String,
        #[yaserde(rename = "createTenantIfNotAlreadyPresent", attribute)]
        pub create_tenant_if_not_already_present: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "UserOrganizationDetailsType",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        prefix = "nsi1"
    )]
    pub struct UserOrganizationDetailsType {
        #[yaserde(rename = "tenant", prefix = "nsi1", default)]
        pub tenant: Tenant,
        #[yaserde(rename = "organizationUnitLevelOne", prefix = "nsi1", default)]
        pub organization_unit_level_one: Option<String>,
        #[yaserde(rename = "organizationUnitLevelTwo", prefix = "nsi1", default)]
        pub organization_unit_level_two: Option<String>,
        #[yaserde(rename = "organizationUnitLevelThree", prefix = "nsi1", default)]
        pub organization_unit_level_three: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlOfficelinxProfile")]
    pub struct XmlOfficelinxProfile {
        #[yaserde(flatten, default)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "officelinxName", default)]
        pub officelinx_name: String,
        #[yaserde(rename = "mailBoxNumber", default)]
        pub mail_box_number: i64,
        #[yaserde(rename = "numericPassword", default)]
        pub numeric_password: Option<String>,
        #[yaserde(rename = "applicationUserPassword", default)]
        pub application_user_password: Option<String>,
        #[yaserde(rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(rename = "department", default)]
        pub department: Option<String>,
        #[yaserde(rename = "featureGroup", default)]
        pub feature_group: Option<String>,
        #[yaserde(rename = "capability", default)]
        pub capability: Option<String>,
        #[yaserde(rename = "domainAccountName", default)]
        pub domain_account_name: Option<String>,
        #[yaserde(rename = "synchronizationUserName", default)]
        pub synchronization_user_name: Option<String>,
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
