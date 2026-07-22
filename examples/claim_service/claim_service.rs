//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.2.5
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use std::{
    io::{Read, Write},
    rc::Rc,
};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_cla {
    use super::*;
    use restrictions::CheckRestrictions;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "GetClaim")]
    pub struct GetClaim {
        #[yaserde(prefix = "cla2", rename = "getClaimRequest")]
        pub get_claim_request: Option<mod_cla2::GetClaimRequest>,
    }
    impl restrictions::CheckRestrictions for GetClaim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_claim_request.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "GetClaimResponse")]
    pub struct GetClaimResponse {
        #[yaserde(prefix = "cla2", rename = "GetClaimResult")]
        pub get_claim_result: Option<mod_cla2::GetClaimResult>,
    }
    impl restrictions::CheckRestrictions for GetClaimResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_claim_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ServiceSecurityHeader")]
    pub struct ServiceSecurityHeader {
        #[yaserde(prefix = "cla", rename = "Token")]
        pub token: Option<String>,
        #[yaserde(prefix = "cla", rename = "AdminAlias")]
        pub admin_alias: Option<String>,
        #[yaserde(prefix = "cla", rename = "NewClaimsLinkSystemResourceId")]
        pub new_claims_link_system_resource_id: Option<i16>,
    }
    impl restrictions::CheckRestrictions for ServiceSecurityHeader {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.token.check_restrictions(restrictions.clone())?;
            self.admin_alias.check_restrictions(restrictions.clone())?;
            self.new_claims_link_system_resource_id
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimBasket")]
    pub struct SubmitClaimBasket {
        #[yaserde(prefix = "cla2", rename = "submitClaimBasketRequest")]
        pub submit_claim_basket_request: Option<mod_cla2::SubmitClaimBasketRequest>,
    }
    impl restrictions::CheckRestrictions for SubmitClaimBasket {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_basket_request
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimBasketResponse")]
    pub struct SubmitClaimBasketResponse {
        #[yaserde(prefix = "cla2", rename = "SubmitClaimBasketResult")]
        pub submit_claim_basket_result: Option<mod_cla2::SubmitClaimBasketResult>,
    }
    impl restrictions::CheckRestrictions for SubmitClaimBasketResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_basket_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpenseBasket")]
    pub struct SubmitExpenseBasket {
        #[yaserde(prefix = "cla2", rename = "submitClaimBasketRequest")]
        pub submit_claim_basket_request: Option<mod_cla2::SubmitClaimBasketRequest>,
    }
    impl restrictions::CheckRestrictions for SubmitExpenseBasket {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_basket_request
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpenseBasketResponse")]
    pub struct SubmitExpenseBasketResponse {
        #[yaserde(prefix = "cla2", rename = "SubmitExpenseBasketResult")]
        pub submit_expense_basket_result: Option<mod_cla2::SubmitExpenseBasketResult>,
    }
    impl restrictions::CheckRestrictions for SubmitExpenseBasketResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_expense_basket_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpenses")]
    pub struct SubmitExpenses {
        #[yaserde(prefix = "cla2", rename = "submitClaimBasketRequest")]
        pub submit_claim_basket_request: Option<mod_cla2::SubmitClaimBasketRequest>,
    }
    impl restrictions::CheckRestrictions for SubmitExpenses {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_basket_request
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpensesResponse")]
    pub struct SubmitExpensesResponse {
        #[yaserde(prefix = "cla2", rename = "SubmitExpensesResult")]
        pub submit_expenses_result: Option<mod_cla2::SubmitExpensesResult>,
    }
    impl restrictions::CheckRestrictions for SubmitExpensesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_expenses_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpenseNoAdvice")]
    pub struct SubmitExpenseNoAdvice {
        #[yaserde(prefix = "cla2", rename = "submitClaimBasketRequest")]
        pub submit_claim_basket_request: Option<mod_cla2::SubmitClaimBasketRequest>,
    }
    impl restrictions::CheckRestrictions for SubmitExpenseNoAdvice {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_basket_request
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd"}, rename = "SubmitExpenseNoAdviceResponse")]
    pub struct SubmitExpenseNoAdviceResponse {
        #[yaserde(prefix = "cla1", rename = "SubmitExpenseNoAdviceResult")]
        pub submit_expense_no_advice_result: Option<mod_cla1::SubmitExpenseNoAdviceResult>,
    }
    impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_expense_no_advice_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ApproveExpenses")]
    pub struct ApproveExpenses {
        #[yaserde(prefix = "cla2", rename = "submitClaimWorksheetRequest")]
        pub submit_claim_worksheet_request: Option<mod_cla2::SubmitClaimWorksheetRequest>,
    }
    impl restrictions::CheckRestrictions for ApproveExpenses {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.submit_claim_worksheet_request
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ApproveExpensesResponse")]
    pub struct ApproveExpensesResponse {
        #[yaserde(prefix = "cla2", rename = "ApproveExpensesResult")]
        pub approve_expenses_result: Option<mod_cla2::ApproveExpensesResult>,
    }
    impl restrictions::CheckRestrictions for ApproveExpensesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.approve_expenses_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetMileageRates")]
    pub struct GetMileageRates {}
    impl restrictions::CheckRestrictions for GetMileageRates {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetMileageRatesResponse")]
    pub struct GetMileageRatesResponse {
        #[yaserde(prefix = "cla", rename = "GetMileageRatesResult")]
        pub get_mileage_rates_result: Option<mod_cla::ArrayOfMileageRate>,
    }
    impl restrictions::CheckRestrictions for GetMileageRatesResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_mileage_rates_result.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfMileageRate")]
    pub struct ArrayOfMileageRate {
        #[yaserde(prefix = "cla", rename = "MileageRate")]
        pub mileage_rate: Vec<mod_cla::MileageRate>,
    }
    impl restrictions::CheckRestrictions for ArrayOfMileageRate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.mileage_rate.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "MileageRate")]
    pub struct MileageRate {
        #[yaserde(prefix = "cla", rename = "MileageRateValue")]
        pub mileage_rate_value: f64,
        #[yaserde(prefix = "cla", rename = "EffectiveDate")]
        pub effective_date: String,
    }
    impl restrictions::CheckRestrictions for MileageRate {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.mileage_rate_value.check_restrictions(restrictions.clone())?;
            self.effective_date.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetPlanMaxAmounts")]
    pub struct GetPlanMaxAmounts {}
    impl restrictions::CheckRestrictions for GetPlanMaxAmounts {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetPlanMaxAmountsResponse")]
    pub struct GetPlanMaxAmountsResponse {
        #[yaserde(prefix = "cla", rename = "GetPlanMaxAmountsResult")]
        pub get_plan_max_amounts_result: Option<mod_cla::ArrayOfPlanMaxAmount>,
    }
    impl restrictions::CheckRestrictions for GetPlanMaxAmountsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_plan_max_amounts_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfPlanMaxAmount")]
    pub struct ArrayOfPlanMaxAmount {
        #[yaserde(prefix = "cla", rename = "PlanMaxAmount")]
        pub plan_max_amount: Vec<mod_cla::PlanMaxAmount>,
    }
    impl restrictions::CheckRestrictions for ArrayOfPlanMaxAmount {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.plan_max_amount.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "PlanMaxAmount")]
    pub struct PlanMaxAmount {
        #[yaserde(prefix = "cla", rename = "PlanType")]
        pub plan_type: mod_cla::PlanType,
        #[yaserde(prefix = "cla", rename = "MaxAmount")]
        pub max_amount: f64,
        #[yaserde(prefix = "cla", rename = "EffectiveDate")]
        pub effective_date: String,
    }
    impl restrictions::CheckRestrictions for PlanMaxAmount {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.plan_type.check_restrictions(restrictions.clone())?;
            self.max_amount.check_restrictions(restrictions.clone())?;
            self.effective_date.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "PlanType")]
    pub struct PlanType {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for PlanType {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec![
                    "MedicalFlex".to_string(),
                    "DependentCare".to_string(),
                    "HRA".to_string(),
                    "HRADeductibles".to_string(),
                    "HRADental".to_string(),
                    "HRAVision".to_string(),
                    "HSA".to_string(),
                    "IndividualPremium".to_string(),
                    "MassTransit".to_string(),
                    "Parking".to_string(),
                    "VEBA".to_string(),
                    "DefinedContribution".to_string(),
                ]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetLifetimeMaximumInformation")]
    pub struct GetLifetimeMaximumInformation {
        #[yaserde(prefix = "cla", rename = "individualId")]
        pub individual_id: i32,
    }
    impl restrictions::CheckRestrictions for GetLifetimeMaximumInformation {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.individual_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetLifetimeMaximumInformationResponse")]
    pub struct GetLifetimeMaximumInformationResponse {
        #[yaserde(prefix = "cla", rename = "GetLifetimeMaximumInformationResult")]
        pub get_lifetime_maximum_information_result: Option<mod_cla::LifetimeMaximumStorage>,
    }
    impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_lifetime_maximum_information_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "LifetimeMaximumStorage")]
    pub struct LifetimeMaximumStorage {
        #[yaserde(prefix = "cla", rename = "tempProperties")]
        pub temp_properties: Option<mod_cla::ArrayOfSerializableKeyValuePairOfInt32String>,
        #[yaserde(prefix = "cla", rename = "Dependents")]
        pub dependents: Option<mod_cla::ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail>,
        #[yaserde(prefix = "cla", rename = "Participants")]
        pub participants: Option<mod_cla::ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail>,
        #[yaserde(prefix = "cla", rename = "LifetimeMaximums")]
        pub lifetime_maximums: Option<mod_cla::ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximum>,
        #[yaserde(prefix = "cla", rename = "OfferingsWithoutLifetimeMaximum")]
        pub offerings_without_lifetime_maximum: Option<mod_cla::ArrayOfInt>,
    }
    impl restrictions::CheckRestrictions for LifetimeMaximumStorage {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.temp_properties.check_restrictions(restrictions.clone())?;
            self.dependents.check_restrictions(restrictions.clone())?;
            self.participants.check_restrictions(restrictions.clone())?;
            self.lifetime_maximums.check_restrictions(restrictions.clone())?;
            self.offerings_without_lifetime_maximum
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfSerializableKeyValuePairOfInt32String")]
    pub struct ArrayOfSerializableKeyValuePairOfInt32String {
        #[yaserde(prefix = "cla", rename = "SerializableKeyValuePairOfInt32String")]
        pub serializable_key_value_pair_of_int_32_string: Vec<mod_cla::SerializableKeyValuePairOfInt32String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSerializableKeyValuePairOfInt32String {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.serializable_key_value_pair_of_int_32_string
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "SerializableKeyValuePairOfInt32String")]
    pub struct SerializableKeyValuePairOfInt32String {
        #[yaserde(prefix = "cla", rename = "Key")]
        pub key: i32,
        #[yaserde(prefix = "cla", rename = "Value")]
        pub value: Option<String>,
    }
    impl restrictions::CheckRestrictions for SerializableKeyValuePairOfInt32String {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.key.check_restrictions(restrictions.clone())?;
            self.value.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail")]
    pub struct ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail {
        #[yaserde(
            prefix = "cla",
            rename = "SerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail"
        )]
        pub serializable_key_value_pair_of_int_32_lifetime_maximum_recipient_detail:
            Vec<mod_cla::SerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.serializable_key_value_pair_of_int_32_lifetime_maximum_recipient_detail
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "SerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail")]
    pub struct SerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail {
        #[yaserde(prefix = "cla", rename = "Key")]
        pub key: i32,
        #[yaserde(prefix = "cla", rename = "Value")]
        pub value: Option<mod_cla::LifetimeMaximumRecipientDetail>,
    }
    impl restrictions::CheckRestrictions for SerializableKeyValuePairOfInt32LifetimeMaximumRecipientDetail {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.key.check_restrictions(restrictions.clone())?;
            self.value.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "LifetimeMaximumRecipientDetail")]
    pub struct LifetimeMaximumRecipientDetail {
        #[yaserde(prefix = "cla", rename = "RecipientId")]
        pub recipient_id: i32,
        #[yaserde(prefix = "cla", rename = "RecipientType")]
        pub recipient_type: mod_cla::RecipientType,
        #[yaserde(prefix = "cla", rename = "FirstName")]
        pub first_name: Option<String>,
        #[yaserde(prefix = "cla", rename = "LastName")]
        pub last_name: Option<String>,
        #[yaserde(prefix = "cla", rename = "ListOfPlanBalance")]
        pub list_of_plan_balance: Option<mod_cla::ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance>,
    }
    impl restrictions::CheckRestrictions for LifetimeMaximumRecipientDetail {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.recipient_id.check_restrictions(restrictions.clone())?;
            self.recipient_type.check_restrictions(restrictions.clone())?;
            self.first_name.check_restrictions(restrictions.clone())?;
            self.last_name.check_restrictions(restrictions.clone())?;
            self.list_of_plan_balance.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "RecipientType")]
    pub struct RecipientType {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for RecipientType {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec![
                    "Individual".to_string(),
                    "Dependent".to_string(),
                    "External".to_string(),
                    "Provider".to_string(),
                    "Unspecified".to_string(),
                ]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance")]
    pub struct ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance {
        #[yaserde(prefix = "cla", rename = "SerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance")]
        pub serializable_key_value_pair_of_int_32_lifetime_maximum_plan_balance:
            Vec<mod_cla::SerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.serializable_key_value_pair_of_int_32_lifetime_maximum_plan_balance
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "SerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance")]
    pub struct SerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance {
        #[yaserde(prefix = "cla", rename = "Key")]
        pub key: i32,
        #[yaserde(prefix = "cla", rename = "Value")]
        pub value: Option<mod_cla::LifetimeMaximumPlanBalance>,
    }
    impl restrictions::CheckRestrictions for SerializableKeyValuePairOfInt32LifetimeMaximumPlanBalance {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.key.check_restrictions(restrictions.clone())?;
            self.value.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "LifetimeMaximumPlanBalance")]
    pub struct LifetimeMaximumPlanBalance {
        #[yaserde(prefix = "cla", rename = "OfferingId")]
        pub offering_id: i32,
        #[yaserde(prefix = "cla", rename = "Submitted")]
        pub submitted: f64,
        #[yaserde(prefix = "cla", rename = "Paid")]
        pub paid: f64,
        #[yaserde(prefix = "cla", rename = "Pending")]
        pub pending: f64,
        #[yaserde(prefix = "cla", rename = "Denied")]
        pub denied: f64,
        #[yaserde(prefix = "cla", rename = "DynamicApprovedAmount")]
        pub dynamic_approved_amount: f64,
        #[yaserde(prefix = "cla", rename = "DynamicProposedAmount")]
        pub dynamic_proposed_amount: f64,
    }
    impl restrictions::CheckRestrictions for LifetimeMaximumPlanBalance {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.offering_id.check_restrictions(restrictions.clone())?;
            self.submitted.check_restrictions(restrictions.clone())?;
            self.paid.check_restrictions(restrictions.clone())?;
            self.pending.check_restrictions(restrictions.clone())?;
            self.denied.check_restrictions(restrictions.clone())?;
            self.dynamic_approved_amount.check_restrictions(restrictions.clone())?;
            self.dynamic_proposed_amount.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximum")]
    pub struct ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximum {
        #[yaserde(prefix = "cla", rename = "SerializableKeyValuePairOfInt32LifetimeMaximum")]
        pub serializable_key_value_pair_of_int_32_lifetime_maximum:
            Vec<mod_cla::SerializableKeyValuePairOfInt32LifetimeMaximum>,
    }
    impl restrictions::CheckRestrictions for ArrayOfSerializableKeyValuePairOfInt32LifetimeMaximum {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.serializable_key_value_pair_of_int_32_lifetime_maximum
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "SerializableKeyValuePairOfInt32LifetimeMaximum")]
    pub struct SerializableKeyValuePairOfInt32LifetimeMaximum {
        #[yaserde(prefix = "cla", rename = "Key")]
        pub key: i32,
        #[yaserde(prefix = "cla", rename = "Value")]
        pub value: Option<mod_cla::LifetimeMaximum>,
    }
    impl restrictions::CheckRestrictions for SerializableKeyValuePairOfInt32LifetimeMaximum {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.key.check_restrictions(restrictions.clone())?;
            self.value.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "LifetimeMaximum")]
    pub struct LifetimeMaximum {
        #[yaserde(prefix = "cla", rename = "LifetimeMaximumID")]
        pub lifetime_maximum_id: i32,
        #[yaserde(prefix = "cla", rename = "SystemName")]
        pub system_name: Option<String>,
        #[yaserde(prefix = "cla", rename = "MaxAmount")]
        pub max_amount: f64,
        #[yaserde(prefix = "cla", rename = "Active")]
        pub active: bool,
        #[yaserde(prefix = "cla", rename = "EmployerId")]
        pub employer_id: i32,
        #[yaserde(prefix = "cla", rename = "StatusChangedDate")]
        pub status_changed_date: String,
        #[yaserde(prefix = "cla", rename = "AllPlans")]
        pub all_plans: bool,
        #[yaserde(prefix = "cla", rename = "OfferingIds")]
        pub offering_ids: Option<mod_cla::ArrayOfInt>,
    }
    impl restrictions::CheckRestrictions for LifetimeMaximum {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.lifetime_maximum_id.check_restrictions(restrictions.clone())?;
            self.system_name.check_restrictions(restrictions.clone())?;
            self.max_amount.check_restrictions(restrictions.clone())?;
            self.active.check_restrictions(restrictions.clone())?;
            self.employer_id.check_restrictions(restrictions.clone())?;
            self.status_changed_date.check_restrictions(restrictions.clone())?;
            self.all_plans.check_restrictions(restrictions.clone())?;
            self.offering_ids.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfInt")]
    pub struct ArrayOfInt {
        #[yaserde(prefix = "cla", rename = "int")]
        pub int: Vec<i32>,
    }
    impl restrictions::CheckRestrictions for ArrayOfInt {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.int.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetExpenseApprovedAmount")]
    pub struct GetExpenseApprovedAmount {
        #[yaserde(prefix = "cla", rename = "individualId")]
        pub individual_id: i32,
        #[yaserde(prefix = "cla", rename = "offeringId")]
        pub offering_id: i32,
        #[yaserde(prefix = "cla", rename = "eligibleExpenseId")]
        pub eligible_expense_id: i32,
    }
    impl restrictions::CheckRestrictions for GetExpenseApprovedAmount {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.individual_id.check_restrictions(restrictions.clone())?;
            self.offering_id.check_restrictions(restrictions.clone())?;
            self.eligible_expense_id.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetExpenseApprovedAmountResponse")]
    pub struct GetExpenseApprovedAmountResponse {
        #[yaserde(prefix = "cla", rename = "GetExpenseApprovedAmountResult")]
        pub get_expense_approved_amount_result: f64,
    }
    impl restrictions::CheckRestrictions for GetExpenseApprovedAmountResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_expense_approved_amount_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetBaseExpenseAccounts")]
    pub struct GetBaseExpenseAccounts {}
    impl restrictions::CheckRestrictions for GetBaseExpenseAccounts {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "GetBaseExpenseAccountsResponse")]
    pub struct GetBaseExpenseAccountsResponse {
        #[yaserde(prefix = "cla", rename = "GetBaseExpenseAccountsResult")]
        pub get_base_expense_accounts_result: Option<mod_cla::ArrayOfExpenseAccountBase>,
    }
    impl restrictions::CheckRestrictions for GetBaseExpenseAccountsResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.get_base_expense_accounts_result
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla", namespaces = {"cla" = "http://services.lighthouse1.com/services/claim/"}, rename = "ArrayOfExpenseAccountBase")]
    pub struct ArrayOfExpenseAccountBase {
        #[yaserde(prefix = "cla", rename = "ExpenseAccountBase")]
        pub expense_account_base: Vec<mod_cla2::ExpenseAccountBase>,
    }
    impl restrictions::CheckRestrictions for ArrayOfExpenseAccountBase {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.expense_account_base.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
}
pub mod mod_cla2 {
    use super::*;
    use restrictions::CheckRestrictions;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "GetClaimRequest")]
    pub struct GetClaimRequest {
        #[yaserde(prefix = "cla2", rename = "ClaimNumber")]
        pub claim_number: Vec<String>,
    }
    impl restrictions::CheckRestrictions for GetClaimRequest {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim_number.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    pub type GetClaimResult = mod_cla2::GetClaimResponse;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "GetClaimResponse")]
    pub struct GetClaimResponse {
        #[yaserde(prefix = "cla2", rename = "Claim")]
        pub claim: Vec<mod_cla2::Claim>,
    }
    impl restrictions::CheckRestrictions for GetClaimResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "Claim")]
    pub struct Claim {
        #[yaserde(prefix = "cla2", rename = "ClaimAccount")]
        pub claim_account: Option<mod_cla2::ClaimAccount>,
        #[yaserde(prefix = "cla2", rename = "ClaimAmount")]
        pub claim_amount: f64,
        #[yaserde(prefix = "cla2", rename = "ClaimNotes")]
        pub claim_notes: Option<mod_cla2::ArrayOfString>,
        #[yaserde(prefix = "cla2", rename = "ClaimStatus")]
        pub claim_status: mod_cla2::ClaimStatus,
        #[yaserde(prefix = "cla2", rename = "DenialCode")]
        pub denial_code: Option<i32>,
        #[yaserde(prefix = "cla2", rename = "MultipleDenialCodes")]
        pub multiple_denial_codes: Option<mod_cla2::ArrayOfInt>,
        #[yaserde(prefix = "cla2", rename = "HasValidReceipt")]
        pub has_valid_receipt: bool,
        #[yaserde(prefix = "cla2", rename = "IsDuplicate")]
        pub is_duplicate: Option<bool>,
        #[yaserde(prefix = "cla2", rename = "MileageAmount")]
        pub mileage_amount: i32,
        #[yaserde(prefix = "cla2", rename = "MileageRate")]
        pub mileage_rate: f64,
        #[yaserde(prefix = "cla2", rename = "OfferingID")]
        pub offering_id: i32,
        #[yaserde(prefix = "cla2", rename = "Provider")]
        pub provider: Option<mod_cla2::Provider>,
        #[yaserde(prefix = "cla2", rename = "ReceiptStatus")]
        pub receipt_status: Option<mod_cla2::ReceiptStatus>,
        #[yaserde(prefix = "cla2", rename = "Recipients")]
        pub recipients: Option<mod_cla2::ArrayOfServiceRecipient>,
        #[yaserde(prefix = "cla2", rename = "SendDenial")]
        pub send_denial: Option<bool>,
        #[yaserde(prefix = "cla2", rename = "CustomDenialText")]
        pub custom_denial_text: Option<String>,
        #[yaserde(prefix = "cla2", rename = "EligibleExpenseId")]
        pub eligible_expense_id: i32,
        #[yaserde(prefix = "cla2", rename = "ServiceCategory")]
        pub service_category: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ServiceDescription")]
        pub service_description: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ServiceEndDate")]
        pub service_end_date: String,
        #[yaserde(prefix = "cla2", rename = "ServiceRecipient")]
        pub service_recipient: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ServiceStartDate")]
        pub service_start_date: String,
        #[yaserde(prefix = "cla2", rename = "ServiceType")]
        pub service_type: Option<String>,
        #[yaserde(prefix = "cla2", rename = "SubmissionDate")]
        pub submission_date: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ImageId")]
        pub image_id: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ReceivedDate")]
        pub received_date: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ReceivedDateSpecified")]
        pub received_date_specified: bool,
        #[yaserde(prefix = "cla2", rename = "MileageRealAmount")]
        pub mileage_real_amount: f64,
        #[yaserde(prefix = "cla2", rename = "AmountOwed")]
        pub amount_owed: Option<f64>,
        #[yaserde(prefix = "cla2", rename = "ClaimIdentifier", attribute = true)]
        pub claim_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimNumber", attribute = true)]
        pub claim_number: Option<String>,
    }
    impl restrictions::CheckRestrictions for Claim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim_account.check_restrictions(restrictions.clone())?;
            self.claim_amount.check_restrictions(restrictions.clone())?;
            self.claim_notes.check_restrictions(restrictions.clone())?;
            self.claim_status.check_restrictions(restrictions.clone())?;
            self.denial_code.check_restrictions(restrictions.clone())?;
            self.multiple_denial_codes.check_restrictions(restrictions.clone())?;
            self.has_valid_receipt.check_restrictions(restrictions.clone())?;
            self.is_duplicate.check_restrictions(restrictions.clone())?;
            self.mileage_amount.check_restrictions(restrictions.clone())?;
            self.mileage_rate.check_restrictions(restrictions.clone())?;
            self.offering_id.check_restrictions(restrictions.clone())?;
            self.provider.check_restrictions(restrictions.clone())?;
            self.receipt_status.check_restrictions(restrictions.clone())?;
            self.recipients.check_restrictions(restrictions.clone())?;
            self.send_denial.check_restrictions(restrictions.clone())?;
            self.custom_denial_text.check_restrictions(restrictions.clone())?;
            self.eligible_expense_id.check_restrictions(restrictions.clone())?;
            self.service_category.check_restrictions(restrictions.clone())?;
            self.service_description.check_restrictions(restrictions.clone())?;
            self.service_end_date.check_restrictions(restrictions.clone())?;
            self.service_recipient.check_restrictions(restrictions.clone())?;
            self.service_start_date.check_restrictions(restrictions.clone())?;
            self.service_type.check_restrictions(restrictions.clone())?;
            self.submission_date.check_restrictions(restrictions.clone())?;
            self.image_id.check_restrictions(restrictions.clone())?;
            self.received_date.check_restrictions(restrictions.clone())?;
            self.received_date_specified.check_restrictions(restrictions.clone())?;
            self.mileage_real_amount.check_restrictions(restrictions.clone())?;
            self.amount_owed.check_restrictions(restrictions.clone())?;
            self.claim_identifier.check_restrictions(restrictions.clone())?;
            self.claim_number.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ClaimAccount")]
    pub struct ClaimAccount {
        #[yaserde(prefix = "cla2", rename = "ApprovedAmount")]
        pub approved_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeniedAmount")]
        pub denied_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PaidAmount")]
        pub paid_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PendingAmount")]
        pub pending_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CopayAmount")]
        pub copay_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CoinsuranceAmount")]
        pub coinsurance_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeductibleAmount")]
        pub deductible_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PostedAmount")]
        pub posted_amount: f64,
        #[yaserde(prefix = "cla2", rename = "SubmittedAmount")]
        pub submitted_amount: f64,
    }
    impl restrictions::CheckRestrictions for ClaimAccount {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.approved_amount.check_restrictions(restrictions.clone())?;
            self.denied_amount.check_restrictions(restrictions.clone())?;
            self.paid_amount.check_restrictions(restrictions.clone())?;
            self.pending_amount.check_restrictions(restrictions.clone())?;
            self.copay_amount.check_restrictions(restrictions.clone())?;
            self.coinsurance_amount.check_restrictions(restrictions.clone())?;
            self.deductible_amount.check_restrictions(restrictions.clone())?;
            self.posted_amount.check_restrictions(restrictions.clone())?;
            self.submitted_amount.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ArrayOfString")]
    pub struct ArrayOfString {
        #[yaserde(prefix = "cla2", rename = "Note")]
        pub note: Vec<String>,
    }
    impl restrictions::CheckRestrictions for ArrayOfString {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.note.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ClaimStatus")]
    pub struct ClaimStatus {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for ClaimStatus {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec![
                    "BankingFailedStatus".to_string(),
                    "DeniedCollectingRepaymentStatus".to_string(),
                    "DeniedStatus".to_string(),
                    "FreezeStatus".to_string(),
                    "HoldStatus".to_string(),
                    "PaidStatus".to_string(),
                    "PendingReceiptStatus".to_string(),
                    "PendingReimbursementStatus".to_string(),
                    "ScheduledReimbursementStatus".to_string(),
                    "Unspecified".to_string(),
                ]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ArrayOfInt")]
    pub struct ArrayOfInt {
        #[yaserde(prefix = "cla2", rename = "int")]
        pub int: Vec<i32>,
    }
    impl restrictions::CheckRestrictions for ArrayOfInt {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.int.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "Provider")]
    pub struct Provider {
        #[yaserde(prefix = "cla2", rename = "PTP")]
        pub ptp: bool,
        #[yaserde(prefix = "cla2", rename = "Id")]
        pub id: Option<i32>,
        #[yaserde(prefix = "cla2", rename = "Type")]
        pub r#type: u8,
        #[yaserde(prefix = "cla2", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "cla2", rename = "Number")]
        pub number: Option<String>,
        #[yaserde(prefix = "cla2", rename = "Tin")]
        pub tin: Option<String>,
        #[yaserde(prefix = "cla2", rename = "AccountNumber")]
        pub account_number: Option<String>,
        #[yaserde(prefix = "cla2", rename = "AddressLine1")]
        pub address_line_1: Option<String>,
        #[yaserde(prefix = "cla2", rename = "AddressLine2")]
        pub address_line_2: Option<String>,
        #[yaserde(prefix = "cla2", rename = "AddressLine3")]
        pub address_line_3: Option<String>,
        #[yaserde(prefix = "cla2", rename = "AddressLine4")]
        pub address_line_4: Option<String>,
        #[yaserde(prefix = "cla2", rename = "City")]
        pub city: Option<String>,
        #[yaserde(prefix = "cla2", rename = "StateCode")]
        pub state_code: Option<String>,
        #[yaserde(prefix = "cla2", rename = "CountryCode")]
        pub country_code: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ZipCode")]
        pub zip_code: Option<String>,
    }
    impl restrictions::CheckRestrictions for Provider {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.ptp.check_restrictions(restrictions.clone())?;
            self.id.check_restrictions(restrictions.clone())?;
            self.r#type.check_restrictions(restrictions.clone())?;
            self.name.check_restrictions(restrictions.clone())?;
            self.number.check_restrictions(restrictions.clone())?;
            self.tin.check_restrictions(restrictions.clone())?;
            self.account_number.check_restrictions(restrictions.clone())?;
            self.address_line_1.check_restrictions(restrictions.clone())?;
            self.address_line_2.check_restrictions(restrictions.clone())?;
            self.address_line_3.check_restrictions(restrictions.clone())?;
            self.address_line_4.check_restrictions(restrictions.clone())?;
            self.city.check_restrictions(restrictions.clone())?;
            self.state_code.check_restrictions(restrictions.clone())?;
            self.country_code.check_restrictions(restrictions.clone())?;
            self.zip_code.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ReceiptStatus")]
    pub struct ReceiptStatus {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for ReceiptStatus {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec![
                    "ReceiptRequired".to_string(),
                    "ReceiptReceived".to_string(),
                    "ReceiptOverdue".to_string(),
                    "ReceiptNotNeeded".to_string(),
                    "Unspecified".to_string(),
                ]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ArrayOfServiceRecipient")]
    pub struct ArrayOfServiceRecipient {
        #[yaserde(prefix = "cla2", rename = "ServiceRecipient")]
        pub service_recipient: Vec<mod_cla2::ServiceRecipient>,
    }
    impl restrictions::CheckRestrictions for ArrayOfServiceRecipient {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.service_recipient.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ServiceRecipient")]
    pub struct ServiceRecipient {
        #[yaserde(prefix = "cla2", rename = "Name")]
        pub name: Option<String>,
        #[yaserde(prefix = "cla2", rename = "RecipientID")]
        pub recipient_id: i32,
        #[yaserde(prefix = "cla2", rename = "RecipientType")]
        pub recipient_type: Option<String>,
    }
    impl restrictions::CheckRestrictions for ServiceRecipient {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.name.check_restrictions(restrictions.clone())?;
            self.recipient_id.check_restrictions(restrictions.clone())?;
            self.recipient_type.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimBasketRequest")]
    pub struct SubmitClaimBasketRequest {
        #[yaserde(prefix = "cla2", rename = "Administrator")]
        pub administrator: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimBasket")]
        pub claim_basket: Option<mod_cla2::ClaimBasket>,
        #[yaserde(prefix = "cla2", rename = "Employer")]
        pub employer: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ParticipantID")]
        pub participant_id: i32,
        #[yaserde(prefix = "cla2", rename = "SendConfirmation")]
        pub send_confirmation: bool,
        #[yaserde(prefix = "cla2", rename = "AllowPayToProvider")]
        pub allow_pay_to_provider: bool,
    }
    impl restrictions::CheckRestrictions for SubmitClaimBasketRequest {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.administrator.check_restrictions(restrictions.clone())?;
            self.claim_basket.check_restrictions(restrictions.clone())?;
            self.employer.check_restrictions(restrictions.clone())?;
            self.participant_id.check_restrictions(restrictions.clone())?;
            self.send_confirmation.check_restrictions(restrictions.clone())?;
            self.allow_pay_to_provider.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ClaimBasket")]
    pub struct ClaimBasket {
        #[yaserde(prefix = "cla2", rename = "Claim")]
        pub claim: Vec<mod_cla2::Claim>,
        #[yaserde(prefix = "cla2", rename = "BasketIdentifier", attribute = true)]
        pub basket_identifier: Option<String>,
    }
    impl restrictions::CheckRestrictions for ClaimBasket {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim.check_restrictions(restrictions.clone())?;
            self.basket_identifier.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    pub type SubmitClaimBasketResult = mod_cla2::SubmitClaimBasketResponse;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimBasketResponse")]
    pub struct SubmitClaimBasketResponse {
        #[yaserde(prefix = "cla2", rename = "BasketIdentifier")]
        pub basket_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "FiledClaims")]
        pub filed_claims: Option<mod_cla2::ArrayOfFiledClaim>,
    }
    impl restrictions::CheckRestrictions for SubmitClaimBasketResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.basket_identifier.check_restrictions(restrictions.clone())?;
            self.filed_claims.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ArrayOfFiledClaim")]
    pub struct ArrayOfFiledClaim {
        #[yaserde(prefix = "cla2", rename = "FiledClaim")]
        pub filed_claim: Vec<mod_cla2::FiledClaim>,
    }
    impl restrictions::CheckRestrictions for ArrayOfFiledClaim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.filed_claim.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "FiledClaim")]
    pub struct FiledClaim {
        #[yaserde(prefix = "cla2", rename = "ApprovedAmount")]
        pub approved_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeniedAmount")]
        pub denied_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PaidAmount")]
        pub paid_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PendingAmount")]
        pub pending_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CopayAmount")]
        pub copay_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CoinsuranceAmount")]
        pub coinsurance_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeductibleAmount")]
        pub deductible_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PostedAmount")]
        pub posted_amount: f64,
        #[yaserde(prefix = "cla2", rename = "SubmittedAmount")]
        pub submitted_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PlanName_PlanYear")]
        pub plan_name_plan_year: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimIdentifier")]
        pub claim_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimNumber")]
        pub claim_number: Option<String>,
        #[yaserde(prefix = "cla2", rename = "FilingStatus")]
        pub filing_status: mod_cla2::FilingStatus,
        #[yaserde(prefix = "cla2", rename = "IsDuplicate")]
        pub is_duplicate: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimStatus")]
        pub claim_status: Option<String>,
        #[yaserde(prefix = "cla2", rename = "PayToProviderFlag")]
        pub pay_to_provider_flag: Option<String>,
        #[yaserde(prefix = "cla2", rename = "MultipleDenialCodes")]
        pub multiple_denial_codes: Option<mod_cla2::ArrayOfInt>,
    }
    impl restrictions::CheckRestrictions for FiledClaim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.approved_amount.check_restrictions(restrictions.clone())?;
            self.denied_amount.check_restrictions(restrictions.clone())?;
            self.paid_amount.check_restrictions(restrictions.clone())?;
            self.pending_amount.check_restrictions(restrictions.clone())?;
            self.copay_amount.check_restrictions(restrictions.clone())?;
            self.coinsurance_amount.check_restrictions(restrictions.clone())?;
            self.deductible_amount.check_restrictions(restrictions.clone())?;
            self.posted_amount.check_restrictions(restrictions.clone())?;
            self.submitted_amount.check_restrictions(restrictions.clone())?;
            self.plan_name_plan_year.check_restrictions(restrictions.clone())?;
            self.claim_identifier.check_restrictions(restrictions.clone())?;
            self.claim_number.check_restrictions(restrictions.clone())?;
            self.filing_status.check_restrictions(restrictions.clone())?;
            self.is_duplicate.check_restrictions(restrictions.clone())?;
            self.claim_status.check_restrictions(restrictions.clone())?;
            self.pay_to_provider_flag.check_restrictions(restrictions.clone())?;
            self.multiple_denial_codes.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "FilingStatus")]
    pub struct FilingStatus {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for FilingStatus {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec!["Success".to_string(), "Failure".to_string()]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    pub type SubmitExpenseBasketResult = mod_cla2::SubmitClaimBasketResponse;
    pub type SubmitExpensesResult = mod_cla2::SubmitClaimWorksheetResponse;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimWorksheetResponse")]
    pub struct SubmitClaimWorksheetResponse {
        #[yaserde(prefix = "cla2", rename = "BasketIdentifier")]
        pub basket_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ProposedClaims")]
        pub proposed_claims: Option<mod_cla2::ArrayOfProposedClaim>,
        #[yaserde(prefix = "cla2", rename = "RecommendationHtml")]
        pub recommendation_html: Option<String>,
        #[yaserde(prefix = "cla2", rename = "HasAutoDenial")]
        pub has_auto_denial: bool,
        #[yaserde(prefix = "cla2", rename = "SpecialDenialCode")]
        pub special_denial_code: mod_cla2::SpecialDenialCode,
    }
    impl restrictions::CheckRestrictions for SubmitClaimWorksheetResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.basket_identifier.check_restrictions(restrictions.clone())?;
            self.proposed_claims.check_restrictions(restrictions.clone())?;
            self.recommendation_html.check_restrictions(restrictions.clone())?;
            self.has_auto_denial.check_restrictions(restrictions.clone())?;
            self.special_denial_code.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ArrayOfProposedClaim")]
    pub struct ArrayOfProposedClaim {
        #[yaserde(prefix = "cla2", rename = "ProposedClaim")]
        pub proposed_claim: Vec<mod_cla2::ProposedClaim>,
    }
    impl restrictions::CheckRestrictions for ArrayOfProposedClaim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.proposed_claim.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ProposedClaim")]
    pub struct ProposedClaim {
        #[yaserde(prefix = "cla2", rename = "ClaimIdentifier")]
        pub claim_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ExpenseDate")]
        pub expense_date: String,
        #[yaserde(prefix = "cla2", rename = "PlanName")]
        pub plan_name: Option<String>,
        #[yaserde(prefix = "cla2", rename = "PlanYear")]
        pub plan_year: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ApprovedAmount")]
        pub approved_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeniedAmount")]
        pub denied_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PaidAmount")]
        pub paid_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PendingAmount")]
        pub pending_amount: f64,
        #[yaserde(prefix = "cla2", rename = "DeductibleAmount")]
        pub deductible_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CoinsuranceAmount")]
        pub coinsurance_amount: f64,
        #[yaserde(prefix = "cla2", rename = "CopayAmount")]
        pub copay_amount: f64,
        #[yaserde(prefix = "cla2", rename = "PostedAmount")]
        pub posted_amount: f64,
        #[yaserde(prefix = "cla2", rename = "SubmittedAmount")]
        pub submitted_amount: f64,
        #[yaserde(prefix = "cla2", rename = "ClaimNumber")]
        pub claim_number: Option<String>,
        #[yaserde(prefix = "cla2", rename = "PayToProviderFlag")]
        pub pay_to_provider_flag: Option<String>,
    }
    impl restrictions::CheckRestrictions for ProposedClaim {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim_identifier.check_restrictions(restrictions.clone())?;
            self.expense_date.check_restrictions(restrictions.clone())?;
            self.plan_name.check_restrictions(restrictions.clone())?;
            self.plan_year.check_restrictions(restrictions.clone())?;
            self.approved_amount.check_restrictions(restrictions.clone())?;
            self.denied_amount.check_restrictions(restrictions.clone())?;
            self.paid_amount.check_restrictions(restrictions.clone())?;
            self.pending_amount.check_restrictions(restrictions.clone())?;
            self.deductible_amount.check_restrictions(restrictions.clone())?;
            self.coinsurance_amount.check_restrictions(restrictions.clone())?;
            self.copay_amount.check_restrictions(restrictions.clone())?;
            self.posted_amount.check_restrictions(restrictions.clone())?;
            self.submitted_amount.check_restrictions(restrictions.clone())?;
            self.claim_number.check_restrictions(restrictions.clone())?;
            self.pay_to_provider_flag.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SpecialDenialCode")]
    pub struct SpecialDenialCode {
        #[yaserde(text = true)]
        pub value: String,
    }
    impl restrictions::CheckRestrictions for SpecialDenialCode {
        fn check_restrictions(&self, _restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            let restrictions = Some(Rc::new(restrictions::Restrictions {
                enumeration: Some(vec![
                    "Default".to_string(),
                    "LifetimeMaximumIsExceeded".to_string(),
                    "ExpenseMaximumIsExceeded".to_string(),
                    "IndividualMaximumIsExceeded".to_string(),
                    "SharedMaximumIsExceeded".to_string(),
                    "SharedIndividualMaximumIsExceeded".to_string(),
                    "GroupMaxReimburseAmtIsExceeded".to_string(),
                    "ExpensePerClaimMaximumExceeded".to_string(),
                    "PlanLimitExceeded".to_string(),
                ]),
                ..Default::default()
            }));
            self.value.check_restrictions(restrictions)
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitExpenseNoAdviceResponse")]
    pub struct SubmitExpenseNoAdviceResponse {
        #[yaserde(prefix = "cla2", rename = "BasketIdentifier")]
        pub basket_identifier: Option<String>,
        #[yaserde(prefix = "cla2", rename = "FiledClaims")]
        pub filed_claims: Option<mod_cla2::ArrayOfFiledClaim>,
        #[yaserde(prefix = "cla2", rename = "ProposedClaims")]
        pub proposed_claims: Option<mod_cla2::ArrayOfProposedClaim>,
        #[yaserde(prefix = "cla2", rename = "RecommendationHtml")]
        pub recommendation_html: Option<String>,
        #[yaserde(prefix = "cla2", rename = "HasAutoDenial")]
        pub has_auto_denial: bool,
        #[yaserde(prefix = "cla2", rename = "HasClaimFiled")]
        pub has_claim_filed: bool,
    }
    impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.basket_identifier.check_restrictions(restrictions.clone())?;
            self.filed_claims.check_restrictions(restrictions.clone())?;
            self.proposed_claims.check_restrictions(restrictions.clone())?;
            self.recommendation_html.check_restrictions(restrictions.clone())?;
            self.has_auto_denial.check_restrictions(restrictions.clone())?;
            self.has_claim_filed.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "SubmitClaimWorksheetRequest")]
    pub struct SubmitClaimWorksheetRequest {
        #[yaserde(prefix = "cla2", rename = "Administrator")]
        pub administrator: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ClaimWorksheet")]
        pub claim_worksheet: Option<mod_cla2::ClaimWorksheet>,
        #[yaserde(prefix = "cla2", rename = "Employer")]
        pub employer: Option<String>,
        #[yaserde(prefix = "cla2", rename = "ParticipantID")]
        pub participant_id: i32,
        #[yaserde(prefix = "cla2", rename = "SendConfirmation")]
        pub send_confirmation: bool,
        #[yaserde(prefix = "cla2", rename = "SendAutoDenial")]
        pub send_auto_denial: bool,
    }
    impl restrictions::CheckRestrictions for SubmitClaimWorksheetRequest {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.administrator.check_restrictions(restrictions.clone())?;
            self.claim_worksheet.check_restrictions(restrictions.clone())?;
            self.employer.check_restrictions(restrictions.clone())?;
            self.participant_id.check_restrictions(restrictions.clone())?;
            self.send_confirmation.check_restrictions(restrictions.clone())?;
            self.send_auto_denial.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ClaimWorksheet")]
    pub struct ClaimWorksheet {
        #[yaserde(prefix = "cla2", rename = "Claim")]
        pub claim: Vec<mod_cla2::Claim>,
        #[yaserde(prefix = "cla2", rename = "WorksheetIdentifier", attribute = true)]
        pub worksheet_identifier: Option<String>,
    }
    impl restrictions::CheckRestrictions for ClaimWorksheet {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.claim.check_restrictions(restrictions.clone())?;
            self.worksheet_identifier.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    pub type ApproveExpensesResult = mod_cla2::SubmitClaimBasketResponse;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "cla2", namespaces = {"cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd"}, rename = "ExpenseAccountBase")]
    pub struct ExpenseAccountBase {
        #[yaserde(prefix = "cla2", rename = "ExpenseAccountId")]
        pub expense_account_id: i32,
        #[yaserde(prefix = "cla2", rename = "Description")]
        pub description: Option<String>,
        #[yaserde(prefix = "cla2", rename = "DisplayName")]
        pub display_name: Option<String>,
    }
    impl restrictions::CheckRestrictions for ExpenseAccountBase {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.expense_account_id.check_restrictions(restrictions.clone())?;
            self.description.check_restrictions(restrictions.clone())?;
            self.display_name.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
}
pub mod mod_cla1 {
    use super::*;
    use restrictions::CheckRestrictions;
    pub type SubmitExpenseNoAdviceResult = mod_cla2::SubmitExpenseNoAdviceResponse;
}

/* GetPlanMaxAmounts */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetPlanMaxAmountsInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetPlanMaxAmountsInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetPlanMaxAmountsInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetPlanMaxAmounts")]
    pub get_plan_max_amounts: mod_cla::GetPlanMaxAmounts,
}
impl restrictions::CheckRestrictions for GetPlanMaxAmountsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_plan_max_amounts.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetPlanMaxAmountsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetPlanMaxAmountsInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetPlanMaxAmountsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetPlanMaxAmountsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetPlanMaxAmountsOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetPlanMaxAmountsResponse")]
    pub get_plan_max_amounts_response: mod_cla::GetPlanMaxAmountsResponse,
}
impl restrictions::CheckRestrictions for GetPlanMaxAmountsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_plan_max_amounts_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetPlanMaxAmountsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetPlanMaxAmountsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetPlanMaxAmountsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_plan_max_amounts(
    req: GetPlanMaxAmountsInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetPlanMaxAmountsOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetPlanMaxAmounts";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetMileageRates */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetMileageRatesInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetMileageRatesInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetMileageRatesInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetMileageRates")]
    pub get_mileage_rates: mod_cla::GetMileageRates,
}
impl restrictions::CheckRestrictions for GetMileageRatesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_mileage_rates.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetMileageRatesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetMileageRatesInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetMileageRatesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetMileageRatesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetMileageRatesOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetMileageRatesResponse")]
    pub get_mileage_rates_response: mod_cla::GetMileageRatesResponse,
}
impl restrictions::CheckRestrictions for GetMileageRatesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_mileage_rates_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetMileageRatesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetMileageRatesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetMileageRatesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_mileage_rates(
    req: GetMileageRatesInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetMileageRatesOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetMileageRates";
    helpers::send_soap_request(url, credentials, req).await
}

/* ApproveExpenses */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct ApproveExpensesInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for ApproveExpensesInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct ApproveExpensesInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "ApproveExpenses")]
    pub approve_expenses: mod_cla::ApproveExpenses,
}
impl restrictions::CheckRestrictions for ApproveExpensesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.approve_expenses.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct ApproveExpensesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: ApproveExpensesInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ApproveExpensesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ApproveExpensesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct ApproveExpensesOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "ApproveExpensesResponse")]
    pub approve_expenses_response: mod_cla::ApproveExpensesResponse,
}
impl restrictions::CheckRestrictions for ApproveExpensesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.approve_expenses_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct ApproveExpensesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: ApproveExpensesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for ApproveExpensesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn approve_expenses(
    req: ApproveExpensesInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<ApproveExpensesOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/ApproveExpenses";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetExpenseApprovedAmount */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetExpenseApprovedAmountInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetExpenseApprovedAmountInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetExpenseApprovedAmountInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetExpenseApprovedAmount")]
    pub get_expense_approved_amount: mod_cla::GetExpenseApprovedAmount,
}
impl restrictions::CheckRestrictions for GetExpenseApprovedAmountInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_expense_approved_amount.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetExpenseApprovedAmountInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetExpenseApprovedAmountInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetExpenseApprovedAmountInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetExpenseApprovedAmountInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetExpenseApprovedAmountOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetExpenseApprovedAmountResponse")]
    pub get_expense_approved_amount_response: mod_cla::GetExpenseApprovedAmountResponse,
}
impl restrictions::CheckRestrictions for GetExpenseApprovedAmountOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_expense_approved_amount_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetExpenseApprovedAmountOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetExpenseApprovedAmountOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetExpenseApprovedAmountOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_expense_approved_amount(
    req: GetExpenseApprovedAmountInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetExpenseApprovedAmountOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetExpenseApprovedAmount";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetBaseExpenseAccounts */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetBaseExpenseAccountsInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetBaseExpenseAccountsInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetBaseExpenseAccountsInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetBaseExpenseAccounts")]
    pub get_base_expense_accounts: mod_cla::GetBaseExpenseAccounts,
}
impl restrictions::CheckRestrictions for GetBaseExpenseAccountsInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_base_expense_accounts.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetBaseExpenseAccountsInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetBaseExpenseAccountsInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetBaseExpenseAccountsInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetBaseExpenseAccountsInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetBaseExpenseAccountsOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetBaseExpenseAccountsResponse")]
    pub get_base_expense_accounts_response: mod_cla::GetBaseExpenseAccountsResponse,
}
impl restrictions::CheckRestrictions for GetBaseExpenseAccountsOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_base_expense_accounts_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetBaseExpenseAccountsOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetBaseExpenseAccountsOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetBaseExpenseAccountsOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_base_expense_accounts(
    req: GetBaseExpenseAccountsInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetBaseExpenseAccountsOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetBaseExpenseAccounts";
    helpers::send_soap_request(url, credentials, req).await
}

