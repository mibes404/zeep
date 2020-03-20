use serde::{Deserialize, Serialize};

pub type SecureStore = XmlSecureStore;

pub type User = XmlUser;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "users", default)]
pub struct Users {
    #[serde(rename = "secureStore", default)]
    pub secure_store: Vec<XmlSecureStore>,
    #[serde(rename = "user", default)]
    pub user: Vec<XmlUser>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "UserProvisionRules", default)]
pub struct UserProvisionRules {
    #[serde(rename = "UserProvisionRuleName", default)]
    pub user_provision_rule_name: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "roles", default)]
pub struct Roles {
    #[serde(rename = "role", default)]
    pub role: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "ownedContactLists", default)]
pub struct OwnedContactLists {
    #[serde(rename = "contactList", default)]
    pub contact_list: Vec<XmlContactList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "ownedContacts", default)]
pub struct OwnedContacts {
    #[serde(rename = "contact", default)]
    pub contact: Vec<XmlContact>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlUser", default)]
pub struct XmlUser {
    #[serde(rename = "UserOrganizationDetails", default)]
    pub user_organization_details: Vec<UserOrganizationDetailsType>,
    #[serde(rename = "authenticationType", default)]
    pub authentication_type: Vec<String>,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "displayName", default)]
    pub display_name: String,
    #[serde(rename = "displayNameAscii", default)]
    pub display_name_ascii: String,
    #[serde(rename = "dn", default)]
    pub dn: String,
    #[serde(rename = "isDuplicatedLoginAllowed", default)]
    pub is_duplicated_login_allowed: bool,
    #[serde(rename = "isEnabled", default)]
    pub is_enabled: Vec<bool>,
    #[serde(rename = "isVirtualUser", default)]
    pub is_virtual_user: bool,
    #[serde(rename = "givenName", default)]
    pub given_name: Vec<String>,
    #[serde(rename = "givenNameAscii", default)]
    pub given_name_ascii: Vec<String>,
    #[serde(rename = "honorific", default)]
    pub honorific: String,
    #[serde(rename = "employeeNo", default)]
    pub employee_no: Vec<String>,
    #[serde(rename = "department", default)]
    pub department: Vec<String>,
    #[serde(rename = "organization", default)]
    pub organization: Vec<String>,
    #[serde(rename = "middleName", default)]
    pub middle_name: String,
    #[serde(rename = "managerName", default)]
    pub manager_name: String,
    #[serde(rename = "preferredGivenName", default)]
    pub preferred_given_name: String,
    #[serde(rename = "preferredLanguage", default)]
    pub preferred_language: String,
    #[serde(rename = "source", default)]
    pub source: Vec<String>,
    #[serde(rename = "sourceUserKey", default)]
    pub source_user_key: Vec<String>,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "suffix", default)]
    pub suffix: String,
    #[serde(rename = "surname", default)]
    pub surname: Vec<String>,
    #[serde(rename = "surnameAscii", default)]
    pub surname_ascii: Vec<String>,
    #[serde(rename = "timeZone", default)]
    pub time_zone: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "userName", default)]
    pub user_name: Vec<String>,
    #[serde(rename = "userPassword", default)]
    pub user_password: String,
    #[serde(rename = "commPassword", default)]
    pub comm_password: String,
    #[serde(rename = "userType", default)]
    pub user_type: Vec<String>,
    #[serde(rename = "localizedNames", default)]
    pub localized_names: Vec<XmLocalizedNames>,
    #[serde(rename = "address", default)]
    pub address: Vec<XmlAddress>,
    #[serde(rename = "securityIdentity", default)]
    pub security_identity: Vec<XmlSecurityIdentity>,
    #[serde(rename = "presenceUserDefault", default)]
    pub presence_user_default: XmlPresUserDefaultType,
    #[serde(rename = "presenceUserACL", default)]
    pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
    #[serde(rename = "presenceUserCLDefault", default)]
    pub presence_user_cl_default: Vec<XmlPresUserCLDefaultType>,
    #[serde(rename = "commProfileSet", default)]
    pub comm_profile_set: Vec<XmlCommProfileSetType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlSecurityIdentity", default)]
