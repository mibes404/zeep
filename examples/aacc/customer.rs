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
pub mod mod_wsd {
    use super::*;
    use restrictions::CheckRestrictions;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddAddress")]
    pub struct AddAddress {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newAddress")]
        pub new_address: Option<mod_wsd::Address>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_address.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Address")]
    pub struct Address {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Line1")]
        pub line_1: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Line2")]
        pub line_2: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Line3")]
        pub line_3: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Line4")]
        pub line_4: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Line5")]
        pub line_5: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ZipCode")]
        pub zip_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Country")]
        pub country: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Default")]
        pub default: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Imported")]
        pub imported: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "FullAddress")]
        pub full_address: Option<String>,
    }
    impl restrictions::CheckRestrictions for Address {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.line_1.check_restrictions(restrictions.clone())?;
            self.line_2.check_restrictions(restrictions.clone())?;
            self.line_3.check_restrictions(restrictions.clone())?;
            self.line_4.check_restrictions(restrictions.clone())?;
            self.line_5.check_restrictions(restrictions.clone())?;
            self.zip_code.check_restrictions(restrictions.clone())?;
            self.country.check_restrictions(restrictions.clone())?;
            self.default.check_restrictions(restrictions.clone())?;
            self.imported.check_restrictions(restrictions.clone())?;
            self.full_address.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddAddressResponse")]
    pub struct AddAddressResponse {
        #[yaserde(prefix = "wsd", rename = "AddAddressResult")]
        pub add_address_result: i64,
    }
    impl restrictions::CheckRestrictions for AddAddressResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_address_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddContact")]
    pub struct AddContact {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "contactParams")]
        pub contact_params: Option<mod_wsd::ContactCreate>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddContact {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.contact_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ContactCreate")]
    pub struct ContactCreate {
        #[yaserde(prefix = "wsd", rename = "NewCount")]
        pub new_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OriginalSubject")]
        pub original_subject: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CustomerID")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Agent")]
        pub agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Skillset")]
        pub skillset: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AcquiredTime")]
        pub acquired_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Source")]
        pub source: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MailTo")]
        pub mail_to: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailFrom")]
        pub mail_from: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailCC")]
        pub mail_cc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "QueueType")]
        pub queue_type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OpenTime")]
        pub open_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClosedTime")]
        pub closed_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AutoResponse")]
        pub auto_response: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Rule")]
        pub rule: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Priority")]
        pub priority: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OpenDuration")]
        pub open_duration: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SubStatus")]
        pub sub_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CMFStatus")]
        pub cmf_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CallId")]
        pub call_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "WebOnHoldTag")]
        pub web_on_hold_tag: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TimeZone")]
        pub time_zone: Option<i16>,
        #[yaserde(prefix = "wsd", rename = "CharSet")]
        pub char_set: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClickStreamText")]
        pub click_stream_text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OutBoundOriginator")]
        pub out_bound_originator: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Campaign")]
        pub campaign: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CampaignRetryCount")]
        pub campaign_retry_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DispositionCode")]
        pub disposition_code: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "PreferredCallBackMedia")]
        pub preferred_call_back_media: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastAction")]
        pub last_action: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ClosureType")]
        pub closure_type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Answers")]
        pub answers: Option<mod_wsd::ArrayOfAnswer>,
        #[yaserde(prefix = "wsd", rename = "ActionCreateList")]
        pub action_create_list: Option<mod_wsd::ArrayOfActionCreate>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "CmfId")]
        pub cmf_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RouteSummary")]
        pub route_summary: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Importance")]
        pub importance: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SocialMediaHeader")]
        pub social_media_header: Option<mod_wsd::SocialMediaHeader>,
        #[yaserde(prefix = "wsd", rename = "SMFToken")]
        pub smf_token: Option<String>,
    }
    impl restrictions::CheckRestrictions for ContactCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.new_count.check_restrictions(restrictions.clone())?;
            self.original_subject.check_restrictions(restrictions.clone())?;
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.agent.check_restrictions(restrictions.clone())?;
            self.skillset.check_restrictions(restrictions.clone())?;
            self.acquired_time.check_restrictions(restrictions.clone())?;
            self.source.check_restrictions(restrictions.clone())?;
            self.mail_to.check_restrictions(restrictions.clone())?;
            self.mail_from.check_restrictions(restrictions.clone())?;
            self.mail_cc.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.queue_type.check_restrictions(restrictions.clone())?;
            self.open_time.check_restrictions(restrictions.clone())?;
            self.closed_time.check_restrictions(restrictions.clone())?;
            self.auto_response.check_restrictions(restrictions.clone())?;
            self.rule.check_restrictions(restrictions.clone())?;
            self.priority.check_restrictions(restrictions.clone())?;
            self.open_duration.check_restrictions(restrictions.clone())?;
            self.sub_status.check_restrictions(restrictions.clone())?;
            self.cmf_status.check_restrictions(restrictions.clone())?;
            self.call_id.check_restrictions(restrictions.clone())?;
            self.web_on_hold_tag.check_restrictions(restrictions.clone())?;
            self.time_zone.check_restrictions(restrictions.clone())?;
            self.char_set.check_restrictions(restrictions.clone())?;
            self.click_stream_text.check_restrictions(restrictions.clone())?;
            self.out_bound_originator.check_restrictions(restrictions.clone())?;
            self.campaign.check_restrictions(restrictions.clone())?;
            self.campaign_retry_count.check_restrictions(restrictions.clone())?;
            self.disposition_code.check_restrictions(restrictions.clone())?;
            self.preferred_call_back_media
                .check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.last_action.check_restrictions(restrictions.clone())?;
            self.closure_type.check_restrictions(restrictions.clone())?;
            self.answers.check_restrictions(restrictions.clone())?;
            self.action_create_list.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            self.cmf_id.check_restrictions(restrictions.clone())?;
            self.route_summary.check_restrictions(restrictions.clone())?;
            self.importance.check_restrictions(restrictions.clone())?;
            self.social_media_header.check_restrictions(restrictions.clone())?;
            self.smf_token.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAnswer")]
    pub struct ArrayOfAnswer {
        #[yaserde(prefix = "wsd", rename = "Answer")]
        pub answer: Vec<mod_wsd::Answer>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAnswer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.answer.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Answer")]
    pub struct Answer {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "QuestionID")]
        pub question_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Answer")]
        pub answer: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FreeText")]
        pub free_text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DateCreated")]
        pub date_created: Option<String>,
    }
    impl restrictions::CheckRestrictions for Answer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.question_id.check_restrictions(restrictions.clone())?;
            self.answer.check_restrictions(restrictions.clone())?;
            self.free_text.check_restrictions(restrictions.clone())?;
            self.date_created.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfActionCreate")]
    pub struct ArrayOfActionCreate {
        #[yaserde(prefix = "wsd", rename = "ActionCreate")]
        pub action_create: Vec<mod_wsd::ActionCreate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfActionCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.action_create.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ActionCreate")]
    pub struct ActionCreate {
        #[yaserde(prefix = "wsd", rename = "ContactId")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Source")]
        pub source: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Subject")]
        pub subject: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Text")]
        pub text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TextHTML")]
        pub text_html: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CharSet")]
        pub char_set: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Attempt")]
        pub attempt: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DispositionCode")]
        pub disposition_code: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MailTo")]
        pub mail_to: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailFrom")]
        pub mail_from: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailCC")]
        pub mail_cc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailBCC")]
        pub mail_bcc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallBackTime")]
        pub call_back_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallBackMedia")]
        pub call_back_media: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CallBackStatus")]
        pub call_back_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Agent")]
        pub agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Comment")]
        pub comment: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TimeAllocated")]
        pub time_allocated: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TemplateLocation")]
        pub template_location: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClosedReason")]
        pub closed_reason: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "HistoryFlag")]
        pub history_flag: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "NumberUsed")]
        pub number_used: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallStartTime")]
        pub call_start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallEndTime")]
        pub call_end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DialStartTime")]
        pub dial_start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DialEndTime")]
        pub dial_end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OpenTime")]
        pub open_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClosedTime")]
        pub closed_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "AttachmentList")]
        pub attachment_list: Option<mod_wsd::ArrayOfAttachmentCreate>,
        #[yaserde(prefix = "wsd", rename = "AutoSuggest")]
        pub auto_suggest: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ActionCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.source.check_restrictions(restrictions.clone())?;
            self.subject.check_restrictions(restrictions.clone())?;
            self.text.check_restrictions(restrictions.clone())?;
            self.text_html.check_restrictions(restrictions.clone())?;
            self.char_set.check_restrictions(restrictions.clone())?;
            self.attempt.check_restrictions(restrictions.clone())?;
            self.disposition_code.check_restrictions(restrictions.clone())?;
            self.mail_to.check_restrictions(restrictions.clone())?;
            self.mail_from.check_restrictions(restrictions.clone())?;
            self.mail_cc.check_restrictions(restrictions.clone())?;
            self.mail_bcc.check_restrictions(restrictions.clone())?;
            self.call_back_time.check_restrictions(restrictions.clone())?;
            self.call_back_media.check_restrictions(restrictions.clone())?;
            self.call_back_status.check_restrictions(restrictions.clone())?;
            self.agent.check_restrictions(restrictions.clone())?;
            self.comment.check_restrictions(restrictions.clone())?;
            self.time_allocated.check_restrictions(restrictions.clone())?;
            self.template_location.check_restrictions(restrictions.clone())?;
            self.closed_reason.check_restrictions(restrictions.clone())?;
            self.history_flag.check_restrictions(restrictions.clone())?;
            self.number_used.check_restrictions(restrictions.clone())?;
            self.call_start_time.check_restrictions(restrictions.clone())?;
            self.call_end_time.check_restrictions(restrictions.clone())?;
            self.dial_start_time.check_restrictions(restrictions.clone())?;
            self.dial_end_time.check_restrictions(restrictions.clone())?;
            self.open_time.check_restrictions(restrictions.clone())?;
            self.closed_time.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            self.attachment_list.check_restrictions(restrictions.clone())?;
            self.auto_suggest.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCustomFieldCreate")]
    pub struct ArrayOfCustomFieldCreate {
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreate")]
        pub custom_field_create: Vec<mod_wsd::CustomFieldCreate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCustomFieldCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.custom_field_create.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomFieldCreate")]
    pub struct CustomFieldCreate {
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Text")]
        pub text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "IsTextVisible")]
        pub is_text_visible: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldTemplate")]
        pub custom_field_template: Option<i64>,
    }
    impl restrictions::CheckRestrictions for CustomFieldCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.name.check_restrictions(restrictions.clone())?;
            self.text.check_restrictions(restrictions.clone())?;
            self.is_text_visible.check_restrictions(restrictions.clone())?;
            self.custom_field_template.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAttachmentCreate")]
    pub struct ArrayOfAttachmentCreate {
        #[yaserde(prefix = "wsd", rename = "AttachmentCreate")]
        pub attachment_create: Vec<mod_wsd::AttachmentCreate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAttachmentCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.attachment_create.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AttachmentCreate")]
    pub struct AttachmentCreate {
        #[yaserde(prefix = "wsd", rename = "DisplayFileName")]
        pub display_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InternalFileName")]
        pub internal_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Direction")]
        pub direction: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "FileContents")]
        pub file_contents: Option<String>,
    }
    impl restrictions::CheckRestrictions for AttachmentCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.display_file_name.check_restrictions(restrictions.clone())?;
            self.internal_file_name.check_restrictions(restrictions.clone())?;
            self.direction.check_restrictions(restrictions.clone())?;
            self.file_contents.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SocialMediaHeader")]
    pub struct SocialMediaHeader {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DesktopUrl")]
        pub desktop_url: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InteractionID")]
        pub interaction_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Query")]
        pub query: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Domain")]
        pub domain: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Classification")]
        pub classification: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Relevance")]
        pub relevance: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Sentiment")]
        pub sentiment: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Language")]
        pub language: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Author")]
        pub author: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Channel")]
        pub channel: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Keywords")]
        pub keywords: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MimeContactType")]
        pub mime_contact_type: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SocialMediaRuleId")]
        pub social_media_rule_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for SocialMediaHeader {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.desktop_url.check_restrictions(restrictions.clone())?;
            self.interaction_id.check_restrictions(restrictions.clone())?;
            self.query.check_restrictions(restrictions.clone())?;
            self.domain.check_restrictions(restrictions.clone())?;
            self.classification.check_restrictions(restrictions.clone())?;
            self.relevance.check_restrictions(restrictions.clone())?;
            self.sentiment.check_restrictions(restrictions.clone())?;
            self.language.check_restrictions(restrictions.clone())?;
            self.author.check_restrictions(restrictions.clone())?;
            self.channel.check_restrictions(restrictions.clone())?;
            self.keywords.check_restrictions(restrictions.clone())?;
            self.mime_contact_type.check_restrictions(restrictions.clone())?;
            self.social_media_rule_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddContactResponse")]
    pub struct AddContactResponse {
        #[yaserde(prefix = "wsd", rename = "AddContactResult")]
        pub add_contact_result: i64,
    }
    impl restrictions::CheckRestrictions for AddContactResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_contact_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddCustomField")]
    pub struct AddCustomField {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "customFieldParams")]
        pub custom_field_params: Option<mod_wsd::CustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddCustomField {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.custom_field_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddCustomFieldResponse")]
    pub struct AddCustomFieldResponse {
        #[yaserde(prefix = "wsd", rename = "AddCustomFieldResult")]
        pub add_custom_field_result: i64,
    }
    impl restrictions::CheckRestrictions for AddCustomFieldResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_custom_field_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddEmailAddress")]
    pub struct AddEmailAddress {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newEmailAddress")]
        pub new_email_address: Option<mod_wsd::EmailAddress>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddEmailAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_email_address.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "EmailAddress")]
    pub struct EmailAddress {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Address")]
        pub address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SearchAddress")]
        pub search_address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Default")]
        pub default: Option<bool>,
    }
    impl restrictions::CheckRestrictions for EmailAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.address.check_restrictions(restrictions.clone())?;
            self.search_address.check_restrictions(restrictions.clone())?;
            self.default.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddEmailAddressResponse")]
    pub struct AddEmailAddressResponse {
        #[yaserde(prefix = "wsd", rename = "AddEmailAddressResult")]
        pub add_email_address_result: i64,
    }
    impl restrictions::CheckRestrictions for AddEmailAddressResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_email_address_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddPhoneNumber")]
    pub struct AddPhoneNumber {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "phoneNumberParams")]
        pub phone_number_params: Option<mod_wsd::PhoneNumberCreate>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddPhoneNumber {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.phone_number_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "PhoneNumberCreate")]
    pub struct PhoneNumberCreate {
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "InternationalCode")]
        pub international_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AreaCode")]
        pub area_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Number")]
        pub number: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DoNotCall")]
        pub do_not_call: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Default")]
        pub default: Option<bool>,
    }
    impl restrictions::CheckRestrictions for PhoneNumberCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.r#type.check_restrictions(restrictions.clone())?;
            self.international_code.check_restrictions(restrictions.clone())?;
            self.area_code.check_restrictions(restrictions.clone())?;
            self.number.check_restrictions(restrictions.clone())?;
            self.do_not_call.check_restrictions(restrictions.clone())?;
            self.default.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddPhoneNumberResponse")]
    pub struct AddPhoneNumberResponse {
        #[yaserde(prefix = "wsd", rename = "AddPhoneNumberResult")]
        pub add_phone_number_result: i64,
    }
    impl restrictions::CheckRestrictions for AddPhoneNumberResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_phone_number_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddSipUri")]
    pub struct AddSipUri {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newSipUri")]
        pub new_sip_uri: Option<mod_wsd::SipUri>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for AddSipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_sip_uri.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SipUri")]
    pub struct SipUri {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Address")]
        pub address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SearchAddress")]
        pub search_address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Default")]
        pub default: Option<bool>,
    }
    impl restrictions::CheckRestrictions for SipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.address.check_restrictions(restrictions.clone())?;
            self.search_address.check_restrictions(restrictions.clone())?;
            self.default.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AddSipUriResponse")]
    pub struct AddSipUriResponse {
        #[yaserde(prefix = "wsd", rename = "AddSipUriResult")]
        pub add_sip_uri_result: i64,
    }
    impl restrictions::CheckRestrictions for AddSipUriResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.add_sip_uri_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CarbonCopy")]
    pub struct CarbonCopy {
        #[yaserde(prefix = "wsd", rename = "customerId")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "contactId")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "copyMode")]
        pub copy_mode: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CarbonCopy {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.copy_mode.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CarbonCopyResponse")]
    pub struct CarbonCopyResponse {
        #[yaserde(prefix = "wsd", rename = "CarbonCopyResult")]
        pub carbon_copy_result: i64,
    }
    impl restrictions::CheckRestrictions for CarbonCopyResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.carbon_copy_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CleanCustomer")]
    pub struct CleanCustomer {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "bRemoveAllContacts")]
        pub b_remove_all_contacts: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CleanCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.b_remove_all_contacts.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CleanCustomerResponse")]
    pub struct CleanCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "CleanCustomerResult")]
        pub clean_customer_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for CleanCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.clean_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Customer")]
    pub struct Customer {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Addresses")]
        pub addresses: Option<mod_wsd::ArrayOfAddress>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumbers")]
        pub phone_numbers: Option<mod_wsd::ArrayOfPhoneNumber>,
        #[yaserde(prefix = "wsd", rename = "EmailAddresses")]
        pub email_addresses: Option<mod_wsd::ArrayOfEmailAddress>,
        #[yaserde(prefix = "wsd", rename = "Contacts")]
        pub contacts: Option<mod_wsd::ArrayOfContact>,
        #[yaserde(prefix = "wsd", rename = "CustomFields")]
        pub custom_fields: Option<mod_wsd::ArrayOfCustomField>,
        #[yaserde(prefix = "wsd", rename = "SipUris")]
        pub sip_uris: Option<mod_wsd::ArrayOfSipUri>,
        #[yaserde(prefix = "wsd", rename = "Title")]
        pub title: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoginPage")]
        pub login_page: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "RegisterDate")]
        pub register_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgent")]
        pub preferred_agent: Option<mod_wsd::User>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgentID")]
        pub preferred_agent_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Objection")]
        pub objection: Option<bool>,
    }
    impl restrictions::CheckRestrictions for Customer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.addresses.check_restrictions(restrictions.clone())?;
            self.phone_numbers.check_restrictions(restrictions.clone())?;
            self.email_addresses.check_restrictions(restrictions.clone())?;
            self.contacts.check_restrictions(restrictions.clone())?;
            self.custom_fields.check_restrictions(restrictions.clone())?;
            self.sip_uris.check_restrictions(restrictions.clone())?;
            self.title.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.login_page.check_restrictions(restrictions.clone())?;
            self.register_date.check_restrictions(restrictions.clone())?;
            self.user_name.check_restrictions(restrictions.clone())?;
            self.preferred_agent.check_restrictions(restrictions.clone())?;
            self.preferred_agent_id.check_restrictions(restrictions.clone())?;
            self.objection.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAddress")]
    pub struct ArrayOfAddress {
        #[yaserde(prefix = "wsd", rename = "Address")]
        pub address: Vec<mod_wsd::Address>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.address.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfPhoneNumber")]
    pub struct ArrayOfPhoneNumber {
        #[yaserde(prefix = "wsd", rename = "PhoneNumber")]
        pub phone_number: Vec<mod_wsd::PhoneNumber>,
    }
    impl restrictions::CheckRestrictions for ArrayOfPhoneNumber {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.phone_number.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "PhoneNumber")]
    pub struct PhoneNumber {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "InternationalCode")]
        pub international_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AreaCode")]
        pub area_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Number")]
        pub number: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FullNumber")]
        pub full_number: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DoNotCall")]
        pub do_not_call: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Default")]
        pub default: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Offset")]
        pub offset: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TimeZoneId")]
        pub time_zone_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for PhoneNumber {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.international_code.check_restrictions(restrictions.clone())?;
            self.area_code.check_restrictions(restrictions.clone())?;
            self.number.check_restrictions(restrictions.clone())?;
            self.full_number.check_restrictions(restrictions.clone())?;
            self.do_not_call.check_restrictions(restrictions.clone())?;
            self.default.check_restrictions(restrictions.clone())?;
            self.offset.check_restrictions(restrictions.clone())?;
            self.time_zone_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CodeMapping")]
    pub struct CodeMapping {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "FieldName")]
        pub field_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NumericValue")]
        pub numeric_value: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TextValue")]
        pub text_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Icon")]
        pub icon: Option<String>,
    }
    impl restrictions::CheckRestrictions for CodeMapping {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.field_name.check_restrictions(restrictions.clone())?;
            self.numeric_value.check_restrictions(restrictions.clone())?;
            self.text_value.check_restrictions(restrictions.clone())?;
            self.icon.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfEmailAddress")]
    pub struct ArrayOfEmailAddress {
        #[yaserde(prefix = "wsd", rename = "EmailAddress")]
        pub email_address: Vec<mod_wsd::EmailAddress>,
    }
    impl restrictions::CheckRestrictions for ArrayOfEmailAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.email_address.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfContact")]
    pub struct ArrayOfContact {
        #[yaserde(prefix = "wsd", rename = "Contact")]
        pub contact: Vec<mod_wsd::Contact>,
    }
    impl restrictions::CheckRestrictions for ArrayOfContact {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.contact.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Contact")]
    pub struct Contact {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ArrivalTime")]
        pub arrival_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OriginalSubject")]
        pub original_subject: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CustomerID")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Agent")]
        pub agent: Option<mod_wsd::User>,
        #[yaserde(prefix = "wsd", rename = "Skillset")]
        pub skillset: Option<mod_wsd::Skillset>,
        #[yaserde(prefix = "wsd", rename = "AcquiredTime")]
        pub acquired_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Source")]
        pub source: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::ContactType>,
        #[yaserde(prefix = "wsd", rename = "MailTo")]
        pub mail_to: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailFrom")]
        pub mail_from: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailCC")]
        pub mail_cc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "QueueType")]
        pub queue_type: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "OpenTime")]
        pub open_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClosedTime")]
        pub closed_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AutoResponse")]
        pub auto_response: Option<mod_wsd::AutoResponse>,
        #[yaserde(prefix = "wsd", rename = "Rule")]
        pub rule: Option<mod_wsd::Rule>,
        #[yaserde(prefix = "wsd", rename = "Priority")]
        pub priority: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OpenDuration")]
        pub open_duration: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SubStatus")]
        pub sub_status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "CMFStatus")]
        pub cmf_status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "CallId")]
        pub call_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "WebOnHoldTag")]
        pub web_on_hold_tag: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TimeZone")]
        pub time_zone: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CharSet")]
        pub char_set: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClickStreamText")]
        pub click_stream_text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NNCCApplicationId")]
        pub nncc_application_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OutBoundOriginator")]
        pub out_bound_originator: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Campaign")]
        pub campaign: Option<mod_wsd::Campaign>,
        #[yaserde(prefix = "wsd", rename = "NoOfCallAttempts")]
        pub no_of_call_attempts: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CampaignRetryCount")]
        pub campaign_retry_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DispositionCode")]
        pub disposition_code: Option<mod_wsd::DispositionCode>,
        #[yaserde(prefix = "wsd", rename = "PreferredCallBackMedia")]
        pub preferred_call_back_media: Option<mod_wsd::ContactType>,
        #[yaserde(prefix = "wsd", rename = "CallTime")]
        pub call_time: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DialTime")]
        pub dial_time: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastAction")]
        pub last_action: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "ClosureType")]
        pub closure_type: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "CmfId")]
        pub cmf_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CustomFields")]
        pub custom_fields: Option<mod_wsd::ArrayOfCustomField>,
        #[yaserde(prefix = "wsd", rename = "Actions")]
        pub actions: Option<mod_wsd::ArrayOfAction>,
        #[yaserde(prefix = "wsd", rename = "Answers")]
        pub answers: Option<mod_wsd::ArrayOfAnswer>,
        #[yaserde(prefix = "wsd", rename = "ClosedReason")]
        pub closed_reason: Option<mod_wsd::ClosedReason>,
        #[yaserde(prefix = "wsd", rename = "RouteSummary")]
        pub route_summary: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CCOpenQueuedTime")]
        pub cc_open_queued_time: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Importance")]
        pub importance: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SocialMediaHeader")]
        pub social_media_header: Option<mod_wsd::SocialMediaHeader>,
        #[yaserde(prefix = "wsd", rename = "ApprovalCheck")]
        pub approval_check: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillsetId")]
        pub approval_skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SMFToken")]
        pub smf_token: Option<String>,
        #[yaserde(prefix = "wsd", rename = "StartingEWT")]
        pub starting_ewt: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "StartingPIQ")]
        pub starting_piq: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Contact {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.arrival_time.check_restrictions(restrictions.clone())?;
            self.original_subject.check_restrictions(restrictions.clone())?;
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.agent.check_restrictions(restrictions.clone())?;
            self.skillset.check_restrictions(restrictions.clone())?;
            self.acquired_time.check_restrictions(restrictions.clone())?;
            self.source.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.mail_to.check_restrictions(restrictions.clone())?;
            self.mail_from.check_restrictions(restrictions.clone())?;
            self.mail_cc.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.queue_type.check_restrictions(restrictions.clone())?;
            self.open_time.check_restrictions(restrictions.clone())?;
            self.closed_time.check_restrictions(restrictions.clone())?;
            self.auto_response.check_restrictions(restrictions.clone())?;
            self.rule.check_restrictions(restrictions.clone())?;
            self.priority.check_restrictions(restrictions.clone())?;
            self.open_duration.check_restrictions(restrictions.clone())?;
            self.sub_status.check_restrictions(restrictions.clone())?;
            self.cmf_status.check_restrictions(restrictions.clone())?;
            self.call_id.check_restrictions(restrictions.clone())?;
            self.web_on_hold_tag.check_restrictions(restrictions.clone())?;
            self.time_zone.check_restrictions(restrictions.clone())?;
            self.char_set.check_restrictions(restrictions.clone())?;
            self.click_stream_text.check_restrictions(restrictions.clone())?;
            self.nncc_application_id.check_restrictions(restrictions.clone())?;
            self.out_bound_originator.check_restrictions(restrictions.clone())?;
            self.campaign.check_restrictions(restrictions.clone())?;
            self.no_of_call_attempts.check_restrictions(restrictions.clone())?;
            self.campaign_retry_count.check_restrictions(restrictions.clone())?;
            self.disposition_code.check_restrictions(restrictions.clone())?;
            self.preferred_call_back_media
                .check_restrictions(restrictions.clone())?;
            self.call_time.check_restrictions(restrictions.clone())?;
            self.dial_time.check_restrictions(restrictions.clone())?;
            self.last_action.check_restrictions(restrictions.clone())?;
            self.closure_type.check_restrictions(restrictions.clone())?;
            self.cmf_id.check_restrictions(restrictions.clone())?;
            self.custom_fields.check_restrictions(restrictions.clone())?;
            self.actions.check_restrictions(restrictions.clone())?;
            self.answers.check_restrictions(restrictions.clone())?;
            self.closed_reason.check_restrictions(restrictions.clone())?;
            self.route_summary.check_restrictions(restrictions.clone())?;
            self.cc_open_queued_time.check_restrictions(restrictions.clone())?;
            self.importance.check_restrictions(restrictions.clone())?;
            self.social_media_header.check_restrictions(restrictions.clone())?;
            self.approval_check.check_restrictions(restrictions.clone())?;
            self.approval_skillset_id.check_restrictions(restrictions.clone())?;
            self.smf_token.check_restrictions(restrictions.clone())?;
            self.starting_ewt.check_restrictions(restrictions.clone())?;
            self.starting_piq.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "User")]
    pub struct User {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CCMSID")]
        pub ccmsid: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Supervisor")]
        pub supervisor: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TelsetLogonId")]
        pub telset_logon_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserClass")]
        pub user_class: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RoutePoint")]
        pub route_point: Option<mod_wsd::RoutePoint>,
        #[yaserde(prefix = "wsd", rename = "Email")]
        pub email: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Fax")]
        pub fax: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LogonId")]
        pub logon_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SipUri")]
        pub sip_uri: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SipTerminal")]
        pub sip_terminal: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletedOnCCMS")]
        pub deleted_on_ccms: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveAgent")]
        pub predictive_agent: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveReportingEnabled")]
        pub predictive_reporting_enabled: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveWrapTimeLimit")]
        pub predictive_wrap_time_limit: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DataPurge")]
        pub data_purge: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "DefaultPassword")]
        pub default_password: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "AgentGreetingState")]
        pub agent_greeting_state: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "BlendingStatus")]
        pub blending_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastBlendedTime")]
        pub last_blended_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "BlendSkillsetId")]
        pub blend_skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AgentURL")]
        pub agent_url: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastCCADBlendedTime")]
        pub last_ccad_blended_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "BlendedEligiblityTime")]
        pub blended_eligiblity_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NailedUp")]
        pub nailed_up: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Blended")]
        pub blended: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "BlendedReason")]
        pub blended_reason: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ACDLogin")]
        pub acd_login: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Extension")]
        pub extension: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ApprovalRatio")]
        pub approval_ratio: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastApprovalUpdate")]
        pub last_approval_update: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CurrentPOMSkillsetId")]
        pub current_pom_skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "POMAgent")]
        pub pom_agent: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SkillsetUpdateTime")]
        pub skillset_update_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FriendlyName")]
        pub friendly_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SoftPhoneEnabled")]
        pub soft_phone_enabled: Option<bool>,
    }
    impl restrictions::CheckRestrictions for User {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.ccmsid.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.supervisor.check_restrictions(restrictions.clone())?;
            self.telset_logon_id.check_restrictions(restrictions.clone())?;
            self.user_class.check_restrictions(restrictions.clone())?;
            self.route_point.check_restrictions(restrictions.clone())?;
            self.email.check_restrictions(restrictions.clone())?;
            self.fax.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.logon_id.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.sip_uri.check_restrictions(restrictions.clone())?;
            self.sip_terminal.check_restrictions(restrictions.clone())?;
            self.deleted_on_ccms.check_restrictions(restrictions.clone())?;
            self.predictive_agent.check_restrictions(restrictions.clone())?;
            self.predictive_reporting_enabled
                .check_restrictions(restrictions.clone())?;
            self.predictive_wrap_time_limit
                .check_restrictions(restrictions.clone())?;
            self.data_purge.check_restrictions(restrictions.clone())?;
            self.default_password.check_restrictions(restrictions.clone())?;
            self.agent_greeting_state.check_restrictions(restrictions.clone())?;
            self.blending_status.check_restrictions(restrictions.clone())?;
            self.last_blended_time.check_restrictions(restrictions.clone())?;
            self.blend_skillset_id.check_restrictions(restrictions.clone())?;
            self.agent_url.check_restrictions(restrictions.clone())?;
            self.last_ccad_blended_time.check_restrictions(restrictions.clone())?;
            self.blended_eligiblity_time.check_restrictions(restrictions.clone())?;
            self.nailed_up.check_restrictions(restrictions.clone())?;
            self.blended.check_restrictions(restrictions.clone())?;
            self.blended_reason.check_restrictions(restrictions.clone())?;
            self.acd_login.check_restrictions(restrictions.clone())?;
            self.extension.check_restrictions(restrictions.clone())?;
            self.approval_ratio.check_restrictions(restrictions.clone())?;
            self.last_approval_update.check_restrictions(restrictions.clone())?;
            self.current_pom_skillset_id.check_restrictions(restrictions.clone())?;
            self.pom_agent.check_restrictions(restrictions.clone())?;
            self.skillset_update_time.check_restrictions(restrictions.clone())?;
            self.friendly_name.check_restrictions(restrictions.clone())?;
            self.soft_phone_enabled.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RoutePoint")]
    pub struct RoutePoint {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RoutePoint")]
        pub route_point: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
    }
    impl restrictions::CheckRestrictions for RoutePoint {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.route_point.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Skillset")]
    pub struct Skillset {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CCMSID")]
        pub ccmsid: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ActivityCode")]
        pub activity_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AutoSignature")]
        pub auto_signature: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "MailBox")]
        pub mail_box: Option<mod_wsd::Inbox>,
        #[yaserde(prefix = "wsd", rename = "RoutePoint")]
        pub route_point: Option<mod_wsd::RoutePoint>,
        #[yaserde(prefix = "wsd", rename = "WebCommsComfortGroup")]
        pub web_comms_comfort_group: Option<mod_wsd::WebCommsComfortGroup>,
        #[yaserde(prefix = "wsd", rename = "WebCommOnHoldGroup")]
        pub web_comm_on_hold_group: Option<mod_wsd::WebCommsComfortGroup>,
        #[yaserde(prefix = "wsd", rename = "Mapping")]
        pub mapping: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Agents")]
        pub agents: Option<mod_wsd::ArrayOfUserPairOfAgentsKeyUser>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Threshold")]
        pub threshold: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "UseOriginalAddress")]
        pub use_original_address: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "WebDescription")]
        pub web_description: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletedOnCCMS")]
        pub deleted_on_ccms: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PagePushURLs")]
        pub page_push_ur_ls: Option<mod_wsd::ArrayOfPagePushURL>,
        #[yaserde(prefix = "wsd", rename = "Phrases")]
        pub phrases: Option<mod_wsd::ArrayOfPhrase>,
        #[yaserde(prefix = "wsd", rename = "WelcomeMessage")]
        pub welcome_message: Option<String>,
        #[yaserde(prefix = "wsd", rename = "WrapUpMessage")]
        pub wrap_up_message: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Expert")]
        pub expert: Option<mod_wsd::ArrayOfExpert>,
        #[yaserde(prefix = "wsd", rename = "SkillsetThresholdTemplate")]
        pub skillset_threshold_template: Option<mod_wsd::SkillsetThresholdTemplate>,
        #[yaserde(prefix = "wsd", rename = "Calendar")]
        pub calendar: Option<mod_wsd::Calendar>,
        #[yaserde(prefix = "wsd", rename = "ChatHistoryHeader")]
        pub chat_history_header: Option<mod_wsd::AutoResponse>,
        #[yaserde(prefix = "wsd", rename = "WebOnHoldURLs")]
        pub web_on_hold_ur_ls: Option<mod_wsd::OnHoldURL>,
        #[yaserde(prefix = "wsd", rename = "ConcurrentWebChats")]
        pub concurrent_web_chats: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalRatio")]
        pub approval_ratio: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastApprovalUpdate")]
        pub last_approval_update: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillset1")]
        pub approval_skillset_1: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillset2")]
        pub approval_skillset_2: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillset3")]
        pub approval_skillset_3: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillset4")]
        pub approval_skillset_4: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ApprovalSkillset5")]
        pub approval_skillset_5: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RejectionLevel1")]
        pub rejection_level_1: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RejectionLevel2")]
        pub rejection_level_2: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RejectionLevel3")]
        pub rejection_level_3: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RejectionLevel4")]
        pub rejection_level_4: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RejectionLevel5")]
        pub rejection_level_5: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RSMStatus")]
        pub rsm_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AutoRejectKeywordGroupID")]
        pub auto_reject_keyword_group_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Skillset {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.ccmsid.check_restrictions(restrictions.clone())?;
            self.activity_code.check_restrictions(restrictions.clone())?;
            self.auto_signature.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.mail_box.check_restrictions(restrictions.clone())?;
            self.route_point.check_restrictions(restrictions.clone())?;
            self.web_comms_comfort_group.check_restrictions(restrictions.clone())?;
            self.web_comm_on_hold_group.check_restrictions(restrictions.clone())?;
            self.mapping.check_restrictions(restrictions.clone())?;
            self.agents.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.threshold.check_restrictions(restrictions.clone())?;
            self.use_original_address.check_restrictions(restrictions.clone())?;
            self.web_description.check_restrictions(restrictions.clone())?;
            self.deleted_on_ccms.check_restrictions(restrictions.clone())?;
            self.page_push_ur_ls.check_restrictions(restrictions.clone())?;
            self.phrases.check_restrictions(restrictions.clone())?;
            self.welcome_message.check_restrictions(restrictions.clone())?;
            self.wrap_up_message.check_restrictions(restrictions.clone())?;
            self.expert.check_restrictions(restrictions.clone())?;
            self.skillset_threshold_template
                .check_restrictions(restrictions.clone())?;
            self.calendar.check_restrictions(restrictions.clone())?;
            self.chat_history_header.check_restrictions(restrictions.clone())?;
            self.web_on_hold_ur_ls.check_restrictions(restrictions.clone())?;
            self.concurrent_web_chats.check_restrictions(restrictions.clone())?;
            self.approval_ratio.check_restrictions(restrictions.clone())?;
            self.last_approval_update.check_restrictions(restrictions.clone())?;
            self.approval_skillset_1.check_restrictions(restrictions.clone())?;
            self.approval_skillset_2.check_restrictions(restrictions.clone())?;
            self.approval_skillset_3.check_restrictions(restrictions.clone())?;
            self.approval_skillset_4.check_restrictions(restrictions.clone())?;
            self.approval_skillset_5.check_restrictions(restrictions.clone())?;
            self.rejection_level_1.check_restrictions(restrictions.clone())?;
            self.rejection_level_2.check_restrictions(restrictions.clone())?;
            self.rejection_level_3.check_restrictions(restrictions.clone())?;
            self.rejection_level_4.check_restrictions(restrictions.clone())?;
            self.rejection_level_5.check_restrictions(restrictions.clone())?;
            self.rsm_status.check_restrictions(restrictions.clone())?;
            self.auto_reject_keyword_group_id
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Inbox")]
    pub struct Inbox {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "InboundMailThreshold")]
        pub inbound_mail_threshold: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DomainName")]
        pub domain_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Enabled")]
        pub enabled: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OutBoundMailThreshold")]
        pub out_bound_mail_threshold: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Server")]
        pub server: Option<mod_wsd::Server>,
        #[yaserde(prefix = "wsd", rename = "SmtpAuthLogon")]
        pub smtp_auth_logon: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SMTPServer")]
        pub smtp_server: Option<mod_wsd::Server>,
        #[yaserde(prefix = "wsd", rename = "WinNTAccount")]
        pub win_nt_account: Option<String>,
        #[yaserde(prefix = "wsd", rename = "StripLeadingDigits")]
        pub strip_leading_digits: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "StripTrailingCharacters")]
        pub strip_trailing_characters: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "RuleGroupID")]
        pub rule_group_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ContactType")]
        pub contact_type: Option<mod_wsd::ContactType>,
        #[yaserde(prefix = "wsd", rename = "LastAccessedUtcTime")]
        pub last_accessed_utc_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastAccessedError")]
        pub last_accessed_error: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastAccessedCount")]
        pub last_accessed_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastAccessed")]
        pub last_accessed: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Inbox {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.inbound_mail_threshold.check_restrictions(restrictions.clone())?;
            self.display_name.check_restrictions(restrictions.clone())?;
            self.domain_name.check_restrictions(restrictions.clone())?;
            self.enabled.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.out_bound_mail_threshold.check_restrictions(restrictions.clone())?;
            self.server.check_restrictions(restrictions.clone())?;
            self.smtp_auth_logon.check_restrictions(restrictions.clone())?;
            self.smtp_server.check_restrictions(restrictions.clone())?;
            self.win_nt_account.check_restrictions(restrictions.clone())?;
            self.strip_leading_digits.check_restrictions(restrictions.clone())?;
            self.strip_trailing_characters
                .check_restrictions(restrictions.clone())?;
            self.rule_group_id.check_restrictions(restrictions.clone())?;
            self.contact_type.check_restrictions(restrictions.clone())?;
            self.last_accessed_utc_time.check_restrictions(restrictions.clone())?;
            self.last_accessed_error.check_restrictions(restrictions.clone())?;
            self.last_accessed_count.check_restrictions(restrictions.clone())?;
            self.last_accessed.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Server")]
    pub struct Server {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: i64,
        #[yaserde(prefix = "wsd", rename = "Auth")]
        pub auth: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "BackUpHost")]
        pub back_up_host: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Port")]
        pub port: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NewType")]
        pub new_type: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Protocol")]
        pub protocol: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Encryption")]
        pub encryption: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Server {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.auth.check_restrictions(restrictions.clone())?;
            self.back_up_host.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.port.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.new_type.check_restrictions(restrictions.clone())?;
            self.protocol.check_restrictions(restrictions.clone())?;
            self.encryption.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ContactType")]
    pub struct ContactType {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NumericValue")]
        pub numeric_value: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TextValue")]
        pub text_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "DefaultSkillset")]
        pub default_skillset: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Parent")]
        pub parent: Option<multi_ref::MultiRef<mod_wsd::ContactType>>,
        #[yaserde(prefix = "wsd", rename = "DefaultClosedReasonID")]
        pub default_closed_reason_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ContactType {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.numeric_value.check_restrictions(restrictions.clone())?;
            self.text_value.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.default_skillset.check_restrictions(restrictions.clone())?;
            self.parent.check_restrictions(restrictions.clone())?;
            self.default_closed_reason_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "WebCommsComfortGroup")]
    pub struct WebCommsComfortGroup {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "GroupName")]
        pub group_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "WebCommsComfortMessages")]
        pub web_comms_comfort_messages: Option<mod_wsd::ArrayOfWebCommsComfortMessage>,
    }
    impl restrictions::CheckRestrictions for WebCommsComfortGroup {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.group_name.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.web_comms_comfort_messages
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfWebCommsComfortMessage")]
    pub struct ArrayOfWebCommsComfortMessage {
        #[yaserde(prefix = "wsd", rename = "WebCommsComfortMessage")]
        pub web_comms_comfort_message: Vec<mod_wsd::WebCommsComfortMessage>,
    }
    impl restrictions::CheckRestrictions for ArrayOfWebCommsComfortMessage {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.web_comms_comfort_message
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "WebCommsComfortMessage")]
    pub struct WebCommsComfortMessage {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Message")]
        pub message: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Delay")]
        pub delay: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Sequence")]
        pub sequence: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "WebCommsComfortGroupID")]
        pub web_comms_comfort_group_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for WebCommsComfortMessage {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.message.check_restrictions(restrictions.clone())?;
            self.delay.check_restrictions(restrictions.clone())?;
            self.sequence.check_restrictions(restrictions.clone())?;
            self.web_comms_comfort_group_id
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfUserPairOfAgentsKeyUser")]
    pub struct ArrayOfUserPairOfAgentsKeyUser {
        #[yaserde(prefix = "wsd", rename = "User")]
        pub user: Vec<mod_wsd::PairOfAgentsKeyUser>,
    }
    impl restrictions::CheckRestrictions for ArrayOfUserPairOfAgentsKeyUser {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.user.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "PairOfAgentsKeyUser")]
    pub struct PairOfAgentsKeyUser {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CCMSID")]
        pub ccmsid: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Supervisor")]
        pub supervisor: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TelsetLogonId")]
        pub telset_logon_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserClass")]
        pub user_class: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RoutePoint")]
        pub route_point: Option<mod_wsd::RoutePoint>,
        #[yaserde(prefix = "wsd", rename = "Email")]
        pub email: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Fax")]
        pub fax: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LogonId")]
        pub logon_id: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SipUri")]
        pub sip_uri: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SipTerminal")]
        pub sip_terminal: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DeletedOnCCMS")]
        pub deleted_on_ccms: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveAgent")]
        pub predictive_agent: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveReportingEnabled")]
        pub predictive_reporting_enabled: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "PredictiveWrapTimeLimit")]
        pub predictive_wrap_time_limit: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DataPurge")]
        pub data_purge: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "DefaultPassword")]
        pub default_password: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "AgentGreetingState")]
        pub agent_greeting_state: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "BlendingStatus")]
        pub blending_status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastBlendedTime")]
        pub last_blended_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "BlendSkillsetId")]
        pub blend_skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AgentURL")]
        pub agent_url: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastCCADBlendedTime")]
        pub last_ccad_blended_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "BlendedEligiblityTime")]
        pub blended_eligiblity_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NailedUp")]
        pub nailed_up: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Blended")]
        pub blended: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "BlendedReason")]
        pub blended_reason: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ACDLogin")]
        pub acd_login: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Extension")]
        pub extension: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ApprovalRatio")]
        pub approval_ratio: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastApprovalUpdate")]
        pub last_approval_update: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CurrentPOMSkillsetId")]
        pub current_pom_skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "POMAgent")]
        pub pom_agent: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SkillsetUpdateTime")]
        pub skillset_update_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FriendlyName")]
        pub friendly_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SoftPhoneEnabled")]
        pub soft_phone_enabled: Option<bool>,
    }
    impl restrictions::CheckRestrictions for PairOfAgentsKeyUser {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.ccmsid.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.supervisor.check_restrictions(restrictions.clone())?;
            self.telset_logon_id.check_restrictions(restrictions.clone())?;
            self.user_class.check_restrictions(restrictions.clone())?;
            self.route_point.check_restrictions(restrictions.clone())?;
            self.email.check_restrictions(restrictions.clone())?;
            self.fax.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.logon_id.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.sip_uri.check_restrictions(restrictions.clone())?;
            self.sip_terminal.check_restrictions(restrictions.clone())?;
            self.deleted_on_ccms.check_restrictions(restrictions.clone())?;
            self.predictive_agent.check_restrictions(restrictions.clone())?;
            self.predictive_reporting_enabled
                .check_restrictions(restrictions.clone())?;
            self.predictive_wrap_time_limit
                .check_restrictions(restrictions.clone())?;
            self.data_purge.check_restrictions(restrictions.clone())?;
            self.default_password.check_restrictions(restrictions.clone())?;
            self.agent_greeting_state.check_restrictions(restrictions.clone())?;
            self.blending_status.check_restrictions(restrictions.clone())?;
            self.last_blended_time.check_restrictions(restrictions.clone())?;
            self.blend_skillset_id.check_restrictions(restrictions.clone())?;
            self.agent_url.check_restrictions(restrictions.clone())?;
            self.last_ccad_blended_time.check_restrictions(restrictions.clone())?;
            self.blended_eligiblity_time.check_restrictions(restrictions.clone())?;
            self.nailed_up.check_restrictions(restrictions.clone())?;
            self.blended.check_restrictions(restrictions.clone())?;
            self.blended_reason.check_restrictions(restrictions.clone())?;
            self.acd_login.check_restrictions(restrictions.clone())?;
            self.extension.check_restrictions(restrictions.clone())?;
            self.approval_ratio.check_restrictions(restrictions.clone())?;
            self.last_approval_update.check_restrictions(restrictions.clone())?;
            self.current_pom_skillset_id.check_restrictions(restrictions.clone())?;
            self.pom_agent.check_restrictions(restrictions.clone())?;
            self.skillset_update_time.check_restrictions(restrictions.clone())?;
            self.friendly_name.check_restrictions(restrictions.clone())?;
            self.soft_phone_enabled.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfPagePushURL")]
    pub struct ArrayOfPagePushURL {
        #[yaserde(prefix = "wsd", rename = "PagePushURL")]
        pub page_push_url: Vec<mod_wsd::PagePushURL>,
    }
    impl restrictions::CheckRestrictions for ArrayOfPagePushURL {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.page_push_url.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "PagePushURL")]
    pub struct PagePushURL {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Description")]
        pub description: Option<String>,
        #[yaserde(prefix = "wsd", rename = "URL")]
        pub url: Option<String>,
    }
    impl restrictions::CheckRestrictions for PagePushURL {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.description.check_restrictions(restrictions.clone())?;
            self.url.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfPhrase")]
    pub struct ArrayOfPhrase {
        #[yaserde(prefix = "wsd", rename = "Phrase")]
        pub phrase: Vec<mod_wsd::Phrase>,
    }
    impl restrictions::CheckRestrictions for ArrayOfPhrase {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.phrase.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Phrase")]
    pub struct Phrase {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Text")]
        pub text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
    }
    impl restrictions::CheckRestrictions for Phrase {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.text.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfExpert")]
    pub struct ArrayOfExpert {
        #[yaserde(prefix = "wsd", rename = "Expert")]
        pub expert: Vec<mod_wsd::Expert>,
    }
    impl restrictions::CheckRestrictions for ArrayOfExpert {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.expert.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Expert")]
    pub struct Expert {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Custom1")]
        pub custom_1: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Custom2")]
        pub custom_2: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ExpertName")]
        pub expert_name: String,
        #[yaserde(prefix = "wsd", rename = "ExpertSipUri")]
        pub expert_sip_uri: String,
        #[yaserde(prefix = "wsd", rename = "Group")]
        pub group: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Phone")]
        pub phone: Option<String>,
    }
    impl restrictions::CheckRestrictions for Expert {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.custom_1.check_restrictions(restrictions.clone())?;
            self.custom_2.check_restrictions(restrictions.clone())?;
            self.expert_name.check_restrictions(restrictions.clone())?;
            self.expert_sip_uri.check_restrictions(restrictions.clone())?;
            self.group.check_restrictions(restrictions.clone())?;
            self.phone.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SkillsetThresholdTemplate")]
    pub struct SkillsetThresholdTemplate {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CCMSID")]
        pub ccmsid: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ThresholdToMonitor")]
        pub threshold_to_monitor: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "AgentsToAssignVoice")]
        pub agents_to_assign_voice: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AgentsToAssignPredictive")]
        pub agents_to_assign_predictive: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ThresholdLevels")]
        pub threshold_levels: Option<mod_wsd::ArrayOfThresholdLevel>,
    }
    impl restrictions::CheckRestrictions for SkillsetThresholdTemplate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.ccmsid.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.threshold_to_monitor.check_restrictions(restrictions.clone())?;
            self.agents_to_assign_voice.check_restrictions(restrictions.clone())?;
            self.agents_to_assign_predictive
                .check_restrictions(restrictions.clone())?;
            self.threshold_levels.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfThresholdLevel")]
    pub struct ArrayOfThresholdLevel {
        #[yaserde(prefix = "wsd", rename = "ThresholdLevel")]
        pub threshold_level: Vec<mod_wsd::ThresholdLevel>,
    }
    impl restrictions::CheckRestrictions for ArrayOfThresholdLevel {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.threshold_level.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ThresholdLevel")]
    pub struct ThresholdLevel {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "BlendingThreshold")]
        pub blending_threshold: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Level1")]
        pub level_1: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Level2")]
        pub level_2: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Enabled")]
        pub enabled: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "ThresholdTemplateID")]
        pub threshold_template_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ThresholdLevel {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.blending_threshold.check_restrictions(restrictions.clone())?;
            self.level_1.check_restrictions(restrictions.clone())?;
            self.level_2.check_restrictions(restrictions.clone())?;
            self.enabled.check_restrictions(restrictions.clone())?;
            self.threshold_template_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Calendar")]
    pub struct Calendar {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Breaks")]
        pub breaks: Option<mod_wsd::ArrayOfBreak>,
    }
    impl restrictions::CheckRestrictions for Calendar {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.breaks.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfBreak")]
    pub struct ArrayOfBreak {
        #[yaserde(prefix = "wsd", rename = "Break")]
        pub r#break: Vec<mod_wsd::Break>,
    }
    impl restrictions::CheckRestrictions for ArrayOfBreak {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.r#break.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Break")]
    pub struct Break {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DayOfWeek")]
        pub day_of_week: Option<String>,
        #[yaserde(prefix = "wsd", rename = "HolidayDate")]
        pub holiday_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "HolidayName")]
        pub holiday_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "StartTime")]
        pub start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "EndTime")]
        pub end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CalendarID")]
        pub calendar_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Break {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.day_of_week.check_restrictions(restrictions.clone())?;
            self.holiday_date.check_restrictions(restrictions.clone())?;
            self.holiday_name.check_restrictions(restrictions.clone())?;
            self.start_time.check_restrictions(restrictions.clone())?;
            self.end_time.check_restrictions(restrictions.clone())?;
            self.calendar_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AutoResponse")]
    pub struct AutoResponse {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Subject")]
        pub subject: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Body")]
        pub body: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Suggestion")]
        pub suggestion: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Attachments")]
        pub attachments: Option<mod_wsd::ArrayOfAutoResponseAttachment>,
        #[yaserde(prefix = "wsd", rename = "Categories")]
        pub categories: Option<mod_wsd::ArrayOfCategory>,
        #[yaserde(prefix = "wsd", rename = "Frequency")]
        pub frequency: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "BodyHTML")]
        pub body_html: Option<String>,
    }
    impl restrictions::CheckRestrictions for AutoResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.subject.check_restrictions(restrictions.clone())?;
            self.body.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.suggestion.check_restrictions(restrictions.clone())?;
            self.attachments.check_restrictions(restrictions.clone())?;
            self.categories.check_restrictions(restrictions.clone())?;
            self.frequency.check_restrictions(restrictions.clone())?;
            self.body_html.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAutoResponseAttachment")]
    pub struct ArrayOfAutoResponseAttachment {
        #[yaserde(prefix = "wsd", rename = "AutoResponseAttachment")]
        pub auto_response_attachment: Vec<mod_wsd::AutoResponseAttachment>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAutoResponseAttachment {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.auto_response_attachment.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "AutoResponseAttachment")]
    pub struct AutoResponseAttachment {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DisplayFileName")]
        pub display_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InternalFileName")]
        pub internal_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Inline")]
        pub inline: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "URL")]
        pub url: Option<String>,
    }
    impl restrictions::CheckRestrictions for AutoResponseAttachment {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.display_file_name.check_restrictions(restrictions.clone())?;
            self.internal_file_name.check_restrictions(restrictions.clone())?;
            self.inline.check_restrictions(restrictions.clone())?;
            self.url.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCategory")]
    pub struct ArrayOfCategory {
        #[yaserde(prefix = "wsd", rename = "Category")]
        pub category: Vec<mod_wsd::Category>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCategory {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.category.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Category")]
    pub struct Category {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
    }
    impl restrictions::CheckRestrictions for Category {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "OnHoldURL")]
    pub struct OnHoldURL {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Description")]
        pub description: Option<String>,
        #[yaserde(prefix = "wsd", rename = "URL")]
        pub url: Option<String>,
        #[yaserde(prefix = "wsd", rename = "HoldTime")]
        pub hold_time: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Tag")]
        pub tag: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Sequence")]
        pub sequence: Option<i64>,
    }
    impl restrictions::CheckRestrictions for OnHoldURL {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.description.check_restrictions(restrictions.clone())?;
            self.url.check_restrictions(restrictions.clone())?;
            self.hold_time.check_restrictions(restrictions.clone())?;
            self.tag.check_restrictions(restrictions.clone())?;
            self.sequence.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Rule")]
    pub struct Rule {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "AutoResponseID")]
        pub auto_response_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SkillsetID")]
        pub skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Sequence")]
        pub sequence: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "IsClosed")]
        pub is_closed: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Priority")]
        pub priority: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "UseOutOfHoursRule")]
        pub use_out_of_hours_rule: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SkillsetReset")]
        pub skillset_reset: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "LastModifiedTime")]
        pub last_modified_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RuleKeywordCriteria")]
        pub rule_keyword_criteria: Option<mod_wsd::ArrayOfRuleKeywordCriterion>,
        #[yaserde(prefix = "wsd", rename = "AutoSuggests")]
        pub auto_suggests: Option<mod_wsd::ArrayOfAutoResponse>,
        #[yaserde(prefix = "wsd", rename = "RuleGroupID")]
        pub rule_group_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "OpenInterfaceMethod")]
        pub open_interface_method: Option<mod_wsd::OpenInterfaceMethod>,
    }
    impl restrictions::CheckRestrictions for Rule {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.auto_response_id.check_restrictions(restrictions.clone())?;
            self.skillset_id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.sequence.check_restrictions(restrictions.clone())?;
            self.is_closed.check_restrictions(restrictions.clone())?;
            self.priority.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.use_out_of_hours_rule.check_restrictions(restrictions.clone())?;
            self.skillset_reset.check_restrictions(restrictions.clone())?;
            self.last_modified_time.check_restrictions(restrictions.clone())?;
            self.rule_keyword_criteria.check_restrictions(restrictions.clone())?;
            self.auto_suggests.check_restrictions(restrictions.clone())?;
            self.rule_group_id.check_restrictions(restrictions.clone())?;
            self.open_interface_method.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfRuleKeywordCriterion")]
    pub struct ArrayOfRuleKeywordCriterion {
        #[yaserde(prefix = "wsd", rename = "RuleKeywordCriterion")]
        pub rule_keyword_criterion: Vec<mod_wsd::RuleKeywordCriterion>,
    }
    impl restrictions::CheckRestrictions for ArrayOfRuleKeywordCriterion {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.rule_keyword_criterion.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RuleKeywordCriterion")]
    pub struct RuleKeywordCriterion {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MatchTypeID")]
        pub match_type_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Weighting")]
        pub weighting: Option<f64>,
        #[yaserde(prefix = "wsd", rename = "RuleKeywordGroups")]
        pub rule_keyword_groups: Option<mod_wsd::ArrayOfRuleKeywordGroup>,
        #[yaserde(prefix = "wsd", rename = "EmailAddressGroupID")]
        pub email_address_group_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastModifiedTime")]
        pub last_modified_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RuleQuery")]
        pub rule_query: Option<String>,
    }
    impl restrictions::CheckRestrictions for RuleKeywordCriterion {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.match_type_id.check_restrictions(restrictions.clone())?;
            self.weighting.check_restrictions(restrictions.clone())?;
            self.rule_keyword_groups.check_restrictions(restrictions.clone())?;
            self.email_address_group_id.check_restrictions(restrictions.clone())?;
            self.last_modified_time.check_restrictions(restrictions.clone())?;
            self.rule_query.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfRuleKeywordGroup")]
    pub struct ArrayOfRuleKeywordGroup {
        #[yaserde(prefix = "wsd", rename = "RuleKeywordGroup")]
        pub rule_keyword_group: Vec<mod_wsd::RuleKeywordGroup>,
    }
    impl restrictions::CheckRestrictions for ArrayOfRuleKeywordGroup {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.rule_keyword_group.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RuleKeywordGroup")]
    pub struct RuleKeywordGroup {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "KeyWordGroupID")]
        pub key_word_group_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Operator")]
        pub operator: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Sequence")]
        pub sequence: Option<i64>,
    }
    impl restrictions::CheckRestrictions for RuleKeywordGroup {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.key_word_group_id.check_restrictions(restrictions.clone())?;
            self.operator.check_restrictions(restrictions.clone())?;
            self.sequence.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAutoResponse")]
    pub struct ArrayOfAutoResponse {
        #[yaserde(prefix = "wsd", rename = "AutoResponse")]
        pub auto_response: Vec<mod_wsd::AutoResponse>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAutoResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.auto_response.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "OpenInterfaceMethod")]
    pub struct OpenInterfaceMethod {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MethodName")]
        pub method_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InputParameterName")]
        pub input_parameter_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InputParameterArrayItem")]
        pub input_parameter_array_item: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InputParameterNamespace")]
        pub input_parameter_namespace: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OuputParameterName")]
        pub ouput_parameter_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InputFieldMap")]
        pub input_field_map: Option<mod_wsd::ArrayOfInputFieldMapItemLong>,
        #[yaserde(prefix = "wsd", rename = "OutputFieldMap")]
        pub output_field_map: Option<mod_wsd::ArrayOfOutputFieldMapItemLong>,
        #[yaserde(prefix = "wsd", rename = "InputFields")]
        pub input_fields: Option<mod_wsd::ListOfCodeMappings>,
        #[yaserde(prefix = "wsd", rename = "OutputFields")]
        pub output_fields: Option<mod_wsd::ListOfCodeMappings>,
        #[yaserde(prefix = "wsd", rename = "OpenInterfaceServiceID")]
        pub open_interface_service_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastModifiedUtcTime")]
        pub last_modified_utc_time: Option<String>,
    }
    impl restrictions::CheckRestrictions for OpenInterfaceMethod {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.method_name.check_restrictions(restrictions.clone())?;
            self.display_name.check_restrictions(restrictions.clone())?;
            self.input_parameter_name.check_restrictions(restrictions.clone())?;
            self.input_parameter_array_item
                .check_restrictions(restrictions.clone())?;
            self.input_parameter_namespace
                .check_restrictions(restrictions.clone())?;
            self.ouput_parameter_name.check_restrictions(restrictions.clone())?;
            self.input_field_map.check_restrictions(restrictions.clone())?;
            self.output_field_map.check_restrictions(restrictions.clone())?;
            self.input_fields.check_restrictions(restrictions.clone())?;
            self.output_fields.check_restrictions(restrictions.clone())?;
            self.open_interface_service_id
                .check_restrictions(restrictions.clone())?;
            self.last_modified_utc_time.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfInputFieldMapItemLong")]
    pub struct ArrayOfInputFieldMapItemLong {
        #[yaserde(prefix = "wsd", rename = "InputFieldMapItem")]
        pub input_field_map_item: Vec<i64>,
    }
    impl restrictions::CheckRestrictions for ArrayOfInputFieldMapItemLong {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.input_field_map_item.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfOutputFieldMapItemLong")]
    pub struct ArrayOfOutputFieldMapItemLong {
        #[yaserde(prefix = "wsd", rename = "OutputFieldMapItem")]
        pub output_field_map_item: Vec<i64>,
    }
    impl restrictions::CheckRestrictions for ArrayOfOutputFieldMapItemLong {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.output_field_map_item.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ListOfCodeMappings")]
    pub struct ListOfCodeMappings {
        #[yaserde(prefix = "wsd", rename = "CodeMappings")]
        pub code_mappings: Option<mod_wsd::ArrayOfCodeMapping>,
    }
    impl restrictions::CheckRestrictions for ListOfCodeMappings {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.code_mappings.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCodeMapping")]
    pub struct ArrayOfCodeMapping {
        #[yaserde(prefix = "wsd", rename = "CodeMapping")]
        pub code_mapping: Vec<mod_wsd::CodeMapping>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCodeMapping {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.code_mapping.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Campaign")]
    pub struct Campaign {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Description")]
        pub description: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NumberOfContacts")]
        pub number_of_contacts: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NumberOfContactsProcessed")]
        pub number_of_contacts_processed: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CampaignScript")]
        pub campaign_script: Option<mod_wsd::CampaignScript>,
        #[yaserde(prefix = "wsd", rename = "DateCreated")]
        pub date_created: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Owner")]
        pub owner: Option<String>,
        #[yaserde(prefix = "wsd", rename = "StartDateTime")]
        pub start_date_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "EndDateTime")]
        pub end_date_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DailyStartTime")]
        pub daily_start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DailyEndTime")]
        pub daily_end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Skillset")]
        pub skillset: Option<mod_wsd::Skillset>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "PreviousStatus")]
        pub previous_status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "StartOfRun")]
        pub start_of_run: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Priority")]
        pub priority: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LastLoadedDate")]
        pub last_loaded_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoadInterval")]
        pub load_interval: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "LoadContactsPerInterval")]
        pub load_contacts_per_interval: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DialingPrefix")]
        pub dialing_prefix: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AutoDialTimeOut")]
        pub auto_dial_time_out: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MinimumRingTime")]
        pub minimum_ring_time: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Locked")]
        pub locked: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "LockedBy")]
        pub locked_by: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LockedDateTime")]
        pub locked_date_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DispositionCodes")]
        pub disposition_codes: Option<mod_wsd::ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode>,
        #[yaserde(prefix = "wsd", rename = "CustomFields")]
        pub custom_fields: Option<mod_wsd::ArrayOfCustomField>,
        #[yaserde(prefix = "wsd", rename = "UseTimeZone")]
        pub use_time_zone: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NumberOfContactsClosed")]
        pub number_of_contacts_closed: Option<i64>,
    }
    impl restrictions::CheckRestrictions for Campaign {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.description.check_restrictions(restrictions.clone())?;
            self.number_of_contacts.check_restrictions(restrictions.clone())?;
            self.number_of_contacts_processed
                .check_restrictions(restrictions.clone())?;
            self.campaign_script.check_restrictions(restrictions.clone())?;
            self.date_created.check_restrictions(restrictions.clone())?;
            self.owner.check_restrictions(restrictions.clone())?;
            self.start_date_time.check_restrictions(restrictions.clone())?;
            self.end_date_time.check_restrictions(restrictions.clone())?;
            self.daily_start_time.check_restrictions(restrictions.clone())?;
            self.daily_end_time.check_restrictions(restrictions.clone())?;
            self.skillset.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.previous_status.check_restrictions(restrictions.clone())?;
            self.start_of_run.check_restrictions(restrictions.clone())?;
            self.priority.check_restrictions(restrictions.clone())?;
            self.last_loaded_date.check_restrictions(restrictions.clone())?;
            self.load_interval.check_restrictions(restrictions.clone())?;
            self.load_contacts_per_interval
                .check_restrictions(restrictions.clone())?;
            self.dialing_prefix.check_restrictions(restrictions.clone())?;
            self.auto_dial_time_out.check_restrictions(restrictions.clone())?;
            self.minimum_ring_time.check_restrictions(restrictions.clone())?;
            self.locked.check_restrictions(restrictions.clone())?;
            self.locked_by.check_restrictions(restrictions.clone())?;
            self.locked_date_time.check_restrictions(restrictions.clone())?;
            self.disposition_codes.check_restrictions(restrictions.clone())?;
            self.custom_fields.check_restrictions(restrictions.clone())?;
            self.use_time_zone.check_restrictions(restrictions.clone())?;
            self.number_of_contacts_closed
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CampaignScript")]
    pub struct CampaignScript {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DateCreated")]
        pub date_created: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Introduction")]
        pub introduction: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Questions")]
        pub questions: Option<mod_wsd::ArrayOfQuestion>,
        #[yaserde(prefix = "wsd", rename = "Conclusion")]
        pub conclusion: Option<String>,
    }
    impl restrictions::CheckRestrictions for CampaignScript {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.date_created.check_restrictions(restrictions.clone())?;
            self.introduction.check_restrictions(restrictions.clone())?;
            self.questions.check_restrictions(restrictions.clone())?;
            self.conclusion.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfQuestion")]
    pub struct ArrayOfQuestion {
        #[yaserde(prefix = "wsd", rename = "Question")]
        pub question: Vec<mod_wsd::Question>,
    }
    impl restrictions::CheckRestrictions for ArrayOfQuestion {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.question.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Question")]
    pub struct Question {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Question")]
        pub question: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AllowedAnswers")]
        pub allowed_answers: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AllowedFreeText")]
        pub allowed_free_text: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "DefaultAnswer")]
        pub default_answer: Option<String>,
        #[yaserde(prefix = "wsd", rename = "GUI")]
        pub gui: Option<mod_wsd::CodeMapping>,
    }
    impl restrictions::CheckRestrictions for Question {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.question.check_restrictions(restrictions.clone())?;
            self.allowed_answers.check_restrictions(restrictions.clone())?;
            self.allowed_free_text.check_restrictions(restrictions.clone())?;
            self.default_answer.check_restrictions(restrictions.clone())?;
            self.gui.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode")]
    pub struct ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode {
        #[yaserde(prefix = "wsd", rename = "DispositionCode")]
        pub disposition_code: Vec<mod_wsd::PairOfDispositionCodesKeyDispositionCode>,
    }
    impl restrictions::CheckRestrictions for ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.disposition_code.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "DispositionCode")]
    pub struct DispositionCode {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RetryTimeout")]
        pub retry_timeout: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NumericValue")]
        pub numeric_value: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MaxRetryCount")]
        pub max_retry_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Deletable")]
        pub deletable: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "CallRequired")]
        pub call_required: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SaveAgentScript")]
        pub save_agent_script: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::CodeMapping>,
    }
    impl restrictions::CheckRestrictions for DispositionCode {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.display_name.check_restrictions(restrictions.clone())?;
            self.retry_timeout.check_restrictions(restrictions.clone())?;
            self.numeric_value.check_restrictions(restrictions.clone())?;
            self.max_retry_count.check_restrictions(restrictions.clone())?;
            self.deletable.check_restrictions(restrictions.clone())?;
            self.call_required.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.save_agent_script.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "PairOfDispositionCodesKeyDispositionCode")]
    pub struct PairOfDispositionCodesKeyDispositionCode {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DisplayName")]
        pub display_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RetryTimeout")]
        pub retry_timeout: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "NumericValue")]
        pub numeric_value: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MaxRetryCount")]
        pub max_retry_count: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Deletable")]
        pub deletable: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "CallRequired")]
        pub call_required: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SaveAgentScript")]
        pub save_agent_script: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::CodeMapping>,
    }
    impl restrictions::CheckRestrictions for PairOfDispositionCodesKeyDispositionCode {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.display_name.check_restrictions(restrictions.clone())?;
            self.retry_timeout.check_restrictions(restrictions.clone())?;
            self.numeric_value.check_restrictions(restrictions.clone())?;
            self.max_retry_count.check_restrictions(restrictions.clone())?;
            self.deletable.check_restrictions(restrictions.clone())?;
            self.call_required.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.save_agent_script.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCustomField")]
    pub struct ArrayOfCustomField {
        #[yaserde(prefix = "wsd", rename = "CustomField")]
        pub custom_field: Vec<mod_wsd::CustomField>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCustomField {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.custom_field.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomField")]
    pub struct CustomField {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "IsTextVisible")]
        pub is_text_visible: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "Text")]
        pub text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldTemplate")]
        pub custom_field_template: Option<mod_wsd::CustomFieldTemplate>,
    }
    impl restrictions::CheckRestrictions for CustomField {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.is_text_visible.check_restrictions(restrictions.clone())?;
            self.text.check_restrictions(restrictions.clone())?;
            self.custom_field_template.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomFieldTemplate")]
    pub struct CustomFieldTemplate {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
    }
    impl restrictions::CheckRestrictions for CustomFieldTemplate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAction")]
    pub struct ArrayOfAction {
        #[yaserde(prefix = "wsd", rename = "Action")]
        pub action: Vec<mod_wsd::Action>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAction {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.action.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Action")]
    pub struct Action {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ContactId")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CreationTime")]
        pub creation_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::ContactType>,
        #[yaserde(prefix = "wsd", rename = "Source")]
        pub source: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Subject")]
        pub subject: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Text")]
        pub text: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TextHTML")]
        pub text_html: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CharSet")]
        pub char_set: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Attempt")]
        pub attempt: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DispositionCode")]
        pub disposition_code: Option<mod_wsd::DispositionCode>,
        #[yaserde(prefix = "wsd", rename = "MailTo")]
        pub mail_to: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailFrom")]
        pub mail_from: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailCC")]
        pub mail_cc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MailBCC")]
        pub mail_bcc: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallBackTime")]
        pub call_back_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallBackMedia")]
        pub call_back_media: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "CallBackStatus")]
        pub call_back_status: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "Agent")]
        pub agent: Option<mod_wsd::User>,
        #[yaserde(prefix = "wsd", rename = "Comment")]
        pub comment: Option<String>,
        #[yaserde(prefix = "wsd", rename = "TimeAllocated")]
        pub time_allocated: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "TemplateLocation")]
        pub template_location: Option<String>,
        #[yaserde(prefix = "wsd", rename = "HistoryFlag")]
        pub history_flag: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "NumberUsed")]
        pub number_used: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallStartTime")]
        pub call_start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CallEndTime")]
        pub call_end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DialStartTime")]
        pub dial_start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "DialEndTime")]
        pub dial_end_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OpenTime")]
        pub open_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ClosedTime")]
        pub closed_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Attachments")]
        pub attachments: Option<mod_wsd::ArrayOfAttachment>,
        #[yaserde(prefix = "wsd", rename = "CustomFields")]
        pub custom_fields: Option<mod_wsd::ArrayOfCustomField>,
        #[yaserde(prefix = "wsd", rename = "ClosedReason")]
        pub closed_reason: Option<mod_wsd::ClosedReason>,
        #[yaserde(prefix = "wsd", rename = "AutoSuggest")]
        pub auto_suggest: Option<mod_wsd::AutoResponse>,
        #[yaserde(prefix = "wsd", rename = "ApprovalAudits")]
        pub approval_audits: Option<mod_wsd::ArrayOfApprovalAudit>,
        #[yaserde(prefix = "wsd", rename = "ApprovalLevel")]
        pub approval_level: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ComfortMessages")]
        pub comfort_messages: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RestCall")]
        pub rest_call: Option<bool>,
    }
    impl restrictions::CheckRestrictions for Action {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.creation_time.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.source.check_restrictions(restrictions.clone())?;
            self.subject.check_restrictions(restrictions.clone())?;
            self.text.check_restrictions(restrictions.clone())?;
            self.text_html.check_restrictions(restrictions.clone())?;
            self.char_set.check_restrictions(restrictions.clone())?;
            self.attempt.check_restrictions(restrictions.clone())?;
            self.disposition_code.check_restrictions(restrictions.clone())?;
            self.mail_to.check_restrictions(restrictions.clone())?;
            self.mail_from.check_restrictions(restrictions.clone())?;
            self.mail_cc.check_restrictions(restrictions.clone())?;
            self.mail_bcc.check_restrictions(restrictions.clone())?;
            self.call_back_time.check_restrictions(restrictions.clone())?;
            self.call_back_media.check_restrictions(restrictions.clone())?;
            self.call_back_status.check_restrictions(restrictions.clone())?;
            self.agent.check_restrictions(restrictions.clone())?;
            self.comment.check_restrictions(restrictions.clone())?;
            self.time_allocated.check_restrictions(restrictions.clone())?;
            self.template_location.check_restrictions(restrictions.clone())?;
            self.history_flag.check_restrictions(restrictions.clone())?;
            self.number_used.check_restrictions(restrictions.clone())?;
            self.call_start_time.check_restrictions(restrictions.clone())?;
            self.call_end_time.check_restrictions(restrictions.clone())?;
            self.dial_start_time.check_restrictions(restrictions.clone())?;
            self.dial_end_time.check_restrictions(restrictions.clone())?;
            self.open_time.check_restrictions(restrictions.clone())?;
            self.closed_time.check_restrictions(restrictions.clone())?;
            self.attachments.check_restrictions(restrictions.clone())?;
            self.custom_fields.check_restrictions(restrictions.clone())?;
            self.closed_reason.check_restrictions(restrictions.clone())?;
            self.auto_suggest.check_restrictions(restrictions.clone())?;
            self.approval_audits.check_restrictions(restrictions.clone())?;
            self.approval_level.check_restrictions(restrictions.clone())?;
            self.comfort_messages.check_restrictions(restrictions.clone())?;
            self.rest_call.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfAttachment")]
    pub struct ArrayOfAttachment {
        #[yaserde(prefix = "wsd", rename = "Attachment")]
        pub attachment: Vec<mod_wsd::Attachment>,
    }
    impl restrictions::CheckRestrictions for ArrayOfAttachment {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.attachment.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "Attachment")]
    pub struct Attachment {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "DisplayFileName")]
        pub display_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InternalFileName")]
        pub internal_file_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Direction")]
        pub direction: Option<mod_wsd::CodeMapping>,
        #[yaserde(prefix = "wsd", rename = "URL")]
        pub url: Option<String>,
    }
    impl restrictions::CheckRestrictions for Attachment {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.display_file_name.check_restrictions(restrictions.clone())?;
            self.internal_file_name.check_restrictions(restrictions.clone())?;
            self.direction.check_restrictions(restrictions.clone())?;
            self.url.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ClosedReason")]
    pub struct ClosedReason {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: String,
        #[yaserde(prefix = "wsd", rename = "Type")]
        pub r#type: Option<mod_wsd::ContactType>,
        #[yaserde(prefix = "wsd", rename = "DeletionTimeStamp")]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MarkAsDeleted")]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "OldCodeMappingID")]
        pub old_code_mapping_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ClosedReason {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.deletion_time_stamp.check_restrictions(restrictions.clone())?;
            self.mark_as_deleted.check_restrictions(restrictions.clone())?;
            self.old_code_mapping_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfApprovalAudit")]
    pub struct ArrayOfApprovalAudit {
        #[yaserde(prefix = "wsd", rename = "ApprovalAudit")]
        pub approval_audit: Vec<mod_wsd::ApprovalAudit>,
    }
    impl restrictions::CheckRestrictions for ArrayOfApprovalAudit {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.approval_audit.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ApprovalAudit")]
    pub struct ApprovalAudit {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "UserId")]
        pub user_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "SkillsetId")]
        pub skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Level")]
        pub level: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Status")]
        pub status: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "UpdatedTimestamp")]
        pub updated_timestamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "CreatedTimestamp")]
        pub created_timestamp: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Comment")]
        pub comment: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ActionId")]
        pub action_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
    }
    impl restrictions::CheckRestrictions for ApprovalAudit {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.user_id.check_restrictions(restrictions.clone())?;
            self.skillset_id.check_restrictions(restrictions.clone())?;
            self.level.check_restrictions(restrictions.clone())?;
            self.status.check_restrictions(restrictions.clone())?;
            self.updated_timestamp.check_restrictions(restrictions.clone())?;
            self.created_timestamp.check_restrictions(restrictions.clone())?;
            self.comment.check_restrictions(restrictions.clone())?;
            self.action_id.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfSipUri")]
    pub struct ArrayOfSipUri {
        #[yaserde(prefix = "wsd", rename = "SipUri")]
        pub sip_uri: Vec<mod_wsd::SipUri>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.sip_uri.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CreateCustomer")]
    pub struct CreateCustomer {
        #[yaserde(prefix = "wsd", rename = "customerParams")]
        pub customer_params: Option<mod_wsd::CustomerCreate>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CreateCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CampaignCustomerCreate")]
    pub struct CampaignCustomerCreate {
        #[yaserde(prefix = "wsd", rename = "Title")]
        pub title: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoginPage")]
        pub login_page: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgent")]
        pub preferred_agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Password")]
        pub password: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RegisterDate")]
        pub register_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Addresses")]
        pub addresses: Option<mod_wsd::ArrayOfAddress>,
        #[yaserde(prefix = "wsd", rename = "EmailAddresses")]
        pub email_addresses: Option<mod_wsd::ArrayOfEmailAddress>,
        #[yaserde(prefix = "wsd", rename = "SipUris")]
        pub sip_uris: Option<mod_wsd::ArrayOfSipUri>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumberCreateList")]
        pub phone_number_create_list: Option<mod_wsd::ArrayOfPhoneNumberCreate>,
        #[yaserde(prefix = "wsd", rename = "ContactCreateList")]
        pub contact_create_list: Option<mod_wsd::ArrayOfContactCreate>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "Objection")]
        pub objection: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "BlockIndex")]
        pub block_index: Option<i64>,
    }
    impl restrictions::CheckRestrictions for CampaignCustomerCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.title.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.login_page.check_restrictions(restrictions.clone())?;
            self.preferred_agent.check_restrictions(restrictions.clone())?;
            self.password.check_restrictions(restrictions.clone())?;
            self.register_date.check_restrictions(restrictions.clone())?;
            self.user_name.check_restrictions(restrictions.clone())?;
            self.addresses.check_restrictions(restrictions.clone())?;
            self.email_addresses.check_restrictions(restrictions.clone())?;
            self.sip_uris.check_restrictions(restrictions.clone())?;
            self.phone_number_create_list.check_restrictions(restrictions.clone())?;
            self.contact_create_list.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            self.objection.check_restrictions(restrictions.clone())?;
            self.block_index.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerCreate")]
    pub struct CustomerCreate {
        #[yaserde(prefix = "wsd", rename = "Title")]
        pub title: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoginPage")]
        pub login_page: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgent")]
        pub preferred_agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Password")]
        pub password: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RegisterDate")]
        pub register_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Addresses")]
        pub addresses: Option<mod_wsd::ArrayOfAddress>,
        #[yaserde(prefix = "wsd", rename = "EmailAddresses")]
        pub email_addresses: Option<mod_wsd::ArrayOfEmailAddress>,
        #[yaserde(prefix = "wsd", rename = "SipUris")]
        pub sip_uris: Option<mod_wsd::ArrayOfSipUri>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumberCreateList")]
        pub phone_number_create_list: Option<mod_wsd::ArrayOfPhoneNumberCreate>,
        #[yaserde(prefix = "wsd", rename = "ContactCreateList")]
        pub contact_create_list: Option<mod_wsd::ArrayOfContactCreate>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "Objection")]
        pub objection: Option<bool>,
    }
    impl restrictions::CheckRestrictions for CustomerCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.title.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.login_page.check_restrictions(restrictions.clone())?;
            self.preferred_agent.check_restrictions(restrictions.clone())?;
            self.password.check_restrictions(restrictions.clone())?;
            self.register_date.check_restrictions(restrictions.clone())?;
            self.user_name.check_restrictions(restrictions.clone())?;
            self.addresses.check_restrictions(restrictions.clone())?;
            self.email_addresses.check_restrictions(restrictions.clone())?;
            self.sip_uris.check_restrictions(restrictions.clone())?;
            self.phone_number_create_list.check_restrictions(restrictions.clone())?;
            self.contact_create_list.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            self.objection.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfPhoneNumberCreate")]
    pub struct ArrayOfPhoneNumberCreate {
        #[yaserde(prefix = "wsd", rename = "PhoneNumberCreate")]
        pub phone_number_create: Vec<mod_wsd::PhoneNumberCreate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfPhoneNumberCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.phone_number_create.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfContactCreate")]
    pub struct ArrayOfContactCreate {
        #[yaserde(prefix = "wsd", rename = "ContactCreate")]
        pub contact_create: Vec<mod_wsd::ContactCreate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfContactCreate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.contact_create.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CampaignCustomerDetails")]
    pub struct CampaignCustomerDetails {
        #[yaserde(prefix = "wsd", rename = "Title")]
        pub title: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoginPage")]
        pub login_page: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgent")]
        pub preferred_agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Password")]
        pub password: Option<String>,
        #[yaserde(prefix = "wsd", rename = "RegisterDate")]
        pub register_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Addresses")]
        pub addresses: Option<mod_wsd::ArrayOfAddress>,
        #[yaserde(prefix = "wsd", rename = "EmailAddresses")]
        pub email_addresses: Option<mod_wsd::ArrayOfEmailAddress>,
        #[yaserde(prefix = "wsd", rename = "SipUris")]
        pub sip_uris: Option<mod_wsd::ArrayOfSipUri>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumberCreateList")]
        pub phone_number_create_list: Option<mod_wsd::ArrayOfPhoneNumberCreate>,
        #[yaserde(prefix = "wsd", rename = "ContactCreateList")]
        pub contact_create_list: Option<mod_wsd::ArrayOfContactCreate>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
        #[yaserde(prefix = "wsd", rename = "Objection")]
        pub objection: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "BlockIndex")]
        pub block_index: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MatchType")]
        pub match_type: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "CustomerID")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "ImportDataOverrides")]
        pub import_data_overrides: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "SkillsetID")]
        pub skillset_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for CampaignCustomerDetails {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.title.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.login_page.check_restrictions(restrictions.clone())?;
            self.preferred_agent.check_restrictions(restrictions.clone())?;
            self.password.check_restrictions(restrictions.clone())?;
            self.register_date.check_restrictions(restrictions.clone())?;
            self.user_name.check_restrictions(restrictions.clone())?;
            self.addresses.check_restrictions(restrictions.clone())?;
            self.email_addresses.check_restrictions(restrictions.clone())?;
            self.sip_uris.check_restrictions(restrictions.clone())?;
            self.phone_number_create_list.check_restrictions(restrictions.clone())?;
            self.contact_create_list.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            self.objection.check_restrictions(restrictions.clone())?;
            self.block_index.check_restrictions(restrictions.clone())?;
            self.match_type.check_restrictions(restrictions.clone())?;
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.import_data_overrides.check_restrictions(restrictions.clone())?;
            self.skillset_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CreateCustomerResponse")]
    pub struct CreateCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "CreateCustomerResult")]
        pub create_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for CreateCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.create_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CreateCustomerBySipUri")]
    pub struct CreateCustomerBySipUri {
        #[yaserde(prefix = "wsd", rename = "sipUri")]
        pub sip_uri: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CreateCustomerBySipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.sip_uri.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CreateCustomerBySipUriResponse")]
    pub struct CreateCustomerBySipUriResponse {
        #[yaserde(prefix = "wsd", rename = "CreateCustomerBySipUriResult")]
        pub create_customer_by_sip_uri_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for CreateCustomerBySipUriResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.create_customer_by_sip_uri_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerAssociation")]
    pub struct CustomerAssociation {
        #[yaserde(prefix = "wsd", rename = "listOfCallDetails")]
        pub list_of_call_details: Option<mod_wsd::ListOfCallDetails>,
        #[yaserde(prefix = "wsd", rename = "partialMatch")]
        pub partial_match: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "maxCustomerMatches")]
        pub max_customer_matches: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CustomerAssociation {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.list_of_call_details.check_restrictions(restrictions.clone())?;
            self.partial_match.check_restrictions(restrictions.clone())?;
            self.max_customer_matches.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ListOfCallDetails")]
    pub struct ListOfCallDetails {
        #[yaserde(prefix = "wsd", rename = "CallDetails")]
        pub call_details: Option<mod_wsd::ArrayOfCallDetails>,
    }
    impl restrictions::CheckRestrictions for ListOfCallDetails {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.call_details.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCallDetails")]
    pub struct ArrayOfCallDetails {
        #[yaserde(prefix = "wsd", rename = "CallDetails")]
        pub call_details: Vec<mod_wsd::CallDetails>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCallDetails {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.call_details.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CallDetails")]
    pub struct CallDetails {
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "InternationalCode")]
        pub international_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AreaCode")]
        pub area_code: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Number")]
        pub number: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Email")]
        pub email: Option<String>,
    }
    impl restrictions::CheckRestrictions for CallDetails {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.international_code.check_restrictions(restrictions.clone())?;
            self.area_code.check_restrictions(restrictions.clone())?;
            self.number.check_restrictions(restrictions.clone())?;
            self.email.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerAssociationResponse")]
    pub struct CustomerAssociationResponse {
        #[yaserde(prefix = "wsd", rename = "CustomerAssociationResult")]
        pub customer_association_result: mod_wsd::ListOfCustomerAssociationResults,
    }
    impl restrictions::CheckRestrictions for CustomerAssociationResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_association_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ListOfCustomerAssociationResults")]
    pub struct ListOfCustomerAssociationResults {
        #[yaserde(prefix = "wsd", rename = "CustomerAssociationResults")]
        pub customer_association_results: Option<mod_wsd::ArrayOfCustomerAssociationResult>,
    }
    impl restrictions::CheckRestrictions for ListOfCustomerAssociationResults {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_association_results
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCustomerAssociationResult")]
    pub struct ArrayOfCustomerAssociationResult {
        #[yaserde(prefix = "wsd", rename = "CustomerAssociationResult")]
        pub customer_association_result: Vec<mod_wsd::CustomerAssociationResult>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCustomerAssociationResult {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_association_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerAssociationResult")]
    pub struct CustomerAssociationResult {
        #[yaserde(prefix = "wsd", rename = "CallDetailIndex")]
        pub call_detail_index: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MatchType")]
        pub match_type: Option<String>,
        #[yaserde(prefix = "wsd", rename = "ExistingCustID")]
        pub existing_cust_id: Option<mod_wsd::ArrayOfExistingCustIDItemString>,
        #[yaserde(prefix = "wsd", rename = "DNCStatus")]
        pub dnc_status: Option<mod_wsd::ArrayOfDNCStatusItemBoolean>,
    }
    impl restrictions::CheckRestrictions for CustomerAssociationResult {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.call_detail_index.check_restrictions(restrictions.clone())?;
            self.match_type.check_restrictions(restrictions.clone())?;
            self.existing_cust_id.check_restrictions(restrictions.clone())?;
            self.dnc_status.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfExistingCustIDItemString")]
    pub struct ArrayOfExistingCustIDItemString {
        #[yaserde(prefix = "wsd", rename = "ExistingCustIDItem")]
        pub existing_cust_id_item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfExistingCustIDItemString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.existing_cust_id_item.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfDNCStatusItemBoolean")]
    pub struct ArrayOfDNCStatusItemBoolean {
        #[yaserde(prefix = "wsd", rename = "DNCStatusItem")]
        pub dnc_status_item: Vec<bool>,
    }
    impl restrictions::CheckRestrictions for ArrayOfDNCStatusItemBoolean {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.dnc_status_item.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerSearch")]
    pub struct CustomerSearch {
        #[yaserde(prefix = "wsd", rename = "searchParams")]
        pub search_params: Option<mod_wsd::SearchObject>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for CustomerSearch {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.search_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SearchObject")]
    pub struct SearchObject {
        #[yaserde(prefix = "wsd", rename = "SearchConditions")]
        pub search_conditions: Option<mod_wsd::ArrayOfSearchCondition>,
        #[yaserde(prefix = "wsd", rename = "DateField")]
        pub date_field: Option<String>,
        #[yaserde(prefix = "wsd", rename = "StartDateRange")]
        pub start_date_range: Option<String>,
        #[yaserde(prefix = "wsd", rename = "EndDateRange")]
        pub end_date_range: Option<String>,
        #[yaserde(prefix = "wsd", rename = "OrderField")]
        pub order_field: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Order")]
        pub order: Option<String>,
        #[yaserde(prefix = "wsd", rename = "MaxNoObjects")]
        pub max_no_objects: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MaxID")]
        pub max_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "StartID")]
        pub start_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Direction")]
        pub direction: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "Timeout")]
        pub timeout: Option<i64>,
    }
    impl restrictions::CheckRestrictions for SearchObject {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.search_conditions.check_restrictions(restrictions.clone())?;
            self.date_field.check_restrictions(restrictions.clone())?;
            self.start_date_range.check_restrictions(restrictions.clone())?;
            self.end_date_range.check_restrictions(restrictions.clone())?;
            self.order_field.check_restrictions(restrictions.clone())?;
            self.order.check_restrictions(restrictions.clone())?;
            self.max_no_objects.check_restrictions(restrictions.clone())?;
            self.max_id.check_restrictions(restrictions.clone())?;
            self.start_id.check_restrictions(restrictions.clone())?;
            self.direction.check_restrictions(restrictions.clone())?;
            self.timeout.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfSearchCondition")]
    pub struct ArrayOfSearchCondition {
        #[yaserde(prefix = "wsd", rename = "SearchCondition")]
        pub search_condition: Vec<mod_wsd::SearchCondition>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSearchCondition {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.search_condition.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SearchCondition")]
    pub struct SearchCondition {
        #[yaserde(prefix = "wsd", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Condition")]
        pub condition: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Value")]
        pub value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "NextConditionValue")]
        pub next_condition_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "AllowWildcards")]
        pub allow_wildcards: Option<bool>,
    }
    impl restrictions::CheckRestrictions for SearchCondition {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.name.check_restrictions(restrictions.clone())?;
            self.condition.check_restrictions(restrictions.clone())?;
            self.value.check_restrictions(restrictions.clone())?;
            self.next_condition_value.check_restrictions(restrictions.clone())?;
            self.allow_wildcards.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerSearchResponse")]
    pub struct CustomerSearchResponse {
        #[yaserde(prefix = "wsd", rename = "CustomerSearchResult")]
        pub customer_search_result: mod_wsd::CustomerSearchResult,
    }
    impl restrictions::CheckRestrictions for CustomerSearchResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_search_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerSearchResult")]
    pub struct CustomerSearchResult {
        #[yaserde(prefix = "wsd", rename = "Customers")]
        pub customers: Option<mod_wsd::ArrayOfSearchCustomer>,
        #[yaserde(prefix = "wsd", rename = "NoObjectsRemaining")]
        pub no_objects_remaining: Option<i64>,
    }
    impl restrictions::CheckRestrictions for CustomerSearchResult {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customers.check_restrictions(restrictions.clone())?;
            self.no_objects_remaining.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfSearchCustomer")]
    pub struct ArrayOfSearchCustomer {
        #[yaserde(prefix = "wsd", rename = "SearchCustomer")]
        pub search_customer: Vec<mod_wsd::SearchCustomer>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSearchCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.search_customer.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SearchCustomer")]
    pub struct SearchCustomer {
        #[yaserde(prefix = "wsd", rename = "ID")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "PrefAgentFirstName")]
        pub pref_agent_first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "PrefAgentLastName")]
        pub pref_agent_last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Address")]
        pub address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumber")]
        pub phone_number: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Email")]
        pub email: Option<String>,
        #[yaserde(prefix = "wsd", rename = "SipUri")]
        pub sip_uri: Option<String>,
    }
    impl restrictions::CheckRestrictions for SearchCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.pref_agent_first_name.check_restrictions(restrictions.clone())?;
            self.pref_agent_last_name.check_restrictions(restrictions.clone())?;
            self.address.check_restrictions(restrictions.clone())?;
            self.phone_number.check_restrictions(restrictions.clone())?;
            self.email.check_restrictions(restrictions.clone())?;
            self.sip_uri.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "DeleteCustomer")]
    pub struct DeleteCustomer {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for DeleteCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "DeleteCustomerResponse")]
    pub struct DeleteCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "DeleteCustomerResult")]
        pub delete_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for DeleteCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.delete_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetAllCustomers")]
    pub struct GetAllCustomers {
        #[yaserde(prefix = "wsd", rename = "maxID")]
        pub max_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "maxNoObjects")]
        pub max_no_objects: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetAllCustomers {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.max_id.check_restrictions(restrictions.clone())?;
            self.max_no_objects.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetAllCustomersResponse")]
    pub struct GetAllCustomersResponse {
        #[yaserde(prefix = "wsd", rename = "GetAllCustomersResult")]
        pub get_all_customers_result: mod_wsd::ListOfCustomers,
    }
    impl restrictions::CheckRestrictions for GetAllCustomersResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_all_customers_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ListOfCustomers")]
    pub struct ListOfCustomers {
        #[yaserde(prefix = "wsd", rename = "Customers")]
        pub customers: Option<mod_wsd::ArrayOfCustomer>,
    }
    impl restrictions::CheckRestrictions for ListOfCustomers {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customers.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCustomer")]
    pub struct ArrayOfCustomer {
        #[yaserde(prefix = "wsd", rename = "Customer")]
        pub customer: Vec<mod_wsd::Customer>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustSQLColumns")]
    pub struct GetCustSQLColumns {
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustSQLColumns {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustSQLColumnsResponse")]
    pub struct GetCustSQLColumnsResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustSQLColumnsResult")]
        pub get_cust_sql_columns_result: mod_wsd::ArrayOfGetCustSQLColumnsResultItemString,
    }
    impl restrictions::CheckRestrictions for GetCustSQLColumnsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_cust_sql_columns_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfGetCustSQLColumnsResultItemString")]
    pub struct ArrayOfGetCustSQLColumnsResultItemString {
        #[yaserde(prefix = "wsd", rename = "GetCustSQLColumnsResultItem")]
        pub get_cust_sql_columns_result_item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfGetCustSQLColumnsResultItemString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_cust_sql_columns_result_item
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomFieldTemplates")]
    pub struct GetCustomFieldTemplates {
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomFieldTemplates {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomFieldTemplatesResponse")]
    pub struct GetCustomFieldTemplatesResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomFieldTemplatesResult")]
        pub get_custom_field_templates_result: mod_wsd::ListOfCustomFieldTemplates,
    }
    impl restrictions::CheckRestrictions for GetCustomFieldTemplatesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_custom_field_templates_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ListOfCustomFieldTemplates")]
    pub struct ListOfCustomFieldTemplates {
        #[yaserde(prefix = "wsd", rename = "CustomFieldTemplates")]
        pub custom_field_templates: Option<mod_wsd::ArrayOfCustomFieldTemplate>,
    }
    impl restrictions::CheckRestrictions for ListOfCustomFieldTemplates {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.custom_field_templates.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfCustomFieldTemplate")]
    pub struct ArrayOfCustomFieldTemplate {
        #[yaserde(prefix = "wsd", rename = "CustomFieldTemplate")]
        pub custom_field_template: Vec<mod_wsd::CustomFieldTemplate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfCustomFieldTemplate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.custom_field_template.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByContactId")]
    pub struct GetCustomerByContactId {
        #[yaserde(prefix = "wsd", rename = "contactID")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerByContactId {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByContactIdResponse")]
    pub struct GetCustomerByContactIdResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByContactIdResult")]
        pub get_customer_by_contact_id_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for GetCustomerByContactIdResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_contact_id_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByEmail")]
    pub struct GetCustomerByEmail {
        #[yaserde(prefix = "wsd", rename = "email")]
        pub email: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerByEmail {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.email.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByEmailResponse")]
    pub struct GetCustomerByEmailResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByEmailResult")]
        pub get_customer_by_email_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for GetCustomerByEmailResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_email_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByName")]
    pub struct GetCustomerByName {
        #[yaserde(prefix = "wsd", rename = "firstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "lastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "fieldToCompare")]
        pub field_to_compare: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerByName {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.field_to_compare.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByNameResponse")]
    pub struct GetCustomerByNameResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByNameResult")]
        pub get_customer_by_name_result: mod_wsd::ArrayOfGetCustomerByNameResultItemString,
    }
    impl restrictions::CheckRestrictions for GetCustomerByNameResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_name_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfGetCustomerByNameResultItemString")]
    pub struct ArrayOfGetCustomerByNameResultItemString {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByNameResultItem")]
        pub get_customer_by_name_result_item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfGetCustomerByNameResultItemString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_name_result_item
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByPhoneNumber")]
    pub struct GetCustomerByPhoneNumber {
        #[yaserde(prefix = "wsd", rename = "phoneNumber")]
        pub phone_number: Option<mod_wsd::PhoneNumber>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerByPhoneNumber {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.phone_number.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByPhoneNumberResponse")]
    pub struct GetCustomerByPhoneNumberResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByPhoneNumberResult")]
        pub get_customer_by_phone_number_result: mod_wsd::ArrayOfGetCustomerByPhoneNumberResultItemString,
    }
    impl restrictions::CheckRestrictions for GetCustomerByPhoneNumberResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_phone_number_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfGetCustomerByPhoneNumberResultItemString")]
    pub struct ArrayOfGetCustomerByPhoneNumberResultItemString {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByPhoneNumberResultItem")]
        pub get_customer_by_phone_number_result_item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfGetCustomerByPhoneNumberResultItemString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_phone_number_result_item
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerBySipUri")]
    pub struct GetCustomerBySipUri {
        #[yaserde(prefix = "wsd", rename = "sipUri")]
        pub sip_uri: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerBySipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.sip_uri.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerBySipUriResponse")]
    pub struct GetCustomerBySipUriResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerBySipUriResult")]
        pub get_customer_by_sip_uri_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for GetCustomerBySipUriResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_sip_uri_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByUserName")]
    pub struct GetCustomerByUserName {
        #[yaserde(prefix = "wsd", rename = "userName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetCustomerByUserName {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.user_name.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetCustomerByUserNameResponse")]
    pub struct GetCustomerByUserNameResponse {
        #[yaserde(prefix = "wsd", rename = "GetCustomerByUserNameResult")]
        pub get_customer_by_user_name_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for GetCustomerByUserNameResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_customer_by_user_name_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetNoCustContactsByTime")]
    pub struct GetNoCustContactsByTime {
        #[yaserde(prefix = "wsd", rename = "customerID")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "startTime")]
        pub start_time: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetNoCustContactsByTime {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.start_time.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetNoCustContactsByTimeResponse")]
    pub struct GetNoCustContactsByTimeResponse {
        #[yaserde(prefix = "wsd", rename = "GetNoCustContactsByTimeResult")]
        pub get_no_cust_contacts_by_time_result: i64,
    }
    impl restrictions::CheckRestrictions for GetNoCustContactsByTimeResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_no_cust_contacts_by_time_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetSearchableFields")]
    pub struct GetSearchableFields {
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for GetSearchableFields {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "GetSearchableFieldsResponse")]
    pub struct GetSearchableFieldsResponse {
        #[yaserde(prefix = "wsd", rename = "GetSearchableFieldsResult")]
        pub get_searchable_fields_result: mod_wsd::ArrayOfGetSearchableFieldsResultItemString,
    }
    impl restrictions::CheckRestrictions for GetSearchableFieldsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_searchable_fields_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ArrayOfGetSearchableFieldsResultItemString")]
    pub struct ArrayOfGetSearchableFieldsResultItemString {
        #[yaserde(prefix = "wsd", rename = "GetSearchableFieldsResultItem")]
        pub get_searchable_fields_result_item: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfGetSearchableFieldsResultItemString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_searchable_fields_result_item
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ImpersonateCustomer")]
    pub struct ImpersonateCustomer {
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
        #[yaserde(prefix = "wsd", rename = "customerID")]
        pub customer_id: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ImpersonateCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.session_key.check_restrictions(restrictions.clone())?;
            self.customer_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ImpersonateCustomerResponse")]
    pub struct ImpersonateCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "ImpersonateCustomerResult")]
        pub impersonate_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for ImpersonateCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.impersonate_customer_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ReadCustomer")]
    pub struct ReadCustomer {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
        #[yaserde(prefix = "wsd", rename = "lite")]
        pub lite: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ReadCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            self.lite.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ReadCustomerResponse")]
    pub struct ReadCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "ReadCustomerResult")]
        pub read_customer_result: mod_wsd::Customer,
    }
    impl restrictions::CheckRestrictions for ReadCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.read_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ReadCustomerHistory")]
    pub struct ReadCustomerHistory {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "maxNoContacts")]
        pub max_no_contacts: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "startContactID")]
        pub start_contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "contactOrder")]
        pub contact_order: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
        #[yaserde(prefix = "wsd", rename = "lite")]
        pub lite: Option<i64>,
    }
    impl restrictions::CheckRestrictions for ReadCustomerHistory {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.max_no_contacts.check_restrictions(restrictions.clone())?;
            self.start_contact_id.check_restrictions(restrictions.clone())?;
            self.contact_order.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            self.lite.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "ReadCustomerHistoryResponse")]
    pub struct ReadCustomerHistoryResponse {
        #[yaserde(prefix = "wsd", rename = "ReadCustomerHistoryResult")]
        pub read_customer_history_result: mod_wsd::CustomerHistoryResult,
    }
    impl restrictions::CheckRestrictions for ReadCustomerHistoryResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.read_customer_history_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerHistoryResult")]
    pub struct CustomerHistoryResult {
        #[yaserde(prefix = "wsd", rename = "Customer")]
        pub customer: Option<mod_wsd::Customer>,
        #[yaserde(prefix = "wsd", rename = "MinID")]
        pub min_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MaxID")]
        pub max_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MinNumberRemaining")]
        pub min_number_remaining: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "MaxNumberRemaining")]
        pub max_number_remaining: Option<i64>,
    }
    impl restrictions::CheckRestrictions for CustomerHistoryResult {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer.check_restrictions(restrictions.clone())?;
            self.min_id.check_restrictions(restrictions.clone())?;
            self.max_id.check_restrictions(restrictions.clone())?;
            self.min_number_remaining.check_restrictions(restrictions.clone())?;
            self.max_number_remaining.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RegisterAnonymousCustomer")]
    pub struct RegisterAnonymousCustomer {
        #[yaserde(prefix = "wsd", rename = "customerParams")]
        pub customer_params: Option<mod_wsd::CustomerCreate>,
    }
    impl restrictions::CheckRestrictions for RegisterAnonymousCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_params.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RegisterAnonymousCustomerResponse")]
    pub struct RegisterAnonymousCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "RegisterAnonymousCustomerResult")]
        pub register_anonymous_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for RegisterAnonymousCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.register_anonymous_customer_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RegisterCustomer")]
    pub struct RegisterCustomer {
        #[yaserde(prefix = "wsd", rename = "customerParams")]
        pub customer_params: Option<mod_wsd::CustomerCreate>,
    }
    impl restrictions::CheckRestrictions for RegisterCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.customer_params.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RegisterCustomerResponse")]
    pub struct RegisterCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "RegisterCustomerResult")]
        pub register_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for RegisterCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.register_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveAddress")]
    pub struct RemoveAddress {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "addressID")]
        pub address_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemoveAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.address_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveAddressResponse")]
    pub struct RemoveAddressResponse {
        #[yaserde(prefix = "wsd", rename = "RemoveAddressResult")]
        pub remove_address_result: i64,
    }
    impl restrictions::CheckRestrictions for RemoveAddressResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_address_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveContact")]
    pub struct RemoveContact {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "contactID")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemoveContact {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveContactResponse")]
    pub struct RemoveContactResponse {
        #[yaserde(prefix = "wsd", rename = "RemoveContactResult")]
        pub remove_contact_result: i64,
    }
    impl restrictions::CheckRestrictions for RemoveContactResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_contact_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveCustomField")]
    pub struct RemoveCustomField {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "customFieldID")]
        pub custom_field_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemoveCustomField {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.custom_field_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveCustomFieldResponse")]
    pub struct RemoveCustomFieldResponse {
        #[yaserde(prefix = "wsd", rename = "RemoveCustomFieldResult")]
        pub remove_custom_field_result: i64,
    }
    impl restrictions::CheckRestrictions for RemoveCustomFieldResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_custom_field_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveEmailAddress")]
    pub struct RemoveEmailAddress {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "emailAddressID")]
        pub email_address_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemoveEmailAddress {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.email_address_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveEmailAddressResponse")]
    pub struct RemoveEmailAddressResponse {
        #[yaserde(prefix = "wsd", rename = "RemoveEmailAddressResult")]
        pub remove_email_address_result: i64,
    }
    impl restrictions::CheckRestrictions for RemoveEmailAddressResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_email_address_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemovePhoneNumber")]
    pub struct RemovePhoneNumber {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "phoneNumberID")]
        pub phone_number_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemovePhoneNumber {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.phone_number_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemovePhoneNumberResponse")]
    pub struct RemovePhoneNumberResponse {
        #[yaserde(prefix = "wsd", rename = "RemovePhoneNumberResult")]
        pub remove_phone_number_result: i64,
    }
    impl restrictions::CheckRestrictions for RemovePhoneNumberResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_phone_number_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveSipUri")]
    pub struct RemoveSipUri {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sipUriID")]
        pub sip_uri_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for RemoveSipUri {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.sip_uri_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "RemoveSipUriResponse")]
    pub struct RemoveSipUriResponse {
        #[yaserde(prefix = "wsd", rename = "RemoveSipUriResult")]
        pub remove_sip_uri_result: i64,
    }
    impl restrictions::CheckRestrictions for RemoveSipUriResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.remove_sip_uri_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SendADPasswordReminder")]
    pub struct SendADPasswordReminder {
        #[yaserde(prefix = "wsd", rename = "contactID")]
        pub contact_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "customerID")]
        pub customer_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "skillsetID")]
        pub skillset_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for SendADPasswordReminder {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.contact_id.check_restrictions(restrictions.clone())?;
            self.customer_id.check_restrictions(restrictions.clone())?;
            self.skillset_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SendADPasswordReminderResponse")]
    pub struct SendADPasswordReminderResponse {
        #[yaserde(prefix = "wsd", rename = "SendADPasswordReminderResult")]
        pub send_ad_password_reminder_result: i64,
    }
    impl restrictions::CheckRestrictions for SendADPasswordReminderResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.send_ad_password_reminder_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SendPasswordReminder")]
    pub struct SendPasswordReminder {
        #[yaserde(prefix = "wsd", rename = "emailAddress")]
        pub email_address: Option<String>,
        #[yaserde(prefix = "wsd", rename = "skillsetName")]
        pub skillset_name: Option<String>,
    }
    impl restrictions::CheckRestrictions for SendPasswordReminder {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.email_address.check_restrictions(restrictions.clone())?;
            self.skillset_name.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SendPasswordReminderResponse")]
    pub struct SendPasswordReminderResponse {
        #[yaserde(prefix = "wsd", rename = "SendPasswordReminderResult")]
        pub send_password_reminder_result: i64,
    }
    impl restrictions::CheckRestrictions for SendPasswordReminderResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.send_password_reminder_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SetAgentID")]
    pub struct SetAgentID {
        #[yaserde(prefix = "wsd", rename = "agentID")]
        pub agent_id: Option<String>,
    }
    impl restrictions::CheckRestrictions for SetAgentID {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.agent_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "SetAgentIDResponse")]
    pub struct SetAgentIDResponse {
        #[yaserde(prefix = "wsd", rename = "SetAgentIDResult")]
        pub set_agent_id_result: i64,
    }
    impl restrictions::CheckRestrictions for SetAgentIDResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.set_agent_id_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateCustomer")]
    pub struct UpdateCustomer {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "customerParams")]
        pub customer_params: Option<mod_wsd::CustomerUpdate>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateCustomer {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.customer_params.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "CustomerUpdate")]
    pub struct CustomerUpdate {
        #[yaserde(prefix = "wsd", rename = "Title")]
        pub title: Option<String>,
        #[yaserde(prefix = "wsd", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "LoginPage")]
        pub login_page: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "PreferredAgent")]
        pub preferred_agent: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "RegisterDate")]
        pub register_date: Option<String>,
        #[yaserde(prefix = "wsd", rename = "UserName")]
        pub user_name: Option<String>,
        #[yaserde(prefix = "wsd", rename = "Addresses")]
        pub addresses: Option<mod_wsd::ArrayOfAddress>,
        #[yaserde(prefix = "wsd", rename = "EmailAddresses")]
        pub email_addresses: Option<mod_wsd::ArrayOfEmailAddress>,
        #[yaserde(prefix = "wsd", rename = "SipUris")]
        pub sip_uris: Option<mod_wsd::ArrayOfSipUri>,
        #[yaserde(prefix = "wsd", rename = "PhoneNumberCreateList")]
        pub phone_number_create_list: Option<mod_wsd::ArrayOfPhoneNumberCreate>,
        #[yaserde(prefix = "wsd", rename = "ContactCreateList")]
        pub contact_create_list: Option<mod_wsd::ArrayOfContactCreate>,
        #[yaserde(prefix = "wsd", rename = "CustomFieldCreateList")]
        pub custom_field_create_list: Option<mod_wsd::ArrayOfCustomFieldCreate>,
    }
    impl restrictions::CheckRestrictions for CustomerUpdate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.title.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.login_page.check_restrictions(restrictions.clone())?;
            self.preferred_agent.check_restrictions(restrictions.clone())?;
            self.register_date.check_restrictions(restrictions.clone())?;
            self.user_name.check_restrictions(restrictions.clone())?;
            self.addresses.check_restrictions(restrictions.clone())?;
            self.email_addresses.check_restrictions(restrictions.clone())?;
            self.sip_uris.check_restrictions(restrictions.clone())?;
            self.phone_number_create_list.check_restrictions(restrictions.clone())?;
            self.contact_create_list.check_restrictions(restrictions.clone())?;
            self.custom_field_create_list.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateCustomerResponse")]
    pub struct UpdateCustomerResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateCustomerResult")]
        pub update_customer_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateCustomerResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_customer_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateFirstName")]
    pub struct UpdateFirstName {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateFirstName {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateFirstNameResponse")]
    pub struct UpdateFirstNameResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateFirstNameResult")]
        pub update_first_name_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateFirstNameResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_first_name_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateLastName")]
    pub struct UpdateLastName {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateLastName {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateLastNameResponse")]
    pub struct UpdateLastNameResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateLastNameResult")]
        pub update_last_name_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateLastNameResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_last_name_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateLoginPage")]
    pub struct UpdateLoginPage {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "loginPageID")]
        pub login_page_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateLoginPage {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.login_page_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateLoginPageResponse")]
    pub struct UpdateLoginPageResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateLoginPageResult")]
        pub update_login_page_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateLoginPageResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_login_page_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateObjection")]
    pub struct UpdateObjection {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<bool>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateObjection {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateObjectionResponse")]
    pub struct UpdateObjectionResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateObjectionResult")]
        pub update_objection_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateObjectionResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_objection_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdatePassword")]
    pub struct UpdatePassword {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "oldPassword")]
        pub old_password: Option<String>,
        #[yaserde(prefix = "wsd", rename = "newPassword")]
        pub new_password: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdatePassword {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.old_password.check_restrictions(restrictions.clone())?;
            self.new_password.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdatePasswordResponse")]
    pub struct UpdatePasswordResponse {
        #[yaserde(prefix = "wsd", rename = "UpdatePasswordResult")]
        pub update_password_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdatePasswordResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_password_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdatePreferredAgent")]
    pub struct UpdatePreferredAgent {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "preferredAgentID")]
        pub preferred_agent_id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdatePreferredAgent {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.preferred_agent_id.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdatePreferredAgentResponse")]
    pub struct UpdatePreferredAgentResponse {
        #[yaserde(prefix = "wsd", rename = "UpdatePreferredAgentResult")]
        pub update_preferred_agent_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdatePreferredAgentResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_preferred_agent_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateRegisterDate")]
    pub struct UpdateRegisterDate {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateRegisterDate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateRegisterDateResponse")]
    pub struct UpdateRegisterDateResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateRegisterDateResult")]
        pub update_register_date_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateRegisterDateResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_register_date_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateTitle")]
    pub struct UpdateTitle {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateTitle {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateTitleResponse")]
    pub struct UpdateTitleResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateTitleResult")]
        pub update_title_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateTitleResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_title_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateUserName")]
    pub struct UpdateUserName {
        #[yaserde(prefix = "wsd", rename = "id")]
        pub id: Option<i64>,
        #[yaserde(prefix = "wsd", rename = "newValue")]
        pub new_value: Option<String>,
        #[yaserde(prefix = "wsd", rename = "sessionKey")]
        pub session_key: Option<String>,
    }
    impl restrictions::CheckRestrictions for UpdateUserName {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.id.check_restrictions(restrictions.clone())?;
            self.new_value.check_restrictions(restrictions.clone())?;
            self.session_key.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "wsd", namespaces = {"wsd" = "http://ws.db.ccmm.applications.nortel.com"}, rename = "UpdateUserNameResponse")]
    pub struct UpdateUserNameResponse {
        #[yaserde(prefix = "wsd", rename = "UpdateUserNameResult")]
        pub update_user_name_result: i64,
    }
    impl restrictions::CheckRestrictions for UpdateUserNameResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.update_user_name_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
}

/* GetSearchableFields */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetSearchableFieldsInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetSearchableFields")]
    pub get_searchable_fields: mod_wsd::GetSearchableFields,
}
impl restrictions::CheckRestrictions for GetSearchableFieldsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_searchable_fields.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetSearchableFieldsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetSearchableFieldsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetSearchableFieldsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetSearchableFieldsOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetSearchableFieldsResponse")]
    pub get_searchable_fields_response: mod_wsd::GetSearchableFieldsResponse,
}
impl restrictions::CheckRestrictions for GetSearchableFieldsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_searchable_fields_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetSearchableFieldsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetSearchableFieldsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetSearchableFieldsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_searchable_fields(
    req: GetSearchableFieldsInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetSearchableFieldsOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetSearchableFields";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateLastName */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLastNameInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateLastName")]
    pub update_last_name: mod_wsd::UpdateLastName,
}
impl restrictions::CheckRestrictions for UpdateLastNameInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_last_name.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLastNameInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateLastNameInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateLastNameInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLastNameOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateLastNameResponse")]
    pub update_last_name_response: mod_wsd::UpdateLastNameResponse,
}
impl restrictions::CheckRestrictions for UpdateLastNameOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_last_name_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLastNameOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateLastNameOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateLastNameOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_last_name(
    req: UpdateLastNameInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateLastNameOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateLastName";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdatePreferredAgent */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePreferredAgentInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdatePreferredAgent")]
    pub update_preferred_agent: mod_wsd::UpdatePreferredAgent,
}
impl restrictions::CheckRestrictions for UpdatePreferredAgentInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_preferred_agent.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePreferredAgentInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdatePreferredAgentInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdatePreferredAgentInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePreferredAgentOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdatePreferredAgentResponse")]
    pub update_preferred_agent_response: mod_wsd::UpdatePreferredAgentResponse,
}
impl restrictions::CheckRestrictions for UpdatePreferredAgentOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_preferred_agent_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePreferredAgentOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdatePreferredAgentOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdatePreferredAgentOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_preferred_agent(
    req: UpdatePreferredAgentInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdatePreferredAgentOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdatePreferredAgent";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerByUserName */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByUserNameInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByUserName")]
    pub get_customer_by_user_name: mod_wsd::GetCustomerByUserName,
}
impl restrictions::CheckRestrictions for GetCustomerByUserNameInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_user_name.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByUserNameInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByUserNameInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByUserNameInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByUserNameOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByUserNameResponse")]
    pub get_customer_by_user_name_response: mod_wsd::GetCustomerByUserNameResponse,
}
impl restrictions::CheckRestrictions for GetCustomerByUserNameOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_user_name_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByUserNameOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByUserNameOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByUserNameOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_user_name(
    req: GetCustomerByUserNameInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerByUserNameOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByUserName";
    helpers::send_soap_request(url, credentials, req).await
}

/* ReadCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ReadCustomer")]
    pub read_customer: mod_wsd::ReadCustomer,
}
impl restrictions::CheckRestrictions for ReadCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.read_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ReadCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ReadCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ReadCustomerResponse")]
    pub read_customer_response: mod_wsd::ReadCustomerResponse,
}
impl restrictions::CheckRestrictions for ReadCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.read_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ReadCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ReadCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn read_customer(
    req: ReadCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<ReadCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ReadCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerByEmail */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByEmailInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByEmail")]
    pub get_customer_by_email: mod_wsd::GetCustomerByEmail,
}
impl restrictions::CheckRestrictions for GetCustomerByEmailInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_email.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByEmailInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByEmailInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByEmailInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByEmailOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByEmailResponse")]
    pub get_customer_by_email_response: mod_wsd::GetCustomerByEmailResponse,
}
impl restrictions::CheckRestrictions for GetCustomerByEmailOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_email_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByEmailOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByEmailOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByEmailOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_email(
    req: GetCustomerByEmailInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerByEmailOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByEmail";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemoveContact */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveContactInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveContact")]
    pub remove_contact: mod_wsd::RemoveContact,
}
impl restrictions::CheckRestrictions for RemoveContactInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_contact.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveContactInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveContactInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveContactInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveContactOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveContactResponse")]
    pub remove_contact_response: mod_wsd::RemoveContactResponse,
}
impl restrictions::CheckRestrictions for RemoveContactOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_contact_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveContactOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveContactOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveContactOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_contact(
    req: RemoveContactInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemoveContactOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveContact";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddPhoneNumber */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddPhoneNumberInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddPhoneNumber")]
    pub add_phone_number: mod_wsd::AddPhoneNumber,
}
impl restrictions::CheckRestrictions for AddPhoneNumberInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_phone_number.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddPhoneNumberInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddPhoneNumberInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddPhoneNumberInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddPhoneNumberOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddPhoneNumberResponse")]
    pub add_phone_number_response: mod_wsd::AddPhoneNumberResponse,
}
impl restrictions::CheckRestrictions for AddPhoneNumberOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_phone_number_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddPhoneNumberOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddPhoneNumberOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddPhoneNumberOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_phone_number(
    req: AddPhoneNumberInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddPhoneNumberOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddPhoneNumber";
    helpers::send_soap_request(url, credentials, req).await
}

/* CleanCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CleanCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CleanCustomer")]
    pub clean_customer: mod_wsd::CleanCustomer,
}
impl restrictions::CheckRestrictions for CleanCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.clean_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CleanCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CleanCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CleanCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CleanCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CleanCustomerResponse")]
    pub clean_customer_response: mod_wsd::CleanCustomerResponse,
}
impl restrictions::CheckRestrictions for CleanCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.clean_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CleanCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CleanCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CleanCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn clean_customer(
    req: CleanCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CleanCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CleanCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* ReadCustomerHistory */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerHistoryInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ReadCustomerHistory")]
    pub read_customer_history: mod_wsd::ReadCustomerHistory,
}
impl restrictions::CheckRestrictions for ReadCustomerHistoryInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.read_customer_history.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerHistoryInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ReadCustomerHistoryInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ReadCustomerHistoryInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerHistoryOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ReadCustomerHistoryResponse")]
    pub read_customer_history_response: mod_wsd::ReadCustomerHistoryResponse,
}
impl restrictions::CheckRestrictions for ReadCustomerHistoryOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.read_customer_history_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ReadCustomerHistoryOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ReadCustomerHistoryOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ReadCustomerHistoryOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn read_customer_history(
    req: ReadCustomerHistoryInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<ReadCustomerHistoryOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ReadCustomerHistory";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemoveEmailAddress */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveEmailAddressInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveEmailAddress")]
    pub remove_email_address: mod_wsd::RemoveEmailAddress,
}
impl restrictions::CheckRestrictions for RemoveEmailAddressInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_email_address.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveEmailAddressInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveEmailAddressInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveEmailAddressInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveEmailAddressOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveEmailAddressResponse")]
    pub remove_email_address_response: mod_wsd::RemoveEmailAddressResponse,
}
impl restrictions::CheckRestrictions for RemoveEmailAddressOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_email_address_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveEmailAddressOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveEmailAddressOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveEmailAddressOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_email_address(
    req: RemoveEmailAddressInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemoveEmailAddressOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveEmailAddress";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateObjection */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateObjectionInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateObjection")]
    pub update_objection: mod_wsd::UpdateObjection,
}
impl restrictions::CheckRestrictions for UpdateObjectionInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_objection.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateObjectionInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateObjectionInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateObjectionInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateObjectionOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateObjectionResponse")]
    pub update_objection_response: mod_wsd::UpdateObjectionResponse,
}
impl restrictions::CheckRestrictions for UpdateObjectionOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_objection_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateObjectionOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateObjectionOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateObjectionOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_objection(
    req: UpdateObjectionInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateObjectionOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateObjection";
    helpers::send_soap_request(url, credentials, req).await
}