/* SubmitExpenses */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpensesInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for SubmitExpensesInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpensesInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpenses")]
    pub submit_expenses: mod_cla::SubmitExpenses,
}
impl restrictions::CheckRestrictions for SubmitExpensesInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expenses.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpensesInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: SubmitExpensesInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpensesInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpensesInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpensesOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpensesResponse")]
    pub submit_expenses_response: mod_cla::SubmitExpensesResponse,
}
impl restrictions::CheckRestrictions for SubmitExpensesOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expenses_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpensesOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpensesOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpensesOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn submit_expenses(
    req: SubmitExpensesInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SubmitExpensesOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/SubmitExpenses";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetLifetimeMaximumInformation */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetLifetimeMaximumInformationInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetLifetimeMaximumInformationInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetLifetimeMaximumInformation")]
    pub get_lifetime_maximum_information: mod_cla::GetLifetimeMaximumInformation,
}
impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_lifetime_maximum_information.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetLifetimeMaximumInformationInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetLifetimeMaximumInformationInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetLifetimeMaximumInformationInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetLifetimeMaximumInformationOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetLifetimeMaximumInformationResponse")]
    pub get_lifetime_maximum_information_response: mod_cla::GetLifetimeMaximumInformationResponse,
}
impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_lifetime_maximum_information_response
            .check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetLifetimeMaximumInformationOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetLifetimeMaximumInformationOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetLifetimeMaximumInformationOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_lifetime_maximum_information(
    req: GetLifetimeMaximumInformationInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetLifetimeMaximumInformationOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetLifetimeMaximumInformation";
    helpers::send_soap_request(url, credentials, req).await
}

