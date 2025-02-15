//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.11
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
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
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddAddressSoapIn")]
    pub struct AddAddressSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddAddress,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddAddressSoapOut")]
    pub struct AddAddressSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddAddressResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddContactSoapIn")]
    pub struct AddContactSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddContact,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddContactSoapOut")]
    pub struct AddContactSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddContactResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddCustomFieldSoapIn")]
    pub struct AddCustomFieldSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddCustomField,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddCustomFieldSoapOut")]
    pub struct AddCustomFieldSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddCustomFieldResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddEmailAddressSoapIn")]
    pub struct AddEmailAddressSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddEmailAddress,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddEmailAddressSoapOut")]
    pub struct AddEmailAddressSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddEmailAddressResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddPhoneNumberSoapIn")]
    pub struct AddPhoneNumberSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddPhoneNumber,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddPhoneNumberSoapOut")]
    pub struct AddPhoneNumberSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddPhoneNumberResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddSipUriSoapIn")]
    pub struct AddSipUriSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::AddSipUri,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "AddSipUriSoapOut")]
    pub struct AddSipUriSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::AddSipUriResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CarbonCopySoapIn")]
    pub struct CarbonCopySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CarbonCopy,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CarbonCopySoapOut")]
    pub struct CarbonCopySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CarbonCopyResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CleanCustomerSoapIn")]
    pub struct CleanCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CleanCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CleanCustomerSoapOut")]
    pub struct CleanCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CleanCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateCustomerSoapIn")]
    pub struct CreateCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CreateCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateCustomerSoapOut")]
    pub struct CreateCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CreateCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateCustomerBySipUriSoapIn")]
    pub struct CreateCustomerBySipUriSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CreateCustomerBySipUri,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CreateCustomerBySipUriSoapOut")]
    pub struct CreateCustomerBySipUriSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CreateCustomerBySipUriResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CustomerAssociationSoapIn")]
    pub struct CustomerAssociationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CustomerAssociation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CustomerAssociationSoapOut")]
    pub struct CustomerAssociationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CustomerAssociationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CustomerSearchSoapIn")]
    pub struct CustomerSearchSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::CustomerSearch,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "CustomerSearchSoapOut")]
    pub struct CustomerSearchSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::CustomerSearchResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "DeleteCustomerSoapIn")]
    pub struct DeleteCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::DeleteCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "DeleteCustomerSoapOut")]
    pub struct DeleteCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::DeleteCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetAllCustomersSoapIn")]
    pub struct GetAllCustomersSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetAllCustomers,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetAllCustomersSoapOut")]
    pub struct GetAllCustomersSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetAllCustomersResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustSQLColumnsSoapIn")]
    pub struct GetCustSQLColumnsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustSQLColumns,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustSQLColumnsSoapOut")]
    pub struct GetCustSQLColumnsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustSQLColumnsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomFieldTemplatesSoapIn")]
    pub struct GetCustomFieldTemplatesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomFieldTemplates,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomFieldTemplatesSoapOut")]
    pub struct GetCustomFieldTemplatesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomFieldTemplatesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByContactIdSoapIn")]
    pub struct GetCustomerByContactIdSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByContactId,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByContactIdSoapOut")]
    pub struct GetCustomerByContactIdSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByContactIdResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByEmailSoapIn")]
    pub struct GetCustomerByEmailSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByEmail,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByEmailSoapOut")]
    pub struct GetCustomerByEmailSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByEmailResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByNameSoapIn")]
    pub struct GetCustomerByNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByNameSoapOut")]
    pub struct GetCustomerByNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByPhoneNumberSoapIn")]
    pub struct GetCustomerByPhoneNumberSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByPhoneNumber,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByPhoneNumberSoapOut")]
    pub struct GetCustomerByPhoneNumberSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByPhoneNumberResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerBySipUriSoapIn")]
    pub struct GetCustomerBySipUriSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerBySipUri,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerBySipUriSoapOut")]
    pub struct GetCustomerBySipUriSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerBySipUriResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByUserNameSoapIn")]
    pub struct GetCustomerByUserNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByUserName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCustomerByUserNameSoapOut")]
    pub struct GetCustomerByUserNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCustomerByUserNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetNoCustContactsByTimeSoapIn")]
    pub struct GetNoCustContactsByTimeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetNoCustContactsByTime,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetNoCustContactsByTimeSoapOut")]
    pub struct GetNoCustContactsByTimeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetNoCustContactsByTimeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetSearchableFieldsSoapIn")]
    pub struct GetSearchableFieldsSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetSearchableFields,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetSearchableFieldsSoapOut")]
    pub struct GetSearchableFieldsSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetSearchableFieldsResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ImpersonateCustomerSoapIn")]
    pub struct ImpersonateCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::ImpersonateCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ImpersonateCustomerSoapOut")]
    pub struct ImpersonateCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::ImpersonateCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ReadCustomerSoapIn")]
    pub struct ReadCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::ReadCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ReadCustomerSoapOut")]
    pub struct ReadCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::ReadCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ReadCustomerHistorySoapIn")]
    pub struct ReadCustomerHistorySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::ReadCustomerHistory,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "ReadCustomerHistorySoapOut")]
    pub struct ReadCustomerHistorySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::ReadCustomerHistoryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RegisterAnonymousCustomerSoapIn")]
    pub struct RegisterAnonymousCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RegisterAnonymousCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RegisterAnonymousCustomerSoapOut")]
    pub struct RegisterAnonymousCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RegisterAnonymousCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RegisterCustomerSoapIn")]
    pub struct RegisterCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RegisterCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RegisterCustomerSoapOut")]
    pub struct RegisterCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RegisterCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveAddressSoapIn")]
    pub struct RemoveAddressSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveAddress,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveAddressSoapOut")]
    pub struct RemoveAddressSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveAddressResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveContactSoapIn")]
    pub struct RemoveContactSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveContact,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveContactSoapOut")]
    pub struct RemoveContactSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveContactResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveCustomFieldSoapIn")]
    pub struct RemoveCustomFieldSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveCustomField,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveCustomFieldSoapOut")]
    pub struct RemoveCustomFieldSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveCustomFieldResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveEmailAddressSoapIn")]
    pub struct RemoveEmailAddressSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveEmailAddress,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveEmailAddressSoapOut")]
    pub struct RemoveEmailAddressSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveEmailAddressResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemovePhoneNumberSoapIn")]
    pub struct RemovePhoneNumberSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemovePhoneNumber,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemovePhoneNumberSoapOut")]
    pub struct RemovePhoneNumberSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemovePhoneNumberResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveSipUriSoapIn")]
    pub struct RemoveSipUriSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveSipUri,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "RemoveSipUriSoapOut")]
    pub struct RemoveSipUriSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::RemoveSipUriResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SendADPasswordReminderSoapIn")]
    pub struct SendADPasswordReminderSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::SendADPasswordReminder,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SendADPasswordReminderSoapOut")]
    pub struct SendADPasswordReminderSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::SendADPasswordReminderResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SendPasswordReminderSoapIn")]
    pub struct SendPasswordReminderSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::SendPasswordReminder,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SendPasswordReminderSoapOut")]
    pub struct SendPasswordReminderSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::SendPasswordReminderResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SetAgentIDSoapIn")]
    pub struct SetAgentIDSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::SetAgentID,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "SetAgentIDSoapOut")]
    pub struct SetAgentIDSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::SetAgentIDResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateCustomerSoapIn")]
    pub struct UpdateCustomerSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateCustomer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateCustomerSoapOut")]
    pub struct UpdateCustomerSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateCustomerResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateFirstNameSoapIn")]
    pub struct UpdateFirstNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateFirstName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateFirstNameSoapOut")]
    pub struct UpdateFirstNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateFirstNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateLastNameSoapIn")]
    pub struct UpdateLastNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateLastName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateLastNameSoapOut")]
    pub struct UpdateLastNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateLastNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateLoginPageSoapIn")]
    pub struct UpdateLoginPageSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateLoginPage,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateLoginPageSoapOut")]
    pub struct UpdateLoginPageSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateLoginPageResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateObjectionSoapIn")]
    pub struct UpdateObjectionSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateObjection,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateObjectionSoapOut")]
    pub struct UpdateObjectionSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateObjectionResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdatePasswordSoapIn")]
    pub struct UpdatePasswordSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdatePassword,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdatePasswordSoapOut")]
    pub struct UpdatePasswordSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdatePasswordResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdatePreferredAgentSoapIn")]
    pub struct UpdatePreferredAgentSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdatePreferredAgent,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdatePreferredAgentSoapOut")]
    pub struct UpdatePreferredAgentSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdatePreferredAgentResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateRegisterDateSoapIn")]
    pub struct UpdateRegisterDateSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateRegisterDate,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateRegisterDateSoapOut")]
    pub struct UpdateRegisterDateSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateRegisterDateResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateTitleSoapIn")]
    pub struct UpdateTitleSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateTitle,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateTitleSoapOut")]
    pub struct UpdateTitleSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateTitleResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateUserNameSoapIn")]
    pub struct UpdateUserNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateUserName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "UpdateUserNameSoapOut")]
    pub struct UpdateUserNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::UpdateUserNameResponse,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddAddress {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newAddress", prefix = "tns", default)]
        pub new_address: Option<Address>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Address",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Address {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Line1", prefix = "tns", default)]
        pub line_1: Option<String>,
        #[yaserde(rename = "Line2", prefix = "tns", default)]
        pub line_2: Option<String>,
        #[yaserde(rename = "Line3", prefix = "tns", default)]
        pub line_3: Option<String>,
        #[yaserde(rename = "Line4", prefix = "tns", default)]
        pub line_4: Option<String>,
        #[yaserde(rename = "Line5", prefix = "tns", default)]
        pub line_5: Option<String>,
        #[yaserde(rename = "ZipCode", prefix = "tns", default)]
        pub zip_code: Option<String>,
        #[yaserde(rename = "Country", prefix = "tns", default)]
        pub country: Option<String>,
        #[yaserde(rename = "Default", prefix = "tns", default)]
        pub default: Option<bool>,
        #[yaserde(rename = "Imported", prefix = "tns", default)]
        pub imported: Option<bool>,
        #[yaserde(rename = "FullAddress", prefix = "tns", default)]
        pub full_address: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddAddressResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddAddressResponse {
        #[yaserde(rename = "AddAddressResult", prefix = "tns", default)]
        pub add_address_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddContact",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddContact {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "contactParams", prefix = "tns", default)]
        pub contact_params: Option<ContactCreate>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ContactCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ContactCreate {
        #[yaserde(rename = "NewCount", prefix = "tns", default)]
        pub new_count: Option<i64>,
        #[yaserde(rename = "OriginalSubject", prefix = "tns", default)]
        pub original_subject: Option<String>,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "Agent", prefix = "tns", default)]
        pub agent: Option<i64>,
        #[yaserde(rename = "Skillset", prefix = "tns", default)]
        pub skillset: Option<i64>,
        #[yaserde(rename = "AcquiredTime", prefix = "tns", default)]
        pub acquired_time: Option<String>,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<i64>,
        #[yaserde(rename = "MailTo", prefix = "tns", default)]
        pub mail_to: Option<String>,
        #[yaserde(rename = "MailFrom", prefix = "tns", default)]
        pub mail_from: Option<String>,
        #[yaserde(rename = "MailCC", prefix = "tns", default)]
        pub mail_cc: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<i64>,
        #[yaserde(rename = "QueueType", prefix = "tns", default)]
        pub queue_type: Option<i64>,
        #[yaserde(rename = "OpenTime", prefix = "tns", default)]
        pub open_time: Option<String>,
        #[yaserde(rename = "ClosedTime", prefix = "tns", default)]
        pub closed_time: Option<String>,
        #[yaserde(rename = "AutoResponse", prefix = "tns", default)]
        pub auto_response: Option<i64>,
        #[yaserde(rename = "Rule", prefix = "tns", default)]
        pub rule: Option<i64>,
        #[yaserde(rename = "Priority", prefix = "tns", default)]
        pub priority: Option<i64>,
        #[yaserde(rename = "OpenDuration", prefix = "tns", default)]
        pub open_duration: Option<i64>,
        #[yaserde(rename = "SubStatus", prefix = "tns", default)]
        pub sub_status: Option<i64>,
        #[yaserde(rename = "CMFStatus", prefix = "tns", default)]
        pub cmf_status: Option<i64>,
        #[yaserde(rename = "CallId", prefix = "tns", default)]
        pub call_id: Option<i64>,
        #[yaserde(rename = "WebOnHoldTag", prefix = "tns", default)]
        pub web_on_hold_tag: Option<String>,
        #[yaserde(rename = "TimeZone", prefix = "tns", default)]
        pub time_zone: Option<i16>,
        #[yaserde(rename = "CharSet", prefix = "tns", default)]
        pub char_set: Option<String>,
        #[yaserde(rename = "ClickStreamText", prefix = "tns", default)]
        pub click_stream_text: Option<String>,
        #[yaserde(rename = "OutBoundOriginator", prefix = "tns", default)]
        pub out_bound_originator: Option<String>,
        #[yaserde(rename = "Campaign", prefix = "tns", default)]
        pub campaign: Option<i64>,
        #[yaserde(rename = "CampaignRetryCount", prefix = "tns", default)]
        pub campaign_retry_count: Option<i64>,
        #[yaserde(rename = "DispositionCode", prefix = "tns", default)]
        pub disposition_code: Option<i64>,
        #[yaserde(rename = "PreferredCallBackMedia", prefix = "tns", default)]
        pub preferred_call_back_media: Option<i64>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "LastAction", prefix = "tns", default)]
        pub last_action: Option<i64>,
        #[yaserde(rename = "ClosureType", prefix = "tns", default)]
        pub closure_type: Option<i64>,
        #[yaserde(rename = "Answers", prefix = "tns", default)]
        pub answers: Option<ArrayOfAnswer>,
        #[yaserde(rename = "ActionCreateList", prefix = "tns", default)]
        pub action_create_list: Option<ArrayOfActionCreate>,
        #[yaserde(rename = "CustomFieldCreateList", prefix = "tns", default)]
        pub custom_field_create_list: Option<ArrayOfCustomFieldCreate>,
        #[yaserde(rename = "CmfId", prefix = "tns", default)]
        pub cmf_id: Option<String>,
        #[yaserde(rename = "RouteSummary", prefix = "tns", default)]
        pub route_summary: Option<String>,
        #[yaserde(rename = "Importance", prefix = "tns", default)]
        pub importance: Option<i64>,
        #[yaserde(rename = "SocialMediaHeader", prefix = "tns", default)]
        pub social_media_header: Option<SocialMediaHeader>,
        #[yaserde(rename = "SMFToken", prefix = "tns", default)]
        pub smf_token: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAnswer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAnswer {
        #[yaserde(rename = "Answer", prefix = "tns", default)]
        pub answer: Vec<Answer>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Answer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Answer {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "QuestionID", prefix = "tns", default)]
        pub question_id: Option<i64>,
        #[yaserde(rename = "Answer", prefix = "tns", default)]
        pub answer: Option<String>,
        #[yaserde(rename = "FreeText", prefix = "tns", default)]
        pub free_text: Option<String>,
        #[yaserde(rename = "DateCreated", prefix = "tns", default)]
        pub date_created: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfActionCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfActionCreate {
        #[yaserde(rename = "ActionCreate", prefix = "tns", default)]
        pub action_create: Vec<ActionCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ActionCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ActionCreate {
        #[yaserde(rename = "ContactId", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<i64>,
        #[yaserde(rename = "Subject", prefix = "tns", default)]
        pub subject: Option<String>,
        #[yaserde(rename = "Text", prefix = "tns", default)]
        pub text: Option<String>,
        #[yaserde(rename = "TextHTML", prefix = "tns", default)]
        pub text_html: Option<String>,
        #[yaserde(rename = "CharSet", prefix = "tns", default)]
        pub char_set: Option<String>,
        #[yaserde(rename = "Attempt", prefix = "tns", default)]
        pub attempt: Option<i64>,
        #[yaserde(rename = "DispositionCode", prefix = "tns", default)]
        pub disposition_code: Option<i64>,
        #[yaserde(rename = "MailTo", prefix = "tns", default)]
        pub mail_to: Option<String>,
        #[yaserde(rename = "MailFrom", prefix = "tns", default)]
        pub mail_from: Option<String>,
        #[yaserde(rename = "MailCC", prefix = "tns", default)]
        pub mail_cc: Option<String>,
        #[yaserde(rename = "MailBCC", prefix = "tns", default)]
        pub mail_bcc: Option<String>,
        #[yaserde(rename = "CallBackTime", prefix = "tns", default)]
        pub call_back_time: Option<String>,
        #[yaserde(rename = "CallBackMedia", prefix = "tns", default)]
        pub call_back_media: Option<i64>,
        #[yaserde(rename = "CallBackStatus", prefix = "tns", default)]
        pub call_back_status: Option<i64>,
        #[yaserde(rename = "Agent", prefix = "tns", default)]
        pub agent: Option<i64>,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "TimeAllocated", prefix = "tns", default)]
        pub time_allocated: Option<i64>,
        #[yaserde(rename = "TemplateLocation", prefix = "tns", default)]
        pub template_location: Option<String>,
        #[yaserde(rename = "ClosedReason", prefix = "tns", default)]
        pub closed_reason: Option<i64>,
        #[yaserde(rename = "HistoryFlag", prefix = "tns", default)]
        pub history_flag: Option<bool>,
        #[yaserde(rename = "NumberUsed", prefix = "tns", default)]
        pub number_used: Option<String>,
        #[yaserde(rename = "CallStartTime", prefix = "tns", default)]
        pub call_start_time: Option<String>,
        #[yaserde(rename = "CallEndTime", prefix = "tns", default)]
        pub call_end_time: Option<String>,
        #[yaserde(rename = "DialStartTime", prefix = "tns", default)]
        pub dial_start_time: Option<String>,
        #[yaserde(rename = "DialEndTime", prefix = "tns", default)]
        pub dial_end_time: Option<String>,
        #[yaserde(rename = "OpenTime", prefix = "tns", default)]
        pub open_time: Option<String>,
        #[yaserde(rename = "ClosedTime", prefix = "tns", default)]
        pub closed_time: Option<String>,
        #[yaserde(rename = "CustomFieldCreateList", prefix = "tns", default)]
        pub custom_field_create_list: Option<ArrayOfCustomFieldCreate>,
        #[yaserde(rename = "AttachmentList", prefix = "tns", default)]
        pub attachment_list: Option<ArrayOfAttachmentCreate>,
        #[yaserde(rename = "AutoSuggest", prefix = "tns", default)]
        pub auto_suggest: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCustomFieldCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCustomFieldCreate {
        #[yaserde(rename = "CustomFieldCreate", prefix = "tns", default)]
        pub custom_field_create: Vec<CustomFieldCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomFieldCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomFieldCreate {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Text", prefix = "tns", default)]
        pub text: Option<String>,
        #[yaserde(rename = "IsTextVisible", prefix = "tns", default)]
        pub is_text_visible: Option<bool>,
        #[yaserde(rename = "CustomFieldTemplate", prefix = "tns", default)]
        pub custom_field_template: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAttachmentCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAttachmentCreate {
        #[yaserde(rename = "AttachmentCreate", prefix = "tns", default)]
        pub attachment_create: Vec<AttachmentCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AttachmentCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct AttachmentCreate {
        #[yaserde(rename = "DisplayFileName", prefix = "tns", default)]
        pub display_file_name: Option<String>,
        #[yaserde(rename = "InternalFileName", prefix = "tns", default)]
        pub internal_file_name: Option<String>,
        #[yaserde(rename = "Direction", prefix = "tns", default)]
        pub direction: Option<i64>,
        #[yaserde(rename = "FileContents", prefix = "tns", default)]
        pub file_contents: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SocialMediaHeader",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SocialMediaHeader {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "DesktopUrl", prefix = "tns", default)]
        pub desktop_url: Option<String>,
        #[yaserde(rename = "InteractionID", prefix = "tns", default)]
        pub interaction_id: Option<String>,
        #[yaserde(rename = "Query", prefix = "tns", default)]
        pub query: Option<String>,
        #[yaserde(rename = "Domain", prefix = "tns", default)]
        pub domain: Option<String>,
        #[yaserde(rename = "Classification", prefix = "tns", default)]
        pub classification: Option<String>,
        #[yaserde(rename = "Relevance", prefix = "tns", default)]
        pub relevance: Option<String>,
        #[yaserde(rename = "Sentiment", prefix = "tns", default)]
        pub sentiment: Option<String>,
        #[yaserde(rename = "Language", prefix = "tns", default)]
        pub language: Option<String>,
        #[yaserde(rename = "Author", prefix = "tns", default)]
        pub author: Option<String>,
        #[yaserde(rename = "Channel", prefix = "tns", default)]
        pub channel: Option<String>,
        #[yaserde(rename = "Keywords", prefix = "tns", default)]
        pub keywords: Option<String>,
        #[yaserde(rename = "MimeContactType", prefix = "tns", default)]
        pub mime_contact_type: Option<String>,
        #[yaserde(rename = "SocialMediaRuleId", prefix = "tns", default)]
        pub social_media_rule_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddContactResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddContactResponse {
        #[yaserde(rename = "AddContactResult", prefix = "tns", default)]
        pub add_contact_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddCustomField",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddCustomField {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "customFieldParams", prefix = "tns", default)]
        pub custom_field_params: Option<CustomFieldCreate>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddCustomFieldResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddCustomFieldResponse {
        #[yaserde(rename = "AddCustomFieldResult", prefix = "tns", default)]
        pub add_custom_field_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddEmailAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddEmailAddress {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newEmailAddress", prefix = "tns", default)]
        pub new_email_address: Option<EmailAddress>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "EmailAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct EmailAddress {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "SearchAddress", prefix = "tns", default)]
        pub search_address: Option<String>,
        #[yaserde(rename = "Default", prefix = "tns", default)]
        pub default: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddEmailAddressResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddEmailAddressResponse {
        #[yaserde(rename = "AddEmailAddressResult", prefix = "tns", default)]
        pub add_email_address_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddPhoneNumber",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddPhoneNumber {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "phoneNumberParams", prefix = "tns", default)]
        pub phone_number_params: Option<PhoneNumberCreate>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PhoneNumberCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct PhoneNumberCreate {
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "InternationalCode", prefix = "tns", default)]
        pub international_code: Option<String>,
        #[yaserde(rename = "AreaCode", prefix = "tns", default)]
        pub area_code: Option<String>,
        #[yaserde(rename = "Number", prefix = "tns", default)]
        pub number: Option<String>,
        #[yaserde(rename = "DoNotCall", prefix = "tns", default)]
        pub do_not_call: Option<bool>,
        #[yaserde(rename = "Default", prefix = "tns", default)]
        pub default: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddPhoneNumberResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddPhoneNumberResponse {
        #[yaserde(rename = "AddPhoneNumberResult", prefix = "tns", default)]
        pub add_phone_number_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddSipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddSipUri {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newSipUri", prefix = "tns", default)]
        pub new_sip_uri: Option<SipUri>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SipUri {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "SearchAddress", prefix = "tns", default)]
        pub search_address: Option<String>,
        #[yaserde(rename = "Default", prefix = "tns", default)]
        pub default: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AddSipUriResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct AddSipUriResponse {
        #[yaserde(rename = "AddSipUriResult", prefix = "tns", default)]
        pub add_sip_uri_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CarbonCopy",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CarbonCopy {
        #[yaserde(rename = "customerId", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "contactId", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "copyMode", prefix = "tns", default)]
        pub copy_mode: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CarbonCopyResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CarbonCopyResponse {
        #[yaserde(rename = "CarbonCopyResult", prefix = "tns", default)]
        pub carbon_copy_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CleanCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CleanCustomer {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "bRemoveAllContacts", prefix = "tns", default)]
        pub b_remove_all_contacts: Option<bool>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CleanCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CleanCustomerResponse {
        #[yaserde(rename = "CleanCustomerResult", prefix = "tns", default)]
        pub clean_customer_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Customer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Customer {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Addresses", prefix = "tns", default)]
        pub addresses: Option<ArrayOfAddress>,
        #[yaserde(rename = "PhoneNumbers", prefix = "tns", default)]
        pub phone_numbers: Option<ArrayOfPhoneNumber>,
        #[yaserde(rename = "EmailAddresses", prefix = "tns", default)]
        pub email_addresses: Option<ArrayOfEmailAddress>,
        #[yaserde(rename = "Contacts", prefix = "tns", default)]
        pub contacts: Option<ArrayOfContact>,
        #[yaserde(rename = "CustomFields", prefix = "tns", default)]
        pub custom_fields: Option<ArrayOfCustomField>,
        #[yaserde(rename = "SipUris", prefix = "tns", default)]
        pub sip_uris: Option<ArrayOfSipUri>,
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LoginPage", prefix = "tns", default)]
        pub login_page: Option<CodeMapping>,
        #[yaserde(rename = "RegisterDate", prefix = "tns", default)]
        pub register_date: Option<String>,
        #[yaserde(rename = "UserName", prefix = "tns", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "PreferredAgent", prefix = "tns", default)]
        pub preferred_agent: Option<User>,
        #[yaserde(rename = "PreferredAgentID", prefix = "tns", default)]
        pub preferred_agent_id: Option<i64>,
        #[yaserde(rename = "Objection", prefix = "tns", default)]
        pub objection: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAddress {
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Vec<Address>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPhoneNumber",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfPhoneNumber {
        #[yaserde(rename = "PhoneNumber", prefix = "tns", default)]
        pub phone_number: Vec<PhoneNumber>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PhoneNumber",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct PhoneNumber {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<CodeMapping>,
        #[yaserde(rename = "InternationalCode", prefix = "tns", default)]
        pub international_code: Option<String>,
        #[yaserde(rename = "AreaCode", prefix = "tns", default)]
        pub area_code: Option<String>,
        #[yaserde(rename = "Number", prefix = "tns", default)]
        pub number: Option<String>,
        #[yaserde(rename = "FullNumber", prefix = "tns", default)]
        pub full_number: Option<String>,
        #[yaserde(rename = "DoNotCall", prefix = "tns", default)]
        pub do_not_call: Option<bool>,
        #[yaserde(rename = "Default", prefix = "tns", default)]
        pub default: Option<bool>,
        #[yaserde(rename = "Offset", prefix = "tns", default)]
        pub offset: Option<i64>,
        #[yaserde(rename = "TimeZoneId", prefix = "tns", default)]
        pub time_zone_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CodeMapping",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CodeMapping {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "FieldName", prefix = "tns", default)]
        pub field_name: Option<String>,
        #[yaserde(rename = "NumericValue", prefix = "tns", default)]
        pub numeric_value: Option<i64>,
        #[yaserde(rename = "TextValue", prefix = "tns", default)]
        pub text_value: Option<String>,
        #[yaserde(rename = "Icon", prefix = "tns", default)]
        pub icon: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfEmailAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfEmailAddress {
        #[yaserde(rename = "EmailAddress", prefix = "tns", default)]
        pub email_address: Vec<EmailAddress>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfContact",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfContact {
        #[yaserde(rename = "Contact", prefix = "tns", default)]
        pub contact: Vec<Contact>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Contact",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Contact {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "ArrivalTime", prefix = "tns", default)]
        pub arrival_time: Option<String>,
        #[yaserde(rename = "OriginalSubject", prefix = "tns", default)]
        pub original_subject: Option<String>,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "Agent", prefix = "tns", default)]
        pub agent: Option<User>,
        #[yaserde(rename = "Skillset", prefix = "tns", default)]
        pub skillset: Option<Skillset>,
        #[yaserde(rename = "AcquiredTime", prefix = "tns", default)]
        pub acquired_time: Option<String>,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<CodeMapping>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<ContactType>,
        #[yaserde(rename = "MailTo", prefix = "tns", default)]
        pub mail_to: Option<String>,
        #[yaserde(rename = "MailFrom", prefix = "tns", default)]
        pub mail_from: Option<String>,
        #[yaserde(rename = "MailCC", prefix = "tns", default)]
        pub mail_cc: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<CodeMapping>,
        #[yaserde(rename = "QueueType", prefix = "tns", default)]
        pub queue_type: Option<CodeMapping>,
        #[yaserde(rename = "OpenTime", prefix = "tns", default)]
        pub open_time: Option<String>,
        #[yaserde(rename = "ClosedTime", prefix = "tns", default)]
        pub closed_time: Option<String>,
        #[yaserde(rename = "AutoResponse", prefix = "tns", default)]
        pub auto_response: Option<AutoResponse>,
        #[yaserde(rename = "Rule", prefix = "tns", default)]
        pub rule: Option<Rule>,
        #[yaserde(rename = "Priority", prefix = "tns", default)]
        pub priority: Option<i64>,
        #[yaserde(rename = "OpenDuration", prefix = "tns", default)]
        pub open_duration: Option<i64>,
        #[yaserde(rename = "SubStatus", prefix = "tns", default)]
        pub sub_status: Option<CodeMapping>,
        #[yaserde(rename = "CMFStatus", prefix = "tns", default)]
        pub cmf_status: Option<CodeMapping>,
        #[yaserde(rename = "CallId", prefix = "tns", default)]
        pub call_id: Option<i64>,
        #[yaserde(rename = "WebOnHoldTag", prefix = "tns", default)]
        pub web_on_hold_tag: Option<String>,
        #[yaserde(rename = "TimeZone", prefix = "tns", default)]
        pub time_zone: Option<i64>,
        #[yaserde(rename = "CharSet", prefix = "tns", default)]
        pub char_set: Option<String>,
        #[yaserde(rename = "ClickStreamText", prefix = "tns", default)]
        pub click_stream_text: Option<String>,
        #[yaserde(rename = "NNCCApplicationId", prefix = "tns", default)]
        pub nncc_application_id: Option<i64>,
        #[yaserde(rename = "OutBoundOriginator", prefix = "tns", default)]
        pub out_bound_originator: Option<String>,
        #[yaserde(rename = "Campaign", prefix = "tns", default)]
        pub campaign: Option<Campaign>,
        #[yaserde(rename = "NoOfCallAttempts", prefix = "tns", default)]
        pub no_of_call_attempts: Option<i64>,
        #[yaserde(rename = "CampaignRetryCount", prefix = "tns", default)]
        pub campaign_retry_count: Option<i64>,
        #[yaserde(rename = "DispositionCode", prefix = "tns", default)]
        pub disposition_code: Option<DispositionCode>,
        #[yaserde(rename = "PreferredCallBackMedia", prefix = "tns", default)]
        pub preferred_call_back_media: Option<ContactType>,
        #[yaserde(rename = "CallTime", prefix = "tns", default)]
        pub call_time: Option<i64>,
        #[yaserde(rename = "DialTime", prefix = "tns", default)]
        pub dial_time: Option<i64>,
        #[yaserde(rename = "LastAction", prefix = "tns", default)]
        pub last_action: Option<CodeMapping>,
        #[yaserde(rename = "ClosureType", prefix = "tns", default)]
        pub closure_type: Option<CodeMapping>,
        #[yaserde(rename = "CmfId", prefix = "tns", default)]
        pub cmf_id: Option<String>,
        #[yaserde(rename = "CustomFields", prefix = "tns", default)]
        pub custom_fields: Option<ArrayOfCustomField>,
        #[yaserde(rename = "Actions", prefix = "tns", default)]
        pub actions: Option<ArrayOfAction>,
        #[yaserde(rename = "Answers", prefix = "tns", default)]
        pub answers: Option<ArrayOfAnswer>,
        #[yaserde(rename = "ClosedReason", prefix = "tns", default)]
        pub closed_reason: Option<ClosedReason>,
        #[yaserde(rename = "RouteSummary", prefix = "tns", default)]
        pub route_summary: Option<String>,
        #[yaserde(rename = "CCOpenQueuedTime", prefix = "tns", default)]
        pub cc_open_queued_time: Option<i64>,
        #[yaserde(rename = "Importance", prefix = "tns", default)]
        pub importance: Option<i64>,
        #[yaserde(rename = "SocialMediaHeader", prefix = "tns", default)]
        pub social_media_header: Option<SocialMediaHeader>,
        #[yaserde(rename = "ApprovalCheck", prefix = "tns", default)]
        pub approval_check: Option<i64>,
        #[yaserde(rename = "ApprovalSkillsetId", prefix = "tns", default)]
        pub approval_skillset_id: Option<i64>,
        #[yaserde(rename = "SMFToken", prefix = "tns", default)]
        pub smf_token: Option<String>,
        #[yaserde(rename = "StartingEWT", prefix = "tns", default)]
        pub starting_ewt: Option<i64>,
        #[yaserde(rename = "StartingPIQ", prefix = "tns", default)]
        pub starting_piq: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "User",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct User {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "CCMSID", prefix = "tns", default)]
        pub ccmsid: Option<i64>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "Supervisor", prefix = "tns", default)]
        pub supervisor: Option<i64>,
        #[yaserde(rename = "TelsetLogonId", prefix = "tns", default)]
        pub telset_logon_id: Option<String>,
        #[yaserde(rename = "UserClass", prefix = "tns", default)]
        pub user_class: Option<String>,
        #[yaserde(rename = "RoutePoint", prefix = "tns", default)]
        pub route_point: Option<RoutePoint>,
        #[yaserde(rename = "Email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "Fax", prefix = "tns", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LogonId", prefix = "tns", default)]
        pub logon_id: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<bool>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "SipUri", prefix = "tns", default)]
        pub sip_uri: Option<String>,
        #[yaserde(rename = "SipTerminal", prefix = "tns", default)]
        pub sip_terminal: Option<String>,
        #[yaserde(rename = "DeletedOnCCMS", prefix = "tns", default)]
        pub deleted_on_ccms: Option<bool>,
        #[yaserde(rename = "PredictiveAgent", prefix = "tns", default)]
        pub predictive_agent: Option<bool>,
        #[yaserde(rename = "PredictiveReportingEnabled", prefix = "tns", default)]
        pub predictive_reporting_enabled: Option<bool>,
        #[yaserde(rename = "PredictiveWrapTimeLimit", prefix = "tns", default)]
        pub predictive_wrap_time_limit: Option<i64>,
        #[yaserde(rename = "DataPurge", prefix = "tns", default)]
        pub data_purge: Option<bool>,
        #[yaserde(rename = "DefaultPassword", prefix = "tns", default)]
        pub default_password: Option<bool>,
        #[yaserde(rename = "AgentGreetingState", prefix = "tns", default)]
        pub agent_greeting_state: Option<i64>,
        #[yaserde(rename = "BlendingStatus", prefix = "tns", default)]
        pub blending_status: Option<i64>,
        #[yaserde(rename = "LastBlendedTime", prefix = "tns", default)]
        pub last_blended_time: Option<String>,
        #[yaserde(rename = "BlendSkillsetId", prefix = "tns", default)]
        pub blend_skillset_id: Option<i64>,
        #[yaserde(rename = "AgentURL", prefix = "tns", default)]
        pub agent_url: Option<String>,
        #[yaserde(rename = "LastCCADBlendedTime", prefix = "tns", default)]
        pub last_ccad_blended_time: Option<String>,
        #[yaserde(rename = "BlendedEligiblityTime", prefix = "tns", default)]
        pub blended_eligiblity_time: Option<String>,
        #[yaserde(rename = "NailedUp", prefix = "tns", default)]
        pub nailed_up: Option<bool>,
        #[yaserde(rename = "Blended", prefix = "tns", default)]
        pub blended: Option<bool>,
        #[yaserde(rename = "BlendedReason", prefix = "tns", default)]
        pub blended_reason: Option<String>,
        #[yaserde(rename = "ACDLogin", prefix = "tns", default)]
        pub acd_login: Option<String>,
        #[yaserde(rename = "Extension", prefix = "tns", default)]
        pub extension: Option<String>,
        #[yaserde(rename = "ApprovalRatio", prefix = "tns", default)]
        pub approval_ratio: Option<i64>,
        #[yaserde(rename = "LastApprovalUpdate", prefix = "tns", default)]
        pub last_approval_update: Option<String>,
        #[yaserde(rename = "CurrentPOMSkillsetId", prefix = "tns", default)]
        pub current_pom_skillset_id: Option<i64>,
        #[yaserde(rename = "POMAgent", prefix = "tns", default)]
        pub pom_agent: Option<bool>,
        #[yaserde(rename = "SkillsetUpdateTime", prefix = "tns", default)]
        pub skillset_update_time: Option<String>,
        #[yaserde(rename = "FriendlyName", prefix = "tns", default)]
        pub friendly_name: Option<String>,
        #[yaserde(rename = "SoftPhoneEnabled", prefix = "tns", default)]
        pub soft_phone_enabled: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RoutePoint",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct RoutePoint {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "RoutePoint", prefix = "tns", default)]
        pub route_point: Option<String>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Skillset",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Skillset {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "CCMSID", prefix = "tns", default)]
        pub ccmsid: Option<i64>,
        #[yaserde(rename = "ActivityCode", prefix = "tns", default)]
        pub activity_code: Option<String>,
        #[yaserde(rename = "AutoSignature", prefix = "tns", default)]
        pub auto_signature: Option<String>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "MailBox", prefix = "tns", default)]
        pub mail_box: Option<Inbox>,
        #[yaserde(rename = "RoutePoint", prefix = "tns", default)]
        pub route_point: Option<RoutePoint>,
        #[yaserde(rename = "WebCommsComfortGroup", prefix = "tns", default)]
        pub web_comms_comfort_group: Option<WebCommsComfortGroup>,
        #[yaserde(rename = "WebCommOnHoldGroup", prefix = "tns", default)]
        pub web_comm_on_hold_group: Option<WebCommsComfortGroup>,
        #[yaserde(rename = "Mapping", prefix = "tns", default)]
        pub mapping: Option<String>,
        #[yaserde(rename = "Agents", prefix = "tns", default)]
        pub agents: Option<ArrayOfUserPairOfAgentsKeyUser>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<CodeMapping>,
        #[yaserde(rename = "Threshold", prefix = "tns", default)]
        pub threshold: Option<i64>,
        #[yaserde(rename = "UseOriginalAddress", prefix = "tns", default)]
        pub use_original_address: Option<i64>,
        #[yaserde(rename = "WebDescription", prefix = "tns", default)]
        pub web_description: Option<String>,
        #[yaserde(rename = "DeletedOnCCMS", prefix = "tns", default)]
        pub deleted_on_ccms: Option<bool>,
        #[yaserde(rename = "PagePushURLs", prefix = "tns", default)]
        pub page_push_ur_ls: Option<ArrayOfPagePushURL>,
        #[yaserde(rename = "Phrases", prefix = "tns", default)]
        pub phrases: Option<ArrayOfPhrase>,
        #[yaserde(rename = "WelcomeMessage", prefix = "tns", default)]
        pub welcome_message: Option<String>,
        #[yaserde(rename = "WrapUpMessage", prefix = "tns", default)]
        pub wrap_up_message: Option<String>,
        #[yaserde(rename = "Expert", prefix = "tns", default)]
        pub expert: Option<ArrayOfExpert>,
        #[yaserde(rename = "SkillsetThresholdTemplate", prefix = "tns", default)]
        pub skillset_threshold_template: Option<SkillsetThresholdTemplate>,
        #[yaserde(rename = "Calendar", prefix = "tns", default)]
        pub calendar: Option<Calendar>,
        #[yaserde(rename = "ChatHistoryHeader", prefix = "tns", default)]
        pub chat_history_header: Option<AutoResponse>,
        #[yaserde(rename = "WebOnHoldURLs", prefix = "tns", default)]
        pub web_on_hold_ur_ls: Option<OnHoldURL>,
        #[yaserde(rename = "ConcurrentWebChats", prefix = "tns", default)]
        pub concurrent_web_chats: Option<i64>,
        #[yaserde(rename = "ApprovalRatio", prefix = "tns", default)]
        pub approval_ratio: Option<i64>,
        #[yaserde(rename = "LastApprovalUpdate", prefix = "tns", default)]
        pub last_approval_update: Option<String>,
        #[yaserde(rename = "ApprovalSkillset1", prefix = "tns", default)]
        pub approval_skillset_1: Option<i64>,
        #[yaserde(rename = "ApprovalSkillset2", prefix = "tns", default)]
        pub approval_skillset_2: Option<i64>,
        #[yaserde(rename = "ApprovalSkillset3", prefix = "tns", default)]
        pub approval_skillset_3: Option<i64>,
        #[yaserde(rename = "ApprovalSkillset4", prefix = "tns", default)]
        pub approval_skillset_4: Option<i64>,
        #[yaserde(rename = "ApprovalSkillset5", prefix = "tns", default)]
        pub approval_skillset_5: Option<i64>,
        #[yaserde(rename = "RejectionLevel1", prefix = "tns", default)]
        pub rejection_level_1: Option<i64>,
        #[yaserde(rename = "RejectionLevel2", prefix = "tns", default)]
        pub rejection_level_2: Option<i64>,
        #[yaserde(rename = "RejectionLevel3", prefix = "tns", default)]
        pub rejection_level_3: Option<i64>,
        #[yaserde(rename = "RejectionLevel4", prefix = "tns", default)]
        pub rejection_level_4: Option<i64>,
        #[yaserde(rename = "RejectionLevel5", prefix = "tns", default)]
        pub rejection_level_5: Option<i64>,
        #[yaserde(rename = "RSMStatus", prefix = "tns", default)]
        pub rsm_status: Option<i64>,
        #[yaserde(rename = "AutoRejectKeywordGroupID", prefix = "tns", default)]
        pub auto_reject_keyword_group_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Inbox",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Inbox {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "InboundMailThreshold", prefix = "tns", default)]
        pub inbound_mail_threshold: Option<i64>,
        #[yaserde(rename = "DisplayName", prefix = "tns", default)]
        pub display_name: Option<String>,
        #[yaserde(rename = "DomainName", prefix = "tns", default)]
        pub domain_name: Option<String>,
        #[yaserde(rename = "Enabled", prefix = "tns", default)]
        pub enabled: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "OutBoundMailThreshold", prefix = "tns", default)]
        pub out_bound_mail_threshold: Option<i64>,
        #[yaserde(rename = "Server", prefix = "tns", default)]
        pub server: Option<Server>,
        #[yaserde(rename = "SmtpAuthLogon", prefix = "tns", default)]
        pub smtp_auth_logon: Option<String>,
        #[yaserde(rename = "SMTPServer", prefix = "tns", default)]
        pub smtp_server: Option<Server>,
        #[yaserde(rename = "WinNTAccount", prefix = "tns", default)]
        pub win_nt_account: Option<String>,
        #[yaserde(rename = "StripLeadingDigits", prefix = "tns", default)]
        pub strip_leading_digits: Option<i64>,
        #[yaserde(rename = "StripTrailingCharacters", prefix = "tns", default)]
        pub strip_trailing_characters: Option<bool>,
        #[yaserde(rename = "RuleGroupID", prefix = "tns", default)]
        pub rule_group_id: Option<i64>,
        #[yaserde(rename = "ContactType", prefix = "tns", default)]
        pub contact_type: Option<ContactType>,
        #[yaserde(rename = "LastAccessedUtcTime", prefix = "tns", default)]
        pub last_accessed_utc_time: Option<String>,
        #[yaserde(rename = "LastAccessedError", prefix = "tns", default)]
        pub last_accessed_error: Option<String>,
        #[yaserde(rename = "LastAccessedCount", prefix = "tns", default)]
        pub last_accessed_count: Option<i64>,
        #[yaserde(rename = "LastAccessed", prefix = "tns", default)]
        pub last_accessed: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Server",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Server {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: i64,
        #[yaserde(rename = "Auth", prefix = "tns", default)]
        pub auth: Option<i64>,
        #[yaserde(rename = "BackUpHost", prefix = "tns", default)]
        pub back_up_host: Option<String>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Port", prefix = "tns", default)]
        pub port: Option<i64>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "NewType", prefix = "tns", default)]
        pub new_type: Option<CodeMapping>,
        #[yaserde(rename = "Protocol", prefix = "tns", default)]
        pub protocol: Option<i64>,
        #[yaserde(rename = "Encryption", prefix = "tns", default)]
        pub encryption: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ContactType",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ContactType {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "NumericValue", prefix = "tns", default)]
        pub numeric_value: Option<i64>,
        #[yaserde(rename = "TextValue", prefix = "tns", default)]
        pub text_value: Option<String>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "DefaultSkillset", prefix = "tns", default)]
        pub default_skillset: Option<i64>,
        #[yaserde(rename = "Parent", prefix = "tns", default)]
        pub parent: Option<multiref::MultiRef<ContactType>>,
        #[yaserde(rename = "DefaultClosedReasonID", prefix = "tns", default)]
        pub default_closed_reason_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WebCommsComfortGroup",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct WebCommsComfortGroup {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "GroupName", prefix = "tns", default)]
        pub group_name: Option<String>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "WebCommsComfortMessages", prefix = "tns", default)]
        pub web_comms_comfort_messages: Option<ArrayOfWebCommsComfortMessage>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfWebCommsComfortMessage",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfWebCommsComfortMessage {
        #[yaserde(rename = "WebCommsComfortMessage", prefix = "tns", default)]
        pub web_comms_comfort_message: Vec<WebCommsComfortMessage>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WebCommsComfortMessage",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct WebCommsComfortMessage {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Message", prefix = "tns", default)]
        pub message: Option<String>,
        #[yaserde(rename = "Delay", prefix = "tns", default)]
        pub delay: Option<i64>,
        #[yaserde(rename = "Sequence", prefix = "tns", default)]
        pub sequence: Option<i64>,
        #[yaserde(rename = "WebCommsComfortGroupID", prefix = "tns", default)]
        pub web_comms_comfort_group_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfUserPairOfAgentsKeyUser",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfUserPairOfAgentsKeyUser {
        #[yaserde(rename = "User", prefix = "tns", default)]
        pub user: Vec<PairOfAgentsKeyUser>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PairOfAgentsKeyUser",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct PairOfAgentsKeyUser {
        #[yaserde(flatten, default)]
        pub user: User,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPagePushURL",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfPagePushURL {
        #[yaserde(rename = "PagePushURL", prefix = "tns", default)]
        pub page_push_url: Vec<PagePushURL>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PagePushURL",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct PagePushURL {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPhrase",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfPhrase {
        #[yaserde(rename = "Phrase", prefix = "tns", default)]
        pub phrase: Vec<Phrase>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Phrase",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Phrase {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Text", prefix = "tns", default)]
        pub text: Option<String>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfExpert",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfExpert {
        #[yaserde(rename = "Expert", prefix = "tns", default)]
        pub expert: Vec<Expert>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Expert",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Expert {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Custom1", prefix = "tns", default)]
        pub custom_1: Option<i64>,
        #[yaserde(rename = "Custom2", prefix = "tns", default)]
        pub custom_2: Option<String>,
        #[yaserde(rename = "ExpertName", prefix = "tns", default)]
        pub expert_name: String,
        #[yaserde(rename = "ExpertSipUri", prefix = "tns", default)]
        pub expert_sip_uri: String,
        #[yaserde(rename = "Group", prefix = "tns", default)]
        pub group: Option<String>,
        #[yaserde(rename = "Phone", prefix = "tns", default)]
        pub phone: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SkillsetThresholdTemplate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SkillsetThresholdTemplate {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "CCMSID", prefix = "tns", default)]
        pub ccmsid: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "ThresholdToMonitor", prefix = "tns", default)]
        pub threshold_to_monitor: Option<CodeMapping>,
        #[yaserde(rename = "AgentsToAssignVoice", prefix = "tns", default)]
        pub agents_to_assign_voice: Option<i64>,
        #[yaserde(rename = "AgentsToAssignPredictive", prefix = "tns", default)]
        pub agents_to_assign_predictive: Option<i64>,
        #[yaserde(rename = "ThresholdLevels", prefix = "tns", default)]
        pub threshold_levels: Option<ArrayOfThresholdLevel>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfThresholdLevel",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfThresholdLevel {
        #[yaserde(rename = "ThresholdLevel", prefix = "tns", default)]
        pub threshold_level: Vec<ThresholdLevel>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ThresholdLevel",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ThresholdLevel {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "BlendingThreshold", prefix = "tns", default)]
        pub blending_threshold: Option<CodeMapping>,
        #[yaserde(rename = "Level1", prefix = "tns", default)]
        pub level_1: Option<i64>,
        #[yaserde(rename = "Level2", prefix = "tns", default)]
        pub level_2: Option<i64>,
        #[yaserde(rename = "Enabled", prefix = "tns", default)]
        pub enabled: Option<bool>,
        #[yaserde(rename = "ThresholdTemplateID", prefix = "tns", default)]
        pub threshold_template_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Calendar",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Calendar {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Breaks", prefix = "tns", default)]
        pub breaks: Option<ArrayOfBreak>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfBreak",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfBreak {
        #[yaserde(rename = "Break", prefix = "tns", default)]
        pub r#break: Vec<Break>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Break",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Break {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "DayOfWeek", prefix = "tns", default)]
        pub day_of_week: Option<i64>,
        #[yaserde(rename = "HolidayDate", prefix = "tns", default)]
        pub holiday_date: Option<String>,
        #[yaserde(rename = "HolidayName", prefix = "tns", default)]
        pub holiday_name: Option<String>,
        #[yaserde(rename = "StartTime", prefix = "tns", default)]
        pub start_time: Option<i64>,
        #[yaserde(rename = "EndTime", prefix = "tns", default)]
        pub end_time: Option<i64>,
        #[yaserde(rename = "CalendarID", prefix = "tns", default)]
        pub calendar_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutoResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct AutoResponse {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Subject", prefix = "tns", default)]
        pub subject: Option<String>,
        #[yaserde(rename = "Body", prefix = "tns", default)]
        pub body: Option<String>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "Suggestion", prefix = "tns", default)]
        pub suggestion: Option<bool>,
        #[yaserde(rename = "Attachments", prefix = "tns", default)]
        pub attachments: Option<ArrayOfAutoResponseAttachment>,
        #[yaserde(rename = "Categories", prefix = "tns", default)]
        pub categories: Option<ArrayOfCategory>,
        #[yaserde(rename = "Frequency", prefix = "tns", default)]
        pub frequency: Option<i64>,
        #[yaserde(rename = "BodyHTML", prefix = "tns", default)]
        pub body_html: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAutoResponseAttachment",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAutoResponseAttachment {
        #[yaserde(rename = "AutoResponseAttachment", prefix = "tns", default)]
        pub auto_response_attachment: Vec<AutoResponseAttachment>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "AutoResponseAttachment",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct AutoResponseAttachment {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "DisplayFileName", prefix = "tns", default)]
        pub display_file_name: Option<String>,
        #[yaserde(rename = "InternalFileName", prefix = "tns", default)]
        pub internal_file_name: Option<String>,
        #[yaserde(rename = "Inline", prefix = "tns", default)]
        pub inline: Option<bool>,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCategory",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCategory {
        #[yaserde(rename = "Category", prefix = "tns", default)]
        pub category: Vec<Category>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Category",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Category {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OnHoldURL",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct OnHoldURL {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: Option<String>,
        #[yaserde(rename = "HoldTime", prefix = "tns", default)]
        pub hold_time: Option<i64>,
        #[yaserde(rename = "Tag", prefix = "tns", default)]
        pub tag: Option<String>,
        #[yaserde(rename = "Sequence", prefix = "tns", default)]
        pub sequence: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Rule",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Rule {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "AutoResponseID", prefix = "tns", default)]
        pub auto_response_id: Option<i64>,
        #[yaserde(rename = "SkillsetID", prefix = "tns", default)]
        pub skillset_id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<i64>,
        #[yaserde(rename = "Sequence", prefix = "tns", default)]
        pub sequence: Option<i64>,
        #[yaserde(rename = "IsClosed", prefix = "tns", default)]
        pub is_closed: Option<bool>,
        #[yaserde(rename = "Priority", prefix = "tns", default)]
        pub priority: Option<i64>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<i64>,
        #[yaserde(rename = "UseOutOfHoursRule", prefix = "tns", default)]
        pub use_out_of_hours_rule: Option<bool>,
        #[yaserde(rename = "SkillsetReset", prefix = "tns", default)]
        pub skillset_reset: Option<bool>,
        #[yaserde(rename = "LastModifiedTime", prefix = "tns", default)]
        pub last_modified_time: Option<String>,
        #[yaserde(rename = "RuleKeywordCriteria", prefix = "tns", default)]
        pub rule_keyword_criteria: Option<ArrayOfRuleKeywordCriterion>,
        #[yaserde(rename = "AutoSuggests", prefix = "tns", default)]
        pub auto_suggests: Option<ArrayOfAutoResponse>,
        #[yaserde(rename = "RuleGroupID", prefix = "tns", default)]
        pub rule_group_id: Option<i64>,
        #[yaserde(rename = "OpenInterfaceMethod", prefix = "tns", default)]
        pub open_interface_method: Option<OpenInterfaceMethod>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfRuleKeywordCriterion",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfRuleKeywordCriterion {
        #[yaserde(rename = "RuleKeywordCriterion", prefix = "tns", default)]
        pub rule_keyword_criterion: Vec<RuleKeywordCriterion>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RuleKeywordCriterion",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct RuleKeywordCriterion {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "MatchTypeID", prefix = "tns", default)]
        pub match_type_id: Option<i64>,
        #[yaserde(rename = "Weighting", prefix = "tns", default)]
        pub weighting: Option<f64>,
        #[yaserde(rename = "RuleKeywordGroups", prefix = "tns", default)]
        pub rule_keyword_groups: Option<ArrayOfRuleKeywordGroup>,
        #[yaserde(rename = "EmailAddressGroupID", prefix = "tns", default)]
        pub email_address_group_id: Option<i64>,
        #[yaserde(rename = "LastModifiedTime", prefix = "tns", default)]
        pub last_modified_time: Option<String>,
        #[yaserde(rename = "RuleQuery", prefix = "tns", default)]
        pub rule_query: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfRuleKeywordGroup",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfRuleKeywordGroup {
        #[yaserde(rename = "RuleKeywordGroup", prefix = "tns", default)]
        pub rule_keyword_group: Vec<RuleKeywordGroup>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RuleKeywordGroup",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct RuleKeywordGroup {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "KeyWordGroupID", prefix = "tns", default)]
        pub key_word_group_id: Option<i64>,
        #[yaserde(rename = "Operator", prefix = "tns", default)]
        pub operator: Option<String>,
        #[yaserde(rename = "Sequence", prefix = "tns", default)]
        pub sequence: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAutoResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAutoResponse {
        #[yaserde(rename = "AutoResponse", prefix = "tns", default)]
        pub auto_response: Vec<AutoResponse>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "OpenInterfaceMethod",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct OpenInterfaceMethod {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "MethodName", prefix = "tns", default)]
        pub method_name: Option<String>,
        #[yaserde(rename = "DisplayName", prefix = "tns", default)]
        pub display_name: Option<String>,
        #[yaserde(rename = "InputParameterName", prefix = "tns", default)]
        pub input_parameter_name: Option<String>,
        #[yaserde(rename = "InputParameterArrayItem", prefix = "tns", default)]
        pub input_parameter_array_item: Option<String>,
        #[yaserde(rename = "InputParameterNamespace", prefix = "tns", default)]
        pub input_parameter_namespace: Option<String>,
        #[yaserde(rename = "OuputParameterName", prefix = "tns", default)]
        pub ouput_parameter_name: Option<String>,
        #[yaserde(rename = "InputFieldMap", prefix = "tns", default)]
        pub input_field_map: Option<ArrayOfInputFieldMapItemLong>,
        #[yaserde(rename = "OutputFieldMap", prefix = "tns", default)]
        pub output_field_map: Option<ArrayOfOutputFieldMapItemLong>,
        #[yaserde(rename = "InputFields", prefix = "tns", default)]
        pub input_fields: Option<ListOfCodeMappings>,
        #[yaserde(rename = "OutputFields", prefix = "tns", default)]
        pub output_fields: Option<ListOfCodeMappings>,
        #[yaserde(rename = "OpenInterfaceServiceID", prefix = "tns", default)]
        pub open_interface_service_id: Option<i64>,
        #[yaserde(rename = "LastModifiedUtcTime", prefix = "tns", default)]
        pub last_modified_utc_time: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfInputFieldMapItemLong",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfInputFieldMapItemLong {
        #[yaserde(rename = "InputFieldMapItem", prefix = "tns", default)]
        pub input_field_map_item: Vec<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfOutputFieldMapItemLong",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfOutputFieldMapItemLong {
        #[yaserde(rename = "OutputFieldMapItem", prefix = "tns", default)]
        pub output_field_map_item: Vec<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListOfCodeMappings",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ListOfCodeMappings {
        #[yaserde(rename = "CodeMappings", prefix = "tns", default)]
        pub code_mappings: Option<ArrayOfCodeMapping>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCodeMapping",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCodeMapping {
        #[yaserde(rename = "CodeMapping", prefix = "tns", default)]
        pub code_mapping: Vec<CodeMapping>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Campaign",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Campaign {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "NumberOfContacts", prefix = "tns", default)]
        pub number_of_contacts: Option<i64>,
        #[yaserde(rename = "NumberOfContactsProcessed", prefix = "tns", default)]
        pub number_of_contacts_processed: Option<i64>,
        #[yaserde(rename = "CampaignScript", prefix = "tns", default)]
        pub campaign_script: Option<CampaignScript>,
        #[yaserde(rename = "DateCreated", prefix = "tns", default)]
        pub date_created: Option<String>,
        #[yaserde(rename = "Owner", prefix = "tns", default)]
        pub owner: Option<String>,
        #[yaserde(rename = "StartDateTime", prefix = "tns", default)]
        pub start_date_time: Option<String>,
        #[yaserde(rename = "EndDateTime", prefix = "tns", default)]
        pub end_date_time: Option<String>,
        #[yaserde(rename = "DailyStartTime", prefix = "tns", default)]
        pub daily_start_time: Option<String>,
        #[yaserde(rename = "DailyEndTime", prefix = "tns", default)]
        pub daily_end_time: Option<String>,
        #[yaserde(rename = "Skillset", prefix = "tns", default)]
        pub skillset: Option<Skillset>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<CodeMapping>,
        #[yaserde(rename = "PreviousStatus", prefix = "tns", default)]
        pub previous_status: Option<CodeMapping>,
        #[yaserde(rename = "StartOfRun", prefix = "tns", default)]
        pub start_of_run: Option<bool>,
        #[yaserde(rename = "Priority", prefix = "tns", default)]
        pub priority: Option<i64>,
        #[yaserde(rename = "LastLoadedDate", prefix = "tns", default)]
        pub last_loaded_date: Option<String>,
        #[yaserde(rename = "LoadInterval", prefix = "tns", default)]
        pub load_interval: Option<i64>,
        #[yaserde(rename = "LoadContactsPerInterval", prefix = "tns", default)]
        pub load_contacts_per_interval: Option<i64>,
        #[yaserde(rename = "DialingPrefix", prefix = "tns", default)]
        pub dialing_prefix: Option<String>,
        #[yaserde(rename = "AutoDialTimeOut", prefix = "tns", default)]
        pub auto_dial_time_out: Option<i64>,
        #[yaserde(rename = "MinimumRingTime", prefix = "tns", default)]
        pub minimum_ring_time: Option<i64>,
        #[yaserde(rename = "Locked", prefix = "tns", default)]
        pub locked: Option<bool>,
        #[yaserde(rename = "LockedBy", prefix = "tns", default)]
        pub locked_by: Option<String>,
        #[yaserde(rename = "LockedDateTime", prefix = "tns", default)]
        pub locked_date_time: Option<String>,
        #[yaserde(rename = "DispositionCodes", prefix = "tns", default)]
        pub disposition_codes: Option<ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode>,
        #[yaserde(rename = "CustomFields", prefix = "tns", default)]
        pub custom_fields: Option<ArrayOfCustomField>,
        #[yaserde(rename = "UseTimeZone", prefix = "tns", default)]
        pub use_time_zone: Option<i64>,
        #[yaserde(rename = "NumberOfContactsClosed", prefix = "tns", default)]
        pub number_of_contacts_closed: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CampaignScript",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CampaignScript {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "DateCreated", prefix = "tns", default)]
        pub date_created: Option<String>,
        #[yaserde(rename = "Introduction", prefix = "tns", default)]
        pub introduction: Option<String>,
        #[yaserde(rename = "Questions", prefix = "tns", default)]
        pub questions: Option<ArrayOfQuestion>,
        #[yaserde(rename = "Conclusion", prefix = "tns", default)]
        pub conclusion: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfQuestion",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfQuestion {
        #[yaserde(rename = "Question", prefix = "tns", default)]
        pub question: Vec<Question>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Question",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Question {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Question", prefix = "tns", default)]
        pub question: Option<String>,
        #[yaserde(rename = "AllowedAnswers", prefix = "tns", default)]
        pub allowed_answers: Option<String>,
        #[yaserde(rename = "AllowedFreeText", prefix = "tns", default)]
        pub allowed_free_text: Option<bool>,
        #[yaserde(rename = "DefaultAnswer", prefix = "tns", default)]
        pub default_answer: Option<String>,
        #[yaserde(rename = "GUI", prefix = "tns", default)]
        pub gui: Option<CodeMapping>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfDispositionCodePairOfDispositionCodesKeyDispositionCode {
        #[yaserde(rename = "DispositionCode", prefix = "tns", default)]
        pub disposition_code: Vec<PairOfDispositionCodesKeyDispositionCode>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "PairOfDispositionCodesKeyDispositionCode",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct PairOfDispositionCodesKeyDispositionCode {
        #[yaserde(flatten, default)]
        pub disposition_code: DispositionCode,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DispositionCode",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct DispositionCode {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "DisplayName", prefix = "tns", default)]
        pub display_name: Option<String>,
        #[yaserde(rename = "RetryTimeout", prefix = "tns", default)]
        pub retry_timeout: Option<i64>,
        #[yaserde(rename = "NumericValue", prefix = "tns", default)]
        pub numeric_value: Option<i64>,
        #[yaserde(rename = "MaxRetryCount", prefix = "tns", default)]
        pub max_retry_count: Option<i64>,
        #[yaserde(rename = "Deletable", prefix = "tns", default)]
        pub deletable: Option<bool>,
        #[yaserde(rename = "CallRequired", prefix = "tns", default)]
        pub call_required: Option<bool>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "SaveAgentScript", prefix = "tns", default)]
        pub save_agent_script: Option<bool>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<CodeMapping>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCustomField",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCustomField {
        #[yaserde(rename = "CustomField", prefix = "tns", default)]
        pub custom_field: Vec<CustomField>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomField",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomField {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "IsTextVisible", prefix = "tns", default)]
        pub is_text_visible: Option<bool>,
        #[yaserde(rename = "Text", prefix = "tns", default)]
        pub text: Option<String>,
        #[yaserde(rename = "CustomFieldTemplate", prefix = "tns", default)]
        pub custom_field_template: Option<CustomFieldTemplate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomFieldTemplate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomFieldTemplate {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAction",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAction {
        #[yaserde(rename = "Action", prefix = "tns", default)]
        pub action: Vec<Action>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Action",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Action {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "ContactId", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "CreationTime", prefix = "tns", default)]
        pub creation_time: Option<String>,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<ContactType>,
        #[yaserde(rename = "Source", prefix = "tns", default)]
        pub source: Option<CodeMapping>,
        #[yaserde(rename = "Subject", prefix = "tns", default)]
        pub subject: Option<String>,
        #[yaserde(rename = "Text", prefix = "tns", default)]
        pub text: Option<String>,
        #[yaserde(rename = "TextHTML", prefix = "tns", default)]
        pub text_html: Option<String>,
        #[yaserde(rename = "CharSet", prefix = "tns", default)]
        pub char_set: Option<String>,
        #[yaserde(rename = "Attempt", prefix = "tns", default)]
        pub attempt: Option<i64>,
        #[yaserde(rename = "DispositionCode", prefix = "tns", default)]
        pub disposition_code: Option<DispositionCode>,
        #[yaserde(rename = "MailTo", prefix = "tns", default)]
        pub mail_to: Option<String>,
        #[yaserde(rename = "MailFrom", prefix = "tns", default)]
        pub mail_from: Option<String>,
        #[yaserde(rename = "MailCC", prefix = "tns", default)]
        pub mail_cc: Option<String>,
        #[yaserde(rename = "MailBCC", prefix = "tns", default)]
        pub mail_bcc: Option<String>,
        #[yaserde(rename = "CallBackTime", prefix = "tns", default)]
        pub call_back_time: Option<String>,
        #[yaserde(rename = "CallBackMedia", prefix = "tns", default)]
        pub call_back_media: Option<CodeMapping>,
        #[yaserde(rename = "CallBackStatus", prefix = "tns", default)]
        pub call_back_status: Option<CodeMapping>,
        #[yaserde(rename = "Agent", prefix = "tns", default)]
        pub agent: Option<User>,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "TimeAllocated", prefix = "tns", default)]
        pub time_allocated: Option<i64>,
        #[yaserde(rename = "TemplateLocation", prefix = "tns", default)]
        pub template_location: Option<String>,
        #[yaserde(rename = "HistoryFlag", prefix = "tns", default)]
        pub history_flag: Option<bool>,
        #[yaserde(rename = "NumberUsed", prefix = "tns", default)]
        pub number_used: Option<String>,
        #[yaserde(rename = "CallStartTime", prefix = "tns", default)]
        pub call_start_time: Option<String>,
        #[yaserde(rename = "CallEndTime", prefix = "tns", default)]
        pub call_end_time: Option<String>,
        #[yaserde(rename = "DialStartTime", prefix = "tns", default)]
        pub dial_start_time: Option<String>,
        #[yaserde(rename = "DialEndTime", prefix = "tns", default)]
        pub dial_end_time: Option<String>,
        #[yaserde(rename = "OpenTime", prefix = "tns", default)]
        pub open_time: Option<String>,
        #[yaserde(rename = "ClosedTime", prefix = "tns", default)]
        pub closed_time: Option<String>,
        #[yaserde(rename = "Attachments", prefix = "tns", default)]
        pub attachments: Option<ArrayOfAttachment>,
        #[yaserde(rename = "CustomFields", prefix = "tns", default)]
        pub custom_fields: Option<ArrayOfCustomField>,
        #[yaserde(rename = "ClosedReason", prefix = "tns", default)]
        pub closed_reason: Option<ClosedReason>,
        #[yaserde(rename = "AutoSuggest", prefix = "tns", default)]
        pub auto_suggest: Option<AutoResponse>,
        #[yaserde(rename = "ApprovalAudits", prefix = "tns", default)]
        pub approval_audits: Option<ArrayOfApprovalAudit>,
        #[yaserde(rename = "ApprovalLevel", prefix = "tns", default)]
        pub approval_level: Option<i64>,
        #[yaserde(rename = "ComfortMessages", prefix = "tns", default)]
        pub comfort_messages: Option<i64>,
        #[yaserde(rename = "RestCall", prefix = "tns", default)]
        pub rest_call: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfAttachment",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfAttachment {
        #[yaserde(rename = "Attachment", prefix = "tns", default)]
        pub attachment: Vec<Attachment>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Attachment",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct Attachment {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "DisplayFileName", prefix = "tns", default)]
        pub display_file_name: Option<String>,
        #[yaserde(rename = "InternalFileName", prefix = "tns", default)]
        pub internal_file_name: Option<String>,
        #[yaserde(rename = "Direction", prefix = "tns", default)]
        pub direction: Option<CodeMapping>,
        #[yaserde(rename = "URL", prefix = "tns", default)]
        pub url: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ClosedReason",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ClosedReason {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: String,
        #[yaserde(rename = "Type", prefix = "tns", default)]
        pub rs_type: Option<ContactType>,
        #[yaserde(rename = "DeletionTimeStamp", prefix = "tns", default)]
        pub deletion_time_stamp: Option<String>,
        #[yaserde(rename = "MarkAsDeleted", prefix = "tns", default)]
        pub mark_as_deleted: Option<bool>,
        #[yaserde(rename = "OldCodeMappingID", prefix = "tns", default)]
        pub old_code_mapping_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfApprovalAudit",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfApprovalAudit {
        #[yaserde(rename = "ApprovalAudit", prefix = "tns", default)]
        pub approval_audit: Vec<ApprovalAudit>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ApprovalAudit",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ApprovalAudit {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "UserId", prefix = "tns", default)]
        pub user_id: Option<i64>,
        #[yaserde(rename = "SkillsetId", prefix = "tns", default)]
        pub skillset_id: Option<i64>,
        #[yaserde(rename = "Level", prefix = "tns", default)]
        pub level: Option<i64>,
        #[yaserde(rename = "Status", prefix = "tns", default)]
        pub status: Option<i64>,
        #[yaserde(rename = "UpdatedTimestamp", prefix = "tns", default)]
        pub updated_timestamp: Option<String>,
        #[yaserde(rename = "CreatedTimestamp", prefix = "tns", default)]
        pub created_timestamp: Option<String>,
        #[yaserde(rename = "Comment", prefix = "tns", default)]
        pub comment: Option<String>,
        #[yaserde(rename = "ActionId", prefix = "tns", default)]
        pub action_id: Option<i64>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfSipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfSipUri {
        #[yaserde(rename = "SipUri", prefix = "tns", default)]
        pub sip_uri: Vec<SipUri>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CreateCustomer {
        #[yaserde(rename = "customerParams", prefix = "tns", default)]
        pub customer_params: Option<CustomerCreate>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CampaignCustomerCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CampaignCustomerCreate {
        #[yaserde(flatten, default)]
        pub customer_create: CustomerCreate,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "BlockIndex", prefix = "tns", default)]
        pub block_index: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomerCreate {
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LoginPage", prefix = "tns", default)]
        pub login_page: Option<i64>,
        #[yaserde(rename = "PreferredAgent", prefix = "tns", default)]
        pub preferred_agent: Option<i64>,
        #[yaserde(rename = "Password", prefix = "tns", default)]
        pub password: Option<String>,
        #[yaserde(rename = "RegisterDate", prefix = "tns", default)]
        pub register_date: Option<String>,
        #[yaserde(rename = "UserName", prefix = "tns", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "Addresses", prefix = "tns", default)]
        pub addresses: Option<ArrayOfAddress>,
        #[yaserde(rename = "EmailAddresses", prefix = "tns", default)]
        pub email_addresses: Option<ArrayOfEmailAddress>,
        #[yaserde(rename = "SipUris", prefix = "tns", default)]
        pub sip_uris: Option<ArrayOfSipUri>,
        #[yaserde(rename = "PhoneNumberCreateList", prefix = "tns", default)]
        pub phone_number_create_list: Option<ArrayOfPhoneNumberCreate>,
        #[yaserde(rename = "ContactCreateList", prefix = "tns", default)]
        pub contact_create_list: Option<ArrayOfContactCreate>,
        #[yaserde(rename = "CustomFieldCreateList", prefix = "tns", default)]
        pub custom_field_create_list: Option<ArrayOfCustomFieldCreate>,
        #[yaserde(rename = "Objection", prefix = "tns", default)]
        pub objection: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfPhoneNumberCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfPhoneNumberCreate {
        #[yaserde(rename = "PhoneNumberCreate", prefix = "tns", default)]
        pub phone_number_create: Vec<PhoneNumberCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfContactCreate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfContactCreate {
        #[yaserde(rename = "ContactCreate", prefix = "tns", default)]
        pub contact_create: Vec<ContactCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CampaignCustomerDetails",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CampaignCustomerDetails {
        #[yaserde(flatten, default)]
        pub customer_create: CustomerCreate,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "BlockIndex", prefix = "tns", default)]
        pub block_index: Option<i64>,
        #[yaserde(rename = "MatchType", prefix = "tns", default)]
        pub match_type: Option<i64>,
        #[yaserde(rename = "CustomerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "ImportDataOverrides", prefix = "tns", default)]
        pub import_data_overrides: Option<bool>,
        #[yaserde(rename = "SkillsetID", prefix = "tns", default)]
        pub skillset_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CreateCustomerResponse {
        #[yaserde(rename = "CreateCustomerResult", prefix = "tns", default)]
        pub create_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomerBySipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CreateCustomerBySipUri {
        #[yaserde(rename = "sipUri", prefix = "tns", default)]
        pub sip_uri: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CreateCustomerBySipUriResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CreateCustomerBySipUriResponse {
        #[yaserde(rename = "CreateCustomerBySipUriResult", prefix = "tns", default)]
        pub create_customer_by_sip_uri_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerAssociation",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CustomerAssociation {
        #[yaserde(rename = "listOfCallDetails", prefix = "tns", default)]
        pub list_of_call_details: Option<ListOfCallDetails>,
        #[yaserde(rename = "partialMatch", prefix = "tns", default)]
        pub partial_match: Option<bool>,
        #[yaserde(rename = "maxCustomerMatches", prefix = "tns", default)]
        pub max_customer_matches: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListOfCallDetails",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ListOfCallDetails {
        #[yaserde(rename = "CallDetails", prefix = "tns", default)]
        pub call_details: Option<ArrayOfCallDetails>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCallDetails",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCallDetails {
        #[yaserde(rename = "CallDetails", prefix = "tns", default)]
        pub call_details: Vec<CallDetails>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CallDetails",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CallDetails {
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "InternationalCode", prefix = "tns", default)]
        pub international_code: Option<String>,
        #[yaserde(rename = "AreaCode", prefix = "tns", default)]
        pub area_code: Option<String>,
        #[yaserde(rename = "Number", prefix = "tns", default)]
        pub number: Option<String>,
        #[yaserde(rename = "Email", prefix = "tns", default)]
        pub email: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerAssociationResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CustomerAssociationResponse {
        #[yaserde(rename = "CustomerAssociationResult", prefix = "tns", default)]
        pub customer_association_result: ListOfCustomerAssociationResults,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListOfCustomerAssociationResults",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ListOfCustomerAssociationResults {
        #[yaserde(rename = "CustomerAssociationResults", prefix = "tns", default)]
        pub customer_association_results: Option<ArrayOfCustomerAssociationResult>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCustomerAssociationResult",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCustomerAssociationResult {
        #[yaserde(rename = "CustomerAssociationResult", prefix = "tns", default)]
        pub customer_association_result: Vec<CustomerAssociationResult>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerAssociationResult",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomerAssociationResult {
        #[yaserde(rename = "CallDetailIndex", prefix = "tns", default)]
        pub call_detail_index: Option<String>,
        #[yaserde(rename = "MatchType", prefix = "tns", default)]
        pub match_type: Option<String>,
        #[yaserde(rename = "ExistingCustID", prefix = "tns", default)]
        pub existing_cust_id: Option<ArrayOfExistingCustIDItemString>,
        #[yaserde(rename = "DNCStatus", prefix = "tns", default)]
        pub dnc_status: Option<ArrayOfDNCStatusItemBoolean>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfExistingCustIDItemString",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfExistingCustIDItemString {
        #[yaserde(rename = "ExistingCustIDItem", prefix = "tns", default)]
        pub existing_cust_id_item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfDNCStatusItemBoolean",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfDNCStatusItemBoolean {
        #[yaserde(rename = "DNCStatusItem", prefix = "tns", default)]
        pub dnc_status_item: Vec<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerSearch",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CustomerSearch {
        #[yaserde(rename = "searchParams", prefix = "tns", default)]
        pub search_params: Option<SearchObject>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SearchObject",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SearchObject {
        #[yaserde(rename = "SearchConditions", prefix = "tns", default)]
        pub search_conditions: Option<ArrayOfSearchCondition>,
        #[yaserde(rename = "DateField", prefix = "tns", default)]
        pub date_field: Option<String>,
        #[yaserde(rename = "StartDateRange", prefix = "tns", default)]
        pub start_date_range: Option<String>,
        #[yaserde(rename = "EndDateRange", prefix = "tns", default)]
        pub end_date_range: Option<String>,
        #[yaserde(rename = "OrderField", prefix = "tns", default)]
        pub order_field: Option<String>,
        #[yaserde(rename = "Order", prefix = "tns", default)]
        pub order: Option<String>,
        #[yaserde(rename = "MaxNoObjects", prefix = "tns", default)]
        pub max_no_objects: Option<i64>,
        #[yaserde(rename = "MaxID", prefix = "tns", default)]
        pub max_id: Option<i64>,
        #[yaserde(rename = "StartID", prefix = "tns", default)]
        pub start_id: Option<i64>,
        #[yaserde(rename = "Direction", prefix = "tns", default)]
        pub direction: Option<i64>,
        #[yaserde(rename = "Timeout", prefix = "tns", default)]
        pub timeout: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfSearchCondition",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfSearchCondition {
        #[yaserde(rename = "SearchCondition", prefix = "tns", default)]
        pub search_condition: Vec<SearchCondition>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SearchCondition",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SearchCondition {
        #[yaserde(rename = "Name", prefix = "tns", default)]
        pub name: Option<String>,
        #[yaserde(rename = "Condition", prefix = "tns", default)]
        pub condition: Option<String>,
        #[yaserde(rename = "Value", prefix = "tns", default)]
        pub value: Option<String>,
        #[yaserde(rename = "NextConditionValue", prefix = "tns", default)]
        pub next_condition_value: Option<String>,
        #[yaserde(rename = "AllowWildcards", prefix = "tns", default)]
        pub allow_wildcards: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerSearchResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct CustomerSearchResponse {
        #[yaserde(rename = "CustomerSearchResult", prefix = "tns", default)]
        pub customer_search_result: CustomerSearchResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerSearchResult",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomerSearchResult {
        #[yaserde(rename = "Customers", prefix = "tns", default)]
        pub customers: Option<ArrayOfSearchCustomer>,
        #[yaserde(rename = "NoObjectsRemaining", prefix = "tns", default)]
        pub no_objects_remaining: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfSearchCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfSearchCustomer {
        #[yaserde(rename = "SearchCustomer", prefix = "tns", default)]
        pub search_customer: Vec<SearchCustomer>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SearchCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct SearchCustomer {
        #[yaserde(rename = "ID", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "PrefAgentFirstName", prefix = "tns", default)]
        pub pref_agent_first_name: Option<String>,
        #[yaserde(rename = "PrefAgentLastName", prefix = "tns", default)]
        pub pref_agent_last_name: Option<String>,
        #[yaserde(rename = "Address", prefix = "tns", default)]
        pub address: Option<String>,
        #[yaserde(rename = "PhoneNumber", prefix = "tns", default)]
        pub phone_number: Option<String>,
        #[yaserde(rename = "Email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "SipUri", prefix = "tns", default)]
        pub sip_uri: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeleteCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteCustomer {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "DeleteCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct DeleteCustomerResponse {
        #[yaserde(rename = "DeleteCustomerResult", prefix = "tns", default)]
        pub delete_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetAllCustomers",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetAllCustomers {
        #[yaserde(rename = "maxID", prefix = "tns", default)]
        pub max_id: Option<i64>,
        #[yaserde(rename = "maxNoObjects", prefix = "tns", default)]
        pub max_no_objects: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetAllCustomersResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetAllCustomersResponse {
        #[yaserde(rename = "GetAllCustomersResult", prefix = "tns", default)]
        pub get_all_customers_result: ListOfCustomers,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListOfCustomers",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ListOfCustomers {
        #[yaserde(rename = "Customers", prefix = "tns", default)]
        pub customers: Option<ArrayOfCustomer>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCustomer {
        #[yaserde(rename = "Customer", prefix = "tns", default)]
        pub customer: Vec<Customer>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustSQLColumns",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustSQLColumns {
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustSQLColumnsResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustSQLColumnsResponse {
        #[yaserde(rename = "GetCustSQLColumnsResult", prefix = "tns", default)]
        pub get_cust_sql_columns_result: ArrayOfGetCustSQLColumnsResultItemString,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfGetCustSQLColumnsResultItemString",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfGetCustSQLColumnsResultItemString {
        #[yaserde(rename = "GetCustSQLColumnsResultItem", prefix = "tns", default)]
        pub get_cust_sql_columns_result_item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomFieldTemplates",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomFieldTemplates {
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomFieldTemplatesResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomFieldTemplatesResponse {
        #[yaserde(rename = "GetCustomFieldTemplatesResult", prefix = "tns", default)]
        pub get_custom_field_templates_result: ListOfCustomFieldTemplates,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ListOfCustomFieldTemplates",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ListOfCustomFieldTemplates {
        #[yaserde(rename = "CustomFieldTemplates", prefix = "tns", default)]
        pub custom_field_templates: Option<ArrayOfCustomFieldTemplate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfCustomFieldTemplate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfCustomFieldTemplate {
        #[yaserde(rename = "CustomFieldTemplate", prefix = "tns", default)]
        pub custom_field_template: Vec<CustomFieldTemplate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByContactId",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByContactId {
        #[yaserde(rename = "contactID", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByContactIdResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByContactIdResponse {
        #[yaserde(rename = "GetCustomerByContactIdResult", prefix = "tns", default)]
        pub get_customer_by_contact_id_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByEmail",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByEmail {
        #[yaserde(rename = "email", prefix = "tns", default)]
        pub email: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByEmailResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByEmailResponse {
        #[yaserde(rename = "GetCustomerByEmailResult", prefix = "tns", default)]
        pub get_customer_by_email_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByName",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByName {
        #[yaserde(rename = "firstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "lastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "fieldToCompare", prefix = "tns", default)]
        pub field_to_compare: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByNameResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByNameResponse {
        #[yaserde(rename = "GetCustomerByNameResult", prefix = "tns", default)]
        pub get_customer_by_name_result: ArrayOfGetCustomerByNameResultItemString,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfGetCustomerByNameResultItemString",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfGetCustomerByNameResultItemString {
        #[yaserde(rename = "GetCustomerByNameResultItem", prefix = "tns", default)]
        pub get_customer_by_name_result_item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByPhoneNumber",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByPhoneNumber {
        #[yaserde(rename = "phoneNumber", prefix = "tns", default)]
        pub phone_number: Option<PhoneNumber>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByPhoneNumberResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByPhoneNumberResponse {
        #[yaserde(rename = "GetCustomerByPhoneNumberResult", prefix = "tns", default)]
        pub get_customer_by_phone_number_result: ArrayOfGetCustomerByPhoneNumberResultItemString,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfGetCustomerByPhoneNumberResultItemString",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfGetCustomerByPhoneNumberResultItemString {
        #[yaserde(rename = "GetCustomerByPhoneNumberResultItem", prefix = "tns", default)]
        pub get_customer_by_phone_number_result_item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerBySipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerBySipUri {
        #[yaserde(rename = "sipUri", prefix = "tns", default)]
        pub sip_uri: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerBySipUriResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerBySipUriResponse {
        #[yaserde(rename = "GetCustomerBySipUriResult", prefix = "tns", default)]
        pub get_customer_by_sip_uri_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByUserName",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByUserName {
        #[yaserde(rename = "userName", prefix = "tns", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCustomerByUserNameResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCustomerByUserNameResponse {
        #[yaserde(rename = "GetCustomerByUserNameResult", prefix = "tns", default)]
        pub get_customer_by_user_name_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetNoCustContactsByTime",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetNoCustContactsByTime {
        #[yaserde(rename = "customerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "startTime", prefix = "tns", default)]
        pub start_time: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetNoCustContactsByTimeResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetNoCustContactsByTimeResponse {
        #[yaserde(rename = "GetNoCustContactsByTimeResult", prefix = "tns", default)]
        pub get_no_cust_contacts_by_time_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetSearchableFields",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetSearchableFields {
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetSearchableFieldsResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetSearchableFieldsResponse {
        #[yaserde(rename = "GetSearchableFieldsResult", prefix = "tns", default)]
        pub get_searchable_fields_result: ArrayOfGetSearchableFieldsResultItemString,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfGetSearchableFieldsResultItemString",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct ArrayOfGetSearchableFieldsResultItemString {
        #[yaserde(rename = "GetSearchableFieldsResultItem", prefix = "tns", default)]
        pub get_searchable_fields_result_item: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ImpersonateCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ImpersonateCustomer {
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
        #[yaserde(rename = "customerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ImpersonateCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ImpersonateCustomerResponse {
        #[yaserde(rename = "ImpersonateCustomerResult", prefix = "tns", default)]
        pub impersonate_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReadCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ReadCustomer {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
        #[yaserde(rename = "lite", prefix = "tns", default)]
        pub lite: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReadCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ReadCustomerResponse {
        #[yaserde(rename = "ReadCustomerResult", prefix = "tns", default)]
        pub read_customer_result: Customer,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReadCustomerHistory",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ReadCustomerHistory {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "maxNoContacts", prefix = "tns", default)]
        pub max_no_contacts: Option<i64>,
        #[yaserde(rename = "startContactID", prefix = "tns", default)]
        pub start_contact_id: Option<i64>,
        #[yaserde(rename = "contactOrder", prefix = "tns", default)]
        pub contact_order: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
        #[yaserde(rename = "lite", prefix = "tns", default)]
        pub lite: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ReadCustomerHistoryResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct ReadCustomerHistoryResponse {
        #[yaserde(rename = "ReadCustomerHistoryResult", prefix = "tns", default)]
        pub read_customer_history_result: CustomerHistoryResult,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerHistoryResult",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomerHistoryResult {
        #[yaserde(rename = "Customer", prefix = "tns", default)]
        pub customer: Option<Customer>,
        #[yaserde(rename = "MinID", prefix = "tns", default)]
        pub min_id: Option<i64>,
        #[yaserde(rename = "MaxID", prefix = "tns", default)]
        pub max_id: Option<i64>,
        #[yaserde(rename = "MinNumberRemaining", prefix = "tns", default)]
        pub min_number_remaining: Option<i64>,
        #[yaserde(rename = "MaxNumberRemaining", prefix = "tns", default)]
        pub max_number_remaining: Option<i64>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RegisterAnonymousCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RegisterAnonymousCustomer {
        #[yaserde(rename = "customerParams", prefix = "tns", default)]
        pub customer_params: Option<CustomerCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RegisterAnonymousCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RegisterAnonymousCustomerResponse {
        #[yaserde(rename = "RegisterAnonymousCustomerResult", prefix = "tns", default)]
        pub register_anonymous_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RegisterCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RegisterCustomer {
        #[yaserde(rename = "customerParams", prefix = "tns", default)]
        pub customer_params: Option<CustomerCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RegisterCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RegisterCustomerResponse {
        #[yaserde(rename = "RegisterCustomerResult", prefix = "tns", default)]
        pub register_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveAddress {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "addressID", prefix = "tns", default)]
        pub address_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveAddressResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveAddressResponse {
        #[yaserde(rename = "RemoveAddressResult", prefix = "tns", default)]
        pub remove_address_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveContact",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveContact {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "contactID", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveContactResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveContactResponse {
        #[yaserde(rename = "RemoveContactResult", prefix = "tns", default)]
        pub remove_contact_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveCustomField",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveCustomField {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "customFieldID", prefix = "tns", default)]
        pub custom_field_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveCustomFieldResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveCustomFieldResponse {
        #[yaserde(rename = "RemoveCustomFieldResult", prefix = "tns", default)]
        pub remove_custom_field_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveEmailAddress",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveEmailAddress {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "emailAddressID", prefix = "tns", default)]
        pub email_address_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveEmailAddressResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveEmailAddressResponse {
        #[yaserde(rename = "RemoveEmailAddressResult", prefix = "tns", default)]
        pub remove_email_address_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemovePhoneNumber",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemovePhoneNumber {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "phoneNumberID", prefix = "tns", default)]
        pub phone_number_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemovePhoneNumberResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemovePhoneNumberResponse {
        #[yaserde(rename = "RemovePhoneNumberResult", prefix = "tns", default)]
        pub remove_phone_number_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveSipUri",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveSipUri {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "sipUriID", prefix = "tns", default)]
        pub sip_uri_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "RemoveSipUriResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct RemoveSipUriResponse {
        #[yaserde(rename = "RemoveSipUriResult", prefix = "tns", default)]
        pub remove_sip_uri_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SendADPasswordReminder",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SendADPasswordReminder {
        #[yaserde(rename = "contactID", prefix = "tns", default)]
        pub contact_id: Option<i64>,
        #[yaserde(rename = "customerID", prefix = "tns", default)]
        pub customer_id: Option<i64>,
        #[yaserde(rename = "skillsetID", prefix = "tns", default)]
        pub skillset_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SendADPasswordReminderResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SendADPasswordReminderResponse {
        #[yaserde(rename = "SendADPasswordReminderResult", prefix = "tns", default)]
        pub send_ad_password_reminder_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SendPasswordReminder",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SendPasswordReminder {
        #[yaserde(rename = "emailAddress", prefix = "tns", default)]
        pub email_address: Option<String>,
        #[yaserde(rename = "skillsetName", prefix = "tns", default)]
        pub skillset_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SendPasswordReminderResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SendPasswordReminderResponse {
        #[yaserde(rename = "SendPasswordReminderResult", prefix = "tns", default)]
        pub send_password_reminder_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetAgentID",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetAgentID {
        #[yaserde(rename = "agentID", prefix = "tns", default)]
        pub agent_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "SetAgentIDResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct SetAgentIDResponse {
        #[yaserde(rename = "SetAgentIDResult", prefix = "tns", default)]
        pub set_agent_id_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateCustomer",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateCustomer {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "customerParams", prefix = "tns", default)]
        pub customer_params: Option<CustomerUpdate>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "CustomerUpdate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        prefix = "tns"
    )]
    pub struct CustomerUpdate {
        #[yaserde(rename = "Title", prefix = "tns", default)]
        pub title: Option<String>,
        #[yaserde(rename = "FirstName", prefix = "tns", default)]
        pub first_name: Option<String>,
        #[yaserde(rename = "LastName", prefix = "tns", default)]
        pub last_name: Option<String>,
        #[yaserde(rename = "LoginPage", prefix = "tns", default)]
        pub login_page: Option<i64>,
        #[yaserde(rename = "PreferredAgent", prefix = "tns", default)]
        pub preferred_agent: Option<i64>,
        #[yaserde(rename = "RegisterDate", prefix = "tns", default)]
        pub register_date: Option<String>,
        #[yaserde(rename = "UserName", prefix = "tns", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "Addresses", prefix = "tns", default)]
        pub addresses: Option<ArrayOfAddress>,
        #[yaserde(rename = "EmailAddresses", prefix = "tns", default)]
        pub email_addresses: Option<ArrayOfEmailAddress>,
        #[yaserde(rename = "SipUris", prefix = "tns", default)]
        pub sip_uris: Option<ArrayOfSipUri>,
        #[yaserde(rename = "PhoneNumberCreateList", prefix = "tns", default)]
        pub phone_number_create_list: Option<ArrayOfPhoneNumberCreate>,
        #[yaserde(rename = "ContactCreateList", prefix = "tns", default)]
        pub contact_create_list: Option<ArrayOfContactCreate>,
        #[yaserde(rename = "CustomFieldCreateList", prefix = "tns", default)]
        pub custom_field_create_list: Option<ArrayOfCustomFieldCreate>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateCustomerResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateCustomerResponse {
        #[yaserde(rename = "UpdateCustomerResult", prefix = "tns", default)]
        pub update_customer_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateFirstName",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateFirstName {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateFirstNameResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateFirstNameResponse {
        #[yaserde(rename = "UpdateFirstNameResult", prefix = "tns", default)]
        pub update_first_name_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateLastName",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateLastName {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateLastNameResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateLastNameResponse {
        #[yaserde(rename = "UpdateLastNameResult", prefix = "tns", default)]
        pub update_last_name_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateLoginPage",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateLoginPage {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "loginPageID", prefix = "tns", default)]
        pub login_page_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateLoginPageResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateLoginPageResponse {
        #[yaserde(rename = "UpdateLoginPageResult", prefix = "tns", default)]
        pub update_login_page_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateObjection",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateObjection {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<bool>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateObjectionResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateObjectionResponse {
        #[yaserde(rename = "UpdateObjectionResult", prefix = "tns", default)]
        pub update_objection_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdatePassword",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdatePassword {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "oldPassword", prefix = "tns", default)]
        pub old_password: Option<String>,
        #[yaserde(rename = "newPassword", prefix = "tns", default)]
        pub new_password: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdatePasswordResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdatePasswordResponse {
        #[yaserde(rename = "UpdatePasswordResult", prefix = "tns", default)]
        pub update_password_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdatePreferredAgent",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdatePreferredAgent {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "preferredAgentID", prefix = "tns", default)]
        pub preferred_agent_id: Option<i64>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdatePreferredAgentResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdatePreferredAgentResponse {
        #[yaserde(rename = "UpdatePreferredAgentResult", prefix = "tns", default)]
        pub update_preferred_agent_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateRegisterDate",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateRegisterDate {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateRegisterDateResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateRegisterDateResponse {
        #[yaserde(rename = "UpdateRegisterDateResult", prefix = "tns", default)]
        pub update_register_date_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateTitle",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateTitle {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateTitleResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateTitleResponse {
        #[yaserde(rename = "UpdateTitleResult", prefix = "tns", default)]
        pub update_title_result: i64,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateUserName",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateUserName {
        #[yaserde(rename = "id", prefix = "tns", default)]
        pub id: Option<i64>,
        #[yaserde(rename = "newValue", prefix = "tns", default)]
        pub new_value: Option<String>,
        #[yaserde(rename = "sessionKey", prefix = "tns", default)]
        pub session_key: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "UpdateUserNameResponse",
        namespace = "tns: http://ws.db.ccmm.applications.nortel.com",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct UpdateUserNameResponse {
        #[yaserde(rename = "UpdateUserNameResult", prefix = "tns", default)]
        pub update_user_name_result: i64,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    pub type AddAddressSoapIn = messages::AddAddressSoapIn;

    pub type AddAddressSoapOut = messages::AddAddressSoapOut;

    pub type AddContactSoapIn = messages::AddContactSoapIn;

    pub type AddContactSoapOut = messages::AddContactSoapOut;

    pub type AddCustomFieldSoapIn = messages::AddCustomFieldSoapIn;

    pub type AddCustomFieldSoapOut = messages::AddCustomFieldSoapOut;

    pub type AddEmailAddressSoapIn = messages::AddEmailAddressSoapIn;

    pub type AddEmailAddressSoapOut = messages::AddEmailAddressSoapOut;

    pub type AddPhoneNumberSoapIn = messages::AddPhoneNumberSoapIn;

    pub type AddPhoneNumberSoapOut = messages::AddPhoneNumberSoapOut;

    pub type AddSipUriSoapIn = messages::AddSipUriSoapIn;

    pub type AddSipUriSoapOut = messages::AddSipUriSoapOut;

    pub type CarbonCopySoapIn = messages::CarbonCopySoapIn;

    pub type CarbonCopySoapOut = messages::CarbonCopySoapOut;

    pub type CleanCustomerSoapIn = messages::CleanCustomerSoapIn;

    pub type CleanCustomerSoapOut = messages::CleanCustomerSoapOut;

    pub type CreateCustomerSoapIn = messages::CreateCustomerSoapIn;

    pub type CreateCustomerSoapOut = messages::CreateCustomerSoapOut;

    pub type CreateCustomerBySipUriSoapIn = messages::CreateCustomerBySipUriSoapIn;

    pub type CreateCustomerBySipUriSoapOut = messages::CreateCustomerBySipUriSoapOut;

    pub type CustomerAssociationSoapIn = messages::CustomerAssociationSoapIn;

    pub type CustomerAssociationSoapOut = messages::CustomerAssociationSoapOut;

    pub type CustomerSearchSoapIn = messages::CustomerSearchSoapIn;

    pub type CustomerSearchSoapOut = messages::CustomerSearchSoapOut;

    pub type DeleteCustomerSoapIn = messages::DeleteCustomerSoapIn;

    pub type DeleteCustomerSoapOut = messages::DeleteCustomerSoapOut;

    pub type GetAllCustomersSoapIn = messages::GetAllCustomersSoapIn;

    pub type GetAllCustomersSoapOut = messages::GetAllCustomersSoapOut;

    pub type GetCustSQLColumnsSoapIn = messages::GetCustSQLColumnsSoapIn;

    pub type GetCustSQLColumnsSoapOut = messages::GetCustSQLColumnsSoapOut;

    pub type GetCustomFieldTemplatesSoapIn = messages::GetCustomFieldTemplatesSoapIn;

    pub type GetCustomFieldTemplatesSoapOut = messages::GetCustomFieldTemplatesSoapOut;

    pub type GetCustomerByContactIdSoapIn = messages::GetCustomerByContactIdSoapIn;

    pub type GetCustomerByContactIdSoapOut = messages::GetCustomerByContactIdSoapOut;

    pub type GetCustomerByEmailSoapIn = messages::GetCustomerByEmailSoapIn;

    pub type GetCustomerByEmailSoapOut = messages::GetCustomerByEmailSoapOut;

    pub type GetCustomerByNameSoapIn = messages::GetCustomerByNameSoapIn;

    pub type GetCustomerByNameSoapOut = messages::GetCustomerByNameSoapOut;

    pub type GetCustomerByPhoneNumberSoapIn = messages::GetCustomerByPhoneNumberSoapIn;

    pub type GetCustomerByPhoneNumberSoapOut = messages::GetCustomerByPhoneNumberSoapOut;

    pub type GetCustomerBySipUriSoapIn = messages::GetCustomerBySipUriSoapIn;

    pub type GetCustomerBySipUriSoapOut = messages::GetCustomerBySipUriSoapOut;

    pub type GetCustomerByUserNameSoapIn = messages::GetCustomerByUserNameSoapIn;

    pub type GetCustomerByUserNameSoapOut = messages::GetCustomerByUserNameSoapOut;

    pub type GetNoCustContactsByTimeSoapIn = messages::GetNoCustContactsByTimeSoapIn;

    pub type GetNoCustContactsByTimeSoapOut = messages::GetNoCustContactsByTimeSoapOut;

    pub type GetSearchableFieldsSoapIn = messages::GetSearchableFieldsSoapIn;

    pub type GetSearchableFieldsSoapOut = messages::GetSearchableFieldsSoapOut;

    pub type ImpersonateCustomerSoapIn = messages::ImpersonateCustomerSoapIn;

    pub type ImpersonateCustomerSoapOut = messages::ImpersonateCustomerSoapOut;

    pub type ReadCustomerSoapIn = messages::ReadCustomerSoapIn;

    pub type ReadCustomerSoapOut = messages::ReadCustomerSoapOut;

    pub type ReadCustomerHistorySoapIn = messages::ReadCustomerHistorySoapIn;

    pub type ReadCustomerHistorySoapOut = messages::ReadCustomerHistorySoapOut;

    pub type RegisterAnonymousCustomerSoapIn = messages::RegisterAnonymousCustomerSoapIn;

    pub type RegisterAnonymousCustomerSoapOut = messages::RegisterAnonymousCustomerSoapOut;

    pub type RegisterCustomerSoapIn = messages::RegisterCustomerSoapIn;

    pub type RegisterCustomerSoapOut = messages::RegisterCustomerSoapOut;

    pub type RemoveAddressSoapIn = messages::RemoveAddressSoapIn;

    pub type RemoveAddressSoapOut = messages::RemoveAddressSoapOut;

    pub type RemoveContactSoapIn = messages::RemoveContactSoapIn;

    pub type RemoveContactSoapOut = messages::RemoveContactSoapOut;

    pub type RemoveCustomFieldSoapIn = messages::RemoveCustomFieldSoapIn;

    pub type RemoveCustomFieldSoapOut = messages::RemoveCustomFieldSoapOut;

    pub type RemoveEmailAddressSoapIn = messages::RemoveEmailAddressSoapIn;

    pub type RemoveEmailAddressSoapOut = messages::RemoveEmailAddressSoapOut;

    pub type RemovePhoneNumberSoapIn = messages::RemovePhoneNumberSoapIn;

    pub type RemovePhoneNumberSoapOut = messages::RemovePhoneNumberSoapOut;

    pub type RemoveSipUriSoapIn = messages::RemoveSipUriSoapIn;

    pub type RemoveSipUriSoapOut = messages::RemoveSipUriSoapOut;

    pub type SendADPasswordReminderSoapIn = messages::SendADPasswordReminderSoapIn;

    pub type SendADPasswordReminderSoapOut = messages::SendADPasswordReminderSoapOut;

    pub type SendPasswordReminderSoapIn = messages::SendPasswordReminderSoapIn;

    pub type SendPasswordReminderSoapOut = messages::SendPasswordReminderSoapOut;

    pub type SetAgentIDSoapIn = messages::SetAgentIDSoapIn;

    pub type SetAgentIDSoapOut = messages::SetAgentIDSoapOut;

    pub type UpdateCustomerSoapIn = messages::UpdateCustomerSoapIn;

    pub type UpdateCustomerSoapOut = messages::UpdateCustomerSoapOut;

    pub type UpdateFirstNameSoapIn = messages::UpdateFirstNameSoapIn;

    pub type UpdateFirstNameSoapOut = messages::UpdateFirstNameSoapOut;

    pub type UpdateLastNameSoapIn = messages::UpdateLastNameSoapIn;

    pub type UpdateLastNameSoapOut = messages::UpdateLastNameSoapOut;

    pub type UpdateLoginPageSoapIn = messages::UpdateLoginPageSoapIn;

    pub type UpdateLoginPageSoapOut = messages::UpdateLoginPageSoapOut;

    pub type UpdateObjectionSoapIn = messages::UpdateObjectionSoapIn;

    pub type UpdateObjectionSoapOut = messages::UpdateObjectionSoapOut;

    pub type UpdatePasswordSoapIn = messages::UpdatePasswordSoapIn;

    pub type UpdatePasswordSoapOut = messages::UpdatePasswordSoapOut;

    pub type UpdatePreferredAgentSoapIn = messages::UpdatePreferredAgentSoapIn;

    pub type UpdatePreferredAgentSoapOut = messages::UpdatePreferredAgentSoapOut;

    pub type UpdateRegisterDateSoapIn = messages::UpdateRegisterDateSoapIn;

    pub type UpdateRegisterDateSoapOut = messages::UpdateRegisterDateSoapOut;

    pub type UpdateTitleSoapIn = messages::UpdateTitleSoapIn;

    pub type UpdateTitleSoapOut = messages::UpdateTitleSoapOut;

    pub type UpdateUserNameSoapIn = messages::UpdateUserNameSoapIn;

    pub type UpdateUserNameSoapOut = messages::UpdateUserNameSoapOut;

    #[async_trait]
    pub trait CustMultimediaSoap {
        async fn add_address(
            &self,
            add_address_soap_in: AddAddressSoapIn,
        ) -> Result<AddAddressSoapOut, Option<SoapFault>>;
        async fn add_contact(
            &self,
            add_contact_soap_in: AddContactSoapIn,
        ) -> Result<AddContactSoapOut, Option<SoapFault>>;
        async fn add_custom_field(
            &self,
            add_custom_field_soap_in: AddCustomFieldSoapIn,
        ) -> Result<AddCustomFieldSoapOut, Option<SoapFault>>;
        async fn add_email_address(
            &self,
            add_email_address_soap_in: AddEmailAddressSoapIn,
        ) -> Result<AddEmailAddressSoapOut, Option<SoapFault>>;
        async fn add_phone_number(
            &self,
            add_phone_number_soap_in: AddPhoneNumberSoapIn,
        ) -> Result<AddPhoneNumberSoapOut, Option<SoapFault>>;
        async fn add_sip_uri(
            &self,
            add_sip_uri_soap_in: AddSipUriSoapIn,
        ) -> Result<AddSipUriSoapOut, Option<SoapFault>>;
        async fn carbon_copy(
            &self,
            carbon_copy_soap_in: CarbonCopySoapIn,
        ) -> Result<CarbonCopySoapOut, Option<SoapFault>>;
        async fn clean_customer(
            &self,
            clean_customer_soap_in: CleanCustomerSoapIn,
        ) -> Result<CleanCustomerSoapOut, Option<SoapFault>>;
        async fn create_customer(
            &self,
            create_customer_soap_in: CreateCustomerSoapIn,
        ) -> Result<CreateCustomerSoapOut, Option<SoapFault>>;
        async fn create_customer_by_sip_uri(
            &self,
            create_customer_by_sip_uri_soap_in: CreateCustomerBySipUriSoapIn,
        ) -> Result<CreateCustomerBySipUriSoapOut, Option<SoapFault>>;
        async fn customer_association(
            &self,
            customer_association_soap_in: CustomerAssociationSoapIn,
        ) -> Result<CustomerAssociationSoapOut, Option<SoapFault>>;
        async fn customer_search(
            &self,
            customer_search_soap_in: CustomerSearchSoapIn,
        ) -> Result<CustomerSearchSoapOut, Option<SoapFault>>;
        async fn delete_customer(
            &self,
            delete_customer_soap_in: DeleteCustomerSoapIn,
        ) -> Result<DeleteCustomerSoapOut, Option<SoapFault>>;
        async fn get_all_customers(
            &self,
            get_all_customers_soap_in: GetAllCustomersSoapIn,
        ) -> Result<GetAllCustomersSoapOut, Option<SoapFault>>;
        async fn get_cust_sql_columns(
            &self,
            get_cust_sql_columns_soap_in: GetCustSQLColumnsSoapIn,
        ) -> Result<GetCustSQLColumnsSoapOut, Option<SoapFault>>;
        async fn get_custom_field_templates(
            &self,
            get_custom_field_templates_soap_in: GetCustomFieldTemplatesSoapIn,
        ) -> Result<GetCustomFieldTemplatesSoapOut, Option<SoapFault>>;
        async fn get_customer_by_contact_id(
            &self,
            get_customer_by_contact_id_soap_in: GetCustomerByContactIdSoapIn,
        ) -> Result<GetCustomerByContactIdSoapOut, Option<SoapFault>>;
        async fn get_customer_by_email(
            &self,
            get_customer_by_email_soap_in: GetCustomerByEmailSoapIn,
        ) -> Result<GetCustomerByEmailSoapOut, Option<SoapFault>>;
        async fn get_customer_by_name(
            &self,
            get_customer_by_name_soap_in: GetCustomerByNameSoapIn,
        ) -> Result<GetCustomerByNameSoapOut, Option<SoapFault>>;
        async fn get_customer_by_phone_number(
            &self,
            get_customer_by_phone_number_soap_in: GetCustomerByPhoneNumberSoapIn,
        ) -> Result<GetCustomerByPhoneNumberSoapOut, Option<SoapFault>>;
        async fn get_customer_by_sip_uri(
            &self,
            get_customer_by_sip_uri_soap_in: GetCustomerBySipUriSoapIn,
        ) -> Result<GetCustomerBySipUriSoapOut, Option<SoapFault>>;
        async fn get_customer_by_user_name(
            &self,
            get_customer_by_user_name_soap_in: GetCustomerByUserNameSoapIn,
        ) -> Result<GetCustomerByUserNameSoapOut, Option<SoapFault>>;
        async fn get_no_cust_contacts_by_time(
            &self,
            get_no_cust_contacts_by_time_soap_in: GetNoCustContactsByTimeSoapIn,
        ) -> Result<GetNoCustContactsByTimeSoapOut, Option<SoapFault>>;
        async fn get_searchable_fields(
            &self,
            get_searchable_fields_soap_in: GetSearchableFieldsSoapIn,
        ) -> Result<GetSearchableFieldsSoapOut, Option<SoapFault>>;
        async fn impersonate_customer(
            &self,
            impersonate_customer_soap_in: ImpersonateCustomerSoapIn,
        ) -> Result<ImpersonateCustomerSoapOut, Option<SoapFault>>;
        async fn read_customer(
            &self,
            read_customer_soap_in: ReadCustomerSoapIn,
        ) -> Result<ReadCustomerSoapOut, Option<SoapFault>>;
        async fn read_customer_history(
            &self,
            read_customer_history_soap_in: ReadCustomerHistorySoapIn,
        ) -> Result<ReadCustomerHistorySoapOut, Option<SoapFault>>;
        async fn register_anonymous_customer(
            &self,
            register_anonymous_customer_soap_in: RegisterAnonymousCustomerSoapIn,
        ) -> Result<RegisterAnonymousCustomerSoapOut, Option<SoapFault>>;
        async fn register_customer(
            &self,
            register_customer_soap_in: RegisterCustomerSoapIn,
        ) -> Result<RegisterCustomerSoapOut, Option<SoapFault>>;
        async fn remove_address(
            &self,
            remove_address_soap_in: RemoveAddressSoapIn,
        ) -> Result<RemoveAddressSoapOut, Option<SoapFault>>;
        async fn remove_contact(
            &self,
            remove_contact_soap_in: RemoveContactSoapIn,
        ) -> Result<RemoveContactSoapOut, Option<SoapFault>>;
        async fn remove_custom_field(
            &self,
            remove_custom_field_soap_in: RemoveCustomFieldSoapIn,
        ) -> Result<RemoveCustomFieldSoapOut, Option<SoapFault>>;
        async fn remove_email_address(
            &self,
            remove_email_address_soap_in: RemoveEmailAddressSoapIn,
        ) -> Result<RemoveEmailAddressSoapOut, Option<SoapFault>>;
        async fn remove_phone_number(
            &self,
            remove_phone_number_soap_in: RemovePhoneNumberSoapIn,
        ) -> Result<RemovePhoneNumberSoapOut, Option<SoapFault>>;
        async fn remove_sip_uri(
            &self,
            remove_sip_uri_soap_in: RemoveSipUriSoapIn,
        ) -> Result<RemoveSipUriSoapOut, Option<SoapFault>>;
        async fn send_ad_password_reminder(
            &self,
            send_ad_password_reminder_soap_in: SendADPasswordReminderSoapIn,
        ) -> Result<SendADPasswordReminderSoapOut, Option<SoapFault>>;
        async fn send_password_reminder(
            &self,
            send_password_reminder_soap_in: SendPasswordReminderSoapIn,
        ) -> Result<SendPasswordReminderSoapOut, Option<SoapFault>>;
        async fn set_agent_id(
            &self,
            set_agent_id_soap_in: SetAgentIDSoapIn,
        ) -> Result<SetAgentIDSoapOut, Option<SoapFault>>;
        async fn update_customer(
            &self,
            update_customer_soap_in: UpdateCustomerSoapIn,
        ) -> Result<UpdateCustomerSoapOut, Option<SoapFault>>;
        async fn update_first_name(
            &self,
            update_first_name_soap_in: UpdateFirstNameSoapIn,
        ) -> Result<UpdateFirstNameSoapOut, Option<SoapFault>>;
        async fn update_last_name(
            &self,
            update_last_name_soap_in: UpdateLastNameSoapIn,
        ) -> Result<UpdateLastNameSoapOut, Option<SoapFault>>;
        async fn update_login_page(
            &self,
            update_login_page_soap_in: UpdateLoginPageSoapIn,
        ) -> Result<UpdateLoginPageSoapOut, Option<SoapFault>>;
        async fn update_objection(
            &self,
            update_objection_soap_in: UpdateObjectionSoapIn,
        ) -> Result<UpdateObjectionSoapOut, Option<SoapFault>>;
        async fn update_password(
            &self,
            update_password_soap_in: UpdatePasswordSoapIn,
        ) -> Result<UpdatePasswordSoapOut, Option<SoapFault>>;
        async fn update_preferred_agent(
            &self,
            update_preferred_agent_soap_in: UpdatePreferredAgentSoapIn,
        ) -> Result<UpdatePreferredAgentSoapOut, Option<SoapFault>>;
        async fn update_register_date(
            &self,
            update_register_date_soap_in: UpdateRegisterDateSoapIn,
        ) -> Result<UpdateRegisterDateSoapOut, Option<SoapFault>>;
        async fn update_title(
            &self,
            update_title_soap_in: UpdateTitleSoapIn,
        ) -> Result<UpdateTitleSoapOut, Option<SoapFault>>;
        async fn update_user_name(
            &self,
            update_user_name_soap_in: UpdateUserNameSoapIn,
        ) -> Result<UpdateUserNameSoapOut, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};

    impl CustMultimediaSoap {
        async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddAddressSoapIn {
        #[yaserde(rename = "tns:AddAddress", default)]
        pub body: ports::AddAddressSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddAddressSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddAddressSoapIn,
    }

    impl AddAddressSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddAddressSoapIn) -> Self {
            AddAddressSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddAddressSoapOut {
        #[yaserde(rename = "AddAddressResponse", default)]
        pub body: Option<ports::AddAddressSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddAddressSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddAddressSoapOut,
    }

    impl AddAddressSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddAddressSoapOut) -> Self {
            AddAddressSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddContactSoapIn {
        #[yaserde(rename = "tns:AddContact", default)]
        pub body: ports::AddContactSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddContactSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddContactSoapIn,
    }

    impl AddContactSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddContactSoapIn) -> Self {
            AddContactSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddContactSoapOut {
        #[yaserde(rename = "AddContactResponse", default)]
        pub body: Option<ports::AddContactSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddContactSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddContactSoapOut,
    }

    impl AddContactSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddContactSoapOut) -> Self {
            AddContactSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddCustomFieldSoapIn {
        #[yaserde(rename = "tns:AddCustomField", default)]
        pub body: ports::AddCustomFieldSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddCustomFieldSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddCustomFieldSoapIn,
    }

    impl AddCustomFieldSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddCustomFieldSoapIn) -> Self {
            AddCustomFieldSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddCustomFieldSoapOut {
        #[yaserde(rename = "AddCustomFieldResponse", default)]
        pub body: Option<ports::AddCustomFieldSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddCustomFieldSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddCustomFieldSoapOut,
    }

    impl AddCustomFieldSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddCustomFieldSoapOut) -> Self {
            AddCustomFieldSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddEmailAddressSoapIn {
        #[yaserde(rename = "tns:AddEmailAddress", default)]
        pub body: ports::AddEmailAddressSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddEmailAddressSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddEmailAddressSoapIn,
    }

    impl AddEmailAddressSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddEmailAddressSoapIn) -> Self {
            AddEmailAddressSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddEmailAddressSoapOut {
        #[yaserde(rename = "AddEmailAddressResponse", default)]
        pub body: Option<ports::AddEmailAddressSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddEmailAddressSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddEmailAddressSoapOut,
    }

    impl AddEmailAddressSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddEmailAddressSoapOut) -> Self {
            AddEmailAddressSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddPhoneNumberSoapIn {
        #[yaserde(rename = "tns:AddPhoneNumber", default)]
        pub body: ports::AddPhoneNumberSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddPhoneNumberSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddPhoneNumberSoapIn,
    }

    impl AddPhoneNumberSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddPhoneNumberSoapIn) -> Self {
            AddPhoneNumberSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddPhoneNumberSoapOut {
        #[yaserde(rename = "AddPhoneNumberResponse", default)]
        pub body: Option<ports::AddPhoneNumberSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddPhoneNumberSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddPhoneNumberSoapOut,
    }

    impl AddPhoneNumberSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddPhoneNumberSoapOut) -> Self {
            AddPhoneNumberSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddSipUriSoapIn {
        #[yaserde(rename = "tns:AddSipUri", default)]
        pub body: ports::AddSipUriSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddSipUriSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddSipUriSoapIn,
    }

    impl AddSipUriSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddSipUriSoapIn) -> Self {
            AddSipUriSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapAddSipUriSoapOut {
        #[yaserde(rename = "AddSipUriResponse", default)]
        pub body: Option<ports::AddSipUriSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct AddSipUriSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapAddSipUriSoapOut,
    }

    impl AddSipUriSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapAddSipUriSoapOut) -> Self {
            AddSipUriSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCarbonCopySoapIn {
        #[yaserde(rename = "tns:CarbonCopy", default)]
        pub body: ports::CarbonCopySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CarbonCopySoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCarbonCopySoapIn,
    }

    impl CarbonCopySoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCarbonCopySoapIn) -> Self {
            CarbonCopySoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCarbonCopySoapOut {
        #[yaserde(rename = "CarbonCopyResponse", default)]
        pub body: Option<ports::CarbonCopySoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CarbonCopySoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCarbonCopySoapOut,
    }

    impl CarbonCopySoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCarbonCopySoapOut) -> Self {
            CarbonCopySoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCleanCustomerSoapIn {
        #[yaserde(rename = "tns:CleanCustomer", default)]
        pub body: ports::CleanCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CleanCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCleanCustomerSoapIn,
    }

    impl CleanCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCleanCustomerSoapIn) -> Self {
            CleanCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCleanCustomerSoapOut {
        #[yaserde(rename = "CleanCustomerResponse", default)]
        pub body: Option<ports::CleanCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CleanCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCleanCustomerSoapOut,
    }

    impl CleanCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCleanCustomerSoapOut) -> Self {
            CleanCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateCustomerSoapIn {
        #[yaserde(rename = "tns:CreateCustomer", default)]
        pub body: ports::CreateCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateCustomerSoapIn,
    }

    impl CreateCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateCustomerSoapIn) -> Self {
            CreateCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateCustomerSoapOut {
        #[yaserde(rename = "CreateCustomerResponse", default)]
        pub body: Option<ports::CreateCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateCustomerSoapOut,
    }

    impl CreateCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateCustomerSoapOut) -> Self {
            CreateCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateCustomerBySipUriSoapIn {
        #[yaserde(rename = "tns:CreateCustomerBySipUri", default)]
        pub body: ports::CreateCustomerBySipUriSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateCustomerBySipUriSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateCustomerBySipUriSoapIn,
    }

    impl CreateCustomerBySipUriSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateCustomerBySipUriSoapIn) -> Self {
            CreateCustomerBySipUriSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCreateCustomerBySipUriSoapOut {
        #[yaserde(rename = "CreateCustomerBySipUriResponse", default)]
        pub body: Option<ports::CreateCustomerBySipUriSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CreateCustomerBySipUriSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCreateCustomerBySipUriSoapOut,
    }

    impl CreateCustomerBySipUriSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCreateCustomerBySipUriSoapOut) -> Self {
            CreateCustomerBySipUriSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCustomerAssociationSoapIn {
        #[yaserde(rename = "tns:CustomerAssociation", default)]
        pub body: ports::CustomerAssociationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CustomerAssociationSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCustomerAssociationSoapIn,
    }

    impl CustomerAssociationSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCustomerAssociationSoapIn) -> Self {
            CustomerAssociationSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCustomerAssociationSoapOut {
        #[yaserde(rename = "CustomerAssociationResponse", default)]
        pub body: Option<ports::CustomerAssociationSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CustomerAssociationSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCustomerAssociationSoapOut,
    }

    impl CustomerAssociationSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCustomerAssociationSoapOut) -> Self {
            CustomerAssociationSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCustomerSearchSoapIn {
        #[yaserde(rename = "tns:CustomerSearch", default)]
        pub body: ports::CustomerSearchSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CustomerSearchSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCustomerSearchSoapIn,
    }

    impl CustomerSearchSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCustomerSearchSoapIn) -> Self {
            CustomerSearchSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapCustomerSearchSoapOut {
        #[yaserde(rename = "CustomerSearchResponse", default)]
        pub body: Option<ports::CustomerSearchSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct CustomerSearchSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapCustomerSearchSoapOut,
    }

    impl CustomerSearchSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapCustomerSearchSoapOut) -> Self {
            CustomerSearchSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapDeleteCustomerSoapIn {
        #[yaserde(rename = "tns:DeleteCustomer", default)]
        pub body: ports::DeleteCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapDeleteCustomerSoapIn,
    }

    impl DeleteCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapDeleteCustomerSoapIn) -> Self {
            DeleteCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapDeleteCustomerSoapOut {
        #[yaserde(rename = "DeleteCustomerResponse", default)]
        pub body: Option<ports::DeleteCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct DeleteCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapDeleteCustomerSoapOut,
    }

    impl DeleteCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapDeleteCustomerSoapOut) -> Self {
            DeleteCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetAllCustomersSoapIn {
        #[yaserde(rename = "tns:GetAllCustomers", default)]
        pub body: ports::GetAllCustomersSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetAllCustomersSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetAllCustomersSoapIn,
    }

    impl GetAllCustomersSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetAllCustomersSoapIn) -> Self {
            GetAllCustomersSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetAllCustomersSoapOut {
        #[yaserde(rename = "GetAllCustomersResponse", default)]
        pub body: Option<ports::GetAllCustomersSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetAllCustomersSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetAllCustomersSoapOut,
    }

    impl GetAllCustomersSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetAllCustomersSoapOut) -> Self {
            GetAllCustomersSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustSQLColumnsSoapIn {
        #[yaserde(rename = "tns:GetCustSQLColumns", default)]
        pub body: ports::GetCustSQLColumnsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustSQLColumnsSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustSQLColumnsSoapIn,
    }

    impl GetCustSQLColumnsSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustSQLColumnsSoapIn) -> Self {
            GetCustSQLColumnsSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustSQLColumnsSoapOut {
        #[yaserde(rename = "GetCustSQLColumnsResponse", default)]
        pub body: Option<ports::GetCustSQLColumnsSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustSQLColumnsSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustSQLColumnsSoapOut,
    }

    impl GetCustSQLColumnsSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustSQLColumnsSoapOut) -> Self {
            GetCustSQLColumnsSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomFieldTemplatesSoapIn {
        #[yaserde(rename = "tns:GetCustomFieldTemplates", default)]
        pub body: ports::GetCustomFieldTemplatesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomFieldTemplatesSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomFieldTemplatesSoapIn,
    }

    impl GetCustomFieldTemplatesSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomFieldTemplatesSoapIn) -> Self {
            GetCustomFieldTemplatesSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomFieldTemplatesSoapOut {
        #[yaserde(rename = "GetCustomFieldTemplatesResponse", default)]
        pub body: Option<ports::GetCustomFieldTemplatesSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomFieldTemplatesSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomFieldTemplatesSoapOut,
    }

    impl GetCustomFieldTemplatesSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomFieldTemplatesSoapOut) -> Self {
            GetCustomFieldTemplatesSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByContactIdSoapIn {
        #[yaserde(rename = "tns:GetCustomerByContactId", default)]
        pub body: ports::GetCustomerByContactIdSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByContactIdSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByContactIdSoapIn,
    }

    impl GetCustomerByContactIdSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByContactIdSoapIn) -> Self {
            GetCustomerByContactIdSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByContactIdSoapOut {
        #[yaserde(rename = "GetCustomerByContactIdResponse", default)]
        pub body: Option<ports::GetCustomerByContactIdSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByContactIdSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByContactIdSoapOut,
    }

    impl GetCustomerByContactIdSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByContactIdSoapOut) -> Self {
            GetCustomerByContactIdSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByEmailSoapIn {
        #[yaserde(rename = "tns:GetCustomerByEmail", default)]
        pub body: ports::GetCustomerByEmailSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByEmailSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByEmailSoapIn,
    }

    impl GetCustomerByEmailSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByEmailSoapIn) -> Self {
            GetCustomerByEmailSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByEmailSoapOut {
        #[yaserde(rename = "GetCustomerByEmailResponse", default)]
        pub body: Option<ports::GetCustomerByEmailSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByEmailSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByEmailSoapOut,
    }

    impl GetCustomerByEmailSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByEmailSoapOut) -> Self {
            GetCustomerByEmailSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByNameSoapIn {
        #[yaserde(rename = "tns:GetCustomerByName", default)]
        pub body: ports::GetCustomerByNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByNameSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByNameSoapIn,
    }

    impl GetCustomerByNameSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByNameSoapIn) -> Self {
            GetCustomerByNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByNameSoapOut {
        #[yaserde(rename = "GetCustomerByNameResponse", default)]
        pub body: Option<ports::GetCustomerByNameSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByNameSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByNameSoapOut,
    }

    impl GetCustomerByNameSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByNameSoapOut) -> Self {
            GetCustomerByNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByPhoneNumberSoapIn {
        #[yaserde(rename = "tns:GetCustomerByPhoneNumber", default)]
        pub body: ports::GetCustomerByPhoneNumberSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByPhoneNumberSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByPhoneNumberSoapIn,
    }

    impl GetCustomerByPhoneNumberSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByPhoneNumberSoapIn) -> Self {
            GetCustomerByPhoneNumberSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByPhoneNumberSoapOut {
        #[yaserde(rename = "GetCustomerByPhoneNumberResponse", default)]
        pub body: Option<ports::GetCustomerByPhoneNumberSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByPhoneNumberSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByPhoneNumberSoapOut,
    }

    impl GetCustomerByPhoneNumberSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByPhoneNumberSoapOut) -> Self {
            GetCustomerByPhoneNumberSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerBySipUriSoapIn {
        #[yaserde(rename = "tns:GetCustomerBySipUri", default)]
        pub body: ports::GetCustomerBySipUriSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerBySipUriSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerBySipUriSoapIn,
    }

    impl GetCustomerBySipUriSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerBySipUriSoapIn) -> Self {
            GetCustomerBySipUriSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerBySipUriSoapOut {
        #[yaserde(rename = "GetCustomerBySipUriResponse", default)]
        pub body: Option<ports::GetCustomerBySipUriSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerBySipUriSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerBySipUriSoapOut,
    }

    impl GetCustomerBySipUriSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerBySipUriSoapOut) -> Self {
            GetCustomerBySipUriSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByUserNameSoapIn {
        #[yaserde(rename = "tns:GetCustomerByUserName", default)]
        pub body: ports::GetCustomerByUserNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByUserNameSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByUserNameSoapIn,
    }

    impl GetCustomerByUserNameSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByUserNameSoapIn) -> Self {
            GetCustomerByUserNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCustomerByUserNameSoapOut {
        #[yaserde(rename = "GetCustomerByUserNameResponse", default)]
        pub body: Option<ports::GetCustomerByUserNameSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCustomerByUserNameSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCustomerByUserNameSoapOut,
    }

    impl GetCustomerByUserNameSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCustomerByUserNameSoapOut) -> Self {
            GetCustomerByUserNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetNoCustContactsByTimeSoapIn {
        #[yaserde(rename = "tns:GetNoCustContactsByTime", default)]
        pub body: ports::GetNoCustContactsByTimeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetNoCustContactsByTimeSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetNoCustContactsByTimeSoapIn,
    }

    impl GetNoCustContactsByTimeSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetNoCustContactsByTimeSoapIn) -> Self {
            GetNoCustContactsByTimeSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetNoCustContactsByTimeSoapOut {
        #[yaserde(rename = "GetNoCustContactsByTimeResponse", default)]
        pub body: Option<ports::GetNoCustContactsByTimeSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetNoCustContactsByTimeSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetNoCustContactsByTimeSoapOut,
    }

    impl GetNoCustContactsByTimeSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetNoCustContactsByTimeSoapOut) -> Self {
            GetNoCustContactsByTimeSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetSearchableFieldsSoapIn {
        #[yaserde(rename = "tns:GetSearchableFields", default)]
        pub body: ports::GetSearchableFieldsSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetSearchableFieldsSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetSearchableFieldsSoapIn,
    }

    impl GetSearchableFieldsSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetSearchableFieldsSoapIn) -> Self {
            GetSearchableFieldsSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetSearchableFieldsSoapOut {
        #[yaserde(rename = "GetSearchableFieldsResponse", default)]
        pub body: Option<ports::GetSearchableFieldsSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetSearchableFieldsSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetSearchableFieldsSoapOut,
    }

    impl GetSearchableFieldsSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetSearchableFieldsSoapOut) -> Self {
            GetSearchableFieldsSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapImpersonateCustomerSoapIn {
        #[yaserde(rename = "tns:ImpersonateCustomer", default)]
        pub body: ports::ImpersonateCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ImpersonateCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapImpersonateCustomerSoapIn,
    }

    impl ImpersonateCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapImpersonateCustomerSoapIn) -> Self {
            ImpersonateCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapImpersonateCustomerSoapOut {
        #[yaserde(rename = "ImpersonateCustomerResponse", default)]
        pub body: Option<ports::ImpersonateCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ImpersonateCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapImpersonateCustomerSoapOut,
    }

    impl ImpersonateCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapImpersonateCustomerSoapOut) -> Self {
            ImpersonateCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapReadCustomerSoapIn {
        #[yaserde(rename = "tns:ReadCustomer", default)]
        pub body: ports::ReadCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ReadCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapReadCustomerSoapIn,
    }

    impl ReadCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapReadCustomerSoapIn) -> Self {
            ReadCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapReadCustomerSoapOut {
        #[yaserde(rename = "ReadCustomerResponse", default)]
        pub body: Option<ports::ReadCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ReadCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapReadCustomerSoapOut,
    }

    impl ReadCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapReadCustomerSoapOut) -> Self {
            ReadCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapReadCustomerHistorySoapIn {
        #[yaserde(rename = "tns:ReadCustomerHistory", default)]
        pub body: ports::ReadCustomerHistorySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ReadCustomerHistorySoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapReadCustomerHistorySoapIn,
    }

    impl ReadCustomerHistorySoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapReadCustomerHistorySoapIn) -> Self {
            ReadCustomerHistorySoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapReadCustomerHistorySoapOut {
        #[yaserde(rename = "ReadCustomerHistoryResponse", default)]
        pub body: Option<ports::ReadCustomerHistorySoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct ReadCustomerHistorySoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapReadCustomerHistorySoapOut,
    }

    impl ReadCustomerHistorySoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapReadCustomerHistorySoapOut) -> Self {
            ReadCustomerHistorySoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRegisterAnonymousCustomerSoapIn {
        #[yaserde(rename = "tns:RegisterAnonymousCustomer", default)]
        pub body: ports::RegisterAnonymousCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RegisterAnonymousCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRegisterAnonymousCustomerSoapIn,
    }

    impl RegisterAnonymousCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRegisterAnonymousCustomerSoapIn) -> Self {
            RegisterAnonymousCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRegisterAnonymousCustomerSoapOut {
        #[yaserde(rename = "RegisterAnonymousCustomerResponse", default)]
        pub body: Option<ports::RegisterAnonymousCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RegisterAnonymousCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRegisterAnonymousCustomerSoapOut,
    }

    impl RegisterAnonymousCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRegisterAnonymousCustomerSoapOut) -> Self {
            RegisterAnonymousCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRegisterCustomerSoapIn {
        #[yaserde(rename = "tns:RegisterCustomer", default)]
        pub body: ports::RegisterCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RegisterCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRegisterCustomerSoapIn,
    }

    impl RegisterCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRegisterCustomerSoapIn) -> Self {
            RegisterCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRegisterCustomerSoapOut {
        #[yaserde(rename = "RegisterCustomerResponse", default)]
        pub body: Option<ports::RegisterCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RegisterCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRegisterCustomerSoapOut,
    }

    impl RegisterCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRegisterCustomerSoapOut) -> Self {
            RegisterCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveAddressSoapIn {
        #[yaserde(rename = "tns:RemoveAddress", default)]
        pub body: ports::RemoveAddressSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveAddressSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveAddressSoapIn,
    }

    impl RemoveAddressSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveAddressSoapIn) -> Self {
            RemoveAddressSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveAddressSoapOut {
        #[yaserde(rename = "RemoveAddressResponse", default)]
        pub body: Option<ports::RemoveAddressSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveAddressSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveAddressSoapOut,
    }

    impl RemoveAddressSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveAddressSoapOut) -> Self {
            RemoveAddressSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveContactSoapIn {
        #[yaserde(rename = "tns:RemoveContact", default)]
        pub body: ports::RemoveContactSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveContactSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveContactSoapIn,
    }

    impl RemoveContactSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveContactSoapIn) -> Self {
            RemoveContactSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveContactSoapOut {
        #[yaserde(rename = "RemoveContactResponse", default)]
        pub body: Option<ports::RemoveContactSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveContactSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveContactSoapOut,
    }

    impl RemoveContactSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveContactSoapOut) -> Self {
            RemoveContactSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveCustomFieldSoapIn {
        #[yaserde(rename = "tns:RemoveCustomField", default)]
        pub body: ports::RemoveCustomFieldSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveCustomFieldSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveCustomFieldSoapIn,
    }

    impl RemoveCustomFieldSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveCustomFieldSoapIn) -> Self {
            RemoveCustomFieldSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveCustomFieldSoapOut {
        #[yaserde(rename = "RemoveCustomFieldResponse", default)]
        pub body: Option<ports::RemoveCustomFieldSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveCustomFieldSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveCustomFieldSoapOut,
    }

    impl RemoveCustomFieldSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveCustomFieldSoapOut) -> Self {
            RemoveCustomFieldSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveEmailAddressSoapIn {
        #[yaserde(rename = "tns:RemoveEmailAddress", default)]
        pub body: ports::RemoveEmailAddressSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveEmailAddressSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveEmailAddressSoapIn,
    }

    impl RemoveEmailAddressSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveEmailAddressSoapIn) -> Self {
            RemoveEmailAddressSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveEmailAddressSoapOut {
        #[yaserde(rename = "RemoveEmailAddressResponse", default)]
        pub body: Option<ports::RemoveEmailAddressSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveEmailAddressSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveEmailAddressSoapOut,
    }

    impl RemoveEmailAddressSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveEmailAddressSoapOut) -> Self {
            RemoveEmailAddressSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemovePhoneNumberSoapIn {
        #[yaserde(rename = "tns:RemovePhoneNumber", default)]
        pub body: ports::RemovePhoneNumberSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemovePhoneNumberSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemovePhoneNumberSoapIn,
    }

    impl RemovePhoneNumberSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemovePhoneNumberSoapIn) -> Self {
            RemovePhoneNumberSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemovePhoneNumberSoapOut {
        #[yaserde(rename = "RemovePhoneNumberResponse", default)]
        pub body: Option<ports::RemovePhoneNumberSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemovePhoneNumberSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemovePhoneNumberSoapOut,
    }

    impl RemovePhoneNumberSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemovePhoneNumberSoapOut) -> Self {
            RemovePhoneNumberSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveSipUriSoapIn {
        #[yaserde(rename = "tns:RemoveSipUri", default)]
        pub body: ports::RemoveSipUriSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveSipUriSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveSipUriSoapIn,
    }

    impl RemoveSipUriSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveSipUriSoapIn) -> Self {
            RemoveSipUriSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapRemoveSipUriSoapOut {
        #[yaserde(rename = "RemoveSipUriResponse", default)]
        pub body: Option<ports::RemoveSipUriSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct RemoveSipUriSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapRemoveSipUriSoapOut,
    }

    impl RemoveSipUriSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapRemoveSipUriSoapOut) -> Self {
            RemoveSipUriSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendADPasswordReminderSoapIn {
        #[yaserde(rename = "tns:SendADPasswordReminder", default)]
        pub body: ports::SendADPasswordReminderSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SendADPasswordReminderSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSendADPasswordReminderSoapIn,
    }

    impl SendADPasswordReminderSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSendADPasswordReminderSoapIn) -> Self {
            SendADPasswordReminderSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendADPasswordReminderSoapOut {
        #[yaserde(rename = "SendADPasswordReminderResponse", default)]
        pub body: Option<ports::SendADPasswordReminderSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SendADPasswordReminderSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSendADPasswordReminderSoapOut,
    }

    impl SendADPasswordReminderSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSendADPasswordReminderSoapOut) -> Self {
            SendADPasswordReminderSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendPasswordReminderSoapIn {
        #[yaserde(rename = "tns:SendPasswordReminder", default)]
        pub body: ports::SendPasswordReminderSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SendPasswordReminderSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSendPasswordReminderSoapIn,
    }

    impl SendPasswordReminderSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSendPasswordReminderSoapIn) -> Self {
            SendPasswordReminderSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSendPasswordReminderSoapOut {
        #[yaserde(rename = "SendPasswordReminderResponse", default)]
        pub body: Option<ports::SendPasswordReminderSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SendPasswordReminderSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSendPasswordReminderSoapOut,
    }

    impl SendPasswordReminderSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSendPasswordReminderSoapOut) -> Self {
            SendPasswordReminderSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSetAgentIDSoapIn {
        #[yaserde(rename = "tns:SetAgentID", default)]
        pub body: ports::SetAgentIDSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SetAgentIDSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSetAgentIDSoapIn,
    }

    impl SetAgentIDSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSetAgentIDSoapIn) -> Self {
            SetAgentIDSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapSetAgentIDSoapOut {
        #[yaserde(rename = "SetAgentIDResponse", default)]
        pub body: Option<ports::SetAgentIDSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct SetAgentIDSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapSetAgentIDSoapOut,
    }

    impl SetAgentIDSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapSetAgentIDSoapOut) -> Self {
            SetAgentIDSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateCustomerSoapIn {
        #[yaserde(rename = "tns:UpdateCustomer", default)]
        pub body: ports::UpdateCustomerSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateCustomerSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateCustomerSoapIn,
    }

    impl UpdateCustomerSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateCustomerSoapIn) -> Self {
            UpdateCustomerSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateCustomerSoapOut {
        #[yaserde(rename = "UpdateCustomerResponse", default)]
        pub body: Option<ports::UpdateCustomerSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateCustomerSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateCustomerSoapOut,
    }

    impl UpdateCustomerSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateCustomerSoapOut) -> Self {
            UpdateCustomerSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateFirstNameSoapIn {
        #[yaserde(rename = "tns:UpdateFirstName", default)]
        pub body: ports::UpdateFirstNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateFirstNameSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateFirstNameSoapIn,
    }

    impl UpdateFirstNameSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateFirstNameSoapIn) -> Self {
            UpdateFirstNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateFirstNameSoapOut {
        #[yaserde(rename = "UpdateFirstNameResponse", default)]
        pub body: Option<ports::UpdateFirstNameSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateFirstNameSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateFirstNameSoapOut,
    }

    impl UpdateFirstNameSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateFirstNameSoapOut) -> Self {
            UpdateFirstNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateLastNameSoapIn {
        #[yaserde(rename = "tns:UpdateLastName", default)]
        pub body: ports::UpdateLastNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateLastNameSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateLastNameSoapIn,
    }

    impl UpdateLastNameSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateLastNameSoapIn) -> Self {
            UpdateLastNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateLastNameSoapOut {
        #[yaserde(rename = "UpdateLastNameResponse", default)]
        pub body: Option<ports::UpdateLastNameSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateLastNameSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateLastNameSoapOut,
    }

    impl UpdateLastNameSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateLastNameSoapOut) -> Self {
            UpdateLastNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateLoginPageSoapIn {
        #[yaserde(rename = "tns:UpdateLoginPage", default)]
        pub body: ports::UpdateLoginPageSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateLoginPageSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateLoginPageSoapIn,
    }

    impl UpdateLoginPageSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateLoginPageSoapIn) -> Self {
            UpdateLoginPageSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateLoginPageSoapOut {
        #[yaserde(rename = "UpdateLoginPageResponse", default)]
        pub body: Option<ports::UpdateLoginPageSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateLoginPageSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateLoginPageSoapOut,
    }

    impl UpdateLoginPageSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateLoginPageSoapOut) -> Self {
            UpdateLoginPageSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateObjectionSoapIn {
        #[yaserde(rename = "tns:UpdateObjection", default)]
        pub body: ports::UpdateObjectionSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateObjectionSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateObjectionSoapIn,
    }

    impl UpdateObjectionSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateObjectionSoapIn) -> Self {
            UpdateObjectionSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateObjectionSoapOut {
        #[yaserde(rename = "UpdateObjectionResponse", default)]
        pub body: Option<ports::UpdateObjectionSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateObjectionSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateObjectionSoapOut,
    }

    impl UpdateObjectionSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateObjectionSoapOut) -> Self {
            UpdateObjectionSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdatePasswordSoapIn {
        #[yaserde(rename = "tns:UpdatePassword", default)]
        pub body: ports::UpdatePasswordSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdatePasswordSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdatePasswordSoapIn,
    }

    impl UpdatePasswordSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdatePasswordSoapIn) -> Self {
            UpdatePasswordSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdatePasswordSoapOut {
        #[yaserde(rename = "UpdatePasswordResponse", default)]
        pub body: Option<ports::UpdatePasswordSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdatePasswordSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdatePasswordSoapOut,
    }

    impl UpdatePasswordSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdatePasswordSoapOut) -> Self {
            UpdatePasswordSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdatePreferredAgentSoapIn {
        #[yaserde(rename = "tns:UpdatePreferredAgent", default)]
        pub body: ports::UpdatePreferredAgentSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdatePreferredAgentSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdatePreferredAgentSoapIn,
    }

    impl UpdatePreferredAgentSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdatePreferredAgentSoapIn) -> Self {
            UpdatePreferredAgentSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdatePreferredAgentSoapOut {
        #[yaserde(rename = "UpdatePreferredAgentResponse", default)]
        pub body: Option<ports::UpdatePreferredAgentSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdatePreferredAgentSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdatePreferredAgentSoapOut,
    }

    impl UpdatePreferredAgentSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdatePreferredAgentSoapOut) -> Self {
            UpdatePreferredAgentSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateRegisterDateSoapIn {
        #[yaserde(rename = "tns:UpdateRegisterDate", default)]
        pub body: ports::UpdateRegisterDateSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateRegisterDateSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateRegisterDateSoapIn,
    }

    impl UpdateRegisterDateSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateRegisterDateSoapIn) -> Self {
            UpdateRegisterDateSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateRegisterDateSoapOut {
        #[yaserde(rename = "UpdateRegisterDateResponse", default)]
        pub body: Option<ports::UpdateRegisterDateSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateRegisterDateSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateRegisterDateSoapOut,
    }

    impl UpdateRegisterDateSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateRegisterDateSoapOut) -> Self {
            UpdateRegisterDateSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateTitleSoapIn {
        #[yaserde(rename = "tns:UpdateTitle", default)]
        pub body: ports::UpdateTitleSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateTitleSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateTitleSoapIn,
    }

    impl UpdateTitleSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateTitleSoapIn) -> Self {
            UpdateTitleSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateTitleSoapOut {
        #[yaserde(rename = "UpdateTitleResponse", default)]
        pub body: Option<ports::UpdateTitleSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateTitleSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateTitleSoapOut,
    }

    impl UpdateTitleSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateTitleSoapOut) -> Self {
            UpdateTitleSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateUserNameSoapIn {
        #[yaserde(rename = "tns:UpdateUserName", default)]
        pub body: ports::UpdateUserNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateUserNameSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateUserNameSoapIn,
    }

    impl UpdateUserNameSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateUserNameSoapIn) -> Self {
            UpdateUserNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapUpdateUserNameSoapOut {
        #[yaserde(rename = "UpdateUserNameResponse", default)]
        pub body: Option<ports::UpdateUserNameSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct UpdateUserNameSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapUpdateUserNameSoapOut,
    }

    impl UpdateUserNameSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapUpdateUserNameSoapOut) -> Self {
            UpdateUserNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for CustMultimediaSoap {
        fn default() -> Self {
            CustMultimediaSoap {
                client: reqwest::Client::new(),
                url: "http://ws.db.ccmm.applications.nortel.com".to_string(),
                credentials: None,
            }
        }
    }
    impl CustMultimediaSoap {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            CustMultimediaSoap {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct CustMultimediaSoap {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::CustMultimediaSoap for CustMultimediaSoap {
        async fn add_address(
            &self,
            add_address_soap_in: ports::AddAddressSoapIn,
        ) -> Result<ports::AddAddressSoapOut, Option<SoapFault>> {
            let __request = AddAddressSoapInSoapEnvelope::new(SoapAddAddressSoapIn {
                body: add_address_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddAddress",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddAddressSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn add_contact(
            &self,
            add_contact_soap_in: ports::AddContactSoapIn,
        ) -> Result<ports::AddContactSoapOut, Option<SoapFault>> {
            let __request = AddContactSoapInSoapEnvelope::new(SoapAddContactSoapIn {
                body: add_contact_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddContact",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddContactSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn add_custom_field(
            &self,
            add_custom_field_soap_in: ports::AddCustomFieldSoapIn,
        ) -> Result<ports::AddCustomFieldSoapOut, Option<SoapFault>> {
            let __request = AddCustomFieldSoapInSoapEnvelope::new(SoapAddCustomFieldSoapIn {
                body: add_custom_field_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddCustomField",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddCustomFieldSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn add_email_address(
            &self,
            add_email_address_soap_in: ports::AddEmailAddressSoapIn,
        ) -> Result<ports::AddEmailAddressSoapOut, Option<SoapFault>> {
            let __request = AddEmailAddressSoapInSoapEnvelope::new(SoapAddEmailAddressSoapIn {
                body: add_email_address_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddEmailAddress",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddEmailAddressSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn add_phone_number(
            &self,
            add_phone_number_soap_in: ports::AddPhoneNumberSoapIn,
        ) -> Result<ports::AddPhoneNumberSoapOut, Option<SoapFault>> {
            let __request = AddPhoneNumberSoapInSoapEnvelope::new(SoapAddPhoneNumberSoapIn {
                body: add_phone_number_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddPhoneNumber",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddPhoneNumberSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn add_sip_uri(
            &self,
            add_sip_uri_soap_in: ports::AddSipUriSoapIn,
        ) -> Result<ports::AddSipUriSoapOut, Option<SoapFault>> {
            let __request = AddSipUriSoapInSoapEnvelope::new(SoapAddSipUriSoapIn {
                body: add_sip_uri_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.AddSipUri",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: AddSipUriSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn carbon_copy(
            &self,
            carbon_copy_soap_in: ports::CarbonCopySoapIn,
        ) -> Result<ports::CarbonCopySoapOut, Option<SoapFault>> {
            let __request = CarbonCopySoapInSoapEnvelope::new(SoapCarbonCopySoapIn {
                body: carbon_copy_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CarbonCopy",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CarbonCopySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn clean_customer(
            &self,
            clean_customer_soap_in: ports::CleanCustomerSoapIn,
        ) -> Result<ports::CleanCustomerSoapOut, Option<SoapFault>> {
            let __request = CleanCustomerSoapInSoapEnvelope::new(SoapCleanCustomerSoapIn {
                body: clean_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CleanCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CleanCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn create_customer(
            &self,
            create_customer_soap_in: ports::CreateCustomerSoapIn,
        ) -> Result<ports::CreateCustomerSoapOut, Option<SoapFault>> {
            let __request = CreateCustomerSoapInSoapEnvelope::new(SoapCreateCustomerSoapIn {
                body: create_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CreateCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CreateCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn create_customer_by_sip_uri(
            &self,
            create_customer_by_sip_uri_soap_in: ports::CreateCustomerBySipUriSoapIn,
        ) -> Result<ports::CreateCustomerBySipUriSoapOut, Option<SoapFault>> {
            let __request = CreateCustomerBySipUriSoapInSoapEnvelope::new(SoapCreateCustomerBySipUriSoapIn {
                body: create_customer_by_sip_uri_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CreateCustomerBySipUri",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CreateCustomerBySipUriSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn customer_association(
            &self,
            customer_association_soap_in: ports::CustomerAssociationSoapIn,
        ) -> Result<ports::CustomerAssociationSoapOut, Option<SoapFault>> {
            let __request = CustomerAssociationSoapInSoapEnvelope::new(SoapCustomerAssociationSoapIn {
                body: customer_association_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CustomerAssociation",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CustomerAssociationSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn customer_search(
            &self,
            customer_search_soap_in: ports::CustomerSearchSoapIn,
        ) -> Result<ports::CustomerSearchSoapOut, Option<SoapFault>> {
            let __request = CustomerSearchSoapInSoapEnvelope::new(SoapCustomerSearchSoapIn {
                body: customer_search_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.CustomerSearch",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: CustomerSearchSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn delete_customer(
            &self,
            delete_customer_soap_in: ports::DeleteCustomerSoapIn,
        ) -> Result<ports::DeleteCustomerSoapOut, Option<SoapFault>> {
            let __request = DeleteCustomerSoapInSoapEnvelope::new(SoapDeleteCustomerSoapIn {
                body: delete_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.DeleteCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: DeleteCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_all_customers(
            &self,
            get_all_customers_soap_in: ports::GetAllCustomersSoapIn,
        ) -> Result<ports::GetAllCustomersSoapOut, Option<SoapFault>> {
            let __request = GetAllCustomersSoapInSoapEnvelope::new(SoapGetAllCustomersSoapIn {
                body: get_all_customers_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetAllCustomers",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetAllCustomersSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_cust_sql_columns(
            &self,
            get_cust_sql_columns_soap_in: ports::GetCustSQLColumnsSoapIn,
        ) -> Result<ports::GetCustSQLColumnsSoapOut, Option<SoapFault>> {
            let __request = GetCustSQLColumnsSoapInSoapEnvelope::new(SoapGetCustSQLColumnsSoapIn {
                body: get_cust_sql_columns_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustSQLColumns",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustSQLColumnsSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_custom_field_templates(
            &self,
            get_custom_field_templates_soap_in: ports::GetCustomFieldTemplatesSoapIn,
        ) -> Result<ports::GetCustomFieldTemplatesSoapOut, Option<SoapFault>> {
            let __request = GetCustomFieldTemplatesSoapInSoapEnvelope::new(SoapGetCustomFieldTemplatesSoapIn {
                body: get_custom_field_templates_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomFieldTemplates",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomFieldTemplatesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_contact_id(
            &self,
            get_customer_by_contact_id_soap_in: ports::GetCustomerByContactIdSoapIn,
        ) -> Result<ports::GetCustomerByContactIdSoapOut, Option<SoapFault>> {
            let __request = GetCustomerByContactIdSoapInSoapEnvelope::new(SoapGetCustomerByContactIdSoapIn {
                body: get_customer_by_contact_id_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByContactId",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerByContactIdSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_email(
            &self,
            get_customer_by_email_soap_in: ports::GetCustomerByEmailSoapIn,
        ) -> Result<ports::GetCustomerByEmailSoapOut, Option<SoapFault>> {
            let __request = GetCustomerByEmailSoapInSoapEnvelope::new(SoapGetCustomerByEmailSoapIn {
                body: get_customer_by_email_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByEmail",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerByEmailSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_name(
            &self,
            get_customer_by_name_soap_in: ports::GetCustomerByNameSoapIn,
        ) -> Result<ports::GetCustomerByNameSoapOut, Option<SoapFault>> {
            let __request = GetCustomerByNameSoapInSoapEnvelope::new(SoapGetCustomerByNameSoapIn {
                body: get_customer_by_name_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerByNameSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_phone_number(
            &self,
            get_customer_by_phone_number_soap_in: ports::GetCustomerByPhoneNumberSoapIn,
        ) -> Result<ports::GetCustomerByPhoneNumberSoapOut, Option<SoapFault>> {
            let __request = GetCustomerByPhoneNumberSoapInSoapEnvelope::new(SoapGetCustomerByPhoneNumberSoapIn {
                body: get_customer_by_phone_number_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByPhoneNumber",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerByPhoneNumberSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_sip_uri(
            &self,
            get_customer_by_sip_uri_soap_in: ports::GetCustomerBySipUriSoapIn,
        ) -> Result<ports::GetCustomerBySipUriSoapOut, Option<SoapFault>> {
            let __request = GetCustomerBySipUriSoapInSoapEnvelope::new(SoapGetCustomerBySipUriSoapIn {
                body: get_customer_by_sip_uri_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerBySipUri",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerBySipUriSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_customer_by_user_name(
            &self,
            get_customer_by_user_name_soap_in: ports::GetCustomerByUserNameSoapIn,
        ) -> Result<ports::GetCustomerByUserNameSoapOut, Option<SoapFault>> {
            let __request = GetCustomerByUserNameSoapInSoapEnvelope::new(SoapGetCustomerByUserNameSoapIn {
                body: get_customer_by_user_name_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetCustomerByUserName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCustomerByUserNameSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_no_cust_contacts_by_time(
            &self,
            get_no_cust_contacts_by_time_soap_in: ports::GetNoCustContactsByTimeSoapIn,
        ) -> Result<ports::GetNoCustContactsByTimeSoapOut, Option<SoapFault>> {
            let __request = GetNoCustContactsByTimeSoapInSoapEnvelope::new(SoapGetNoCustContactsByTimeSoapIn {
                body: get_no_cust_contacts_by_time_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetNoCustContactsByTime",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetNoCustContactsByTimeSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_searchable_fields(
            &self,
            get_searchable_fields_soap_in: ports::GetSearchableFieldsSoapIn,
        ) -> Result<ports::GetSearchableFieldsSoapOut, Option<SoapFault>> {
            let __request = GetSearchableFieldsSoapInSoapEnvelope::new(SoapGetSearchableFieldsSoapIn {
                body: get_searchable_fields_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.GetSearchableFields",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetSearchableFieldsSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn impersonate_customer(
            &self,
            impersonate_customer_soap_in: ports::ImpersonateCustomerSoapIn,
        ) -> Result<ports::ImpersonateCustomerSoapOut, Option<SoapFault>> {
            let __request = ImpersonateCustomerSoapInSoapEnvelope::new(SoapImpersonateCustomerSoapIn {
                body: impersonate_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ImpersonateCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: ImpersonateCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn read_customer(
            &self,
            read_customer_soap_in: ports::ReadCustomerSoapIn,
        ) -> Result<ports::ReadCustomerSoapOut, Option<SoapFault>> {
            let __request = ReadCustomerSoapInSoapEnvelope::new(SoapReadCustomerSoapIn {
                body: read_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ReadCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: ReadCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn read_customer_history(
            &self,
            read_customer_history_soap_in: ports::ReadCustomerHistorySoapIn,
        ) -> Result<ports::ReadCustomerHistorySoapOut, Option<SoapFault>> {
            let __request = ReadCustomerHistorySoapInSoapEnvelope::new(SoapReadCustomerHistorySoapIn {
                body: read_customer_history_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.ReadCustomerHistory",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: ReadCustomerHistorySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn register_anonymous_customer(
            &self,
            register_anonymous_customer_soap_in: ports::RegisterAnonymousCustomerSoapIn,
        ) -> Result<ports::RegisterAnonymousCustomerSoapOut, Option<SoapFault>> {
            let __request = RegisterAnonymousCustomerSoapInSoapEnvelope::new(SoapRegisterAnonymousCustomerSoapIn {
                body: register_anonymous_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RegisterAnonymousCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RegisterAnonymousCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn register_customer(
            &self,
            register_customer_soap_in: ports::RegisterCustomerSoapIn,
        ) -> Result<ports::RegisterCustomerSoapOut, Option<SoapFault>> {
            let __request = RegisterCustomerSoapInSoapEnvelope::new(SoapRegisterCustomerSoapIn {
                body: register_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RegisterCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RegisterCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_address(
            &self,
            remove_address_soap_in: ports::RemoveAddressSoapIn,
        ) -> Result<ports::RemoveAddressSoapOut, Option<SoapFault>> {
            let __request = RemoveAddressSoapInSoapEnvelope::new(SoapRemoveAddressSoapIn {
                body: remove_address_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveAddress",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemoveAddressSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_contact(
            &self,
            remove_contact_soap_in: ports::RemoveContactSoapIn,
        ) -> Result<ports::RemoveContactSoapOut, Option<SoapFault>> {
            let __request = RemoveContactSoapInSoapEnvelope::new(SoapRemoveContactSoapIn {
                body: remove_contact_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveContact",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemoveContactSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_custom_field(
            &self,
            remove_custom_field_soap_in: ports::RemoveCustomFieldSoapIn,
        ) -> Result<ports::RemoveCustomFieldSoapOut, Option<SoapFault>> {
            let __request = RemoveCustomFieldSoapInSoapEnvelope::new(SoapRemoveCustomFieldSoapIn {
                body: remove_custom_field_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveCustomField",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemoveCustomFieldSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_email_address(
            &self,
            remove_email_address_soap_in: ports::RemoveEmailAddressSoapIn,
        ) -> Result<ports::RemoveEmailAddressSoapOut, Option<SoapFault>> {
            let __request = RemoveEmailAddressSoapInSoapEnvelope::new(SoapRemoveEmailAddressSoapIn {
                body: remove_email_address_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveEmailAddress",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemoveEmailAddressSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_phone_number(
            &self,
            remove_phone_number_soap_in: ports::RemovePhoneNumberSoapIn,
        ) -> Result<ports::RemovePhoneNumberSoapOut, Option<SoapFault>> {
            let __request = RemovePhoneNumberSoapInSoapEnvelope::new(SoapRemovePhoneNumberSoapIn {
                body: remove_phone_number_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemovePhoneNumber",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemovePhoneNumberSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn remove_sip_uri(
            &self,
            remove_sip_uri_soap_in: ports::RemoveSipUriSoapIn,
        ) -> Result<ports::RemoveSipUriSoapOut, Option<SoapFault>> {
            let __request = RemoveSipUriSoapInSoapEnvelope::new(SoapRemoveSipUriSoapIn {
                body: remove_sip_uri_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.RemoveSipUri",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: RemoveSipUriSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn send_ad_password_reminder(
            &self,
            send_ad_password_reminder_soap_in: ports::SendADPasswordReminderSoapIn,
        ) -> Result<ports::SendADPasswordReminderSoapOut, Option<SoapFault>> {
            let __request = SendADPasswordReminderSoapInSoapEnvelope::new(SoapSendADPasswordReminderSoapIn {
                body: send_ad_password_reminder_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SendADPasswordReminder",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: SendADPasswordReminderSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn send_password_reminder(
            &self,
            send_password_reminder_soap_in: ports::SendPasswordReminderSoapIn,
        ) -> Result<ports::SendPasswordReminderSoapOut, Option<SoapFault>> {
            let __request = SendPasswordReminderSoapInSoapEnvelope::new(SoapSendPasswordReminderSoapIn {
                body: send_password_reminder_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SendPasswordReminder",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: SendPasswordReminderSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn set_agent_id(
            &self,
            set_agent_id_soap_in: ports::SetAgentIDSoapIn,
        ) -> Result<ports::SetAgentIDSoapOut, Option<SoapFault>> {
            let __request = SetAgentIDSoapInSoapEnvelope::new(SoapSetAgentIDSoapIn {
                body: set_agent_id_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.SetAgentID",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: SetAgentIDSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_customer(
            &self,
            update_customer_soap_in: ports::UpdateCustomerSoapIn,
        ) -> Result<ports::UpdateCustomerSoapOut, Option<SoapFault>> {
            let __request = UpdateCustomerSoapInSoapEnvelope::new(SoapUpdateCustomerSoapIn {
                body: update_customer_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateCustomer",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateCustomerSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_first_name(
            &self,
            update_first_name_soap_in: ports::UpdateFirstNameSoapIn,
        ) -> Result<ports::UpdateFirstNameSoapOut, Option<SoapFault>> {
            let __request = UpdateFirstNameSoapInSoapEnvelope::new(SoapUpdateFirstNameSoapIn {
                body: update_first_name_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateFirstName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateFirstNameSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_last_name(
            &self,
            update_last_name_soap_in: ports::UpdateLastNameSoapIn,
        ) -> Result<ports::UpdateLastNameSoapOut, Option<SoapFault>> {
            let __request = UpdateLastNameSoapInSoapEnvelope::new(SoapUpdateLastNameSoapIn {
                body: update_last_name_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateLastName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateLastNameSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_login_page(
            &self,
            update_login_page_soap_in: ports::UpdateLoginPageSoapIn,
        ) -> Result<ports::UpdateLoginPageSoapOut, Option<SoapFault>> {
            let __request = UpdateLoginPageSoapInSoapEnvelope::new(SoapUpdateLoginPageSoapIn {
                body: update_login_page_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateLoginPage",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateLoginPageSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_objection(
            &self,
            update_objection_soap_in: ports::UpdateObjectionSoapIn,
        ) -> Result<ports::UpdateObjectionSoapOut, Option<SoapFault>> {
            let __request = UpdateObjectionSoapInSoapEnvelope::new(SoapUpdateObjectionSoapIn {
                body: update_objection_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateObjection",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateObjectionSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_password(
            &self,
            update_password_soap_in: ports::UpdatePasswordSoapIn,
        ) -> Result<ports::UpdatePasswordSoapOut, Option<SoapFault>> {
            let __request = UpdatePasswordSoapInSoapEnvelope::new(SoapUpdatePasswordSoapIn {
                body: update_password_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdatePassword",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdatePasswordSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_preferred_agent(
            &self,
            update_preferred_agent_soap_in: ports::UpdatePreferredAgentSoapIn,
        ) -> Result<ports::UpdatePreferredAgentSoapOut, Option<SoapFault>> {
            let __request = UpdatePreferredAgentSoapInSoapEnvelope::new(SoapUpdatePreferredAgentSoapIn {
                body: update_preferred_agent_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdatePreferredAgent",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdatePreferredAgentSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_register_date(
            &self,
            update_register_date_soap_in: ports::UpdateRegisterDateSoapIn,
        ) -> Result<ports::UpdateRegisterDateSoapOut, Option<SoapFault>> {
            let __request = UpdateRegisterDateSoapInSoapEnvelope::new(SoapUpdateRegisterDateSoapIn {
                body: update_register_date_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateRegisterDate",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateRegisterDateSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_title(
            &self,
            update_title_soap_in: ports::UpdateTitleSoapIn,
        ) -> Result<ports::UpdateTitleSoapOut, Option<SoapFault>> {
            let __request = UpdateTitleSoapInSoapEnvelope::new(SoapUpdateTitleSoapIn {
                body: update_title_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateTitle",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateTitleSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn update_user_name(
            &self,
            update_user_name_soap_in: ports::UpdateUserNameSoapIn,
        ) -> Result<ports::UpdateUserNameSoapOut, Option<SoapFault>> {
            let __request = UpdateUserNameSoapInSoapEnvelope::new(SoapUpdateUserNameSoapIn {
                body: update_user_name_soap_in,
                xmlns: Some("http://ws.db.ccmm.applications.nortel.com".to_string()),
            });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.db.ccmm.applications.nortel.com/ws.Customer.UpdateUserName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: UpdateUserNameSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    pub struct CustMultimedia {}
    impl CustMultimedia {
        #[must_use]
        pub fn new_client(credentials: Option<(String, String)>) -> bindings::CustMultimediaSoap {
            Self::new_client_with_url("https://aacc1ver7:443/csp/multimedia/ws.Customer.cls", credentials)
        }

        #[must_use]
        pub fn new_client_with_url(url: &str, credentials: Option<(String, String)>) -> bindings::CustMultimediaSoap {
            bindings::CustMultimediaSoap::new(url, credentials)
        }
    }
}

pub mod multiref {
    //! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
    //! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
    //! Needs `xml-rs` and `yaserde` as dependencies.

    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Rc::new(RefCell::new(inner)),
            })
        }
    }

    impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.as_ref().borrow().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
            self.inner.as_ref().borrow().serialize_attributes(attributes, namespace)
        }
    }

    impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self { inner: Rc::default() }
        }
    }

    impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.as_ref().borrow().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