/* SetAgentID */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SetAgentIDInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SetAgentID")]
    pub set_agent_id: mod_wsd::SetAgentID,
}
impl restrictions::CheckRestrictions for SetAgentIDInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.set_agent_id.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SetAgentIDInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SetAgentIDInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SetAgentIDInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SetAgentIDOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SetAgentIDResponse")]
    pub set_agent_id_response: mod_wsd::SetAgentIDResponse,
}
impl restrictions::CheckRestrictions for SetAgentIDOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.set_agent_id_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SetAgentIDOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SetAgentIDOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SetAgentIDOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn set_agent_id(
    req: SetAgentIDInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SetAgentIDOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SetAgentID";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddContact */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddContactInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddContact")]
    pub add_contact: mod_wsd::AddContact,
}
impl restrictions::CheckRestrictions for AddContactInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_contact.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddContactInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddContactInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddContactInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddContactOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddContactResponse")]
    pub add_contact_response: mod_wsd::AddContactResponse,
}
impl restrictions::CheckRestrictions for AddContactOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_contact_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddContactOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddContactOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddContactOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_contact(
    req: AddContactInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddContactOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddContact";
    helpers::send_soap_request(url, credentials, req).await
}

/* CustomerAssociation */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerAssociationInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CustomerAssociation")]
    pub customer_association: mod_wsd::CustomerAssociation,
}
impl restrictions::CheckRestrictions for CustomerAssociationInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.customer_association.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerAssociationInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CustomerAssociationInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CustomerAssociationInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerAssociationOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CustomerAssociationResponse")]
    pub customer_association_response: mod_wsd::CustomerAssociationResponse,
}
impl restrictions::CheckRestrictions for CustomerAssociationOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.customer_association_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerAssociationOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CustomerAssociationOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CustomerAssociationOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn customer_association(
    req: CustomerAssociationInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CustomerAssociationOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CustomerAssociation";
    helpers::send_soap_request(url, credentials, req).await
}

/* RegisterCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RegisterCustomer")]
    pub register_customer: mod_wsd::RegisterCustomer,
}
impl restrictions::CheckRestrictions for RegisterCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.register_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RegisterCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RegisterCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RegisterCustomerResponse")]
    pub register_customer_response: mod_wsd::RegisterCustomerResponse,
}
impl restrictions::CheckRestrictions for RegisterCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.register_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RegisterCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RegisterCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn register_customer(
    req: RegisterCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RegisterCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RegisterCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemoveAddress */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveAddressInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveAddress")]
    pub remove_address: mod_wsd::RemoveAddress,
}
impl restrictions::CheckRestrictions for RemoveAddressInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_address.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveAddressInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveAddressInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveAddressInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveAddressOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveAddressResponse")]
    pub remove_address_response: mod_wsd::RemoveAddressResponse,
}
impl restrictions::CheckRestrictions for RemoveAddressOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_address_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveAddressOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveAddressOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveAddressOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_address(
    req: RemoveAddressInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemoveAddressOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveAddress";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateRegisterDate */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateRegisterDateInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateRegisterDate")]
    pub update_register_date: mod_wsd::UpdateRegisterDate,
}
impl restrictions::CheckRestrictions for UpdateRegisterDateInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_register_date.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateRegisterDateInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateRegisterDateInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateRegisterDateInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateRegisterDateOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateRegisterDateResponse")]
    pub update_register_date_response: mod_wsd::UpdateRegisterDateResponse,
}
impl restrictions::CheckRestrictions for UpdateRegisterDateOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_register_date_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateRegisterDateOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateRegisterDateOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateRegisterDateOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_register_date(
    req: UpdateRegisterDateInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateRegisterDateOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateRegisterDate";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddCustomField */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddCustomFieldInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddCustomField")]
    pub add_custom_field: mod_wsd::AddCustomField,
}
impl restrictions::CheckRestrictions for AddCustomFieldInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_custom_field.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddCustomFieldInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddCustomFieldInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddCustomFieldInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddCustomFieldOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddCustomFieldResponse")]
    pub add_custom_field_response: mod_wsd::AddCustomFieldResponse,
}
impl restrictions::CheckRestrictions for AddCustomFieldOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_custom_field_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddCustomFieldOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddCustomFieldOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddCustomFieldOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_custom_field(
    req: AddCustomFieldInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddCustomFieldOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddCustomField";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddSipUri */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddSipUriInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddSipUri")]
    pub add_sip_uri: mod_wsd::AddSipUri,
}
impl restrictions::CheckRestrictions for AddSipUriInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_sip_uri.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddSipUriInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddSipUriInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddSipUriInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddSipUriOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddSipUriResponse")]
    pub add_sip_uri_response: mod_wsd::AddSipUriResponse,
}
impl restrictions::CheckRestrictions for AddSipUriOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_sip_uri_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddSipUriOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddSipUriOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddSipUriOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_sip_uri(
    req: AddSipUriInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddSipUriOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddSipUri";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerByName */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByNameInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByName")]
    pub get_customer_by_name: mod_wsd::GetCustomerByName,
}
impl restrictions::CheckRestrictions for GetCustomerByNameInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_name.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByNameInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByNameInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByNameInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByNameOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByNameResponse")]
    pub get_customer_by_name_response: mod_wsd::GetCustomerByNameResponse,
}
impl restrictions::CheckRestrictions for GetCustomerByNameOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_name_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByNameOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByNameOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByNameOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_name(
    req: GetCustomerByNameInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerByNameOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByName";
    helpers::send_soap_request(url, credentials, req).await
}

/* RegisterAnonymousCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterAnonymousCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RegisterAnonymousCustomer")]
    pub register_anonymous_customer: mod_wsd::RegisterAnonymousCustomer,
}
impl restrictions::CheckRestrictions for RegisterAnonymousCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.register_anonymous_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterAnonymousCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RegisterAnonymousCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RegisterAnonymousCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterAnonymousCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RegisterAnonymousCustomerResponse")]
    pub register_anonymous_customer_response: mod_wsd::RegisterAnonymousCustomerResponse,
}
impl restrictions::CheckRestrictions for RegisterAnonymousCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.register_anonymous_customer_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RegisterAnonymousCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RegisterAnonymousCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RegisterAnonymousCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn register_anonymous_customer(
    req: RegisterAnonymousCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RegisterAnonymousCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RegisterAnonymousCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* CreateCustomerBySipUri */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerBySipUriInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CreateCustomerBySipUri")]
    pub create_customer_by_sip_uri: mod_wsd::CreateCustomerBySipUri,
}
impl restrictions::CheckRestrictions for CreateCustomerBySipUriInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create_customer_by_sip_uri.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerBySipUriInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateCustomerBySipUriInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateCustomerBySipUriInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerBySipUriOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CreateCustomerBySipUriResponse")]
    pub create_customer_by_sip_uri_response: mod_wsd::CreateCustomerBySipUriResponse,
}
impl restrictions::CheckRestrictions for CreateCustomerBySipUriOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create_customer_by_sip_uri_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerBySipUriOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateCustomerBySipUriOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateCustomerBySipUriOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn create_customer_by_sip_uri(
    req: CreateCustomerBySipUriInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CreateCustomerBySipUriOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CreateCustomerBySipUri";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateCustomer")]
    pub update_customer: mod_wsd::UpdateCustomer,
}
impl restrictions::CheckRestrictions for UpdateCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateCustomerResponse")]
    pub update_customer_response: mod_wsd::UpdateCustomerResponse,
}
impl restrictions::CheckRestrictions for UpdateCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_customer(
    req: UpdateCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdatePassword */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePasswordInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdatePassword")]
    pub update_password: mod_wsd::UpdatePassword,
}
impl restrictions::CheckRestrictions for UpdatePasswordInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_password.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePasswordInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdatePasswordInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdatePasswordInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePasswordOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdatePasswordResponse")]
    pub update_password_response: mod_wsd::UpdatePasswordResponse,
}
impl restrictions::CheckRestrictions for UpdatePasswordOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_password_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdatePasswordOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdatePasswordOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdatePasswordOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_password(
    req: UpdatePasswordInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdatePasswordOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdatePassword";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateTitle */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateTitleInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateTitle")]
    pub update_title: mod_wsd::UpdateTitle,
}
impl restrictions::CheckRestrictions for UpdateTitleInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_title.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateTitleInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateTitleInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateTitleInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateTitleOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateTitleResponse")]
    pub update_title_response: mod_wsd::UpdateTitleResponse,
}
impl restrictions::CheckRestrictions for UpdateTitleOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_title_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateTitleOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateTitleOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateTitleOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_title(
    req: UpdateTitleInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateTitleOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateTitle";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateUserName */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateUserNameInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateUserName")]
    pub update_user_name: mod_wsd::UpdateUserName,
}
impl restrictions::CheckRestrictions for UpdateUserNameInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_user_name.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateUserNameInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateUserNameInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateUserNameInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateUserNameOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateUserNameResponse")]
    pub update_user_name_response: mod_wsd::UpdateUserNameResponse,
}
impl restrictions::CheckRestrictions for UpdateUserNameOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_user_name_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateUserNameOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateUserNameOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateUserNameOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_user_name(
    req: UpdateUserNameInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateUserNameOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateUserName";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateLoginPage */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLoginPageInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateLoginPage")]
    pub update_login_page: mod_wsd::UpdateLoginPage,
}
impl restrictions::CheckRestrictions for UpdateLoginPageInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_login_page.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLoginPageInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateLoginPageInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateLoginPageInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLoginPageOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateLoginPageResponse")]
    pub update_login_page_response: mod_wsd::UpdateLoginPageResponse,
}
impl restrictions::CheckRestrictions for UpdateLoginPageOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_login_page_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateLoginPageOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateLoginPageOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateLoginPageOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_login_page(
    req: UpdateLoginPageInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateLoginPageOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateLoginPage";
    helpers::send_soap_request(url, credentials, req).await
}

