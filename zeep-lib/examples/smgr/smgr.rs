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
    use crate::smgr_agent::types::XmlAgentProfile;
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
        rename = "users",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        namespace = "agent: http://xml.avaya.com/schema/import_csm_agent",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        namespace = "ps: http://xml.avaya.com/schema/presence",
        namespace = "asm: http://xml.avaya.com/schema/import_sessionmanager",
        namespace = "ol: http://xml.avaya.com/schema/import_mem_officelinx",
        namespace = "delta: http://xml.avaya.com/schema/deltaImport",
        prefix = "tns"
    )]
    pub struct Users {
        #[yaserde(rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(rename = "user", prefix = "tns", default)]
        pub user: Vec<XmlUser>,
        #[yaserde(attribute, prefix = "xsi", rename = "schemaLocation")]
        pub schema_location: String,
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
    #[yaserde(
        rename = "xmlCommProfileType",
        namespace = "agent: http://xml.avaya.com/schema/import_csm_agent",
        namespace = "csm: http://xml.avaya.com/schema/import_csm_cm",
        namespace = "ps: http://xml.avaya.com/schema/presence",
        namespace = "asm: http://xml.avaya.com/schema/import_sessionmanager",
        namespace = "ol: http://xml.avaya.com/schema/import_mem_officelinx"
    )]
    pub struct XmlCommProfileType {
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Option<String>,
        #[yaserde(flatten)]
        pub station: Option<XmlStationProfile>,
        #[yaserde(flatten)]
        pub ps: Option<XmlPsCommProfile>,
        #[yaserde(flatten)]
        pub sm: Option<SessionManagerCommProfXML>,
        #[yaserde(flatten)]
        pub agent: Option<XmlAgentProfile>,
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
