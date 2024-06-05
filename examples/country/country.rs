//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.7
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

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
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeSoapIn")]
    pub struct GetCountryByCountryCodeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountryByCountryCode,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeSoapOut")]
    pub struct GetCountryByCountryCodeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountryByCountryCodeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDSoapIn")]
    pub struct GetISDSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetISD,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDSoapOut")]
    pub struct GetISDSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetISDResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesSoapIn")]
    pub struct GetCountriesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountries,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesSoapOut")]
    pub struct GetCountriesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountriesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameSoapIn")]
    pub struct GetCurrencyCodeByCurrencyNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyCodeByCurrencyName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameSoapOut")]
    pub struct GetCurrencyCodeByCurrencyNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyCodeByCurrencyNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameSoapIn")]
    pub struct GetISOCountryCodeByCountyNameSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetISOCountryCodeByCountyName,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameSoapOut")]
    pub struct GetISOCountryCodeByCountyNameSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetISOCountryCodeByCountyNameResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeSoapIn")]
    pub struct GetCurrencyCodeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyCode,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeSoapOut")]
    pub struct GetCurrencyCodeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyCodeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeSoapIn")]
    pub struct GetCountryByCurrencyCodeSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountryByCurrencyCode,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeSoapOut")]
    pub struct GetCountryByCurrencyCodeSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCountryByCurrencyCodeResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesSoapIn")]
    pub struct GetCurrenciesSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencies,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesSoapOut")]
    pub struct GetCurrenciesSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrenciesResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountrySoapIn")]
    pub struct GetCurrencyByCountrySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyByCountry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountrySoapOut")]
    pub struct GetCurrencyByCountrySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCurrencyByCountryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountrySoapIn")]
    pub struct GetGMTbyCountrySoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetGMTbyCountry,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountrySoapOut")]
    pub struct GetGMTbyCountrySoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetGMTbyCountryResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeHttpGetIn")]
    pub struct GetCountryByCountryCodeHttpGetIn {
        #[yaserde(rename = "CountryCode", default)]
        pub country_code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeHttpGetOut")]
    pub struct GetCountryByCountryCodeHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDHttpGetIn")]
    pub struct GetISDHttpGetIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDHttpGetOut")]
    pub struct GetISDHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesHttpGetIn")]
    pub struct GetCountriesHttpGetIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesHttpGetOut")]
    pub struct GetCountriesHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameHttpGetIn")]
    pub struct GetCurrencyCodeByCurrencyNameHttpGetIn {
        #[yaserde(rename = "CurrencyName", default)]
        pub currency_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameHttpGetOut")]
    pub struct GetCurrencyCodeByCurrencyNameHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameHttpGetIn")]
    pub struct GetISOCountryCodeByCountyNameHttpGetIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameHttpGetOut")]
    pub struct GetISOCountryCodeByCountyNameHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeHttpGetIn")]
    pub struct GetCurrencyCodeHttpGetIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeHttpGetOut")]
    pub struct GetCurrencyCodeHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeHttpGetIn")]
    pub struct GetCountryByCurrencyCodeHttpGetIn {
        #[yaserde(rename = "CurrencyCode", default)]
        pub currency_code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeHttpGetOut")]
    pub struct GetCountryByCurrencyCodeHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesHttpGetIn")]
    pub struct GetCurrenciesHttpGetIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesHttpGetOut")]
    pub struct GetCurrenciesHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountryHttpGetIn")]
    pub struct GetCurrencyByCountryHttpGetIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountryHttpGetOut")]
    pub struct GetCurrencyByCountryHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountryHttpGetIn")]
    pub struct GetGMTbyCountryHttpGetIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountryHttpGetOut")]
    pub struct GetGMTbyCountryHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeHttpPostIn")]
    pub struct GetCountryByCountryCodeHttpPostIn {
        #[yaserde(rename = "CountryCode", default)]
        pub country_code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCountryCodeHttpPostOut")]
    pub struct GetCountryByCountryCodeHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDHttpPostIn")]
    pub struct GetISDHttpPostIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISDHttpPostOut")]
    pub struct GetISDHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesHttpPostIn")]
    pub struct GetCountriesHttpPostIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountriesHttpPostOut")]
    pub struct GetCountriesHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameHttpPostIn")]
    pub struct GetCurrencyCodeByCurrencyNameHttpPostIn {
        #[yaserde(rename = "CurrencyName", default)]
        pub currency_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeByCurrencyNameHttpPostOut")]
    pub struct GetCurrencyCodeByCurrencyNameHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameHttpPostIn")]
    pub struct GetISOCountryCodeByCountyNameHttpPostIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetISOCountryCodeByCountyNameHttpPostOut")]
    pub struct GetISOCountryCodeByCountyNameHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeHttpPostIn")]
    pub struct GetCurrencyCodeHttpPostIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyCodeHttpPostOut")]
    pub struct GetCurrencyCodeHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeHttpPostIn")]
    pub struct GetCountryByCurrencyCodeHttpPostIn {
        #[yaserde(rename = "CurrencyCode", default)]
        pub currency_code: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCountryByCurrencyCodeHttpPostOut")]
    pub struct GetCountryByCurrencyCodeHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesHttpPostIn")]
    pub struct GetCurrenciesHttpPostIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrenciesHttpPostOut")]
    pub struct GetCurrenciesHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountryHttpPostIn")]
    pub struct GetCurrencyByCountryHttpPostIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCurrencyByCountryHttpPostOut")]
    pub struct GetCurrencyByCountryHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountryHttpPostIn")]
    pub struct GetGMTbyCountryHttpPostIn {
        #[yaserde(rename = "CountryName", default)]
        pub country_name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetGMTbyCountryHttpPostOut")]
    pub struct GetGMTbyCountryHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::String,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountryByCountryCode",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountryByCountryCode {
        #[yaserde(rename = "CountryCode", prefix = "tns", default)]
        pub country_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountryByCountryCodeResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountryByCountryCodeResponse {
        #[yaserde(rename = "GetCountryByCountryCodeResult", prefix = "tns", default)]
        pub get_country_by_country_code_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetISD",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetISD {
        #[yaserde(rename = "CountryName", prefix = "tns", default)]
        pub country_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetISDResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetISDResponse {
        #[yaserde(rename = "GetISDResult", prefix = "tns", default)]
        pub get_isd_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountries",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountries {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountriesResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountriesResponse {
        #[yaserde(rename = "GetCountriesResult", prefix = "tns", default)]
        pub get_countries_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyCodeByCurrencyName",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyCodeByCurrencyName {
        #[yaserde(rename = "CurrencyName", prefix = "tns", default)]
        pub currency_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyCodeByCurrencyNameResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyCodeByCurrencyNameResponse {
        #[yaserde(
            rename = "GetCurrencyCodeByCurrencyNameResult",
            prefix = "tns",
            default
        )]
        pub get_currency_code_by_currency_name_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetISOCountryCodeByCountyName",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetISOCountryCodeByCountyName {
        #[yaserde(rename = "CountryName", prefix = "tns", default)]
        pub country_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetISOCountryCodeByCountyNameResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetISOCountryCodeByCountyNameResponse {
        #[yaserde(
            rename = "GetISOCountryCodeByCountyNameResult",
            prefix = "tns",
            default
        )]
        pub get_iso_country_code_by_county_name_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyCode",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyCode {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyCodeResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyCodeResponse {
        #[yaserde(rename = "GetCurrencyCodeResult", prefix = "tns", default)]
        pub get_currency_code_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountryByCurrencyCode",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountryByCurrencyCode {
        #[yaserde(rename = "CurrencyCode", prefix = "tns", default)]
        pub currency_code: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCountryByCurrencyCodeResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCountryByCurrencyCodeResponse {
        #[yaserde(rename = "GetCountryByCurrencyCodeResult", prefix = "tns", default)]
        pub get_country_by_currency_code_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencies",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencies {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrenciesResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrenciesResponse {
        #[yaserde(rename = "GetCurrenciesResult", prefix = "tns", default)]
        pub get_currencies_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyByCountry",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyByCountry {
        #[yaserde(rename = "CountryName", prefix = "tns", default)]
        pub country_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCurrencyByCountryResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCurrencyByCountryResponse {
        #[yaserde(rename = "GetCurrencyByCountryResult", prefix = "tns", default)]
        pub get_currency_by_country_result: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetGMTbyCountry",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetGMTbyCountry {
        #[yaserde(rename = "CountryName", prefix = "tns", default)]
        pub country_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetGMTbyCountryResponse",
        namespace = "tns: http://www.webserviceX.NET",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetGMTbyCountryResponse {
        #[yaserde(rename = "GetGMTbyCountryResult", prefix = "tns", default)]
        pub get_gm_tby_country_result: Option<String>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type GetCountryByCountryCodeSoapIn = messages::GetCountryByCountryCodeSoapIn;

    pub type GetCountryByCountryCodeSoapOut = messages::GetCountryByCountryCodeSoapOut;

    pub type GetISDSoapIn = messages::GetISDSoapIn;

    pub type GetISDSoapOut = messages::GetISDSoapOut;

    pub type GetCountriesSoapIn = messages::GetCountriesSoapIn;

    pub type GetCountriesSoapOut = messages::GetCountriesSoapOut;

    pub type GetCurrencyCodeByCurrencyNameSoapIn = messages::GetCurrencyCodeByCurrencyNameSoapIn;

    pub type GetCurrencyCodeByCurrencyNameSoapOut = messages::GetCurrencyCodeByCurrencyNameSoapOut;

    pub type GetISOCountryCodeByCountyNameSoapIn = messages::GetISOCountryCodeByCountyNameSoapIn;

    pub type GetISOCountryCodeByCountyNameSoapOut = messages::GetISOCountryCodeByCountyNameSoapOut;

    pub type GetCurrencyCodeSoapIn = messages::GetCurrencyCodeSoapIn;

    pub type GetCurrencyCodeSoapOut = messages::GetCurrencyCodeSoapOut;

    pub type GetCountryByCurrencyCodeSoapIn = messages::GetCountryByCurrencyCodeSoapIn;

    pub type GetCountryByCurrencyCodeSoapOut = messages::GetCountryByCurrencyCodeSoapOut;

    pub type GetCurrenciesSoapIn = messages::GetCurrenciesSoapIn;

    pub type GetCurrenciesSoapOut = messages::GetCurrenciesSoapOut;

    pub type GetCurrencyByCountrySoapIn = messages::GetCurrencyByCountrySoapIn;

    pub type GetCurrencyByCountrySoapOut = messages::GetCurrencyByCountrySoapOut;

    pub type GetGMTbyCountrySoapIn = messages::GetGMTbyCountrySoapIn;

    pub type GetGMTbyCountrySoapOut = messages::GetGMTbyCountrySoapOut;

    #[async_trait]
    pub trait CountrySoap {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_soap_in: GetCountryByCountryCodeSoapIn,
        ) -> Result<GetCountryByCountryCodeSoapOut, Option<SoapFault>>;
        async fn get_isd(
            &self,
            get_isd_soap_in: GetISDSoapIn,
        ) -> Result<GetISDSoapOut, Option<SoapFault>>;
        async fn get_countries(
            &self,
            get_countries_soap_in: GetCountriesSoapIn,
        ) -> Result<GetCountriesSoapOut, Option<SoapFault>>;
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_soap_in: GetCurrencyCodeByCurrencyNameSoapIn,
        ) -> Result<GetCurrencyCodeByCurrencyNameSoapOut, Option<SoapFault>>;
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_soap_in: GetISOCountryCodeByCountyNameSoapIn,
        ) -> Result<GetISOCountryCodeByCountyNameSoapOut, Option<SoapFault>>;
        async fn get_currency_code(
            &self,
            get_currency_code_soap_in: GetCurrencyCodeSoapIn,
        ) -> Result<GetCurrencyCodeSoapOut, Option<SoapFault>>;
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_soap_in: GetCountryByCurrencyCodeSoapIn,
        ) -> Result<GetCountryByCurrencyCodeSoapOut, Option<SoapFault>>;
        async fn get_currencies(
            &self,
            get_currencies_soap_in: GetCurrenciesSoapIn,
        ) -> Result<GetCurrenciesSoapOut, Option<SoapFault>>;
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_soap_in: GetCurrencyByCountrySoapIn,
        ) -> Result<GetCurrencyByCountrySoapOut, Option<SoapFault>>;
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_soap_in: GetGMTbyCountrySoapIn,
        ) -> Result<GetGMTbyCountrySoapOut, Option<SoapFault>>;
    }
    pub type GetCountryByCountryCodeHttpGetIn = messages::GetCountryByCountryCodeHttpGetIn;

    pub type GetCountryByCountryCodeHttpGetOut = messages::GetCountryByCountryCodeHttpGetOut;

    pub type GetISDHttpGetIn = messages::GetISDHttpGetIn;

    pub type GetISDHttpGetOut = messages::GetISDHttpGetOut;

    pub type GetCountriesHttpGetIn = messages::GetCountriesHttpGetIn;

    pub type GetCountriesHttpGetOut = messages::GetCountriesHttpGetOut;

    pub type GetCurrencyCodeByCurrencyNameHttpGetIn =
        messages::GetCurrencyCodeByCurrencyNameHttpGetIn;

    pub type GetCurrencyCodeByCurrencyNameHttpGetOut =
        messages::GetCurrencyCodeByCurrencyNameHttpGetOut;

    pub type GetISOCountryCodeByCountyNameHttpGetIn =
        messages::GetISOCountryCodeByCountyNameHttpGetIn;

    pub type GetISOCountryCodeByCountyNameHttpGetOut =
        messages::GetISOCountryCodeByCountyNameHttpGetOut;

    pub type GetCurrencyCodeHttpGetIn = messages::GetCurrencyCodeHttpGetIn;

    pub type GetCurrencyCodeHttpGetOut = messages::GetCurrencyCodeHttpGetOut;

    pub type GetCountryByCurrencyCodeHttpGetIn = messages::GetCountryByCurrencyCodeHttpGetIn;

    pub type GetCountryByCurrencyCodeHttpGetOut = messages::GetCountryByCurrencyCodeHttpGetOut;

    pub type GetCurrenciesHttpGetIn = messages::GetCurrenciesHttpGetIn;

    pub type GetCurrenciesHttpGetOut = messages::GetCurrenciesHttpGetOut;

    pub type GetCurrencyByCountryHttpGetIn = messages::GetCurrencyByCountryHttpGetIn;

    pub type GetCurrencyByCountryHttpGetOut = messages::GetCurrencyByCountryHttpGetOut;

    pub type GetGMTbyCountryHttpGetIn = messages::GetGMTbyCountryHttpGetIn;

    pub type GetGMTbyCountryHttpGetOut = messages::GetGMTbyCountryHttpGetOut;

    #[async_trait]
    pub trait CountryHttpGet {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_http_get_in: GetCountryByCountryCodeHttpGetIn,
        ) -> Result<GetCountryByCountryCodeHttpGetOut, Option<SoapFault>>;
        async fn get_isd(
            &self,
            get_isd_http_get_in: GetISDHttpGetIn,
        ) -> Result<GetISDHttpGetOut, Option<SoapFault>>;
        async fn get_countries(
            &self,
            get_countries_http_get_in: GetCountriesHttpGetIn,
        ) -> Result<GetCountriesHttpGetOut, Option<SoapFault>>;
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_http_get_in: GetCurrencyCodeByCurrencyNameHttpGetIn,
        ) -> Result<GetCurrencyCodeByCurrencyNameHttpGetOut, Option<SoapFault>>;
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_http_get_in: GetISOCountryCodeByCountyNameHttpGetIn,
        ) -> Result<GetISOCountryCodeByCountyNameHttpGetOut, Option<SoapFault>>;
        async fn get_currency_code(
            &self,
            get_currency_code_http_get_in: GetCurrencyCodeHttpGetIn,
        ) -> Result<GetCurrencyCodeHttpGetOut, Option<SoapFault>>;
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_http_get_in: GetCountryByCurrencyCodeHttpGetIn,
        ) -> Result<GetCountryByCurrencyCodeHttpGetOut, Option<SoapFault>>;
        async fn get_currencies(
            &self,
            get_currencies_http_get_in: GetCurrenciesHttpGetIn,
        ) -> Result<GetCurrenciesHttpGetOut, Option<SoapFault>>;
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_http_get_in: GetCurrencyByCountryHttpGetIn,
        ) -> Result<GetCurrencyByCountryHttpGetOut, Option<SoapFault>>;
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_http_get_in: GetGMTbyCountryHttpGetIn,
        ) -> Result<GetGMTbyCountryHttpGetOut, Option<SoapFault>>;
    }
    pub type GetCountryByCountryCodeHttpPostIn = messages::GetCountryByCountryCodeHttpPostIn;

    pub type GetCountryByCountryCodeHttpPostOut = messages::GetCountryByCountryCodeHttpPostOut;

    pub type GetISDHttpPostIn = messages::GetISDHttpPostIn;

    pub type GetISDHttpPostOut = messages::GetISDHttpPostOut;

    pub type GetCountriesHttpPostIn = messages::GetCountriesHttpPostIn;

    pub type GetCountriesHttpPostOut = messages::GetCountriesHttpPostOut;

    pub type GetCurrencyCodeByCurrencyNameHttpPostIn =
        messages::GetCurrencyCodeByCurrencyNameHttpPostIn;

    pub type GetCurrencyCodeByCurrencyNameHttpPostOut =
        messages::GetCurrencyCodeByCurrencyNameHttpPostOut;

    pub type GetISOCountryCodeByCountyNameHttpPostIn =
        messages::GetISOCountryCodeByCountyNameHttpPostIn;

    pub type GetISOCountryCodeByCountyNameHttpPostOut =
        messages::GetISOCountryCodeByCountyNameHttpPostOut;

    pub type GetCurrencyCodeHttpPostIn = messages::GetCurrencyCodeHttpPostIn;

    pub type GetCurrencyCodeHttpPostOut = messages::GetCurrencyCodeHttpPostOut;

    pub type GetCountryByCurrencyCodeHttpPostIn = messages::GetCountryByCurrencyCodeHttpPostIn;

    pub type GetCountryByCurrencyCodeHttpPostOut = messages::GetCountryByCurrencyCodeHttpPostOut;

    pub type GetCurrenciesHttpPostIn = messages::GetCurrenciesHttpPostIn;

    pub type GetCurrenciesHttpPostOut = messages::GetCurrenciesHttpPostOut;

    pub type GetCurrencyByCountryHttpPostIn = messages::GetCurrencyByCountryHttpPostIn;

    pub type GetCurrencyByCountryHttpPostOut = messages::GetCurrencyByCountryHttpPostOut;

    pub type GetGMTbyCountryHttpPostIn = messages::GetGMTbyCountryHttpPostIn;

    pub type GetGMTbyCountryHttpPostOut = messages::GetGMTbyCountryHttpPostOut;

    #[async_trait]
    pub trait CountryHttpPost {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_http_post_in: GetCountryByCountryCodeHttpPostIn,
        ) -> Result<GetCountryByCountryCodeHttpPostOut, Option<SoapFault>>;
        async fn get_isd(
            &self,
            get_isd_http_post_in: GetISDHttpPostIn,
        ) -> Result<GetISDHttpPostOut, Option<SoapFault>>;
        async fn get_countries(
            &self,
            get_countries_http_post_in: GetCountriesHttpPostIn,
        ) -> Result<GetCountriesHttpPostOut, Option<SoapFault>>;
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_http_post_in: GetCurrencyCodeByCurrencyNameHttpPostIn,
        ) -> Result<GetCurrencyCodeByCurrencyNameHttpPostOut, Option<SoapFault>>;
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_http_post_in: GetISOCountryCodeByCountyNameHttpPostIn,
        ) -> Result<GetISOCountryCodeByCountyNameHttpPostOut, Option<SoapFault>>;
        async fn get_currency_code(
            &self,
            get_currency_code_http_post_in: GetCurrencyCodeHttpPostIn,
        ) -> Result<GetCurrencyCodeHttpPostOut, Option<SoapFault>>;
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_http_post_in: GetCountryByCurrencyCodeHttpPostIn,
        ) -> Result<GetCountryByCurrencyCodeHttpPostOut, Option<SoapFault>>;
        async fn get_currencies(
            &self,
            get_currencies_http_post_in: GetCurrenciesHttpPostIn,
        ) -> Result<GetCurrenciesHttpPostOut, Option<SoapFault>>;
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_http_post_in: GetCurrencyByCountryHttpPostIn,
        ) -> Result<GetCurrencyByCountryHttpPostOut, Option<SoapFault>>;
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_http_post_in: GetGMTbyCountryHttpPostIn,
        ) -> Result<GetGMTbyCountryHttpPostOut, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl CountrySoap {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeSoapIn {
        #[yaserde(rename = "GetCountryByCountryCode", default)]
        pub body: ports::GetCountryByCountryCodeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeSoapInSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeSoapIn,
    }

    impl GetCountryByCountryCodeSoapInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeSoapIn) -> Self {
            GetCountryByCountryCodeSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeSoapOut {
        #[yaserde(rename = "GetCountryByCountryCodeResponse", default)]
        pub body: ports::GetCountryByCountryCodeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeSoapOutSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeSoapOut,
    }

    impl GetCountryByCountryCodeSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeSoapOut) -> Self {
            GetCountryByCountryCodeSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDSoapIn {
        #[yaserde(rename = "GetISD", default)]
        pub body: ports::GetISDSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDSoapInSoapEnvelope {
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
        pub body: SoapGetISDSoapIn,
    }

    impl GetISDSoapInSoapEnvelope {
        pub fn new(body: SoapGetISDSoapIn) -> Self {
            GetISDSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDSoapOut {
        #[yaserde(rename = "GetISDResponse", default)]
        pub body: ports::GetISDSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDSoapOutSoapEnvelope {
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
        pub body: SoapGetISDSoapOut,
    }

    impl GetISDSoapOutSoapEnvelope {
        pub fn new(body: SoapGetISDSoapOut) -> Self {
            GetISDSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesSoapIn {
        #[yaserde(rename = "GetCountries", default)]
        pub body: ports::GetCountriesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesSoapInSoapEnvelope {
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
        pub body: SoapGetCountriesSoapIn,
    }

    impl GetCountriesSoapInSoapEnvelope {
        pub fn new(body: SoapGetCountriesSoapIn) -> Self {
            GetCountriesSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesSoapOut {
        #[yaserde(rename = "GetCountriesResponse", default)]
        pub body: ports::GetCountriesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesSoapOutSoapEnvelope {
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
        pub body: SoapGetCountriesSoapOut,
    }

    impl GetCountriesSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCountriesSoapOut) -> Self {
            GetCountriesSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameSoapIn {
        #[yaserde(rename = "GetCurrencyCodeByCurrencyName", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameSoapInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameSoapIn,
    }

    impl GetCurrencyCodeByCurrencyNameSoapInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameSoapIn) -> Self {
            GetCurrencyCodeByCurrencyNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameSoapOut {
        #[yaserde(rename = "GetCurrencyCodeByCurrencyNameResponse", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameSoapOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameSoapOut,
    }

    impl GetCurrencyCodeByCurrencyNameSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameSoapOut) -> Self {
            GetCurrencyCodeByCurrencyNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameSoapIn {
        #[yaserde(rename = "GetISOCountryCodeByCountyName", default)]
        pub body: ports::GetISOCountryCodeByCountyNameSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameSoapInSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameSoapIn,
    }

    impl GetISOCountryCodeByCountyNameSoapInSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameSoapIn) -> Self {
            GetISOCountryCodeByCountyNameSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameSoapOut {
        #[yaserde(rename = "GetISOCountryCodeByCountyNameResponse", default)]
        pub body: ports::GetISOCountryCodeByCountyNameSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameSoapOutSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameSoapOut,
    }

    impl GetISOCountryCodeByCountyNameSoapOutSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameSoapOut) -> Self {
            GetISOCountryCodeByCountyNameSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeSoapIn {
        #[yaserde(rename = "GetCurrencyCode", default)]
        pub body: ports::GetCurrencyCodeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeSoapInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeSoapIn,
    }

    impl GetCurrencyCodeSoapInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeSoapIn) -> Self {
            GetCurrencyCodeSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeSoapOut {
        #[yaserde(rename = "GetCurrencyCodeResponse", default)]
        pub body: ports::GetCurrencyCodeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeSoapOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeSoapOut,
    }

    impl GetCurrencyCodeSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeSoapOut) -> Self {
            GetCurrencyCodeSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeSoapIn {
        #[yaserde(rename = "GetCountryByCurrencyCode", default)]
        pub body: ports::GetCountryByCurrencyCodeSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeSoapInSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeSoapIn,
    }

    impl GetCountryByCurrencyCodeSoapInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeSoapIn) -> Self {
            GetCountryByCurrencyCodeSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeSoapOut {
        #[yaserde(rename = "GetCountryByCurrencyCodeResponse", default)]
        pub body: ports::GetCountryByCurrencyCodeSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeSoapOutSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeSoapOut,
    }

    impl GetCountryByCurrencyCodeSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeSoapOut) -> Self {
            GetCountryByCurrencyCodeSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesSoapIn {
        #[yaserde(rename = "GetCurrencies", default)]
        pub body: ports::GetCurrenciesSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesSoapInSoapEnvelope {
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
        pub body: SoapGetCurrenciesSoapIn,
    }

    impl GetCurrenciesSoapInSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesSoapIn) -> Self {
            GetCurrenciesSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesSoapOut {
        #[yaserde(rename = "GetCurrenciesResponse", default)]
        pub body: ports::GetCurrenciesSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesSoapOutSoapEnvelope {
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
        pub body: SoapGetCurrenciesSoapOut,
    }

    impl GetCurrenciesSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesSoapOut) -> Self {
            GetCurrenciesSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountrySoapIn {
        #[yaserde(rename = "GetCurrencyByCountry", default)]
        pub body: ports::GetCurrencyByCountrySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountrySoapInSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountrySoapIn,
    }

    impl GetCurrencyByCountrySoapInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountrySoapIn) -> Self {
            GetCurrencyByCountrySoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountrySoapOut {
        #[yaserde(rename = "GetCurrencyByCountryResponse", default)]
        pub body: ports::GetCurrencyByCountrySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountrySoapOutSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountrySoapOut,
    }

    impl GetCurrencyByCountrySoapOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountrySoapOut) -> Self {
            GetCurrencyByCountrySoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountrySoapIn {
        #[yaserde(rename = "GetGMTbyCountry", default)]
        pub body: ports::GetGMTbyCountrySoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountrySoapInSoapEnvelope {
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
        pub body: SoapGetGMTbyCountrySoapIn,
    }

    impl GetGMTbyCountrySoapInSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountrySoapIn) -> Self {
            GetGMTbyCountrySoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountrySoapOut {
        #[yaserde(rename = "GetGMTbyCountryResponse", default)]
        pub body: ports::GetGMTbyCountrySoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountrySoapOutSoapEnvelope {
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
        pub body: SoapGetGMTbyCountrySoapOut,
    }

    impl GetGMTbyCountrySoapOutSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountrySoapOut) -> Self {
            GetGMTbyCountrySoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for CountrySoap {
        fn default() -> Self {
            CountrySoap {
                client: reqwest::Client::new(),
                url: "http://www.webserviceX.NET".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl CountrySoap {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            CountrySoap {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct CountrySoap {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::CountrySoap for CountrySoap {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_soap_in: ports::GetCountryByCountryCodeSoapIn,
        ) -> Result<ports::GetCountryByCountryCodeSoapOut, Option<SoapFault>> {
            let __request =
                GetCountryByCountryCodeSoapInSoapEnvelope::new(SoapGetCountryByCountryCodeSoapIn {
                    body: get_country_by_country_code_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCountryCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCountryCodeSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_isd(
            &self,
            get_isd_soap_in: ports::GetISDSoapIn,
        ) -> Result<ports::GetISDSoapOut, Option<SoapFault>> {
            let __request = GetISDSoapInSoapEnvelope::new(SoapGetISDSoapIn {
                body: get_isd_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetISD")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISDSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_countries(
            &self,
            get_countries_soap_in: ports::GetCountriesSoapIn,
        ) -> Result<ports::GetCountriesSoapOut, Option<SoapFault>> {
            let __request = GetCountriesSoapInSoapEnvelope::new(SoapGetCountriesSoapIn {
                body: get_countries_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCountries")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountriesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_soap_in: ports::GetCurrencyCodeByCurrencyNameSoapIn,
        ) -> Result<ports::GetCurrencyCodeByCurrencyNameSoapOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeByCurrencyNameSoapInSoapEnvelope::new(
                SoapGetCurrencyCodeByCurrencyNameSoapIn {
                    body: get_currency_code_by_currency_name_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyCodeByCurrencyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeByCurrencyNameSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_soap_in: ports::GetISOCountryCodeByCountyNameSoapIn,
        ) -> Result<ports::GetISOCountryCodeByCountyNameSoapOut, Option<SoapFault>> {
            let __request = GetISOCountryCodeByCountyNameSoapInSoapEnvelope::new(
                SoapGetISOCountryCodeByCountyNameSoapIn {
                    body: get_iso_country_code_by_county_name_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetISOCountryCodeByCountyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISOCountryCodeByCountyNameSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code(
            &self,
            get_currency_code_soap_in: ports::GetCurrencyCodeSoapIn,
        ) -> Result<ports::GetCurrencyCodeSoapOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeSoapInSoapEnvelope::new(SoapGetCurrencyCodeSoapIn {
                body: get_currency_code_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencyCode")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_soap_in: ports::GetCountryByCurrencyCodeSoapIn,
        ) -> Result<ports::GetCountryByCurrencyCodeSoapOut, Option<SoapFault>> {
            let __request = GetCountryByCurrencyCodeSoapInSoapEnvelope::new(
                SoapGetCountryByCurrencyCodeSoapIn {
                    body: get_country_by_currency_code_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCurrencyCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCurrencyCodeSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currencies(
            &self,
            get_currencies_soap_in: ports::GetCurrenciesSoapIn,
        ) -> Result<ports::GetCurrenciesSoapOut, Option<SoapFault>> {
            let __request = GetCurrenciesSoapInSoapEnvelope::new(SoapGetCurrenciesSoapIn {
                body: get_currencies_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencies")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrenciesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_soap_in: ports::GetCurrencyByCountrySoapIn,
        ) -> Result<ports::GetCurrencyByCountrySoapOut, Option<SoapFault>> {
            let __request =
                GetCurrencyByCountrySoapInSoapEnvelope::new(SoapGetCurrencyByCountrySoapIn {
                    body: get_currency_by_country_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyByCountry",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyByCountrySoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_soap_in: ports::GetGMTbyCountrySoapIn,
        ) -> Result<ports::GetGMTbyCountrySoapOut, Option<SoapFault>> {
            let __request = GetGMTbyCountrySoapInSoapEnvelope::new(SoapGetGMTbyCountrySoapIn {
                body: get_gm_tby_country_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetGMTbyCountry")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetGMTbyCountrySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl CountrySoap12 {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    impl Default for CountrySoap12 {
        fn default() -> Self {
            CountrySoap12 {
                client: reqwest::Client::new(),
                url: "http://www.webserviceX.NET".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl CountrySoap12 {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            CountrySoap12 {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct CountrySoap12 {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::CountrySoap for CountrySoap12 {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_soap_in: ports::GetCountryByCountryCodeSoapIn,
        ) -> Result<ports::GetCountryByCountryCodeSoapOut, Option<SoapFault>> {
            let __request =
                GetCountryByCountryCodeSoapInSoapEnvelope::new(SoapGetCountryByCountryCodeSoapIn {
                    body: get_country_by_country_code_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCountryCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCountryCodeSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_isd(
            &self,
            get_isd_soap_in: ports::GetISDSoapIn,
        ) -> Result<ports::GetISDSoapOut, Option<SoapFault>> {
            let __request = GetISDSoapInSoapEnvelope::new(SoapGetISDSoapIn {
                body: get_isd_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetISD")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISDSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_countries(
            &self,
            get_countries_soap_in: ports::GetCountriesSoapIn,
        ) -> Result<ports::GetCountriesSoapOut, Option<SoapFault>> {
            let __request = GetCountriesSoapInSoapEnvelope::new(SoapGetCountriesSoapIn {
                body: get_countries_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCountries")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountriesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_soap_in: ports::GetCurrencyCodeByCurrencyNameSoapIn,
        ) -> Result<ports::GetCurrencyCodeByCurrencyNameSoapOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeByCurrencyNameSoapInSoapEnvelope::new(
                SoapGetCurrencyCodeByCurrencyNameSoapIn {
                    body: get_currency_code_by_currency_name_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyCodeByCurrencyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeByCurrencyNameSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_soap_in: ports::GetISOCountryCodeByCountyNameSoapIn,
        ) -> Result<ports::GetISOCountryCodeByCountyNameSoapOut, Option<SoapFault>> {
            let __request = GetISOCountryCodeByCountyNameSoapInSoapEnvelope::new(
                SoapGetISOCountryCodeByCountyNameSoapIn {
                    body: get_iso_country_code_by_county_name_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetISOCountryCodeByCountyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISOCountryCodeByCountyNameSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code(
            &self,
            get_currency_code_soap_in: ports::GetCurrencyCodeSoapIn,
        ) -> Result<ports::GetCurrencyCodeSoapOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeSoapInSoapEnvelope::new(SoapGetCurrencyCodeSoapIn {
                body: get_currency_code_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencyCode")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_soap_in: ports::GetCountryByCurrencyCodeSoapIn,
        ) -> Result<ports::GetCountryByCurrencyCodeSoapOut, Option<SoapFault>> {
            let __request = GetCountryByCurrencyCodeSoapInSoapEnvelope::new(
                SoapGetCountryByCurrencyCodeSoapIn {
                    body: get_country_by_currency_code_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCurrencyCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCurrencyCodeSoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currencies(
            &self,
            get_currencies_soap_in: ports::GetCurrenciesSoapIn,
        ) -> Result<ports::GetCurrenciesSoapOut, Option<SoapFault>> {
            let __request = GetCurrenciesSoapInSoapEnvelope::new(SoapGetCurrenciesSoapIn {
                body: get_currencies_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencies")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrenciesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_soap_in: ports::GetCurrencyByCountrySoapIn,
        ) -> Result<ports::GetCurrencyByCountrySoapOut, Option<SoapFault>> {
            let __request =
                GetCurrencyByCountrySoapInSoapEnvelope::new(SoapGetCurrencyByCountrySoapIn {
                    body: get_currency_by_country_soap_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyByCountry",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyByCountrySoapOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_soap_in: ports::GetGMTbyCountrySoapIn,
        ) -> Result<ports::GetGMTbyCountrySoapOut, Option<SoapFault>> {
            let __request = GetGMTbyCountrySoapInSoapEnvelope::new(SoapGetGMTbyCountrySoapIn {
                body: get_gm_tby_country_soap_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetGMTbyCountry")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetGMTbyCountrySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl CountryHttpGet {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeHttpGetIn {
        #[yaserde(rename = "GetCountryByCountryCode", default)]
        pub body: ports::GetCountryByCountryCodeHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeHttpGetInSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeHttpGetIn,
    }

    impl GetCountryByCountryCodeHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeHttpGetIn) -> Self {
            GetCountryByCountryCodeHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountryByCountryCodeHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeHttpGetOut,
    }

    impl GetCountryByCountryCodeHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeHttpGetOut) -> Self {
            GetCountryByCountryCodeHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDHttpGetIn {
        #[yaserde(rename = "GetISD", default)]
        pub body: ports::GetISDHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDHttpGetInSoapEnvelope {
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
        pub body: SoapGetISDHttpGetIn,
    }

    impl GetISDHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetISDHttpGetIn) -> Self {
            GetISDHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetISDHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDHttpGetOutSoapEnvelope {
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
        pub body: SoapGetISDHttpGetOut,
    }

    impl GetISDHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetISDHttpGetOut) -> Self {
            GetISDHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesHttpGetIn {
        #[yaserde(rename = "GetCountries", default)]
        pub body: ports::GetCountriesHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesHttpGetInSoapEnvelope {
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
        pub body: SoapGetCountriesHttpGetIn,
    }

    impl GetCountriesHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCountriesHttpGetIn) -> Self {
            GetCountriesHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountriesHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCountriesHttpGetOut,
    }

    impl GetCountriesHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCountriesHttpGetOut) -> Self {
            GetCountriesHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameHttpGetIn {
        #[yaserde(rename = "GetCurrencyCodeByCurrencyName", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameHttpGetInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameHttpGetIn,
    }

    impl GetCurrencyCodeByCurrencyNameHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameHttpGetIn) -> Self {
            GetCurrencyCodeByCurrencyNameHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameHttpGetOut,
    }

    impl GetCurrencyCodeByCurrencyNameHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameHttpGetOut) -> Self {
            GetCurrencyCodeByCurrencyNameHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameHttpGetIn {
        #[yaserde(rename = "GetISOCountryCodeByCountyName", default)]
        pub body: ports::GetISOCountryCodeByCountyNameHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameHttpGetInSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameHttpGetIn,
    }

    impl GetISOCountryCodeByCountyNameHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameHttpGetIn) -> Self {
            GetISOCountryCodeByCountyNameHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetISOCountryCodeByCountyNameHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameHttpGetOutSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameHttpGetOut,
    }

    impl GetISOCountryCodeByCountyNameHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameHttpGetOut) -> Self {
            GetISOCountryCodeByCountyNameHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeHttpGetIn {
        #[yaserde(rename = "GetCurrencyCode", default)]
        pub body: ports::GetCurrencyCodeHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeHttpGetInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeHttpGetIn,
    }

    impl GetCurrencyCodeHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeHttpGetIn) -> Self {
            GetCurrencyCodeHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyCodeHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeHttpGetOut,
    }

    impl GetCurrencyCodeHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeHttpGetOut) -> Self {
            GetCurrencyCodeHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeHttpGetIn {
        #[yaserde(rename = "GetCountryByCurrencyCode", default)]
        pub body: ports::GetCountryByCurrencyCodeHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeHttpGetInSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeHttpGetIn,
    }

    impl GetCountryByCurrencyCodeHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeHttpGetIn) -> Self {
            GetCountryByCurrencyCodeHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountryByCurrencyCodeHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeHttpGetOut,
    }

    impl GetCountryByCurrencyCodeHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeHttpGetOut) -> Self {
            GetCountryByCurrencyCodeHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesHttpGetIn {
        #[yaserde(rename = "GetCurrencies", default)]
        pub body: ports::GetCurrenciesHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesHttpGetInSoapEnvelope {
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
        pub body: SoapGetCurrenciesHttpGetIn,
    }

    impl GetCurrenciesHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesHttpGetIn) -> Self {
            GetCurrenciesHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrenciesHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCurrenciesHttpGetOut,
    }

    impl GetCurrenciesHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesHttpGetOut) -> Self {
            GetCurrenciesHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountryHttpGetIn {
        #[yaserde(rename = "GetCurrencyByCountry", default)]
        pub body: ports::GetCurrencyByCountryHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountryHttpGetInSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountryHttpGetIn,
    }

    impl GetCurrencyByCountryHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountryHttpGetIn) -> Self {
            GetCurrencyByCountryHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountryHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyByCountryHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountryHttpGetOutSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountryHttpGetOut,
    }

    impl GetCurrencyByCountryHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountryHttpGetOut) -> Self {
            GetCurrencyByCountryHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountryHttpGetIn {
        #[yaserde(rename = "GetGMTbyCountry", default)]
        pub body: ports::GetGMTbyCountryHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountryHttpGetInSoapEnvelope {
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
        pub body: SoapGetGMTbyCountryHttpGetIn,
    }

    impl GetGMTbyCountryHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountryHttpGetIn) -> Self {
            GetGMTbyCountryHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountryHttpGetOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetGMTbyCountryHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountryHttpGetOutSoapEnvelope {
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
        pub body: SoapGetGMTbyCountryHttpGetOut,
    }

    impl GetGMTbyCountryHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountryHttpGetOut) -> Self {
            GetGMTbyCountryHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for CountryHttpGet {
        fn default() -> Self {
            CountryHttpGet {
                client: reqwest::Client::new(),
                url: "http://www.webserviceX.NET".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl CountryHttpGet {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            CountryHttpGet {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct CountryHttpGet {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::CountryHttpGet for CountryHttpGet {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_http_get_in: ports::GetCountryByCountryCodeHttpGetIn,
        ) -> Result<ports::GetCountryByCountryCodeHttpGetOut, Option<SoapFault>> {
            let __request = GetCountryByCountryCodeHttpGetInSoapEnvelope::new(
                SoapGetCountryByCountryCodeHttpGetIn {
                    body: get_country_by_country_code_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCountryCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCountryCodeHttpGetOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_isd(
            &self,
            get_isd_http_get_in: ports::GetISDHttpGetIn,
        ) -> Result<ports::GetISDHttpGetOut, Option<SoapFault>> {
            let __request = GetISDHttpGetInSoapEnvelope::new(SoapGetISDHttpGetIn {
                body: get_isd_http_get_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetISD")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISDHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_countries(
            &self,
            get_countries_http_get_in: ports::GetCountriesHttpGetIn,
        ) -> Result<ports::GetCountriesHttpGetOut, Option<SoapFault>> {
            let __request = GetCountriesHttpGetInSoapEnvelope::new(SoapGetCountriesHttpGetIn {
                body: get_countries_http_get_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCountries")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountriesHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_http_get_in: ports::GetCurrencyCodeByCurrencyNameHttpGetIn,
        ) -> Result<ports::GetCurrencyCodeByCurrencyNameHttpGetOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeByCurrencyNameHttpGetInSoapEnvelope::new(
                SoapGetCurrencyCodeByCurrencyNameHttpGetIn {
                    body: get_currency_code_by_currency_name_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyCodeByCurrencyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeByCurrencyNameHttpGetOutSoapEnvelope = from_str(&response)
                .map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_http_get_in: ports::GetISOCountryCodeByCountyNameHttpGetIn,
        ) -> Result<ports::GetISOCountryCodeByCountyNameHttpGetOut, Option<SoapFault>> {
            let __request = GetISOCountryCodeByCountyNameHttpGetInSoapEnvelope::new(
                SoapGetISOCountryCodeByCountyNameHttpGetIn {
                    body: get_iso_country_code_by_county_name_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetISOCountryCodeByCountyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISOCountryCodeByCountyNameHttpGetOutSoapEnvelope = from_str(&response)
                .map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code(
            &self,
            get_currency_code_http_get_in: ports::GetCurrencyCodeHttpGetIn,
        ) -> Result<ports::GetCurrencyCodeHttpGetOut, Option<SoapFault>> {
            let __request =
                GetCurrencyCodeHttpGetInSoapEnvelope::new(SoapGetCurrencyCodeHttpGetIn {
                    body: get_currency_code_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencyCode")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_http_get_in: ports::GetCountryByCurrencyCodeHttpGetIn,
        ) -> Result<ports::GetCountryByCurrencyCodeHttpGetOut, Option<SoapFault>> {
            let __request = GetCountryByCurrencyCodeHttpGetInSoapEnvelope::new(
                SoapGetCountryByCurrencyCodeHttpGetIn {
                    body: get_country_by_currency_code_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCurrencyCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCurrencyCodeHttpGetOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currencies(
            &self,
            get_currencies_http_get_in: ports::GetCurrenciesHttpGetIn,
        ) -> Result<ports::GetCurrenciesHttpGetOut, Option<SoapFault>> {
            let __request = GetCurrenciesHttpGetInSoapEnvelope::new(SoapGetCurrenciesHttpGetIn {
                body: get_currencies_http_get_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencies")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrenciesHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_http_get_in: ports::GetCurrencyByCountryHttpGetIn,
        ) -> Result<ports::GetCurrencyByCountryHttpGetOut, Option<SoapFault>> {
            let __request =
                GetCurrencyByCountryHttpGetInSoapEnvelope::new(SoapGetCurrencyByCountryHttpGetIn {
                    body: get_currency_by_country_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyByCountry",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyByCountryHttpGetOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_http_get_in: ports::GetGMTbyCountryHttpGetIn,
        ) -> Result<ports::GetGMTbyCountryHttpGetOut, Option<SoapFault>> {
            let __request =
                GetGMTbyCountryHttpGetInSoapEnvelope::new(SoapGetGMTbyCountryHttpGetIn {
                    body: get_gm_tby_country_http_get_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetGMTbyCountry")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetGMTbyCountryHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl CountryHttpPost {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeHttpPostIn {
        #[yaserde(rename = "GetCountryByCountryCode", default)]
        pub body: ports::GetCountryByCountryCodeHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeHttpPostInSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeHttpPostIn,
    }

    impl GetCountryByCountryCodeHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeHttpPostIn) -> Self {
            GetCountryByCountryCodeHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCountryCodeHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountryByCountryCodeHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCountryCodeHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCountryByCountryCodeHttpPostOut,
    }

    impl GetCountryByCountryCodeHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCountryCodeHttpPostOut) -> Self {
            GetCountryByCountryCodeHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDHttpPostIn {
        #[yaserde(rename = "GetISD", default)]
        pub body: ports::GetISDHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDHttpPostInSoapEnvelope {
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
        pub body: SoapGetISDHttpPostIn,
    }

    impl GetISDHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetISDHttpPostIn) -> Self {
            GetISDHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISDHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetISDHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISDHttpPostOutSoapEnvelope {
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
        pub body: SoapGetISDHttpPostOut,
    }

    impl GetISDHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetISDHttpPostOut) -> Self {
            GetISDHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesHttpPostIn {
        #[yaserde(rename = "GetCountries", default)]
        pub body: ports::GetCountriesHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesHttpPostInSoapEnvelope {
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
        pub body: SoapGetCountriesHttpPostIn,
    }

    impl GetCountriesHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCountriesHttpPostIn) -> Self {
            GetCountriesHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountriesHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountriesHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountriesHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCountriesHttpPostOut,
    }

    impl GetCountriesHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCountriesHttpPostOut) -> Self {
            GetCountriesHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameHttpPostIn {
        #[yaserde(rename = "GetCurrencyCodeByCurrencyName", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameHttpPostInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameHttpPostIn,
    }

    impl GetCurrencyCodeByCurrencyNameHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameHttpPostIn) -> Self {
            GetCurrencyCodeByCurrencyNameHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeByCurrencyNameHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyCodeByCurrencyNameHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeByCurrencyNameHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeByCurrencyNameHttpPostOut,
    }

    impl GetCurrencyCodeByCurrencyNameHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeByCurrencyNameHttpPostOut) -> Self {
            GetCurrencyCodeByCurrencyNameHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameHttpPostIn {
        #[yaserde(rename = "GetISOCountryCodeByCountyName", default)]
        pub body: ports::GetISOCountryCodeByCountyNameHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameHttpPostInSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameHttpPostIn,
    }

    impl GetISOCountryCodeByCountyNameHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameHttpPostIn) -> Self {
            GetISOCountryCodeByCountyNameHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetISOCountryCodeByCountyNameHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetISOCountryCodeByCountyNameHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetISOCountryCodeByCountyNameHttpPostOutSoapEnvelope {
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
        pub body: SoapGetISOCountryCodeByCountyNameHttpPostOut,
    }

    impl GetISOCountryCodeByCountyNameHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetISOCountryCodeByCountyNameHttpPostOut) -> Self {
            GetISOCountryCodeByCountyNameHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeHttpPostIn {
        #[yaserde(rename = "GetCurrencyCode", default)]
        pub body: ports::GetCurrencyCodeHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeHttpPostInSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeHttpPostIn,
    }

    impl GetCurrencyCodeHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeHttpPostIn) -> Self {
            GetCurrencyCodeHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyCodeHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyCodeHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyCodeHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCurrencyCodeHttpPostOut,
    }

    impl GetCurrencyCodeHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyCodeHttpPostOut) -> Self {
            GetCurrencyCodeHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeHttpPostIn {
        #[yaserde(rename = "GetCountryByCurrencyCode", default)]
        pub body: ports::GetCountryByCurrencyCodeHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeHttpPostInSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeHttpPostIn,
    }

    impl GetCountryByCurrencyCodeHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeHttpPostIn) -> Self {
            GetCountryByCurrencyCodeHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCountryByCurrencyCodeHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCountryByCurrencyCodeHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCountryByCurrencyCodeHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCountryByCurrencyCodeHttpPostOut,
    }

    impl GetCountryByCurrencyCodeHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCountryByCurrencyCodeHttpPostOut) -> Self {
            GetCountryByCurrencyCodeHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesHttpPostIn {
        #[yaserde(rename = "GetCurrencies", default)]
        pub body: ports::GetCurrenciesHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesHttpPostInSoapEnvelope {
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
        pub body: SoapGetCurrenciesHttpPostIn,
    }

    impl GetCurrenciesHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesHttpPostIn) -> Self {
            GetCurrenciesHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrenciesHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrenciesHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrenciesHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCurrenciesHttpPostOut,
    }

    impl GetCurrenciesHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCurrenciesHttpPostOut) -> Self {
            GetCurrenciesHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountryHttpPostIn {
        #[yaserde(rename = "GetCurrencyByCountry", default)]
        pub body: ports::GetCurrencyByCountryHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountryHttpPostInSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountryHttpPostIn,
    }

    impl GetCurrencyByCountryHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountryHttpPostIn) -> Self {
            GetCurrencyByCountryHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCurrencyByCountryHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetCurrencyByCountryHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCurrencyByCountryHttpPostOutSoapEnvelope {
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
        pub body: SoapGetCurrencyByCountryHttpPostOut,
    }

    impl GetCurrencyByCountryHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCurrencyByCountryHttpPostOut) -> Self {
            GetCurrencyByCountryHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountryHttpPostIn {
        #[yaserde(rename = "GetGMTbyCountry", default)]
        pub body: ports::GetGMTbyCountryHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountryHttpPostInSoapEnvelope {
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
        pub body: SoapGetGMTbyCountryHttpPostIn,
    }

    impl GetGMTbyCountryHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountryHttpPostIn) -> Self {
            GetGMTbyCountryHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetGMTbyCountryHttpPostOut {
        #[yaserde(rename = "String", default)]
        pub body: ports::GetGMTbyCountryHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetGMTbyCountryHttpPostOutSoapEnvelope {
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
        pub body: SoapGetGMTbyCountryHttpPostOut,
    }

    impl GetGMTbyCountryHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetGMTbyCountryHttpPostOut) -> Self {
            GetGMTbyCountryHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Option::Some("http://www.webserviceX.NET".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for CountryHttpPost {
        fn default() -> Self {
            CountryHttpPost {
                client: reqwest::Client::new(),
                url: "http://www.webserviceX.NET".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl CountryHttpPost {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            CountryHttpPost {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct CountryHttpPost {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::CountryHttpPost for CountryHttpPost {
        async fn get_country_by_country_code(
            &self,
            get_country_by_country_code_http_post_in: ports::GetCountryByCountryCodeHttpPostIn,
        ) -> Result<ports::GetCountryByCountryCodeHttpPostOut, Option<SoapFault>> {
            let __request = GetCountryByCountryCodeHttpPostInSoapEnvelope::new(
                SoapGetCountryByCountryCodeHttpPostIn {
                    body: get_country_by_country_code_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCountryCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCountryCodeHttpPostOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_isd(
            &self,
            get_isd_http_post_in: ports::GetISDHttpPostIn,
        ) -> Result<ports::GetISDHttpPostOut, Option<SoapFault>> {
            let __request = GetISDHttpPostInSoapEnvelope::new(SoapGetISDHttpPostIn {
                body: get_isd_http_post_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetISD")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISDHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_countries(
            &self,
            get_countries_http_post_in: ports::GetCountriesHttpPostIn,
        ) -> Result<ports::GetCountriesHttpPostOut, Option<SoapFault>> {
            let __request = GetCountriesHttpPostInSoapEnvelope::new(SoapGetCountriesHttpPostIn {
                body: get_countries_http_post_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCountries")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountriesHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code_by_currency_name(
            &self,
            get_currency_code_by_currency_name_http_post_in: ports::GetCurrencyCodeByCurrencyNameHttpPostIn,
        ) -> Result<ports::GetCurrencyCodeByCurrencyNameHttpPostOut, Option<SoapFault>> {
            let __request = GetCurrencyCodeByCurrencyNameHttpPostInSoapEnvelope::new(
                SoapGetCurrencyCodeByCurrencyNameHttpPostIn {
                    body: get_currency_code_by_currency_name_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyCodeByCurrencyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeByCurrencyNameHttpPostOutSoapEnvelope = from_str(&response)
                .map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_iso_country_code_by_county_name(
            &self,
            get_iso_country_code_by_county_name_http_post_in: ports::GetISOCountryCodeByCountyNameHttpPostIn,
        ) -> Result<ports::GetISOCountryCodeByCountyNameHttpPostOut, Option<SoapFault>> {
            let __request = GetISOCountryCodeByCountyNameHttpPostInSoapEnvelope::new(
                SoapGetISOCountryCodeByCountyNameHttpPostIn {
                    body: get_iso_country_code_by_county_name_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetISOCountryCodeByCountyName",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetISOCountryCodeByCountyNameHttpPostOutSoapEnvelope = from_str(&response)
                .map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_code(
            &self,
            get_currency_code_http_post_in: ports::GetCurrencyCodeHttpPostIn,
        ) -> Result<ports::GetCurrencyCodeHttpPostOut, Option<SoapFault>> {
            let __request =
                GetCurrencyCodeHttpPostInSoapEnvelope::new(SoapGetCurrencyCodeHttpPostIn {
                    body: get_currency_code_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencyCode")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyCodeHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_country_by_currency_code(
            &self,
            get_country_by_currency_code_http_post_in: ports::GetCountryByCurrencyCodeHttpPostIn,
        ) -> Result<ports::GetCountryByCurrencyCodeHttpPostOut, Option<SoapFault>> {
            let __request = GetCountryByCurrencyCodeHttpPostInSoapEnvelope::new(
                SoapGetCountryByCurrencyCodeHttpPostIn {
                    body: get_country_by_currency_code_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCountryByCurrencyCode",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCountryByCurrencyCodeHttpPostOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currencies(
            &self,
            get_currencies_http_post_in: ports::GetCurrenciesHttpPostIn,
        ) -> Result<ports::GetCurrenciesHttpPostOut, Option<SoapFault>> {
            let __request = GetCurrenciesHttpPostInSoapEnvelope::new(SoapGetCurrenciesHttpPostIn {
                body: get_currencies_http_post_in,
                xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetCurrencies")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrenciesHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_currency_by_country(
            &self,
            get_currency_by_country_http_post_in: ports::GetCurrencyByCountryHttpPostIn,
        ) -> Result<ports::GetCurrencyByCountryHttpPostOut, Option<SoapFault>> {
            let __request = GetCurrencyByCountryHttpPostInSoapEnvelope::new(
                SoapGetCurrencyByCountryHttpPostIn {
                    body: get_currency_by_country_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://www.webserviceX.NET/GetCurrencyByCountry",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCurrencyByCountryHttpPostOutSoapEnvelope =
                from_str(&response).map_err(|err| {
                    warn!("Failed to unmarshal SOAP response: {:?}", err);
                    None
                })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
        async fn get_gm_tby_country(
            &self,
            get_gm_tby_country_http_post_in: ports::GetGMTbyCountryHttpPostIn,
        ) -> Result<ports::GetGMTbyCountryHttpPostOut, Option<SoapFault>> {
            let __request =
                GetGMTbyCountryHttpPostInSoapEnvelope::new(SoapGetGMTbyCountryHttpPostIn {
                    body: get_gm_tby_country_http_post_in,
                    xmlns: Option::Some("http://www.webserviceX.NET".to_string()),
                });

            let (status, response) = self
                .send_soap_request(&__request, "http://www.webserviceX.NET/GetGMTbyCountry")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetGMTbyCountryHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub struct Country {}
    impl Country {
        pub fn new_client(credentials: Option<(String, String)>) -> bindings::CountrySoap {
            bindings::CountrySoap::new("http://www.webservicex.net/country.asmx", credentials)
        }
    }
}