/* CarbonCopy */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CarbonCopyInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CarbonCopy")]
    pub carbon_copy: mod_wsd::CarbonCopy,
}
impl restrictions::CheckRestrictions for CarbonCopyInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.carbon_copy.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CarbonCopyInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CarbonCopyInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CarbonCopyInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CarbonCopyOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CarbonCopyResponse")]
    pub carbon_copy_response: mod_wsd::CarbonCopyResponse,
}
impl restrictions::CheckRestrictions for CarbonCopyOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.carbon_copy_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CarbonCopyOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CarbonCopyOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CarbonCopyOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn carbon_copy(
    req: CarbonCopyInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CarbonCopyOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CarbonCopy";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetAllCustomers */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetAllCustomersInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetAllCustomers")]
    pub get_all_customers: mod_wsd::GetAllCustomers,
}
impl restrictions::CheckRestrictions for GetAllCustomersInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_all_customers.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetAllCustomersInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetAllCustomersInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetAllCustomersInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetAllCustomersOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetAllCustomersResponse")]
    pub get_all_customers_response: mod_wsd::GetAllCustomersResponse,
}
impl restrictions::CheckRestrictions for GetAllCustomersOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_all_customers_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetAllCustomersOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetAllCustomersOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetAllCustomersOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_all_customers(
    req: GetAllCustomersInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetAllCustomersOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetAllCustomers";
    helpers::send_soap_request(url, credentials, req).await
}

/* SendADPasswordReminder */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendADPasswordReminderInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SendADPasswordReminder")]
    pub send_ad_password_reminder: mod_wsd::SendADPasswordReminder,
}
impl restrictions::CheckRestrictions for SendADPasswordReminderInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.send_ad_password_reminder.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendADPasswordReminderInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SendADPasswordReminderInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SendADPasswordReminderInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendADPasswordReminderOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SendADPasswordReminderResponse")]
    pub send_ad_password_reminder_response: mod_wsd::SendADPasswordReminderResponse,
}
impl restrictions::CheckRestrictions for SendADPasswordReminderOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.send_ad_password_reminder_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendADPasswordReminderOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SendADPasswordReminderOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SendADPasswordReminderOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn send_ad_password_reminder(
    req: SendADPasswordReminderInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SendADPasswordReminderOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SendADPasswordReminder";
    helpers::send_soap_request(url, credentials, req).await
}

