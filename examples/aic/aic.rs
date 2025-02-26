//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.2.0
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::{
    io::{Read, Write},
    rc::Rc,
};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_71 {
    use super::*;
    use restrictions::CheckRestrictions;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Get")]
    pub struct Get {
        #[yaserde(prefix = "71", rename = "loginId")]
        pub login_id: String,
    }
    impl restrictions::CheckRestrictions for Get {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.login_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "GetResponse")]
    pub struct GetResponse {
        #[yaserde(prefix = "71", rename = "GetReturn")]
        pub get_return: mod_71::Agent,
    }
    impl restrictions::CheckRestrictions for GetResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAdvocateInfo")]
    pub struct AgentAdvocateInfo {
        #[yaserde(prefix = "71", rename = "LRMID")]
        pub lrmid: String,
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "telephonyLinkGroup")]
        pub telephony_link_group: String,
    }
    impl restrictions::CheckRestrictions for AgentAdvocateInfo {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lrmid.check_restrictions(restrictions.clone())?;
            self.enabled.check_restrictions(restrictions.clone())?;
            self.telephony_link_group.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "ArrayOf_xsd_string")]
    pub struct ArrayOfXsdString {
        #[yaserde(prefix = "71", rename = "item")]
        pub item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfXsdString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.item.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentBasicProfile")]
    pub struct AgentBasicProfile {
        #[yaserde(prefix = "71", rename = "domain")]
        pub domain: String,
        #[yaserde(prefix = "71", rename = "employeeId")]
        pub employee_id: String,
        #[yaserde(prefix = "71", rename = "externalAgent")]
        pub external_agent: bool,
        #[yaserde(prefix = "71", rename = "firstName")]
        pub first_name: String,
        #[yaserde(prefix = "71", rename = "lastName")]
        pub last_name: String,
        #[yaserde(prefix = "71", rename = "middleName")]
        pub middle_name: String,
        #[yaserde(prefix = "71", rename = "outOfOffice")]
        pub out_of_office: bool,
        #[yaserde(prefix = "71", rename = "preferredName")]
        pub preferred_name: String,
        #[yaserde(prefix = "71", rename = "site")]
        pub site: String,
        #[yaserde(prefix = "71", rename = "softwareAgent")]
        pub software_agent: bool,
        #[yaserde(prefix = "71", rename = "title")]
        pub title: String,
        #[yaserde(prefix = "71", rename = "userAddressable")]
        pub user_addressable: bool,
        #[yaserde(prefix = "71", rename = "workgroups")]
        pub workgroups: mod_71::ArrayOfXsdString,
    }
    impl restrictions::CheckRestrictions for AgentBasicProfile {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.domain.check_restrictions(restrictions.clone())?;
            self.employee_id.check_restrictions(restrictions.clone())?;
            self.external_agent.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.middle_name.check_restrictions(restrictions.clone())?;
            self.out_of_office.check_restrictions(restrictions.clone())?;
            self.preferred_name.check_restrictions(restrictions.clone())?;
            self.site.check_restrictions(restrictions.clone())?;
            self.software_agent.check_restrictions(restrictions.clone())?;
            self.title.check_restrictions(restrictions.clone())?;
            self.user_addressable.check_restrictions(restrictions.clone())?;
            self.workgroups.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentChatChannel")]
    pub struct AgentChatChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    impl restrictions::CheckRestrictions for AgentChatChannel {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.enabled.check_restrictions(restrictions.clone())?;
            self.task_ceiling.check_restrictions(restrictions.clone())?;
            self.task_load.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentEmailChannel")]
    pub struct AgentEmailChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "fromAddress")]
        pub from_address: String,
        #[yaserde(prefix = "71", rename = "showFullHeader")]
        pub show_full_header: bool,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    impl restrictions::CheckRestrictions for AgentEmailChannel {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.enabled.check_restrictions(restrictions.clone())?;
            self.from_address.check_restrictions(restrictions.clone())?;
            self.show_full_header.check_restrictions(restrictions.clone())?;
            self.task_ceiling.check_restrictions(restrictions.clone())?;
            self.task_load.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAddressInfo")]
    pub struct AgentAddressInfo {
        #[yaserde(prefix = "71", rename = "POBox")]
        pub po_box: String,
        #[yaserde(prefix = "71", rename = "address1")]
        pub address_1: String,
        #[yaserde(prefix = "71", rename = "address2")]
        pub address_2: String,
        #[yaserde(prefix = "71", rename = "building")]
        pub building: String,
        #[yaserde(prefix = "71", rename = "city")]
        pub city: String,
        #[yaserde(prefix = "71", rename = "company")]
        pub company: String,
        #[yaserde(prefix = "71", rename = "countryOrRegion")]
        pub country_or_region: String,
        #[yaserde(prefix = "71", rename = "mailStop")]
        pub mail_stop: String,
        #[yaserde(prefix = "71", rename = "stateOrProvince")]
        pub state_or_province: String,
        #[yaserde(prefix = "71", rename = "zipOrPostalCode")]
        pub zip_or_postal_code: String,
    }
    impl restrictions::CheckRestrictions for AgentAddressInfo {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.po_box.check_restrictions(restrictions.clone())?;
            self.address_1.check_restrictions(restrictions.clone())?;
            self.address_2.check_restrictions(restrictions.clone())?;
            self.building.check_restrictions(restrictions.clone())?;
            self.city.check_restrictions(restrictions.clone())?;
            self.company.check_restrictions(restrictions.clone())?;
            self.country_or_region.check_restrictions(restrictions.clone())?;
            self.mail_stop.check_restrictions(restrictions.clone())?;
            self.state_or_province.check_restrictions(restrictions.clone())?;
            self.zip_or_postal_code.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentAddress")]
    pub struct AgentAddress {
        #[yaserde(prefix = "71", rename = "home")]
        pub home: mod_71::AgentAddressInfo,
        #[yaserde(prefix = "71", rename = "office")]
        pub office: mod_71::AgentAddressInfo,
        #[yaserde(prefix = "71", rename = "other")]
        pub other: mod_71::AgentAddressInfo,
    }
    impl restrictions::CheckRestrictions for AgentAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.home.check_restrictions(restrictions.clone())?;
            self.office.check_restrictions(restrictions.clone())?;
            self.other.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentEmail")]
    pub struct AgentEmail {
        #[yaserde(prefix = "71", rename = "internal")]
        pub internal: String,
        #[yaserde(prefix = "71", rename = "mobileDevice")]
        pub mobile_device: String,
        #[yaserde(prefix = "71", rename = "personal")]
        pub personal: String,
        #[yaserde(prefix = "71", rename = "primary")]
        pub primary: String,
    }
    impl restrictions::CheckRestrictions for AgentEmail {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.internal.check_restrictions(restrictions.clone())?;
            self.mobile_device.check_restrictions(restrictions.clone())?;
            self.personal.check_restrictions(restrictions.clone())?;
            self.primary.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentPhoneInfo")]
    pub struct AgentPhoneInfo {
        #[yaserde(prefix = "71", rename = "extension")]
        pub extension: String,
        #[yaserde(prefix = "71", rename = "phoneNumber")]
        pub phone_number: String,
    }
    impl restrictions::CheckRestrictions for AgentPhoneInfo {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.extension.check_restrictions(restrictions.clone())?;
            self.phone_number.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentPhone")]
    pub struct AgentPhone {
        #[yaserde(prefix = "71", rename = "fax")]
        pub fax: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "home")]
        pub home: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "mobile")]
        pub mobile: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "pager")]
        pub pager: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "primary")]
        pub primary: mod_71::AgentPhoneInfo,
        #[yaserde(prefix = "71", rename = "secondary")]
        pub secondary: mod_71::AgentPhoneInfo,
    }
    impl restrictions::CheckRestrictions for AgentPhone {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.fax.check_restrictions(restrictions.clone())?;
            self.home.check_restrictions(restrictions.clone())?;
            self.mobile.check_restrictions(restrictions.clone())?;
            self.pager.check_restrictions(restrictions.clone())?;
            self.primary.check_restrictions(restrictions.clone())?;
            self.secondary.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentExtendedProfile")]
    pub struct AgentExtendedProfile {
        #[yaserde(prefix = "71", rename = "address")]
        pub address: mod_71::AgentAddress,
        #[yaserde(prefix = "71", rename = "email")]
        pub email: mod_71::AgentEmail,
        #[yaserde(prefix = "71", rename = "phone")]
        pub phone: mod_71::AgentPhone,
    }
    impl restrictions::CheckRestrictions for AgentExtendedProfile {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.address.check_restrictions(restrictions.clone())?;
            self.email.check_restrictions(restrictions.clone())?;
            self.phone.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentSecurity")]
    pub struct AgentSecurity {
        #[yaserde(prefix = "71", rename = "disableLogin")]
        pub disable_login: bool,
        #[yaserde(prefix = "71", rename = "forcePwdChange")]
        pub force_pwd_change: bool,
        #[yaserde(prefix = "71", rename = "password")]
        pub password: String,
        #[yaserde(prefix = "71", rename = "roleAdmin")]
        pub role_admin: bool,
        #[yaserde(prefix = "71", rename = "roleAgent")]
        pub role_agent: bool,
        #[yaserde(prefix = "71", rename = "roleClerk")]
        pub role_clerk: bool,
        #[yaserde(prefix = "71", rename = "roleEditor")]
        pub role_editor: bool,
        #[yaserde(prefix = "71", rename = "roleOperator")]
        pub role_operator: bool,
        #[yaserde(prefix = "71", rename = "rolePostmaster")]
        pub role_postmaster: bool,
        #[yaserde(prefix = "71", rename = "roleSupervisor")]
        pub role_supervisor: bool,
        #[yaserde(prefix = "71", rename = "roleSupport")]
        pub role_support: bool,
    }
    impl restrictions::CheckRestrictions for AgentSecurity {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.disable_login.check_restrictions(restrictions.clone())?;
            self.force_pwd_change.check_restrictions(restrictions.clone())?;
            self.password.check_restrictions(restrictions.clone())?;
            self.role_admin.check_restrictions(restrictions.clone())?;
            self.role_agent.check_restrictions(restrictions.clone())?;
            self.role_clerk.check_restrictions(restrictions.clone())?;
            self.role_editor.check_restrictions(restrictions.clone())?;
            self.role_operator.check_restrictions(restrictions.clone())?;
            self.role_postmaster.check_restrictions(restrictions.clone())?;
            self.role_supervisor.check_restrictions(restrictions.clone())?;
            self.role_support.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentTaskLoad")]
    pub struct AgentTaskLoad {
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    impl restrictions::CheckRestrictions for AgentTaskLoad {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.task_ceiling.check_restrictions(restrictions.clone())?;
            self.task_load.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AgentVoiceChannel")]
    pub struct AgentVoiceChannel {
        #[yaserde(prefix = "71", rename = "enabled")]
        pub enabled: bool,
        #[yaserde(prefix = "71", rename = "equipment")]
        pub equipment: String,
        #[yaserde(prefix = "71", rename = "password")]
        pub password: String,
        #[yaserde(prefix = "71", rename = "phoneId")]
        pub phone_id: String,
        #[yaserde(prefix = "71", rename = "phoneType")]
        pub phone_type: String,
        #[yaserde(prefix = "71", rename = "queue")]
        pub queue: String,
        #[yaserde(prefix = "71", rename = "taskCeiling")]
        pub task_ceiling: i16,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: i16,
    }
    impl restrictions::CheckRestrictions for AgentVoiceChannel {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.enabled.check_restrictions(restrictions.clone())?;
            self.equipment.check_restrictions(restrictions.clone())?;
            self.password.check_restrictions(restrictions.clone())?;
            self.phone_id.check_restrictions(restrictions.clone())?;
            self.phone_type.check_restrictions(restrictions.clone())?;
            self.queue.check_restrictions(restrictions.clone())?;
            self.task_ceiling.check_restrictions(restrictions.clone())?;
            self.task_load.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Agent")]
    pub struct Agent {
        #[yaserde(prefix = "71", rename = "advocateInfo")]
        pub advocate_info: mod_71::AgentAdvocateInfo,
        #[yaserde(prefix = "71", rename = "basicProfile")]
        pub basic_profile: mod_71::AgentBasicProfile,
        #[yaserde(prefix = "71", rename = "chatChannel")]
        pub chat_channel: mod_71::AgentChatChannel,
        #[yaserde(prefix = "71", rename = "emailChannel")]
        pub email_channel: mod_71::AgentEmailChannel,
        #[yaserde(prefix = "71", rename = "extendedProfile")]
        pub extended_profile: mod_71::AgentExtendedProfile,
        #[yaserde(prefix = "71", rename = "loginId")]
        pub login_id: String,
        #[yaserde(prefix = "71", rename = "security")]
        pub security: mod_71::AgentSecurity,
        #[yaserde(prefix = "71", rename = "taskLoad")]
        pub task_load: mod_71::AgentTaskLoad,
        #[yaserde(prefix = "71", rename = "voiceChannel")]
        pub voice_channel: mod_71::AgentVoiceChannel,
    }
    impl restrictions::CheckRestrictions for Agent {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.advocate_info.check_restrictions(restrictions.clone())?;
            self.basic_profile.check_restrictions(restrictions.clone())?;
            self.chat_channel.check_restrictions(restrictions.clone())?;
            self.email_channel.check_restrictions(restrictions.clone())?;
            self.extended_profile.check_restrictions(restrictions.clone())?;
            self.login_id.check_restrictions(restrictions.clone())?;
            self.security.check_restrictions(restrictions.clone())?;
            self.task_load.check_restrictions(restrictions.clone())?;
            self.voice_channel.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "AicServiceFault")]
    pub struct AicServiceFault {}
    impl restrictions::CheckRestrictions for AicServiceFault {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    pub type Fault = mod_71::AicServiceFault;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Update")]
    pub struct Update {
        #[yaserde(prefix = "71", rename = "agent")]
        pub agent: mod_71::Agent,
    }
    impl restrictions::CheckRestrictions for Update {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.agent.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "UpdateResponse")]
    pub struct UpdateResponse {
        #[yaserde(prefix = "71", rename = "UpdateReturn")]
        pub update_return: bool,
    }
    impl restrictions::CheckRestrictions for UpdateResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Delete")]
    pub struct Delete {
        #[yaserde(prefix = "71", rename = "loginId")]
        pub login_id: String,
    }
    impl restrictions::CheckRestrictions for Delete {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.login_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "DeleteResponse")]
    pub struct DeleteResponse {
        #[yaserde(prefix = "71", rename = "DeleteReturn")]
        pub delete_return: bool,
    }
    impl restrictions::CheckRestrictions for DeleteResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.delete_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupAgentIds")]
    pub struct LookupAgentIds {}
    impl restrictions::CheckRestrictions for LookupAgentIds {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupAgentIdsResponse")]
    pub struct LookupAgentIdsResponse {
        #[yaserde(prefix = "71", rename = "LookupAgentIdsReturn")]
        pub lookup_agent_ids_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupAgentIdsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_agent_ids_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupLRMIds")]
    pub struct LookupLRMIds {}
    impl restrictions::CheckRestrictions for LookupLRMIds {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupLRMIdsResponse")]
    pub struct LookupLRMIdsResponse {
        #[yaserde(prefix = "71", rename = "LookupLRMIdsReturn")]
        pub lookup_lrm_ids_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupLRMIdsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_lrm_ids_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupWorkgroups")]
    pub struct LookupWorkgroups {}
    impl restrictions::CheckRestrictions for LookupWorkgroups {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupWorkgroupsResponse")]
    pub struct LookupWorkgroupsResponse {
        #[yaserde(prefix = "71", rename = "LookupWorkgroupsReturn")]
        pub lookup_workgroups_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupWorkgroupsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_workgroups_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupDomains")]
    pub struct LookupDomains {}
    impl restrictions::CheckRestrictions for LookupDomains {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupDomainsResponse")]
    pub struct LookupDomainsResponse {
        #[yaserde(prefix = "71", rename = "LookupDomainsReturn")]
        pub lookup_domains_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupDomainsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_domains_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupLinkGroups")]
    pub struct LookupLinkGroups {}
    impl restrictions::CheckRestrictions for LookupLinkGroups {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupLinkGroupsResponse")]
    pub struct LookupLinkGroupsResponse {
        #[yaserde(prefix = "71", rename = "LookupLinkGroupsReturn")]
        pub lookup_link_groups_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupLinkGroupsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_link_groups_return
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupPhoneTypes")]
    pub struct LookupPhoneTypes {}
    impl restrictions::CheckRestrictions for LookupPhoneTypes {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupPhoneTypesResponse")]
    pub struct LookupPhoneTypesResponse {
        #[yaserde(prefix = "71", rename = "LookupPhoneTypesReturn")]
        pub lookup_phone_types_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupPhoneTypesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_phone_types_return
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupSites")]
    pub struct LookupSites {}
    impl restrictions::CheckRestrictions for LookupSites {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "LookupSitesResponse")]
    pub struct LookupSitesResponse {
        #[yaserde(prefix = "71", rename = "LookupSitesReturn")]
        pub lookup_sites_return: Vec<String>,
    }
    impl restrictions::CheckRestrictions for LookupSitesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lookup_sites_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "Create")]
    pub struct Create {
        #[yaserde(prefix = "71", rename = "agent")]
        pub agent: mod_71::Agent,
    }
    impl restrictions::CheckRestrictions for Create {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.agent.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "71", namespaces = {"71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71"}, rename = "CreateResponse")]
    pub struct CreateResponse {
        #[yaserde(prefix = "71", rename = "CreateReturn")]
        pub create_return: bool,
    }
    impl restrictions::CheckRestrictions for CreateResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.create_return.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
}

/* LookupSites */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupSites")]
    pub lookup_sites: mod_71::LookupSites,
}
impl restrictions::CheckRestrictions for LookupSitesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_sites.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupSitesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupSitesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupSitesResponse")]
    pub lookup_sites_response: mod_71::LookupSitesResponse,
}
impl restrictions::CheckRestrictions for LookupSitesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_sites_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupSitesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupSitesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupSitesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* Delete */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Delete")]
    pub delete: mod_71::Delete,
}
impl restrictions::CheckRestrictions for DeleteInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.delete.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for DeleteInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "DeleteResponse")]
    pub delete_response: mod_71::DeleteResponse,
}
impl restrictions::CheckRestrictions for DeleteOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.delete_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct DeleteOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for DeleteOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupLRMIds */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLRMIds")]
    pub lookup_lrm_ids: mod_71::LookupLRMIds,
}
impl restrictions::CheckRestrictions for LookupLRMIdsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_lrm_ids.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLRMIdsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupLRMIdsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLRMIdsResponse")]
    pub lookup_lrm_ids_response: mod_71::LookupLRMIdsResponse,
}
impl restrictions::CheckRestrictions for LookupLRMIdsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_lrm_ids_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLRMIdsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLRMIdsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupLRMIdsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupWorkgroups */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupWorkgroups")]
    pub lookup_workgroups: mod_71::LookupWorkgroups,
}
impl restrictions::CheckRestrictions for LookupWorkgroupsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_workgroups.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupWorkgroupsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupWorkgroupsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupWorkgroupsResponse")]
    pub lookup_workgroups_response: mod_71::LookupWorkgroupsResponse,
}
impl restrictions::CheckRestrictions for LookupWorkgroupsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_workgroups_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupWorkgroupsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupWorkgroupsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupWorkgroupsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupAgentIds */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupAgentIds")]
    pub lookup_agent_ids: mod_71::LookupAgentIds,
}
impl restrictions::CheckRestrictions for LookupAgentIdsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_agent_ids.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupAgentIdsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupAgentIdsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupAgentIdsResponse")]
    pub lookup_agent_ids_response: mod_71::LookupAgentIdsResponse,
}
impl restrictions::CheckRestrictions for LookupAgentIdsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_agent_ids_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupAgentIdsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupAgentIdsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupAgentIdsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupLinkGroups */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLinkGroups")]
    pub lookup_link_groups: mod_71::LookupLinkGroups,
}
impl restrictions::CheckRestrictions for LookupLinkGroupsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_link_groups.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLinkGroupsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupLinkGroupsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupLinkGroupsResponse")]
    pub lookup_link_groups_response: mod_71::LookupLinkGroupsResponse,
}
impl restrictions::CheckRestrictions for LookupLinkGroupsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_link_groups_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupLinkGroupsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupLinkGroupsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupLinkGroupsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupDomains */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupDomains")]
    pub lookup_domains: mod_71::LookupDomains,
}
impl restrictions::CheckRestrictions for LookupDomainsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_domains.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupDomainsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupDomainsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupDomainsResponse")]
    pub lookup_domains_response: mod_71::LookupDomainsResponse,
}
impl restrictions::CheckRestrictions for LookupDomainsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_domains_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupDomainsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupDomainsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupDomainsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* Update */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Update")]
    pub update: mod_71::Update,
}
impl restrictions::CheckRestrictions for UpdateInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "UpdateResponse")]
    pub update_response: mod_71::UpdateResponse,
}
impl restrictions::CheckRestrictions for UpdateOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct UpdateOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* Create */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Create")]
    pub create: mod_71::Create,
}
impl restrictions::CheckRestrictions for CreateInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "CreateResponse")]
    pub create_response: mod_71::CreateResponse,
}
impl restrictions::CheckRestrictions for CreateOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct CreateOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* LookupPhoneTypes */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupPhoneTypes")]
    pub lookup_phone_types: mod_71::LookupPhoneTypes,
}
impl restrictions::CheckRestrictions for LookupPhoneTypesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_phone_types.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupPhoneTypesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupPhoneTypesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "LookupPhoneTypesResponse")]
    pub lookup_phone_types_response: mod_71::LookupPhoneTypesResponse,
}
impl restrictions::CheckRestrictions for LookupPhoneTypesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.lookup_phone_types_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct LookupPhoneTypesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: LookupPhoneTypesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for LookupPhoneTypesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* Get */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetInputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "Get")]
    pub get: mod_71::Get,
}
impl restrictions::CheckRestrictions for GetInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "71", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetOutputEnvelopeBody {
    #[yaserde(prefix = "71", rename = "GetResponse")]
    pub get_response: mod_71::GetResponse,
}
impl restrictions::CheckRestrictions for GetOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "71" = "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71" })]
pub struct GetOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub struct AicAgentAdminService {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl AicAgentAdminService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "http://aiccore.avayacloud.com:9800/webservices/services/AicAgentAdmin".to_string(),
            credentials,
        }
    }

    pub async fn lookup_sites(&self, req: LookupSitesInputEnvelope) -> error::SoapResult<LookupSitesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn delete(&self, req: DeleteInputEnvelope) -> error::SoapResult<DeleteOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_lrm_ids(
        &self,
        req: LookupLRMIdsInputEnvelope,
    ) -> error::SoapResult<LookupLRMIdsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_workgroups(
        &self,
        req: LookupWorkgroupsInputEnvelope,
    ) -> error::SoapResult<LookupWorkgroupsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_agent_ids(
        &self,
        req: LookupAgentIdsInputEnvelope,
    ) -> error::SoapResult<LookupAgentIdsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_link_groups(
        &self,
        req: LookupLinkGroupsInputEnvelope,
    ) -> error::SoapResult<LookupLinkGroupsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_domains(
        &self,
        req: LookupDomainsInputEnvelope,
    ) -> error::SoapResult<LookupDomainsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update(&self, req: UpdateInputEnvelope) -> error::SoapResult<UpdateOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn create(&self, req: CreateInputEnvelope) -> error::SoapResult<CreateOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn lookup_phone_types(
        &self,
        req: LookupPhoneTypesInputEnvelope,
    ) -> error::SoapResult<LookupPhoneTypesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get(&self, req: GetInputEnvelope) -> error::SoapResult<GetOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }
}
pub mod error {
    #![allow(dead_code)]