pub struct XmlSecurityIdentity {
    #[serde(rename = "identity", default)]
    pub identity: Vec<String>,
    #[serde(rename = "realm", default)]
    pub realm: String,
    #[serde(rename = "type", default)]
    pub rs_type: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresInfoTypeAccessType", default)]
pub struct XmlPresInfoTypeAccessType {
    #[serde(rename = "infoType", default)]
    pub info_type: Vec<XmlPresInfoTypeType>,
    #[serde(rename = "access", default)]
    pub access: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresACRuleType", default)]
pub struct XmlPresACRuleType {
    #[serde(rename = "infoTypeAccess", default)]
    pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresUserDefaultType", default)]
pub struct XmlPresUserDefaultType {
    #[serde(flatten)]
    pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresUserCLDefaultType", default)]
pub struct XmlPresUserCLDefaultType {
    #[serde(flatten)]
    pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresUserACLEntryType", default)]
pub struct XmlPresUserACLEntryType {
    #[serde(flatten)]
    pub xml_pres_ac_rule_type: XmlPresACRuleType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlPresInfoTypeType", default)]
pub struct XmlPresInfoTypeType {
    #[serde(rename = "label", default)]
    pub label: Vec<String>,
    #[serde(rename = "filter", default)]
    pub filter: Vec<String>,
    #[serde(rename = "specFlags", default)]
    pub spec_flags: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlContactList", default)]
pub struct XmlContactList {
    #[serde(rename = "name", default)]
    pub name: Vec<String>,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "isPublic", default)]
    pub is_public: Vec<bool>,
    #[serde(rename = "members", default)]
    pub members: Vec<XmlContactListMember>,
    #[serde(rename = "contactListType", default)]
    pub contact_list_type: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlContactListMember", default)]
pub struct XmlContactListMember {
    #[serde(rename = "isFavorite", default)]
    pub is_favorite: Vec<bool>,
    #[serde(rename = "isSpeedDial", default)]
    pub is_speed_dial: bool,
    #[serde(rename = "speedDialEntry", default)]
    pub speed_dial_entry: u64,
    #[serde(rename = "isPresenceBuddy", default)]
    pub is_presence_buddy: Vec<bool>,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "altLabel", default)]
    pub alt_label: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "priorityLevel", default)]
    pub priority_level: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlContactAddress", default)]
pub struct XmlContactAddress {
    #[serde(rename = "address", default)]
    pub address: Vec<String>,
    #[serde(rename = "altLabel", default)]
    pub alt_label: String,
    #[serde(rename = "contactCategory", default)]
    pub contact_category: Vec<String>,
    #[serde(rename = "contactType", default)]
    pub contact_type: Vec<String>,
    #[serde(rename = "label", default)]
    pub label: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlAddress", default)]
pub struct XmlAddress {
    #[serde(rename = "addressType", default)]
    pub address_type: Vec<String>,
    #[serde(rename = "name", default)]
    pub name: Vec<String>,
    #[serde(rename = "building", default)]
    pub building: String,
    #[serde(rename = "localityName", default)]
    pub locality_name: String,
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    #[serde(rename = "room", default)]
    pub room: String,
    #[serde(rename = "stateOrProvince", default)]
    pub state_or_province: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "street", default)]
    pub street: String,
    #[serde(rename = "businessphone", default)]
    pub businessphone: String,
    #[serde(rename = "otherbusinessphone", default)]
    pub otherbusinessphone: String,
    #[serde(rename = "fax", default)]
    pub fax: String,
    #[serde(rename = "homephone", default)]
    pub homephone: String,
    #[serde(rename = "otherhomephone", default)]
    pub otherhomephone: String,
    #[serde(rename = "mobilephone", default)]
    pub mobilephone: String,
    #[serde(rename = "othermobilephone", default)]
    pub othermobilephone: String,
    #[serde(rename = "pager", default)]
    pub pager: String,
    #[serde(rename = "pager2", default)]
    pub pager_2: String,
    #[serde(rename = "isPrivate", default)]
    pub is_private: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlContact", default)]
pub struct XmlContact {
    #[serde(rename = "company", default)]
    pub company: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "displayName", default)]
    pub display_name: Vec<String>,
    #[serde(rename = "displayNameAscii", default)]
    pub display_name_ascii: Vec<String>,
    #[serde(rename = "dn", default)]
    pub dn: String,
    #[serde(rename = "givenName", default)]
    pub given_name: Vec<String>,
    #[serde(rename = "givenNameAscii", default)]
    pub given_name_ascii: Vec<String>,
    #[serde(rename = "initials", default)]
    pub initials: String,
    #[serde(rename = "middleName", default)]
    pub middle_name: String,
    #[serde(rename = "preferredGivenName", default)]
    pub preferred_given_name: Vec<String>,
    #[serde(rename = "preferredLanguage", default)]
    pub preferred_language: String,
    #[serde(rename = "isPublic", default)]
    pub is_public: Vec<bool>,
    #[serde(rename = "source", default)]
    pub source: Vec<String>,
    #[serde(rename = "sourceUserKey", default)]
    pub source_user_key: Vec<String>,
    #[serde(rename = "suffix", default)]
    pub suffix: String,
    #[serde(rename = "surname", default)]
    pub surname: Vec<String>,
    #[serde(rename = "surnameAscii", default)]
    pub surname_ascii: Vec<String>,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "ContactAddress", default)]
    pub contact_address: Vec<XmlContactAddress>,
    #[serde(rename = "addresses", default)]
    pub addresses: Vec<XmlAddress>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlHandle", default)]