/* CustomerSearch */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerSearchInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CustomerSearch")]
    pub customer_search: mod_wsd::CustomerSearch,
}
impl restrictions::CheckRestrictions for CustomerSearchInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.customer_search.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerSearchInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CustomerSearchInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CustomerSearchInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerSearchOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CustomerSearchResponse")]
    pub customer_search_response: mod_wsd::CustomerSearchResponse,
}
impl restrictions::CheckRestrictions for CustomerSearchOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.customer_search_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CustomerSearchOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CustomerSearchOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CustomerSearchOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn customer_search(
    req: CustomerSearchInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CustomerSearchOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CustomerSearch";
    helpers::send_soap_request(url, credentials, req).await
}

/* UpdateFirstName */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateFirstNameInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateFirstName")]
    pub update_first_name: mod_wsd::UpdateFirstName,
}
impl restrictions::CheckRestrictions for UpdateFirstNameInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_first_name.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateFirstNameInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateFirstNameInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateFirstNameInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateFirstNameOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "UpdateFirstNameResponse")]
    pub update_first_name_response: mod_wsd::UpdateFirstNameResponse,
}
impl restrictions::CheckRestrictions for UpdateFirstNameOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.update_first_name_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct UpdateFirstNameOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: UpdateFirstNameOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for UpdateFirstNameOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn update_first_name(
    req: UpdateFirstNameInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<UpdateFirstNameOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateFirstName";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddAddress */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddAddressInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddAddress")]
    pub add_address: mod_wsd::AddAddress,
}
impl restrictions::CheckRestrictions for AddAddressInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_address.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddAddressInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddAddressInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddAddressInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddAddressOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddAddressResponse")]
    pub add_address_response: mod_wsd::AddAddressResponse,
}
impl restrictions::CheckRestrictions for AddAddressOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_address_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddAddressOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddAddressOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddAddressOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_address(
    req: AddAddressInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddAddressOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddAddress";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerBySipUri */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerBySipUriInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerBySipUri")]
    pub get_customer_by_sip_uri: mod_wsd::GetCustomerBySipUri,
}
impl restrictions::CheckRestrictions for GetCustomerBySipUriInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_sip_uri.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerBySipUriInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerBySipUriInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerBySipUriInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerBySipUriOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerBySipUriResponse")]
    pub get_customer_by_sip_uri_response: mod_wsd::GetCustomerBySipUriResponse,
}
impl restrictions::CheckRestrictions for GetCustomerBySipUriOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_sip_uri_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerBySipUriOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerBySipUriOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerBySipUriOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_sip_uri(
    req: GetCustomerBySipUriInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerBySipUriOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerBySipUri";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustSQLColumns */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustSQLColumnsInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustSQLColumns")]
    pub get_cust_sql_columns: mod_wsd::GetCustSQLColumns,
}
impl restrictions::CheckRestrictions for GetCustSQLColumnsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_cust_sql_columns.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustSQLColumnsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustSQLColumnsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustSQLColumnsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustSQLColumnsOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustSQLColumnsResponse")]
    pub get_cust_sql_columns_response: mod_wsd::GetCustSQLColumnsResponse,
}
impl restrictions::CheckRestrictions for GetCustSQLColumnsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_cust_sql_columns_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustSQLColumnsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustSQLColumnsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustSQLColumnsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_cust_sql_columns(
    req: GetCustSQLColumnsInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustSQLColumnsOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustSQLColumns";
    helpers::send_soap_request(url, credentials, req).await
}

/* ImpersonateCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ImpersonateCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ImpersonateCustomer")]
    pub impersonate_customer: mod_wsd::ImpersonateCustomer,
}
impl restrictions::CheckRestrictions for ImpersonateCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.impersonate_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ImpersonateCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ImpersonateCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ImpersonateCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ImpersonateCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "ImpersonateCustomerResponse")]
    pub impersonate_customer_response: mod_wsd::ImpersonateCustomerResponse,
}
impl restrictions::CheckRestrictions for ImpersonateCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.impersonate_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct ImpersonateCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ImpersonateCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ImpersonateCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn impersonate_customer(
    req: ImpersonateCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<ImpersonateCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ImpersonateCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* SendPasswordReminder */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendPasswordReminderInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SendPasswordReminder")]
    pub send_password_reminder: mod_wsd::SendPasswordReminder,
}
impl restrictions::CheckRestrictions for SendPasswordReminderInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.send_password_reminder.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendPasswordReminderInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SendPasswordReminderInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SendPasswordReminderInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendPasswordReminderOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "SendPasswordReminderResponse")]
    pub send_password_reminder_response: mod_wsd::SendPasswordReminderResponse,
}
impl restrictions::CheckRestrictions for SendPasswordReminderOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.send_password_reminder_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct SendPasswordReminderOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SendPasswordReminderOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SendPasswordReminderOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn send_password_reminder(
    req: SendPasswordReminderInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SendPasswordReminderOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SendPasswordReminder";
    helpers::send_soap_request(url, credentials, req).await
}

/* CreateCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CreateCustomer")]
    pub create_customer: mod_wsd::CreateCustomer,
}
impl restrictions::CheckRestrictions for CreateCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "CreateCustomerResponse")]
    pub create_customer_response: mod_wsd::CreateCustomerResponse,
}
impl restrictions::CheckRestrictions for CreateCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.create_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct CreateCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CreateCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CreateCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn create_customer(
    req: CreateCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<CreateCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CreateCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomFieldTemplates */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomFieldTemplatesInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomFieldTemplates")]
    pub get_custom_field_templates: mod_wsd::GetCustomFieldTemplates,
}
impl restrictions::CheckRestrictions for GetCustomFieldTemplatesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_custom_field_templates.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomFieldTemplatesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomFieldTemplatesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomFieldTemplatesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomFieldTemplatesOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomFieldTemplatesResponse")]
    pub get_custom_field_templates_response: mod_wsd::GetCustomFieldTemplatesResponse,
}
impl restrictions::CheckRestrictions for GetCustomFieldTemplatesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_custom_field_templates_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomFieldTemplatesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomFieldTemplatesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomFieldTemplatesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_custom_field_templates(
    req: GetCustomFieldTemplatesInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomFieldTemplatesOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomFieldTemplates";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerByContactId */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByContactIdInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByContactId")]
    pub get_customer_by_contact_id: mod_wsd::GetCustomerByContactId,
}
impl restrictions::CheckRestrictions for GetCustomerByContactIdInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_contact_id.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByContactIdInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByContactIdInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByContactIdInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByContactIdOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByContactIdResponse")]
    pub get_customer_by_contact_id_response: mod_wsd::GetCustomerByContactIdResponse,
}
impl restrictions::CheckRestrictions for GetCustomerByContactIdOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_contact_id_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByContactIdOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByContactIdOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByContactIdOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_contact_id(
    req: GetCustomerByContactIdInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerByContactIdOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByContactId";
    helpers::send_soap_request(url, credentials, req).await
}

/* DeleteCustomer */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct DeleteCustomerInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "DeleteCustomer")]
    pub delete_customer: mod_wsd::DeleteCustomer,
}
impl restrictions::CheckRestrictions for DeleteCustomerInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.delete_customer.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct DeleteCustomerInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteCustomerInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for DeleteCustomerInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct DeleteCustomerOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "DeleteCustomerResponse")]
    pub delete_customer_response: mod_wsd::DeleteCustomerResponse,
}
impl restrictions::CheckRestrictions for DeleteCustomerOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.delete_customer_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct DeleteCustomerOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: DeleteCustomerOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for DeleteCustomerOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn delete_customer(
    req: DeleteCustomerInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<DeleteCustomerOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.DeleteCustomer";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetCustomerByPhoneNumber */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByPhoneNumberInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByPhoneNumber")]
    pub get_customer_by_phone_number: mod_wsd::GetCustomerByPhoneNumber,
}
impl restrictions::CheckRestrictions for GetCustomerByPhoneNumberInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_phone_number.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByPhoneNumberInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByPhoneNumberInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByPhoneNumberInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByPhoneNumberOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetCustomerByPhoneNumberResponse")]
    pub get_customer_by_phone_number_response: mod_wsd::GetCustomerByPhoneNumberResponse,
}
impl restrictions::CheckRestrictions for GetCustomerByPhoneNumberOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_customer_by_phone_number_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetCustomerByPhoneNumberOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetCustomerByPhoneNumberOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetCustomerByPhoneNumberOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_customer_by_phone_number(
    req: GetCustomerByPhoneNumberInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetCustomerByPhoneNumberOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByPhoneNumber";
    helpers::send_soap_request(url, credentials, req).await
}

/* AddEmailAddress */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddEmailAddressInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddEmailAddress")]
    pub add_email_address: mod_wsd::AddEmailAddress,
}
impl restrictions::CheckRestrictions for AddEmailAddressInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_email_address.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddEmailAddressInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddEmailAddressInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddEmailAddressInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddEmailAddressOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "AddEmailAddressResponse")]
    pub add_email_address_response: mod_wsd::AddEmailAddressResponse,
}
impl restrictions::CheckRestrictions for AddEmailAddressOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.add_email_address_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct AddEmailAddressOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: AddEmailAddressOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for AddEmailAddressOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn add_email_address(
    req: AddEmailAddressInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<AddEmailAddressOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddEmailAddress";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemoveCustomField */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveCustomFieldInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveCustomField")]
    pub remove_custom_field: mod_wsd::RemoveCustomField,
}
impl restrictions::CheckRestrictions for RemoveCustomFieldInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_custom_field.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveCustomFieldInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveCustomFieldInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveCustomFieldInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveCustomFieldOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveCustomFieldResponse")]
    pub remove_custom_field_response: mod_wsd::RemoveCustomFieldResponse,
}
impl restrictions::CheckRestrictions for RemoveCustomFieldOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_custom_field_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveCustomFieldOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveCustomFieldOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveCustomFieldOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_custom_field(
    req: RemoveCustomFieldInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemoveCustomFieldOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveCustomField";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetNoCustContactsByTime */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetNoCustContactsByTimeInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetNoCustContactsByTime")]
    pub get_no_cust_contacts_by_time: mod_wsd::GetNoCustContactsByTime,
}
impl restrictions::CheckRestrictions for GetNoCustContactsByTimeInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_no_cust_contacts_by_time.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetNoCustContactsByTimeInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetNoCustContactsByTimeInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetNoCustContactsByTimeInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetNoCustContactsByTimeOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "GetNoCustContactsByTimeResponse")]
    pub get_no_cust_contacts_by_time_response: mod_wsd::GetNoCustContactsByTimeResponse,
}
impl restrictions::CheckRestrictions for GetNoCustContactsByTimeOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_no_cust_contacts_by_time_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct GetNoCustContactsByTimeOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetNoCustContactsByTimeOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetNoCustContactsByTimeOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_no_cust_contacts_by_time(
    req: GetNoCustContactsByTimeInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetNoCustContactsByTimeOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetNoCustContactsByTime";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemoveSipUri */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveSipUriInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveSipUri")]
    pub remove_sip_uri: mod_wsd::RemoveSipUri,
}
impl restrictions::CheckRestrictions for RemoveSipUriInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_sip_uri.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveSipUriInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveSipUriInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveSipUriInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveSipUriOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemoveSipUriResponse")]
    pub remove_sip_uri_response: mod_wsd::RemoveSipUriResponse,
}
impl restrictions::CheckRestrictions for RemoveSipUriOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_sip_uri_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemoveSipUriOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemoveSipUriOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemoveSipUriOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_sip_uri(
    req: RemoveSipUriInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemoveSipUriOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveSipUri";
    helpers::send_soap_request(url, credentials, req).await
}

/* RemovePhoneNumber */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemovePhoneNumberInputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemovePhoneNumber")]
    pub remove_phone_number: mod_wsd::RemovePhoneNumber,
}
impl restrictions::CheckRestrictions for RemovePhoneNumberInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_phone_number.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemovePhoneNumberInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemovePhoneNumberInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemovePhoneNumberInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "wsd", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemovePhoneNumberOutputEnvelopeBody {
    #[yaserde(prefix = "wsd", rename = "RemovePhoneNumberResponse")]
    pub remove_phone_number_response: mod_wsd::RemovePhoneNumberResponse,
}
impl restrictions::CheckRestrictions for RemovePhoneNumberOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.remove_phone_number_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "wsd" = "http://ws.db.ccmm.applications.nortel.com" })]
pub struct RemovePhoneNumberOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: RemovePhoneNumberOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for RemovePhoneNumberOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn remove_phone_number(
    req: RemovePhoneNumberInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<RemovePhoneNumberOutputEnvelope> {
    let url = "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemovePhoneNumber";
    helpers::send_soap_request(url, credentials, req).await
}
pub struct CustMultimedia {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl CustMultimedia {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "https://aacc1ver7/csp/multimedia/ws.Customer.cls".to_string(),
            credentials,
        }
    }

    pub async fn get_searchable_fields(
        &self,
        req: GetSearchableFieldsInputEnvelope,
    ) -> error::SoapResult<GetSearchableFieldsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_last_name(
        &self,
        req: UpdateLastNameInputEnvelope,
    ) -> error::SoapResult<UpdateLastNameOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_preferred_agent(
        &self,
        req: UpdatePreferredAgentInputEnvelope,
    ) -> error::SoapResult<UpdatePreferredAgentOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_user_name(
        &self,
        req: GetCustomerByUserNameInputEnvelope,
    ) -> error::SoapResult<GetCustomerByUserNameOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn read_customer(&self, req: ReadCustomerInputEnvelope) -> error::SoapResult<ReadCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_email(
        &self,
        req: GetCustomerByEmailInputEnvelope,
    ) -> error::SoapResult<GetCustomerByEmailOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_contact(
        &self,
        req: RemoveContactInputEnvelope,
    ) -> error::SoapResult<RemoveContactOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_phone_number(
        &self,
        req: AddPhoneNumberInputEnvelope,
    ) -> error::SoapResult<AddPhoneNumberOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn clean_customer(
        &self,
        req: CleanCustomerInputEnvelope,
    ) -> error::SoapResult<CleanCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn read_customer_history(
        &self,
        req: ReadCustomerHistoryInputEnvelope,
    ) -> error::SoapResult<ReadCustomerHistoryOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_email_address(
        &self,
        req: RemoveEmailAddressInputEnvelope,
    ) -> error::SoapResult<RemoveEmailAddressOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_objection(
        &self,
        req: UpdateObjectionInputEnvelope,
    ) -> error::SoapResult<UpdateObjectionOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn set_agent_id(&self, req: SetAgentIDInputEnvelope) -> error::SoapResult<SetAgentIDOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_contact(&self, req: AddContactInputEnvelope) -> error::SoapResult<AddContactOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn customer_association(
        &self,
        req: CustomerAssociationInputEnvelope,
    ) -> error::SoapResult<CustomerAssociationOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn register_customer(
        &self,
        req: RegisterCustomerInputEnvelope,
    ) -> error::SoapResult<RegisterCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_address(
        &self,
        req: RemoveAddressInputEnvelope,
    ) -> error::SoapResult<RemoveAddressOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_register_date(
        &self,
        req: UpdateRegisterDateInputEnvelope,
    ) -> error::SoapResult<UpdateRegisterDateOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_custom_field(
        &self,
        req: AddCustomFieldInputEnvelope,
    ) -> error::SoapResult<AddCustomFieldOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_sip_uri(&self, req: AddSipUriInputEnvelope) -> error::SoapResult<AddSipUriOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_name(
        &self,
        req: GetCustomerByNameInputEnvelope,
    ) -> error::SoapResult<GetCustomerByNameOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn register_anonymous_customer(
        &self,
        req: RegisterAnonymousCustomerInputEnvelope,
    ) -> error::SoapResult<RegisterAnonymousCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn create_customer_by_sip_uri(
        &self,
        req: CreateCustomerBySipUriInputEnvelope,
    ) -> error::SoapResult<CreateCustomerBySipUriOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_customer(
        &self,
        req: UpdateCustomerInputEnvelope,
    ) -> error::SoapResult<UpdateCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_password(
        &self,
        req: UpdatePasswordInputEnvelope,
    ) -> error::SoapResult<UpdatePasswordOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_title(&self, req: UpdateTitleInputEnvelope) -> error::SoapResult<UpdateTitleOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_user_name(
        &self,
        req: UpdateUserNameInputEnvelope,
    ) -> error::SoapResult<UpdateUserNameOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_login_page(
        &self,
        req: UpdateLoginPageInputEnvelope,
    ) -> error::SoapResult<UpdateLoginPageOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn carbon_copy(&self, req: CarbonCopyInputEnvelope) -> error::SoapResult<CarbonCopyOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_all_customers(
        &self,
        req: GetAllCustomersInputEnvelope,
    ) -> error::SoapResult<GetAllCustomersOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn send_ad_password_reminder(
        &self,
        req: SendADPasswordReminderInputEnvelope,
    ) -> error::SoapResult<SendADPasswordReminderOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn customer_search(
        &self,
        req: CustomerSearchInputEnvelope,
    ) -> error::SoapResult<CustomerSearchOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn update_first_name(
        &self,
        req: UpdateFirstNameInputEnvelope,
    ) -> error::SoapResult<UpdateFirstNameOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_address(&self, req: AddAddressInputEnvelope) -> error::SoapResult<AddAddressOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_sip_uri(
        &self,
        req: GetCustomerBySipUriInputEnvelope,
    ) -> error::SoapResult<GetCustomerBySipUriOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_cust_sql_columns(
        &self,
        req: GetCustSQLColumnsInputEnvelope,
    ) -> error::SoapResult<GetCustSQLColumnsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn impersonate_customer(
        &self,
        req: ImpersonateCustomerInputEnvelope,
    ) -> error::SoapResult<ImpersonateCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn send_password_reminder(
        &self,
        req: SendPasswordReminderInputEnvelope,
    ) -> error::SoapResult<SendPasswordReminderOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn create_customer(
        &self,
        req: CreateCustomerInputEnvelope,
    ) -> error::SoapResult<CreateCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_custom_field_templates(
        &self,
        req: GetCustomFieldTemplatesInputEnvelope,
    ) -> error::SoapResult<GetCustomFieldTemplatesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_contact_id(
        &self,
        req: GetCustomerByContactIdInputEnvelope,
    ) -> error::SoapResult<GetCustomerByContactIdOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn delete_customer(
        &self,
        req: DeleteCustomerInputEnvelope,
    ) -> error::SoapResult<DeleteCustomerOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_customer_by_phone_number(
        &self,
        req: GetCustomerByPhoneNumberInputEnvelope,
    ) -> error::SoapResult<GetCustomerByPhoneNumberOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn add_email_address(
        &self,
        req: AddEmailAddressInputEnvelope,
    ) -> error::SoapResult<AddEmailAddressOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_custom_field(
        &self,
        req: RemoveCustomFieldInputEnvelope,
    ) -> error::SoapResult<RemoveCustomFieldOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_no_cust_contacts_by_time(
        &self,
        req: GetNoCustContactsByTimeInputEnvelope,
    ) -> error::SoapResult<GetNoCustContactsByTimeOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_sip_uri(
        &self,
        req: RemoveSipUriInputEnvelope,
    ) -> error::SoapResult<RemoveSipUriOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn remove_phone_number(
        &self,
        req: RemovePhoneNumberInputEnvelope,
    ) -> error::SoapResult<RemovePhoneNumberOutputEnvelope> {
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
