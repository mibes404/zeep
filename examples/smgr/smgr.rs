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
pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
    use super::*;
    use crate::smgr_presence::types::XmlPsCommProfile;
    use crate::smgr_sm::types::SessionManagerCommProfXML;
    use crate::smgr_station::types::XmlStationProfile;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    pub type SecureStore = XmlSecureStore;

    pub type User = XmlUser;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "users",
        default
    )]
    pub struct Users {
        #[yaserde(prefix = "tns", rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(prefix = "tns", rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "UserProvisionRules",
        default
    )]
    pub struct UserProvisionRules {
        #[yaserde(prefix = "tns", rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "roles",
        default
    )]
    pub struct Roles {
        #[yaserde(prefix = "tns", rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "ownedContactLists",
        default
    )]
    pub struct OwnedContactLists {
        #[yaserde(prefix = "tns", rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "ownedContacts",
        default
    )]
    pub struct OwnedContacts {
        #[yaserde(prefix = "tns", rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlUser",
        default
    )]
    pub struct XmlUser {
        #[yaserde(prefix = "tns", rename = "UserOrganizationDetails", default)]
        pub user_organization_details: Option<UserOrganizationDetailsType>,
        #[yaserde(prefix = "tns", rename = "UserProvisionRules", default)]
        pub user_provision_rules: Option<UserProvisionRules>,
        #[yaserde(prefix = "tns", rename = "authenticationType", default)]
        pub authentication_type: String,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayName", default)]
        pub display_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayNameAscii", default)]
        pub display_name_ascii: Option<String>,
        #[yaserde(prefix = "tns", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "tns", rename = "isDuplicatedLoginAllowed", default)]
        pub is_duplicated_login_allowed: Option<bool>,
        #[yaserde(prefix = "tns", rename = "isEnabled", default)]
        pub is_enabled: Option<bool>,
        #[yaserde(prefix = "tns", rename = "isVirtualUser", default)]
        pub is_virtual_user: Option<bool>,
        #[yaserde(prefix = "tns", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "tns", rename = "honorific", default)]
        pub honorific: Option<String>,
        #[yaserde(prefix = "tns", rename = "loginName", default)]
        pub login_name: String,
        #[yaserde(prefix = "tns", rename = "newLoginName", default)]
        pub new_login_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "employeeNo", default)]
        pub employee_no: Option<String>,
        #[yaserde(prefix = "tns", rename = "department", default)]
        pub department: Option<String>,
        #[yaserde(prefix = "tns", rename = "organization", default)]
        pub organization: Option<String>,
        #[yaserde(prefix = "tns", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "managerName", default)]
        pub manager_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "tns", rename = "source", default)]
        pub source: Option<String>,
        #[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
        pub source_user_key: Option<String>,
        #[yaserde(prefix = "tns", rename = "status", default)]
        pub status: Option<String>,
        #[yaserde(prefix = "tns", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "tns", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "tns", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "tns", rename = "timeZone", default)]
        pub time_zone: Option<String>,
        #[yaserde(prefix = "tns", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "tns", rename = "userName", default)]
        pub user_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "userPassword", default)]
        pub user_password: Option<String>,
        #[yaserde(prefix = "tns", rename = "commPassword", default)]
        pub comm_password: Option<String>,
        #[yaserde(prefix = "tns", rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(prefix = "tns", rename = "roles", default)]
        pub roles: Option<Roles>,
        #[yaserde(prefix = "tns", rename = "localizedNames", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(prefix = "tns", rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(prefix = "tns", rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(prefix = "tns", rename = "ownedContactLists", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(prefix = "tns", rename = "ownedContacts", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(prefix = "tns", rename = "presenceUserDefault", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(prefix = "tns", rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(prefix = "tns", rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(prefix = "tns", rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlSecurityIdentity",
        default
    )]
    pub struct XmlSecurityIdentity {
        #[yaserde(prefix = "tns", rename = "identity", default)]
        pub identity: String,
        #[yaserde(prefix = "tns", rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(prefix = "tns", rename = "type", default)]
        pub rs_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresInfoTypeAccessType",
        default
    )]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(prefix = "tns", rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(prefix = "tns", rename = "access", default)]
        pub access: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresACRuleType",
        default
    )]
    pub struct XmlPresACRuleType {
        #[yaserde(prefix = "tns", rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresUserDefaultType",
        default
    )]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresUserCLDefaultType",
        default
    )]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresUserACLEntryType",
        default
    )]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlPresInfoTypeType",
        default
    )]
    pub struct XmlPresInfoTypeType {
        #[yaserde(prefix = "tns", rename = "label", default)]
        pub label: String,
        #[yaserde(prefix = "tns", rename = "filter", default)]
        pub filter: String,
        #[yaserde(prefix = "tns", rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlContactList",
        default
    )]
    pub struct XmlContactList {
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "tns", rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(prefix = "tns", rename = "contactListType", default)]
        pub contact_list_type: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlContactListMember",
        default
    )]
    pub struct XmlContactListMember {
        #[yaserde(prefix = "tns", rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(prefix = "tns", rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(prefix = "tns", rename = "speedDialEntry", default)]
        pub speed_dial_entry: Option<u64>,
        #[yaserde(prefix = "tns", rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: bool,
        #[yaserde(prefix = "tns", rename = "label", default)]
        pub label: Option<String>,
        #[yaserde(prefix = "tns", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "priorityLevel", default)]
        pub priority_level: Option<u64>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlContactAddress",
        default
    )]
    pub struct XmlContactAddress {
        #[yaserde(prefix = "tns", rename = "address", default)]
        pub address: String,
        #[yaserde(prefix = "tns", rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(prefix = "tns", rename = "contactCategory", default)]
        pub contact_category: String,
        #[yaserde(prefix = "tns", rename = "contactType", default)]
        pub contact_type: String,
        #[yaserde(prefix = "tns", rename = "label", default)]
        pub label: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlAddress",
        default
    )]
    pub struct XmlAddress {
        #[yaserde(prefix = "tns", rename = "addressType", default)]
        pub address_type: String,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
        #[yaserde(prefix = "tns", rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(prefix = "tns", rename = "localityName", default)]
        pub locality_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "postalCode", default)]
        pub postal_code: Option<String>,
        #[yaserde(prefix = "tns", rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(prefix = "tns", rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(prefix = "tns", rename = "country", default)]
        pub country: Option<String>,
        #[yaserde(prefix = "tns", rename = "street", default)]
        pub street: Option<String>,
        #[yaserde(prefix = "tns", rename = "businessphone", default)]
        pub businessphone: Option<String>,
        #[yaserde(prefix = "tns", rename = "otherbusinessphone", default)]
        pub otherbusinessphone: Option<String>,
        #[yaserde(prefix = "tns", rename = "fax", default)]
        pub fax: Option<String>,
        #[yaserde(prefix = "tns", rename = "homephone", default)]
        pub homephone: Option<String>,
        #[yaserde(prefix = "tns", rename = "otherhomephone", default)]
        pub otherhomephone: Option<String>,
        #[yaserde(prefix = "tns", rename = "mobilephone", default)]
        pub mobilephone: Option<String>,
        #[yaserde(prefix = "tns", rename = "othermobilephone", default)]
        pub othermobilephone: Option<String>,
        #[yaserde(prefix = "tns", rename = "pager", default)]
        pub pager: Option<String>,
        #[yaserde(prefix = "tns", rename = "pager2", default)]
        pub pager_2: Option<String>,
        #[yaserde(prefix = "tns", rename = "postalAddress", default)]
        pub postal_address: Option<String>,
        #[yaserde(prefix = "tns", rename = "isPrivate", default)]
        pub is_private: Option<bool>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlContact",
        default
    )]
    pub struct XmlContact {
        #[yaserde(prefix = "tns", rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(prefix = "tns", rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(prefix = "tns", rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(prefix = "tns", rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(prefix = "tns", rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(prefix = "tns", rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(prefix = "tns", rename = "initials", default)]
        pub initials: Option<String>,
        #[yaserde(prefix = "tns", rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(prefix = "tns", rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(prefix = "tns", rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(prefix = "tns", rename = "source", default)]
        pub source: String,
        #[yaserde(prefix = "tns", rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(prefix = "tns", rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(prefix = "tns", rename = "surname", default)]
        pub surname: String,
        #[yaserde(prefix = "tns", rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(prefix = "tns", rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(prefix = "tns", rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(prefix = "tns", rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlHandle",
        default
    )]
    pub struct XmlHandle {
        #[yaserde(prefix = "tns", rename = "handleName", default)]
        pub handle_name: String,
        #[yaserde(prefix = "tns", rename = "handleType", default)]
        pub handle_type: String,
        #[yaserde(prefix = "tns", rename = "handleSubType", default)]
        pub handle_sub_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "domainName", default)]
        pub domain_name: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlCommProfileType",
        namespace = "ns2: http://xml.avaya.com/schema/import_csm_cm",
        namespace = "ns3: http://xml.avaya.com/schema/presence",
        namespace = "ns7: http://xml.avaya.com/schema/import_sessionmanager",
        default
    )]
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "tns", rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(prefix = "tns", rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(prefix = "tns", rename = "jobId", default)]
        pub job_id: Option<String>,
        #[yaserde(flatten)]
        pub station: Option<XmlStationProfile>,
        #[yaserde(flatten)]
        pub ps: Option<XmlPsCommProfile>,
        #[yaserde(flatten)]
        pub sm: Option<SessionManagerCommProfXML>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "handleList",
        default
    )]
    pub struct HandleList {
        #[yaserde(prefix = "tns", rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "commProfileList",
        default
    )]
    pub struct CommProfileList {
        #[yaserde(prefix = "tns", rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlCommProfileSetType",
        default
    )]
    pub struct XmlCommProfileSetType {
        #[yaserde(prefix = "tns", rename = "commProfileSetName", default)]
        pub comm_profile_set_name: String,
        #[yaserde(prefix = "tns", rename = "isPrimary", default)]
        pub is_primary: bool,
        #[yaserde(prefix = "tns", rename = "handleList", default)]
        pub handle_list: Option<HandleList>,
        #[yaserde(prefix = "tns", rename = "commProfileList", default)]
        pub comm_profile_list: Option<CommProfileList>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "ForgeinCommProfileType",
        default
    )]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "tns", rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<u64>,
        #[yaserde(prefix = "tns", rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(prefix = "tns", rename = "serviceData", default)]
        pub service_data: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlSecureStore",
        default
    )]
    pub struct XmlSecureStore {
        #[yaserde(prefix = "tns", rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(prefix = "tns", rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmlLocalizedName",
        default
    )]
    pub struct XmlLocalizedName {
        #[yaserde(prefix = "tns", rename = "locale", default)]
        pub locale: String,
        #[yaserde(prefix = "tns", rename = "name", default)]
        pub name: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "xmLocalizedNames",
        default
    )]
    pub struct XmLocalizedNames {
        #[yaserde(prefix = "tns", rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
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
        prefix = "tns",
        namespace = "tns: http://xml.avaya.com/schema/import",
        root = "UserOrganizationDetailsType",
        default
    )]
    pub struct UserOrganizationDetailsType {
        #[yaserde(prefix = "tns", rename = "tenant", default)]
        pub tenant: Tenant,
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: Option<String>,
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: Option<String>,
        #[yaserde(prefix = "tns", rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: Option<String>,
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