/* SubmitExpenseNoAdvice */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseNoAdviceInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseNoAdviceInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpenseNoAdvice")]
    pub submit_expense_no_advice: mod_cla::SubmitExpenseNoAdvice,
}
impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expense_no_advice.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseNoAdviceInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: SubmitExpenseNoAdviceInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpenseNoAdviceInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseNoAdviceOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpenseNoAdviceResponse")]
    pub submit_expense_no_advice_response: mod_cla::SubmitExpenseNoAdviceResponse,
}
impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expense_no_advice_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseNoAdviceOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpenseNoAdviceOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpenseNoAdviceOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn submit_expense_no_advice(
    req: SubmitExpenseNoAdviceInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SubmitExpenseNoAdviceOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/SubmitExpenseNoAdvice";
    helpers::send_soap_request(url, credentials, req).await
}

/* GetClaim */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetClaimInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for GetClaimInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetClaimInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetClaim")]
    pub get_claim: mod_cla::GetClaim,
}
impl restrictions::CheckRestrictions for GetClaimInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_claim.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetClaimInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: GetClaimInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetClaimInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetClaimInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetClaimOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "GetClaimResponse")]
    pub get_claim_response: mod_cla::GetClaimResponse,
}
impl restrictions::CheckRestrictions for GetClaimOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.get_claim_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct GetClaimOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: GetClaimOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for GetClaimOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn get_claim(
    req: GetClaimInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<GetClaimOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/GetClaim";
    helpers::send_soap_request(url, credentials, req).await
}