    use std::{error::Error, num::ParseIntError};

    #[derive(Debug)]
    pub enum SoapError {
        YaserdeError(String),
        Http(reqwest::Error),
        Restriction(String),
    }

    pub type SoapResult<T> = Result<T, SoapError>;

    impl std::fmt::Display for SoapError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SoapError::YaserdeError(e) => write!(f, "Yaserde error: {e}"),
                SoapError::Http(e) => write!(f, "HTTP error: {e}"),
                SoapError::Restriction(e) => write!(f, "Restriction not met: {e}"),
            }
        }
    }

    impl Error for SoapError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                SoapError::YaserdeError(_) | SoapError::Restriction(_) => None,
                SoapError::Http(e) => Some(e),
            }
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }

    impl From<reqwest::Error> for SoapError {
        fn from(e: reqwest::Error) -> Self {
            SoapError::Http(e)
        }
    }

    impl From<ParseIntError> for SoapError {
        fn from(err: ParseIntError) -> Self {
            SoapError::Restriction(format!("invalid restriction in XSD: {err}"))
        }
    }
}

mod helpers {
    #![allow(dead_code)]

    use super::{
        error::{SoapError, SoapResult},
        restrictions::CheckRestrictions,
    };
    use reqwest::Client;
    use std::fmt;
    use yaserde::{YaDeserialize, YaSerialize};