pub struct XmlHandle {
    #[serde(rename = "handleName", default)]
    pub handle_name: Vec<String>,
    #[serde(rename = "handleType", default)]
    pub handle_type: Vec<String>,
    #[serde(rename = "handleSubType", default)]
    pub handle_sub_type: Vec<String>,
    #[serde(rename = "domainName", default)]
    pub domain_name: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlCommProfileType", default)]
pub struct XmlCommProfileType {
    #[serde(rename = "commProfileType", default)]
    pub comm_profile_type: Vec<String>,
    #[serde(rename = "commProfileSubType", default)]
    pub comm_profile_sub_type: Vec<String>,
    #[serde(rename = "jobId", default)]
    pub job_id: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "handleList", default)]
pub struct HandleList {
    #[serde(rename = "handle", default)]
    pub handle: Vec<XmlHandle>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "commProfileList", default)]
pub struct CommProfileList {
    #[serde(rename = "commProfile", default)]
    pub comm_profile: Vec<XmlCommProfileType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlCommProfileSetType", default)]
pub struct XmlCommProfileSetType {
    #[serde(rename = "commProfileSetName", default)]
    pub comm_profile_set_name: Vec<String>,
    #[serde(rename = "isPrimary", default)]
    pub is_primary: Vec<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "ForgeinCommProfileType", default)]
pub struct ForgeinCommProfileType {
    #[serde(flatten)]
    pub xml_comm_profile_type: XmlCommProfileType,
    #[serde(rename = "csEncryptionKeyId", default)]
    pub cs_encryption_key_id: Vec<u64>,
    #[serde(rename = "servicePassword", default)]
    pub service_password: Vec<String>,
    #[serde(rename = "serviceData", default)]
    pub service_data: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlSecureStore", default)]
pub struct XmlSecureStore {
    #[serde(rename = "secureStoreData", default)]
    pub secure_store_data: Vec<String>,
    #[serde(rename = "passwordEncrypted", default)]
    pub password_encrypted: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlLocalizedName", default)]
pub struct XmlLocalizedName {
    #[serde(rename = "locale", default)]
    pub locale: Vec<String>,
    #[serde(rename = "name", default)]
    pub name: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmLocalizedNames", default)]
pub struct XmLocalizedNames {
    #[serde(rename = "localizedName", default)]
    pub localized_name: Vec<XmlLocalizedName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "tenant", default)]
pub struct Tenant {}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "UserOrganizationDetailsType", default)]
pub struct UserOrganizationDetailsType {
    #[serde(rename = "organizationUnitLevelOne", default)]
    pub organization_unit_level_one: Vec<String>,
    #[serde(rename = "organizationUnitLevelTwo", default)]
    pub organization_unit_level_two: Vec<String>,
    #[serde(rename = "organizationUnitLevelThree", default)]
    pub organization_unit_level_three: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlAgentProfile", default)]
pub struct XmlAgentProfile {
    #[serde(flatten)]
    pub xml_comm_profile_type: XmlCommProfileType,
    #[serde(rename = "cmName", default)]
    pub cm_name: Vec<String>,
    #[serde(rename = "useExistingAgent", default)]
    pub use_existing_agent: Vec<bool>,
    #[serde(rename = "template", default)]
    pub template: Vec<String>,
    #[serde(rename = "aas", default)]
    pub aas: Vec<bool>,
    #[serde(rename = "audix", default)]
    pub audix: Vec<bool>,
    #[serde(rename = "deleteOnUnassign", default)]
    pub delete_on_unassign: Vec<bool>,
    #[serde(rename = "lwcLogExternalCalls", default)]
    pub lwc_log_external_calls: Vec<bool>,
    #[serde(rename = "audixNameforMessaging", default)]
    pub audix_namefor_messaging: Vec<String>,
    #[serde(rename = "hearsServiceObservingTone", default)]
    pub hears_service_observing_tone: Vec<bool>,
    #[serde(rename = "loginIDforISDNSIPDisplay", default)]
    pub login_i_dfor_isdnsip_display: Vec<bool>,
    #[serde(rename = "serviceObjective", default)]
    pub service_objective: Vec<bool>,
    #[serde(rename = "directAgentCallsFirst", default)]
    pub direct_agent_calls_first: Vec<bool>,
    #[serde(rename = "localCallPreference", default)]
    pub local_call_preference: Vec<bool>,
    #[serde(rename = "skills", default)]
    pub skills: Vec<XmlAgentLoginIdSkillsData>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "xmlAgentLoginIdSkillsData", default)]
pub struct XmlAgentLoginIdSkillsData {
    #[serde(rename = "number", default)]
    pub number: Vec<String>,
}