/* SubmitClaimBasket */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitClaimBasketInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for SubmitClaimBasketInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitClaimBasketInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitClaimBasket")]
    pub submit_claim_basket: mod_cla::SubmitClaimBasket,
}
impl restrictions::CheckRestrictions for SubmitClaimBasketInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_claim_basket.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitClaimBasketInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: SubmitClaimBasketInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitClaimBasketInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitClaimBasketInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitClaimBasketOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitClaimBasketResponse")]
    pub submit_claim_basket_response: mod_cla::SubmitClaimBasketResponse,
}
impl restrictions::CheckRestrictions for SubmitClaimBasketOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_claim_basket_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitClaimBasketOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitClaimBasketOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitClaimBasketOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn submit_claim_basket(
    req: SubmitClaimBasketInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SubmitClaimBasketOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/SubmitClaimBasket";
    helpers::send_soap_request(url, credentials, req).await
}

/* SubmitExpenseBasket */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseBasketInputEnvelopeHeader {
    #[yaserde(prefix = "cla", rename = "ServiceSecurityHeader")]
    pub service_security_header: Option<mod_cla::ServiceSecurityHeader>,
}
impl restrictions::CheckRestrictions for SubmitExpenseBasketInputEnvelopeHeader {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.service_security_header.check_restrictions(restrictions.clone())?;
        Ok(())
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseBasketInputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpenseBasket")]
    pub submit_expense_basket: mod_cla::SubmitExpenseBasket,
}
impl restrictions::CheckRestrictions for SubmitExpenseBasketInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expense_basket.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseBasketInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Header")]
    pub header: SubmitExpenseBasketInputEnvelopeHeader,
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpenseBasketInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpenseBasketInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.header.check_restrictions(restrictions.clone())?;
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "cla", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseBasketOutputEnvelopeBody {
    #[yaserde(prefix = "cla", rename = "SubmitExpenseBasketResponse")]
    pub submit_expense_basket_response: mod_cla::SubmitExpenseBasketResponse,
}
impl restrictions::CheckRestrictions for SubmitExpenseBasketOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.submit_expense_basket_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "cla" = "http://services.lighthouse1.com/services/claim/", "cla2" = "http://services.lighthouse1.com/ClaimService/ClaimMessages.xsd", "cla1" = "http://services.lighthouse1.com/ClaimService/ClaimMseeages.xsd" })]
pub struct SubmitExpenseBasketOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: SubmitExpenseBasketOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for SubmitExpenseBasketOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub async fn submit_expense_basket(
    req: SubmitExpenseBasketInputEnvelope,
    credentials: Option<(String, String)>,
) -> error::SoapResult<SubmitExpenseBasketOutputEnvelope> {
    let url = "http://services.lighthouse1.com/services/claim/SubmitExpenseBasket";
    helpers::send_soap_request(url, credentials, req).await
}
pub struct ClaimService {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl ClaimService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "https://trainingservices.navigatorsuite.com/ClaimService/ClaimService.asmx".to_string(),
            credentials,
        }
    }

    pub fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "https://trainingservices.navigatorsuite.com/ClaimService/ClaimService.asmx".to_string(),
            credentials: None,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = location.into();
        self
    }

    pub fn with_credentials(mut self, credentials: (String, String)) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub async fn get_plan_max_amounts(
        &self,
        req: GetPlanMaxAmountsInputEnvelope,
    ) -> error::SoapResult<GetPlanMaxAmountsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_mileage_rates(
        &self,
        req: GetMileageRatesInputEnvelope,
    ) -> error::SoapResult<GetMileageRatesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn approve_expenses(
        &self,
        req: ApproveExpensesInputEnvelope,
    ) -> error::SoapResult<ApproveExpensesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_expense_approved_amount(
        &self,
        req: GetExpenseApprovedAmountInputEnvelope,
    ) -> error::SoapResult<GetExpenseApprovedAmountOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_base_expense_accounts(
        &self,
        req: GetBaseExpenseAccountsInputEnvelope,
    ) -> error::SoapResult<GetBaseExpenseAccountsOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn submit_expenses(
        &self,
        req: SubmitExpensesInputEnvelope,
    ) -> error::SoapResult<SubmitExpensesOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_lifetime_maximum_information(
        &self,
        req: GetLifetimeMaximumInformationInputEnvelope,
    ) -> error::SoapResult<GetLifetimeMaximumInformationOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn submit_expense_no_advice(
        &self,
        req: SubmitExpenseNoAdviceInputEnvelope,
    ) -> error::SoapResult<SubmitExpenseNoAdviceOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn get_claim(&self, req: GetClaimInputEnvelope) -> error::SoapResult<GetClaimOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn submit_claim_basket(
        &self,
        req: SubmitClaimBasketInputEnvelope,
    ) -> error::SoapResult<SubmitClaimBasketOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn submit_expense_basket(
        &self,
        req: SubmitExpenseBasketInputEnvelope,
    ) -> error::SoapResult<SubmitExpenseBasketOutputEnvelope> {
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
                if let Some(min_inclusive) = restrictions.min_inclusive
                    && *self <= min_inclusive
                {
                    return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
                }

                if let Some(max_inclusive) = restrictions.max_inclusive
                    && max_inclusive <= *self
                {
                    return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
                }

                if let Some(min_exclusive) = restrictions.min_exclusive
                    && *self < min_exclusive
                {
                    return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
                }

                if let Some(max_exclusive) = restrictions.max_exclusive
                    && max_exclusive < *self
                {
                    return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
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

            if let Some(min_length) = restrictions.min_length
                && s_len < min_length
            {
                return Err(SoapError::Restriction("minLength restriction not met".to_string()));
            }

            if let Some(max_length) = restrictions.max_length
                && max_length < s_len
            {
                return Err(SoapError::Restriction("maxLength restriction not met".to_string()));
            }

            if let Some(length) = restrictions.length
                && length != s_len
            {
                return Err(SoapError::Restriction("length restriction not met".to_string()));
            }

            // Enumerations
            if let Some(enumeration) = restrictions.enumeration.as_ref()
                && !enumeration.contains(self)
            {
                return Err(SoapError::Restriction("enumeration restriction not met".to_string()));
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

            if let Some(min_inclusive) = restrictions.min_inclusive
                && value <= min_inclusive
            {
                return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
            }

            if let Some(max_inclusive) = restrictions.max_inclusive
                && max_inclusive <= value
            {
                return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
            }

            if let Some(min_exclusive) = restrictions.min_exclusive
                && value < min_exclusive
            {
                return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
            }

            if let Some(max_exclusive) = restrictions.max_exclusive
                && max_exclusive < value
            {
                return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
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