    pub(super) async fn send_soap_request<YI, YO, U, P>(
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize + CheckRestrictions,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        let client = Client::new();
        send_soap_request_using_client(&client, url, credentials, req).await
    }

    pub(super) async fn send_soap_request_using_client<YI, YO, U, P>(
        client: &Client,
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize + CheckRestrictions,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        req.check_restrictions(None)?;
        let body = yaserde::ser::to_string(&req).map_err(SoapError::YaserdeError)?;
        let mut req = client.post(url).body(body);
        if let Some((username, password)) = credentials {
            req = req.basic_auth(username, Some(password));
        }
        let response = req.send().await?;
        response.error_for_status_ref()?;
        let response_body = response.text().await?;
        let response = yaserde::de::from_str(&response_body).map_err(SoapError::YaserdeError)?;
        Ok(response)
    }
}

pub mod restrictions {
    use super::error::{SoapError, SoapResult};
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Default)]
    pub struct Restrictions {
        pub min_inclusive: Option<i32>,
        pub max_inclusive: Option<i32>,
        pub min_exclusive: Option<i32>,
        pub max_exclusive: Option<i32>,
        pub length: Option<usize>,
        pub min_length: Option<usize>,
        pub max_length: Option<usize>,
        pub enumeration: Option<Vec<String>>,
    }

    pub trait CheckRestrictions {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            Ok(())
        }
    }

    impl<C> CheckRestrictions for Vec<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            for c in self {
                c.check_restrictions(restrictions.clone())?;
            }
            Ok(())
        }
    }

    impl<C> CheckRestrictions for Option<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            if let Some(c) = self {
                c.check_restrictions(restrictions)?;
            }
            Ok(())
        }
    }

    impl CheckRestrictions for i32 {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            if let Some(restrictions) = restrictions {
                if let Some(min_inclusive) = restrictions.min_inclusive {
                    if *self <= min_inclusive {
                        return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
                    }
                }

                if let Some(max_inclusive) = restrictions.max_inclusive {
                    if max_inclusive <= *self {
                        return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
                    }
                }

                if let Some(min_exclusive) = restrictions.min_exclusive {
                    if *self < min_exclusive {
                        return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
                    }
                }

                if let Some(max_exclusive) = restrictions.max_exclusive {
                    if max_exclusive < *self {
                        return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
                    }
                }
            }

            Ok(())
        }
    }

    macro_rules! impl_check_restrictions_for_int {
    ($($t:ty),*) => {
        $(
            impl CheckRestrictions for $t {
                fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
                    let value = i32::try_from(*self).map_err(|e| SoapError::Restriction(e.to_string()))?;
                    value.check_restrictions(restrictions)
                }
            }
        )*
    }
}

    impl_check_restrictions_for_int!(i8, u8, i16, u16, u32, i64, u64);

    impl CheckRestrictions for bool {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for f32 {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for f64 {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for String {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            let Some(restrictions) = restrictions else {
                return Ok(());
            };

            let s_len = self.chars().count();

            if let Some(min_length) = restrictions.min_length {
                if s_len < min_length {
                    return Err(SoapError::Restriction("minLength restriction not met".to_string()));
                }
            }

            if let Some(max_length) = restrictions.max_length {
                if max_length < s_len {
                    return Err(SoapError::Restriction("maxLength restriction not met".to_string()));
                }
            }

            if let Some(length) = restrictions.length {
                if length != s_len {
                    return Err(SoapError::Restriction("length restriction not met".to_string()));
                }
            }

            // Enumerations
            if let Some(enumeration) = restrictions.enumeration.as_ref() {
                if !enumeration.contains(self) {
                    return Err(SoapError::Restriction("enumeration restriction not met".to_string()));
                }
            }

            // Number-type checks; see if any of these are set
            if restrictions.min_inclusive.is_none()
                && restrictions.max_inclusive.is_none()
                && restrictions.min_exclusive.is_none()
                && restrictions.max_exclusive.is_none()
            {
                return Ok(());
            }

            let value = self.parse::<i32>()?;

            if let Some(min_inclusive) = restrictions.min_inclusive {
                if value <= min_inclusive {
                    return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
                }
            }

            if let Some(max_inclusive) = restrictions.max_inclusive {
                if max_inclusive <= value {
                    return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
                }
            }

            if let Some(min_exclusive) = restrictions.min_exclusive {
                if value < min_exclusive {
                    return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
                }
            }

            if let Some(max_exclusive) = restrictions.max_exclusive {
                if max_exclusive < value {
                    return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
                }
            }

            Ok(())
        }
    }
}

/// This module contains the `MultiRef` type which is a wrapper around `Arc<RwLock<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
/// Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
/// Needs `xml-rs`, `tokio` and `yaserde` as dependencies.
pub mod multi_ref {
    use super::{
        error::SoapResult,
        restrictions::{CheckRestrictions, Restrictions},
    };
    use std::{ops::Deref, rc::Rc, sync::Arc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Arc<T>,
    }

    impl<T> MultiRef<T> {
        #[allow(dead_code)]
        pub fn new(inner: T) -> Self {
            Self { inner: Arc::new(inner) }
        }
    }

    impl<C> CheckRestrictions for MultiRef<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            self.inner.check_restrictions(restrictions)
        }
    }

    impl<T: YaDeserialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self { inner: Arc::new(inner) })
        }
    }

    impl<T: YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
            self.inner.serialize_attributes(attributes, namespace)
        }
    }

    impl<T: Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self { inner: Arc::default() }
        }
    }

    impl<T: Clone> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Arc<T>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
