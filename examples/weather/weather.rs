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
pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetWeatherInformation",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetWeatherInformation {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetWeatherInformationResponse",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetWeatherInformationResponse {
        #[yaserde(prefix = "tns", rename = "GetWeatherInformationResult", default)]
        pub get_weather_information_result: Option<ArrayOfWeatherDescription>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "ArrayOfWeatherDescription",
        default
    )]
    pub struct ArrayOfWeatherDescription {
        #[yaserde(prefix = "tns", rename = "WeatherDescription", default)]
        pub weather_description: Vec<WeatherDescription>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "WeatherDescription",
        default
    )]
    pub struct WeatherDescription {
        #[yaserde(prefix = "tns", rename = "WeatherID", default)]
        pub weather_id: i16,
        #[yaserde(prefix = "tns", rename = "Description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "PictureURL", default)]
        pub picture_url: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetCityForecastByZIP",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetCityForecastByZIP {
        #[yaserde(prefix = "tns", rename = "ZIP", default)]
        pub zip: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetCityForecastByZIPResponse",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetCityForecastByZIPResponse {
        #[yaserde(prefix = "tns", rename = "GetCityForecastByZIPResult", default)]
        pub get_city_forecast_by_zip_result: Option<ForecastReturn>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "ForecastReturn",
        default
    )]
    pub struct ForecastReturn {
        #[yaserde(prefix = "tns", rename = "Success", default)]
        pub success: bool,
        #[yaserde(prefix = "tns", rename = "ResponseText", default)]
        pub response_text: Option<String>,
        #[yaserde(prefix = "tns", rename = "State", default)]
        pub state: Option<String>,
        #[yaserde(prefix = "tns", rename = "City", default)]
        pub city: Option<String>,
        #[yaserde(prefix = "tns", rename = "WeatherStationCity", default)]
        pub weather_station_city: Option<String>,
        #[yaserde(prefix = "tns", rename = "ForecastResult", default)]
        pub forecast_result: Option<ArrayOfForecast>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "ArrayOfForecast",
        default
    )]
    pub struct ArrayOfForecast {
        #[yaserde(prefix = "tns", rename = "Forecast", default)]
        pub forecast: Vec<Forecast>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "Forecast",
        default
    )]
    pub struct Forecast {
        #[yaserde(prefix = "tns", rename = "Date", default)]
        pub date: String,
        #[yaserde(prefix = "tns", rename = "WeatherID", default)]
        pub weather_id: i16,
        #[yaserde(prefix = "tns", rename = "Desciption", default)]
        pub desciption: Option<String>,
        #[yaserde(prefix = "tns", rename = "Temperatures", default)]
        pub temperatures: Temp,
        #[yaserde(prefix = "tns", rename = "ProbabilityOfPrecipiation", default)]
        pub probability_of_precipiation: Pop,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "temp",
        default
    )]
    pub struct Temp {
        #[yaserde(prefix = "tns", rename = "MorningLow", default)]
        pub morning_low: Option<String>,
        #[yaserde(prefix = "tns", rename = "DaytimeHigh", default)]
        pub daytime_high: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "POP",
        default
    )]
    pub struct Pop {
        #[yaserde(prefix = "tns", rename = "Nighttime", default)]
        pub nighttime: Option<String>,
        #[yaserde(prefix = "tns", rename = "Daytime", default)]
        pub daytime: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetCityWeatherByZIP",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetCityWeatherByZIP {
        #[yaserde(prefix = "tns", rename = "ZIP", default)]
        pub zip: Option<String>,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        root = "GetCityWeatherByZIPResponse",
        default,
        namespace = "http://ws.cdyne.com/WeatherWS/",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct GetCityWeatherByZIPResponse {
        #[yaserde(prefix = "tns", rename = "GetCityWeatherByZIPResult", default)]
        pub get_city_weather_by_zip_result: WeatherReturn,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "tns",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        root = "WeatherReturn",
        default
    )]
    pub struct WeatherReturn {
        #[yaserde(prefix = "tns", rename = "Success", default)]
        pub success: bool,
        #[yaserde(prefix = "tns", rename = "ResponseText", default)]
        pub response_text: Option<String>,
        #[yaserde(prefix = "tns", rename = "State", default)]
        pub state: Option<String>,
        #[yaserde(prefix = "tns", rename = "City", default)]
        pub city: Option<String>,
        #[yaserde(prefix = "tns", rename = "WeatherStationCity", default)]
        pub weather_station_city: Option<String>,
        #[yaserde(prefix = "tns", rename = "WeatherID", default)]
        pub weather_id: i16,
        #[yaserde(prefix = "tns", rename = "Description", default)]
        pub description: Option<String>,
        #[yaserde(prefix = "tns", rename = "Temperature", default)]
        pub temperature: Option<String>,
        #[yaserde(prefix = "tns", rename = "RelativeHumidity", default)]
        pub relative_humidity: Option<String>,
        #[yaserde(prefix = "tns", rename = "Wind", default)]
        pub wind: Option<String>,
        #[yaserde(prefix = "tns", rename = "Pressure", default)]
        pub pressure: Option<String>,
        #[yaserde(prefix = "tns", rename = "Visibility", default)]
        pub visibility: Option<String>,
        #[yaserde(prefix = "tns", rename = "WindChill", default)]
        pub wind_chill: Option<String>,
        #[yaserde(prefix = "tns", rename = "Remarks", default)]
        pub remarks: Option<String>,
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl WeatherSoap {
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
                    Option::from(credentials.1.to_string()),
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
    pub struct WeatherSoap {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::WeatherSoap for WeatherSoap {
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: ports::GetWeatherInformationSoapIn,
        ) -> Result<ports::GetWeatherInformationSoapOut, Option<SoapFault>> {
            let __request =
                GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
                    body: get_weather_information_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetWeatherInformation",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationSoapOutSoapEnvelope =
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
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn,
        ) -> Result<ports::GetCityForecastByZIPSoapOut, Option<SoapFault>> {
            let __request =
                GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
                    body: get_city_forecast_by_zip_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetCityForecastByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPSoapOutSoapEnvelope =
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
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn,
        ) -> Result<ports::GetCityWeatherByZIPSoapOut, Option<SoapFault>> {
            let __request =
                GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
                    body: get_city_weather_by_zip_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetCityWeatherByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
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

    impl Default for WeatherSoap {
        fn default() -> Self {
            WeatherSoap {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl WeatherSoap {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherSoap {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationSoapIn {
        #[yaserde(rename = "GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationSoapIn,
    }

    impl GetWeatherInformationSoapInSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationSoapIn) -> Self {
            GetWeatherInformationSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationSoapOut {
        #[yaserde(rename = "GetWeatherInformationSoapOut", default)]
        pub body: ports::GetWeatherInformationSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationSoapOut,
    }

    impl GetWeatherInformationSoapOutSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationSoapOut) -> Self {
            GetWeatherInformationSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPSoapIn {
        #[yaserde(rename = "GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPSoapIn,
    }

    impl GetCityForecastByZIPSoapInSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPSoapIn) -> Self {
            GetCityForecastByZIPSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPSoapOut {
        #[yaserde(rename = "GetCityForecastByZIPSoapOut", default)]
        pub body: ports::GetCityForecastByZIPSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPSoapOut,
    }

    impl GetCityForecastByZIPSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPSoapOut) -> Self {
            GetCityForecastByZIPSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPSoapIn {
        #[yaserde(rename = "GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPSoapIn,
    }

    impl GetCityWeatherByZIPSoapInSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPSoapIn) -> Self {
            GetCityWeatherByZIPSoapInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPSoapOut {
        #[yaserde(rename = "GetCityWeatherByZIPSoapOut", default)]
        pub body: ports::GetCityWeatherByZIPSoapOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPSoapOut,
    }

    impl GetCityWeatherByZIPSoapOutSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPSoapOut) -> Self {
            GetCityWeatherByZIPSoapOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl WeatherSoap12 {
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
                    Option::from(credentials.1.to_string()),
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
    pub struct WeatherSoap12 {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::WeatherSoap for WeatherSoap12 {
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: ports::GetWeatherInformationSoapIn,
        ) -> Result<ports::GetWeatherInformationSoapOut, Option<SoapFault>> {
            let __request =
                GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
                    body: get_weather_information_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetWeatherInformation",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationSoapOutSoapEnvelope =
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
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn,
        ) -> Result<ports::GetCityForecastByZIPSoapOut, Option<SoapFault>> {
            let __request =
                GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
                    body: get_city_forecast_by_zip_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetCityForecastByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPSoapOutSoapEnvelope =
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
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn,
        ) -> Result<ports::GetCityWeatherByZIPSoapOut, Option<SoapFault>> {
            let __request =
                GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
                    body: get_city_weather_by_zip_soap_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS/GetCityWeatherByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
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

    impl Default for WeatherSoap12 {
        fn default() -> Self {
            WeatherSoap12 {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl WeatherSoap12 {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherSoap12 {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }

    impl WeatherHttpGet {
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
                    Option::from(credentials.1.to_string()),
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
    pub struct WeatherHttpGet {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::WeatherHttpGet for WeatherHttpGet {
        async fn get_weather_information(
            &self,
            get_weather_information_http_get_in: ports::GetWeatherInformationHttpGetIn,
        ) -> Result<ports::GetWeatherInformationHttpGetOut, Option<SoapFault>> {
            let __request = GetWeatherInformationHttpGetInSoapEnvelope::new(
                SoapGetWeatherInformationHttpGetIn {
                    body: get_weather_information_http_get_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetWeatherInformation",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationHttpGetOutSoapEnvelope =
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
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_get_in: ports::GetCityForecastByZIPHttpGetIn,
        ) -> Result<ports::GetCityForecastByZIPHttpGetOut, Option<SoapFault>> {
            let __request =
                GetCityForecastByZIPHttpGetInSoapEnvelope::new(SoapGetCityForecastByZIPHttpGetIn {
                    body: get_city_forecast_by_zip_http_get_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetCityForecastByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPHttpGetOutSoapEnvelope =
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
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_get_in: ports::GetCityWeatherByZIPHttpGetIn,
        ) -> Result<ports::GetCityWeatherByZIPHttpGetOut, Option<SoapFault>> {
            let __request =
                GetCityWeatherByZIPHttpGetInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpGetIn {
                    body: get_city_weather_by_zip_http_get_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetCityWeatherByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPHttpGetOutSoapEnvelope =
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
    }

    impl Default for WeatherHttpGet {
        fn default() -> Self {
            WeatherHttpGet {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl WeatherHttpGet {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherHttpGet {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpGetIn {
        #[yaserde(rename = "GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpGetIn,
    }

    impl GetWeatherInformationHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationHttpGetIn) -> Self {
            GetWeatherInformationHttpGetInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpGetOut {
        #[yaserde(rename = "GetWeatherInformationHttpGetOut", default)]
        pub body: ports::GetWeatherInformationHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpGetOut,
    }

    impl GetWeatherInformationHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationHttpGetOut) -> Self {
            GetWeatherInformationHttpGetOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpGetIn {
        #[yaserde(rename = "GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpGetIn,
    }

    impl GetCityForecastByZIPHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPHttpGetIn) -> Self {
            GetCityForecastByZIPHttpGetInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpGetOut {
        #[yaserde(rename = "GetCityForecastByZIPHttpGetOut", default)]
        pub body: ports::GetCityForecastByZIPHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpGetOut,
    }

    impl GetCityForecastByZIPHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPHttpGetOut) -> Self {
            GetCityForecastByZIPHttpGetOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpGetIn {
        #[yaserde(rename = "GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpGetIn,
    }

    impl GetCityWeatherByZIPHttpGetInSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPHttpGetIn) -> Self {
            GetCityWeatherByZIPHttpGetInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpGetOut {
        #[yaserde(rename = "GetCityWeatherByZIPHttpGetOut", default)]
        pub body: ports::GetCityWeatherByZIPHttpGetOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpGetOut,
    }

    impl GetCityWeatherByZIPHttpGetOutSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPHttpGetOut) -> Self {
            GetCityWeatherByZIPHttpGetOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl WeatherHttpPost {
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
                    Option::from(credentials.1.to_string()),
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
    pub struct WeatherHttpPost {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }

    #[async_trait]
    impl ports::WeatherHttpPost for WeatherHttpPost {
        async fn get_weather_information(
            &self,
            get_weather_information_http_post_in: ports::GetWeatherInformationHttpPostIn,
        ) -> Result<ports::GetWeatherInformationHttpPostOut, Option<SoapFault>> {
            let __request = GetWeatherInformationHttpPostInSoapEnvelope::new(
                SoapGetWeatherInformationHttpPostIn {
                    body: get_weather_information_http_post_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetWeatherInformation",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationHttpPostOutSoapEnvelope =
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
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_post_in: ports::GetCityForecastByZIPHttpPostIn,
        ) -> Result<ports::GetCityForecastByZIPHttpPostOut, Option<SoapFault>> {
            let __request = GetCityForecastByZIPHttpPostInSoapEnvelope::new(
                SoapGetCityForecastByZIPHttpPostIn {
                    body: get_city_forecast_by_zip_http_post_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                },
            );

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetCityForecastByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPHttpPostOutSoapEnvelope =
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
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_post_in: ports::GetCityWeatherByZIPHttpPostIn,
        ) -> Result<ports::GetCityWeatherByZIPHttpPostOut, Option<SoapFault>> {
            let __request =
                GetCityWeatherByZIPHttpPostInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpPostIn {
                    body: get_city_weather_by_zip_http_post_in,
                    xmlns: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                });

            let (status, response) = self
                .send_soap_request(
                    &__request,
                    "http://ws.cdyne.com/WeatherWS//GetCityWeatherByZIP",
                )
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPHttpPostOutSoapEnvelope =
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
    }

    impl Default for WeatherHttpPost {
        fn default() -> Self {
            WeatherHttpPost {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl WeatherHttpPost {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherHttpPost {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpPostIn {
        #[yaserde(rename = "GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpPostIn,
    }

    impl GetWeatherInformationHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationHttpPostIn) -> Self {
            GetWeatherInformationHttpPostInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpPostOut {
        #[yaserde(rename = "GetWeatherInformationHttpPostOut", default)]
        pub body: ports::GetWeatherInformationHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpPostOut,
    }

    impl GetWeatherInformationHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetWeatherInformationHttpPostOut) -> Self {
            GetWeatherInformationHttpPostOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpPostIn {
        #[yaserde(rename = "GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpPostIn,
    }

    impl GetCityForecastByZIPHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPHttpPostIn) -> Self {
            GetCityForecastByZIPHttpPostInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpPostOut {
        #[yaserde(rename = "GetCityForecastByZIPHttpPostOut", default)]
        pub body: ports::GetCityForecastByZIPHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpPostOut,
    }

    impl GetCityForecastByZIPHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCityForecastByZIPHttpPostOut) -> Self {
            GetCityForecastByZIPHttpPostOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpPostIn {
        #[yaserde(rename = "GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpPostIn,
    }

    impl GetCityWeatherByZIPHttpPostInSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPHttpPostIn) -> Self {
            GetCityWeatherByZIPHttpPostInSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpPostOut {
        #[yaserde(rename = "GetCityWeatherByZIPHttpPostOut", default)]
        pub body: ports::GetCityWeatherByZIPHttpPostOut,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        root = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpPostOut,
    }

    impl GetCityWeatherByZIPHttpPostOutSoapEnvelope {
        pub fn new(body: SoapGetCityWeatherByZIPHttpPostOut) -> Self {
            GetCityWeatherByZIPHttpPostOutSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::from("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[async_trait]
    pub trait WeatherSoap {
        /// Gets Information for each WeatherID
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: GetWeatherInformationSoapIn,
        ) -> Result<GetWeatherInformationSoapOut, Option<SoapFault>>;
        /// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: GetCityForecastByZIPSoapIn,
        ) -> Result<GetCityForecastByZIPSoapOut, Option<SoapFault>>;
        /// Allows you to get your City's Weather, which is updated hourly. U.S. Only
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: GetCityWeatherByZIPSoapIn,
        ) -> Result<GetCityWeatherByZIPSoapOut, Option<SoapFault>>;
    }

    pub type GetWeatherInformationSoapIn = messages::GetWeatherInformationSoapIn;
    pub type GetWeatherInformationSoapOut = messages::GetWeatherInformationSoapOut;
    pub type GetCityForecastByZIPSoapIn = messages::GetCityForecastByZIPSoapIn;
    pub type GetCityForecastByZIPSoapOut = messages::GetCityForecastByZIPSoapOut;
    pub type GetCityWeatherByZIPSoapIn = messages::GetCityWeatherByZIPSoapIn;
    pub type GetCityWeatherByZIPSoapOut = messages::GetCityWeatherByZIPSoapOut;
    #[async_trait]
    pub trait WeatherHttpGet {
        /// Gets Information for each WeatherID
        async fn get_weather_information(
            &self,
            get_weather_information_http_get_in: GetWeatherInformationHttpGetIn,
        ) -> Result<GetWeatherInformationHttpGetOut, Option<SoapFault>>;
        /// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_get_in: GetCityForecastByZIPHttpGetIn,
        ) -> Result<GetCityForecastByZIPHttpGetOut, Option<SoapFault>>;
        /// Allows you to get your City's Weather, which is updated hourly. U.S. Only
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_get_in: GetCityWeatherByZIPHttpGetIn,
        ) -> Result<GetCityWeatherByZIPHttpGetOut, Option<SoapFault>>;
    }

    pub type GetWeatherInformationHttpGetIn = messages::GetWeatherInformationHttpGetIn;
    pub type GetWeatherInformationHttpGetOut = messages::GetWeatherInformationHttpGetOut;
    pub type GetCityForecastByZIPHttpGetIn = messages::GetCityForecastByZIPHttpGetIn;
    pub type GetCityForecastByZIPHttpGetOut = messages::GetCityForecastByZIPHttpGetOut;
    pub type GetCityWeatherByZIPHttpGetIn = messages::GetCityWeatherByZIPHttpGetIn;
    pub type GetCityWeatherByZIPHttpGetOut = messages::GetCityWeatherByZIPHttpGetOut;
    #[async_trait]
    pub trait WeatherHttpPost {
        /// Gets Information for each WeatherID
        async fn get_weather_information(
            &self,
            get_weather_information_http_post_in: GetWeatherInformationHttpPostIn,
        ) -> Result<GetWeatherInformationHttpPostOut, Option<SoapFault>>;
        /// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_post_in: GetCityForecastByZIPHttpPostIn,
        ) -> Result<GetCityForecastByZIPHttpPostOut, Option<SoapFault>>;
        /// Allows you to get your City's Weather, which is updated hourly. U.S. Only
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_post_in: GetCityWeatherByZIPHttpPostIn,
        ) -> Result<GetCityWeatherByZIPHttpPostOut, Option<SoapFault>>;
    }

    pub type GetWeatherInformationHttpPostIn = messages::GetWeatherInformationHttpPostIn;
    pub type GetWeatherInformationHttpPostOut = messages::GetWeatherInformationHttpPostOut;
    pub type GetCityForecastByZIPHttpPostIn = messages::GetCityForecastByZIPHttpPostIn;
    pub type GetCityForecastByZIPHttpPostOut = messages::GetCityForecastByZIPHttpPostOut;
    pub type GetCityWeatherByZIPHttpPostIn = messages::GetCityWeatherByZIPHttpPostIn;
    pub type GetCityWeatherByZIPHttpPostOut = messages::GetCityWeatherByZIPHttpPostOut;
}

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationSoapIn", default)]
    pub struct GetWeatherInformationSoapIn {
        #[yaserde(flatten)]
        pub parameters: types::GetWeatherInformation,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationSoapOut", default)]
    pub struct GetWeatherInformationSoapOut {
        #[yaserde(flatten)]
        pub parameters: types::GetWeatherInformationResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPSoapIn", default)]
    pub struct GetCityForecastByZIPSoapIn {
        #[yaserde(flatten)]
        pub parameters: types::GetCityForecastByZIP,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPSoapOut", default)]
    pub struct GetCityForecastByZIPSoapOut {
        #[yaserde(flatten)]
        pub parameters: types::GetCityForecastByZIPResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPSoapIn", default)]
    pub struct GetCityWeatherByZIPSoapIn {
        #[yaserde(flatten)]
        pub parameters: types::GetCityWeatherByZIP,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPSoapOut", default)]
    pub struct GetCityWeatherByZIPSoapOut {
        #[yaserde(flatten)]
        pub parameters: types::GetCityWeatherByZIPResponse,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationHttpGetIn", default)]
    pub struct GetWeatherInformationHttpGetIn {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationHttpGetOut", default)]
    pub struct GetWeatherInformationHttpGetOut {
        #[yaserde(flatten)]
        pub body: types::ArrayOfWeatherDescription,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPHttpGetIn", default)]
    pub struct GetCityForecastByZIPHttpGetIn {
        #[yaserde(rename = "ZIP")]
        pub zip: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPHttpGetOut", default)]
    pub struct GetCityForecastByZIPHttpGetOut {
        #[yaserde(flatten)]
        pub body: types::ForecastReturn,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPHttpGetIn", default)]
    pub struct GetCityWeatherByZIPHttpGetIn {
        #[yaserde(rename = "ZIP")]
        pub zip: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPHttpGetOut", default)]
    pub struct GetCityWeatherByZIPHttpGetOut {
        #[yaserde(flatten)]
        pub body: types::WeatherReturn,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationHttpPostIn", default)]
    pub struct GetWeatherInformationHttpPostIn {}

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetWeatherInformationHttpPostOut", default)]
    pub struct GetWeatherInformationHttpPostOut {
        #[yaserde(flatten)]
        pub body: types::ArrayOfWeatherDescription,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPHttpPostIn", default)]
    pub struct GetCityForecastByZIPHttpPostIn {
        #[yaserde(rename = "ZIP")]
        pub zip: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityForecastByZIPHttpPostOut", default)]
    pub struct GetCityForecastByZIPHttpPostOut {
        #[yaserde(flatten)]
        pub body: types::ForecastReturn,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPHttpPostIn", default)]
    pub struct GetCityWeatherByZIPHttpPostIn {
        #[yaserde(rename = "ZIP")]
        pub zip: String,
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "GetCityWeatherByZIPHttpPostOut", default)]
    pub struct GetCityWeatherByZIPHttpPostOut {
        #[yaserde(flatten)]
        pub body: types::WeatherReturn,
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct Weather {}
    impl Weather {
        pub fn new_client(credentials: Option<(String, String)>) -> bindings::WeatherSoap {
            bindings::WeatherSoap::new("http://wsf.cdyne.com/WeatherWS/Weather.asmx", credentials)
        }
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

type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;
