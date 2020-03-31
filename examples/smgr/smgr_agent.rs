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
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "users",
        default
    )]
    pub struct Users {
        #[yaserde(prefix = "nsi1", rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(prefix = "nsi1", rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "UserProvisionRules",
        default
    )]
    pub struct UserProvisionRules {
        #[yaserde(prefix = "nsi1", rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "roles",
        default
    )]
    pub struct Roles {
        #[yaserde(prefix = "nsi1", rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "ownedContactLists",
        default
    )]
    pub struct OwnedContactLists {
        #[yaserde(prefix = "nsi1", rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "ownedContacts",
        default
    )]
    pub struct OwnedContacts {
        #[yaserde(prefix = "nsi1", rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlUser",
        default
    )]
    pub struct XmlUser {
        #[yaserde(prefix = "nsi1", rename = "UserOrganizationDetails", default)]
        pub user_organization_details: Option<UserOrganizationDetailsType>,
        #[yaserde(prefix = "nsi1", rename = "UserProvisionRules", default)]
        pub user_provision_rules: Option<UserProvisionRules>,
        #[yaserde(prefix = "nsi1", rename = "authenticationType", default)]
        pub authentication_type: String,
        #[yaserde(prefix = "nsi1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "displayName", default)]
        pub display_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "displayNameAscii", default)]
        pub display_name_ascii: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "isDuplicatedLoginAllowed", default)]
        pub is_duplicated_login_allowed: Option<bool>,
        #[yaserde(prefix = "nsi1", rename = "isEnabled", default)]
        pub is_enabled: Option<bool>,
        #[yaserde(prefix = "nsi1", rename = "isVirtualUser", default)]
        pub is_virtual_user: Option<bool>,
        #[yaserde(prefix = "nsi1", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "nsi1", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "honorific", default)]
        pub honorific: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "loginName", default)]
        pub login_name: String,
        #[yaserde(prefix = "nsi1", rename = "newLoginName", default)]
        pub new_login_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "employeeNo", default)]
        pub employee_no: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "department", default)]
        pub department: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "organization", default)]
        pub organization: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "managerName", default)]
        pub manager_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "source", default)]
        pub source: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "sourceUserKey", default)]
        pub source_user_key: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "status", default)]
        pub status: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "nsi1", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "timeZone", default)]
        pub time_zone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "userName", default)]
        pub user_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "userPassword", default)]
        pub user_password: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "commPassword", default)]
        pub comm_password: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(prefix = "nsi1", rename = "roles", default)]
        pub roles: Option<Roles>,
        #[yaserde(prefix = "nsi1", rename = "localizedNames", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(prefix = "nsi1", rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(prefix = "nsi1", rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(prefix = "nsi1", rename = "ownedContactLists", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(prefix = "nsi1", rename = "ownedContacts", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(prefix = "nsi1", rename = "presenceUserDefault", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(prefix = "nsi1", rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(prefix = "nsi1", rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(prefix = "nsi1", rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlSecurityIdentity",
        default
    )]
    pub struct XmlSecurityIdentity {
        #[yaserde(prefix = "nsi1", rename = "identity", default)]
        pub identity: String,
        #[yaserde(prefix = "nsi1", rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "type", default)]
        pub rs_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresInfoTypeAccessType",
        default
    )]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(prefix = "nsi1", rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(prefix = "nsi1", rename = "access", default)]
        pub access: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresACRuleType",
        default
    )]
    pub struct XmlPresACRuleType {
        #[yaserde(prefix = "nsi1", rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresUserDefaultType",
        default
    )]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresUserCLDefaultType",
        default
    )]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresUserACLEntryType",
        default
    )]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlPresInfoTypeType",
        default
    )]
    pub struct XmlPresInfoTypeType {
        #[yaserde(prefix = "nsi1", rename = "label", default)]
        pub label: String,
        #[yaserde(prefix = "nsi1", rename = "filter", default)]
        pub filter: String,
        #[yaserde(prefix = "nsi1", rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlContactList",
        default
    )]
    pub struct XmlContactList {
        #[yaserde(prefix = "nsi1", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "nsi1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "nsi1", rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(prefix = "nsi1", rename = "contactListType", default)]
        pub contact_list_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlContactListMember",
        default
    )]
    pub struct XmlContactListMember {
        #[yaserde(prefix = "nsi1", rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(prefix = "nsi1", rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(prefix = "nsi1", rename = "speedDialEntry", default)]
        pub speed_dial_entry: Option<u64>,
        #[yaserde(prefix = "nsi1", rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: bool,
        #[yaserde(prefix = "nsi1", rename = "label", default)]
        pub label: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "priorityLevel", default)]
        pub priority_level: Option<u64>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlContactAddress",
        default
    )]
    pub struct XmlContactAddress {
        #[yaserde(prefix = "nsi1", rename = "address", default)]
        pub address: String,
        #[yaserde(prefix = "nsi1", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "contactCategory", default)]
        pub contact_category: String,
        #[yaserde(prefix = "nsi1", rename = "contactType", default)]
        pub contact_type: String,
        #[yaserde(prefix = "nsi1", rename = "label", default)]
        pub label: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlAddress",
        default
    )]
    pub struct XmlAddress {
        #[yaserde(prefix = "nsi1", rename = "addressType", default)]
        pub address_type: String,
        #[yaserde(prefix = "nsi1", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "nsi1", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "localityName", default)]
        pub locality_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "postalCode", default)]
        pub postal_code: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "country", default)]
        pub country: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "street", default)]
        pub street: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "businessphone", default)]
        pub businessphone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "otherbusinessphone", default)]
        pub otherbusinessphone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "fax", default)]
        pub fax: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "homephone", default)]
        pub homephone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "otherhomephone", default)]
        pub otherhomephone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "mobilephone", default)]
        pub mobilephone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "othermobilephone", default)]
        pub othermobilephone: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "pager", default)]
        pub pager: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "pager2", default)]
        pub pager_2: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "postalAddress", default)]
        pub postal_address: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "isPrivate", default)]
        pub is_private: Option<bool>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlContact",
        default
    )]
    pub struct XmlContact {
        #[yaserde(prefix = "nsi1", rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(prefix = "nsi1", rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(prefix = "nsi1", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "nsi1", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "initials", default)]
        pub initials: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "nsi1", rename = "source", default)]
        pub source: String,
        #[yaserde(prefix = "nsi1", rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(prefix = "nsi1", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "nsi1", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(prefix = "nsi1", rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlHandle",
        default
    )]
    pub struct XmlHandle {
        #[yaserde(prefix = "nsi1", rename = "handleName", default)]
        pub handle_name: String,
        #[yaserde(prefix = "nsi1", rename = "handleType", default)]
        pub handle_type: String,
        #[yaserde(prefix = "nsi1", rename = "handleSubType", default)]
        pub handle_sub_type: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "domainName", default)]
        pub domain_name: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlCommProfileType",
        default
    )]
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "nsi1", rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(prefix = "nsi1", rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "jobId", default)]
        pub job_id: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "handleList",
        default
    )]
    pub struct HandleList {
        #[yaserde(prefix = "nsi1", rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "commProfileList",
        default
    )]
    pub struct CommProfileList {
        #[yaserde(prefix = "nsi1", rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlCommProfileSetType",
        default
    )]
    pub struct XmlCommProfileSetType {
        #[yaserde(prefix = "nsi1", rename = "commProfileSetName", default)]
        pub comm_profile_set_name: String,
        #[yaserde(prefix = "nsi1", rename = "isPrimary", default)]
        pub is_primary: bool,
        #[yaserde(prefix = "nsi1", rename = "handleList", default)]
        pub handle_list: Option<HandleList>,
        #[yaserde(prefix = "nsi1", rename = "commProfileList", default)]
        pub comm_profile_list: Option<CommProfileList>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "ForgeinCommProfileType",
        default
    )]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "nsi1", rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<u64>,
        #[yaserde(prefix = "nsi1", rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "serviceData", default)]
        pub service_data: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlSecureStore",
        default
    )]
    pub struct XmlSecureStore {
        #[yaserde(prefix = "nsi1", rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(prefix = "nsi1", rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmlLocalizedName",
        default
    )]
    pub struct XmlLocalizedName {
        #[yaserde(prefix = "nsi1", rename = "locale", default)]
        pub locale: String,
        #[yaserde(prefix = "nsi1", rename = "name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "xmLocalizedNames",
        default
    )]
    pub struct XmLocalizedNames {
        #[yaserde(prefix = "nsi1", rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "tenant",
        default
    )]
    pub struct Tenant {
        #[yaserde(rename = "name", attribute)]
        pub name: String,
        #[yaserde(rename = "createTenantIfNotAlreadyPresent", attribute)]
        pub create_tenant_if_not_already_present: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "nsi1",
        namespace = "nsi1: http://xml.avaya.com/schema/import",
        root = "UserOrganizationDetailsType",
        default
    )]
    pub struct UserOrganizationDetailsType {
        #[yaserde(prefix = "nsi1", rename = "tenant", default)]
        pub tenant: Tenant,
        #[yaserde(prefix = "nsi1", rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: Option<String>,
        #[yaserde(prefix = "nsi1", rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import_csm_agent",
        root = "xmlAgentProfile",
        default
    )]
    pub struct XmlAgentProfile {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "ns1", rename = "cmName", default)]
        pub cm_name: String,
        #[yaserde(prefix = "ns1", rename = "useExistingAgent", default)]
        pub use_existing_agent: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "loginIdExtension", default)]
        pub login_id_extension: String,
        #[yaserde(prefix = "ns1", rename = "template", default)]
        pub template: Option<String>,
        #[yaserde(prefix = "ns1", rename = "securityCode", default)]
        pub security_code: Option<String>,
        #[yaserde(prefix = "ns1", rename = "aas", default)]
        pub aas: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "audix", default)]
        pub audix: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "password", default)]
        pub password: Option<String>,
        #[yaserde(prefix = "ns1", rename = "portExtension", default)]
        pub port_extension: Option<String>,
        #[yaserde(prefix = "ns1", rename = "deleteOnUnassign", default)]
        pub delete_on_unassign: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "tn", default)]
        pub tn: Option<u64>,
        #[yaserde(prefix = "ns1", rename = "cor", default)]
        pub cor: Option<u64>,
        #[yaserde(prefix = "ns1", rename = "coveragePath", default)]
        pub coverage_path: Option<String>,
        #[yaserde(prefix = "ns1", rename = "lwcReception", default)]
        pub lwc_reception: Option<String>,
        #[yaserde(prefix = "ns1", rename = "lwcLogExternalCalls", default)]
        pub lwc_log_external_calls: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "audixNameforMessaging", default)]
        pub audix_namefor_messaging: Option<String>,
        #[yaserde(prefix = "ns1", rename = "hearsServiceObservingTone", default)]
        pub hears_service_observing_tone: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "loginIDforISDNSIPDisplay", default)]
        pub login_i_dfor_isdnsip_display: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "autoAnswer", default)]
        pub auto_answer: Option<String>,
        #[yaserde(prefix = "ns1", rename = "miaAcrossSkills", default)]
        pub mia_across_skills: Option<String>,
        #[yaserde(prefix = "ns1", rename = "acwAgentConsideredIdle", default)]
        pub acw_agent_considered_idle: Option<String>,
        #[yaserde(prefix = "ns1", rename = "auxWorkReasonCodeType", default)]
        pub aux_work_reason_code_type: Option<String>,
        #[yaserde(prefix = "ns1", rename = "logoutReasonCodeType", default)]
        pub logout_reason_code_type: Option<String>,
        #[yaserde(
            prefix = "ns1",
            rename = "maximumTimeAgentInAcwBeforeLogoutSec",
            default
        )]
        pub maximum_time_agent_in_acw_before_logout_sec: Option<String>,
        #[yaserde(prefix = "ns1", rename = "forcedAgentLogoutTimeHr", default)]
        pub forced_agent_logout_time_hr: Option<String>,
        #[yaserde(prefix = "ns1", rename = "forcedAgentLogoutTimeSec", default)]
        pub forced_agent_logout_time_sec: Option<String>,
        #[yaserde(prefix = "ns1", rename = "directAgentSkill", default)]
        pub direct_agent_skill: Option<String>,
        #[yaserde(prefix = "ns1", rename = "callHandlingPreference", default)]
        pub call_handling_preference: Option<String>,
        #[yaserde(prefix = "ns1", rename = "serviceObjective", default)]
        pub service_objective: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "directAgentCallsFirst", default)]
        pub direct_agent_calls_first: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "localCallPreference", default)]
        pub local_call_preference: Option<bool>,
        #[yaserde(prefix = "ns1", rename = "skills", default)]
        pub skills: Vec<XmlAgentLoginIdSkillsData>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "ns1",
        namespace = "ns1: http://xml.avaya.com/schema/import_csm_agent",
        root = "xmlAgentLoginIdSkillsData",
        default
    )]
    pub struct XmlAgentLoginIdSkillsData {
        #[yaserde(prefix = "ns1", rename = "number", default)]
        pub number: String,
        #[yaserde(prefix = "ns1", rename = "skillNumber", default)]
        pub skill_number: String,
        #[yaserde(prefix = "ns1", rename = "reserveLevel", default)]
        pub reserve_level: Option<String>,
        #[yaserde(prefix = "ns1", rename = "skillLevel", default)]
        pub skill_level: Option<String>,
        #[yaserde(prefix = "ns1", rename = "percentAllocation", default)]
        pub percent_allocation: Option<String>,
    }
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

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}
