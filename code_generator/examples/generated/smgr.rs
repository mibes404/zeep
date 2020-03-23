pub mod messages {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
}

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub mod types {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;

    pub type SecureStore = XmlSecureStore;

    pub type User = XmlUser;

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "users", default)]
    pub struct Users {
        #[yaserde(rename = "secureStore", default)]
        pub secure_store: Vec<XmlSecureStore>,
        #[yaserde(rename = "user", default)]
        pub user: Vec<XmlUser>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "UserProvisionRules", default)]
    pub struct UserProvisionRules {
        #[yaserde(rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "roles", default)]
    pub struct Roles {
        #[yaserde(rename = "role", default)]
        pub role: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ownedContactLists", default)]
    pub struct OwnedContactLists {
        #[yaserde(rename = "contactList", default)]
        pub contact_list: Vec<XmlContactList>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ownedContacts", default)]
    pub struct OwnedContacts {
        #[yaserde(rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlUser", default)]
    pub struct XmlUser {
        #[yaserde(rename = "UserOrganizationDetails", default)]
        pub user_organization_details: Vec<UserOrganizationDetailsType>,
        #[yaserde(rename = "authenticationType", default)]
        pub authentication_type: Vec<String>,
        #[yaserde(rename = "description", default)]
        pub description: String,
        #[yaserde(rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(rename = "dn", default)]
        pub dn: String,
        #[yaserde(rename = "isDuplicatedLoginAllowed", default)]
        pub is_duplicated_login_allowed: bool,
        #[yaserde(rename = "isEnabled", default)]
        pub is_enabled: Vec<bool>,
        #[yaserde(rename = "isVirtualUser", default)]
        pub is_virtual_user: bool,
        #[yaserde(rename = "givenName", default)]
        pub given_name: Vec<String>,
        #[yaserde(rename = "givenNameAscii", default)]
        pub given_name_ascii: Vec<String>,
        #[yaserde(rename = "honorific", default)]
        pub honorific: String,
        #[yaserde(rename = "employeeNo", default)]
        pub employee_no: Vec<String>,
        #[yaserde(rename = "department", default)]
        pub department: Vec<String>,
        #[yaserde(rename = "organization", default)]
        pub organization: Vec<String>,
        #[yaserde(rename = "middleName", default)]
        pub middle_name: String,
        #[yaserde(rename = "managerName", default)]
        pub manager_name: String,
        #[yaserde(rename = "preferredGivenName", default)]
        pub preferred_given_name: String,
        #[yaserde(rename = "preferredLanguage", default)]
        pub preferred_language: String,
        #[yaserde(rename = "source", default)]
        pub source: Vec<String>,
        #[yaserde(rename = "sourceUserKey", default)]
        pub source_user_key: Vec<String>,
        #[yaserde(rename = "status", default)]
        pub status: String,
        #[yaserde(rename = "suffix", default)]
        pub suffix: String,
        #[yaserde(rename = "surname", default)]
        pub surname: Vec<String>,
        #[yaserde(rename = "surnameAscii", default)]
        pub surname_ascii: Vec<String>,
        #[yaserde(rename = "timeZone", default)]
        pub time_zone: String,
        #[yaserde(rename = "title", default)]
        pub title: String,
        #[yaserde(rename = "userName", default)]
        pub user_name: Vec<String>,
        #[yaserde(rename = "userPassword", default)]
        pub user_password: String,
        #[yaserde(rename = "commPassword", default)]
        pub comm_password: String,
        #[yaserde(rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(rename = "localizedNames", default)]
        pub localized_names: Vec<XmLocalizedNames>,
        #[yaserde(rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(rename = "presenceUserDefault", default)]
        pub presence_user_default: XmlPresUserDefaultType,
        #[yaserde(rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Vec<XmlPresUserCLDefaultType>,
        #[yaserde(rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlSecurityIdentity", default)]
    pub struct XmlSecurityIdentity {
        #[yaserde(rename = "identity", default)]
        pub identity: Vec<String>,
        #[yaserde(rename = "realm", default)]
        pub realm: String,
        #[yaserde(rename = "type", default)]
        pub rs_type: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresInfoTypeAccessType", default)]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(rename = "infoType", default)]
        pub info_type: Vec<XmlPresInfoTypeType>,
        #[yaserde(rename = "access", default)]
        pub access: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresACRuleType", default)]
    pub struct XmlPresACRuleType {
        #[yaserde(rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresUserDefaultType", default)]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresUserCLDefaultType", default)]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresUserACLEntryType", default)]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlPresInfoTypeType", default)]
    pub struct XmlPresInfoTypeType {
        #[yaserde(rename = "label", default)]
        pub label: Vec<String>,
        #[yaserde(rename = "filter", default)]
        pub filter: Vec<String>,
        #[yaserde(rename = "specFlags", default)]
        pub spec_flags: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlContactList", default)]
    pub struct XmlContactList {
        #[yaserde(rename = "name", default)]
        pub name: Vec<String>,
        #[yaserde(rename = "description", default)]
        pub description: String,
        #[yaserde(rename = "isPublic", default)]
        pub is_public: Vec<bool>,
        #[yaserde(rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(rename = "contactListType", default)]
        pub contact_list_type: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlContactListMember", default)]
    pub struct XmlContactListMember {
        #[yaserde(rename = "isFavorite", default)]
        pub is_favorite: Vec<bool>,
        #[yaserde(rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(rename = "speedDialEntry", default)]
        pub speed_dial_entry: u64,
        #[yaserde(rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: Vec<bool>,
        #[yaserde(rename = "label", default)]
        pub label: String,
        #[yaserde(rename = "altLabel", default)]
        pub alt_label: String,
        #[yaserde(rename = "description", default)]
        pub description: String,
        #[yaserde(rename = "priorityLevel", default)]
        pub priority_level: u64,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlContactAddress", default)]
    pub struct XmlContactAddress {
        #[yaserde(rename = "address", default)]
        pub address: Vec<String>,
        #[yaserde(rename = "altLabel", default)]
        pub alt_label: String,
        #[yaserde(rename = "contactCategory", default)]
        pub contact_category: Vec<String>,
        #[yaserde(rename = "contactType", default)]
        pub contact_type: Vec<String>,
        #[yaserde(rename = "label", default)]
        pub label: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlAddress", default)]
    pub struct XmlAddress {
        #[yaserde(rename = "addressType", default)]
        pub address_type: Vec<String>,
        #[yaserde(rename = "name", default)]
        pub name: Vec<String>,
        #[yaserde(rename = "building", default)]
        pub building: String,
        #[yaserde(rename = "localityName", default)]
        pub locality_name: String,
        #[yaserde(rename = "postalCode", default)]
        pub postal_code: String,
        #[yaserde(rename = "room", default)]
        pub room: String,
        #[yaserde(rename = "stateOrProvince", default)]
        pub state_or_province: String,
        #[yaserde(rename = "country", default)]
        pub country: String,
        #[yaserde(rename = "street", default)]
        pub street: String,
        #[yaserde(rename = "businessphone", default)]
        pub businessphone: String,
        #[yaserde(rename = "otherbusinessphone", default)]
        pub otherbusinessphone: String,
        #[yaserde(rename = "fax", default)]
        pub fax: String,
        #[yaserde(rename = "homephone", default)]
        pub homephone: String,
        #[yaserde(rename = "otherhomephone", default)]
        pub otherhomephone: String,
        #[yaserde(rename = "mobilephone", default)]
        pub mobilephone: String,
        #[yaserde(rename = "othermobilephone", default)]
        pub othermobilephone: String,
        #[yaserde(rename = "pager", default)]
        pub pager: String,
        #[yaserde(rename = "pager2", default)]
        pub pager_2: String,
        #[yaserde(rename = "isPrivate", default)]
        pub is_private: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlContact", default)]
    pub struct XmlContact {
        #[yaserde(rename = "company", default)]
        pub company: String,
        #[yaserde(rename = "description", default)]
        pub description: String,
        #[yaserde(rename = "displayName", default)]
        pub display_name: Vec<String>,
        #[yaserde(rename = "displayNameAscii", default)]
        pub display_name_ascii: Vec<String>,
        #[yaserde(rename = "dn", default)]
        pub dn: String,
        #[yaserde(rename = "givenName", default)]
        pub given_name: Vec<String>,
        #[yaserde(rename = "givenNameAscii", default)]
        pub given_name_ascii: Vec<String>,
        #[yaserde(rename = "initials", default)]
        pub initials: String,
        #[yaserde(rename = "middleName", default)]
        pub middle_name: String,
        #[yaserde(rename = "preferredGivenName", default)]
        pub preferred_given_name: Vec<String>,
        #[yaserde(rename = "preferredLanguage", default)]
        pub preferred_language: String,
        #[yaserde(rename = "isPublic", default)]
        pub is_public: Vec<bool>,
        #[yaserde(rename = "source", default)]
        pub source: Vec<String>,
        #[yaserde(rename = "sourceUserKey", default)]
        pub source_user_key: Vec<String>,
        #[yaserde(rename = "suffix", default)]
        pub suffix: String,
        #[yaserde(rename = "surname", default)]
        pub surname: Vec<String>,
        #[yaserde(rename = "surnameAscii", default)]
        pub surname_ascii: Vec<String>,
        #[yaserde(rename = "title", default)]
        pub title: String,
        #[yaserde(rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlHandle", default)]
    pub struct XmlHandle {
        #[yaserde(rename = "handleName", default)]
        pub handle_name: Vec<String>,
        #[yaserde(rename = "handleType", default)]
        pub handle_type: Vec<String>,
        #[yaserde(rename = "handleSubType", default)]
        pub handle_sub_type: Vec<String>,
        #[yaserde(rename = "domainName", default)]
        pub domain_name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlCommProfileType", default)]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: Vec<String>,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Vec<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "handleList", default)]
    pub struct HandleList {
        #[yaserde(rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "commProfileList", default)]
    pub struct CommProfileList {
        #[yaserde(rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlCommProfileSetType", default)]
    pub struct XmlCommProfileSetType {
        #[yaserde(rename = "commProfileSetName", default)]
        pub comm_profile_set_name: Vec<String>,
        #[yaserde(rename = "isPrimary", default)]
        pub is_primary: Vec<bool>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "ForgeinCommProfileType", default)]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Vec<u64>,
        #[yaserde(rename = "servicePassword", default)]
        pub service_password: Vec<String>,
        #[yaserde(rename = "serviceData", default)]
        pub service_data: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlSecureStore", default)]
    pub struct XmlSecureStore {
        #[yaserde(rename = "secureStoreData", default)]
        pub secure_store_data: Vec<String>,
        #[yaserde(rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlLocalizedName", default)]
    pub struct XmlLocalizedName {
        #[yaserde(rename = "locale", default)]
        pub locale: Vec<String>,
        #[yaserde(rename = "name", default)]
        pub name: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmLocalizedNames", default)]
    pub struct XmLocalizedNames {
        #[yaserde(rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "tenant", default)]
    pub struct Tenant {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "UserOrganizationDetailsType", default)]
    pub struct UserOrganizationDetailsType {
        #[yaserde(rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: Vec<String>,
        #[yaserde(rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: Vec<String>,
        #[yaserde(rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: Vec<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlAgentProfile", default)]
    pub struct XmlAgentProfile {
        #[yaserde(flatten)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(rename = "cmName", default)]
        pub cm_name: Vec<String>,
        #[yaserde(rename = "useExistingAgent", default)]
        pub use_existing_agent: Vec<bool>,
        #[yaserde(rename = "template", default)]
        pub template: Vec<String>,
        #[yaserde(rename = "aas", default)]
        pub aas: Vec<bool>,
        #[yaserde(rename = "audix", default)]
        pub audix: Vec<bool>,
        #[yaserde(rename = "deleteOnUnassign", default)]
        pub delete_on_unassign: Vec<bool>,
        #[yaserde(rename = "lwcLogExternalCalls", default)]
        pub lwc_log_external_calls: Vec<bool>,
        #[yaserde(rename = "audixNameforMessaging", default)]
        pub audix_namefor_messaging: Vec<String>,
        #[yaserde(rename = "hearsServiceObservingTone", default)]
        pub hears_service_observing_tone: Vec<bool>,
        #[yaserde(rename = "loginIDforISDNSIPDisplay", default)]
        pub login_i_dfor_isdnsip_display: Vec<bool>,
        #[yaserde(rename = "serviceObjective", default)]
        pub service_objective: Vec<bool>,
        #[yaserde(rename = "directAgentCallsFirst", default)]
        pub direct_agent_calls_first: Vec<bool>,
        #[yaserde(rename = "localCallPreference", default)]
        pub local_call_preference: Vec<bool>,
        #[yaserde(rename = "skills", default)]
        pub skills: Vec<XmlAgentLoginIdSkillsData>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(rename = "xmlAgentLoginIdSkillsData", default)]
    pub struct XmlAgentLoginIdSkillsData {
        #[yaserde(rename = "number", default)]
        pub number: Vec<String>,
    }
}

pub mod ports {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
}

pub mod bindings {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
}
