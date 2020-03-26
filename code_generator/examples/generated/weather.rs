pub mod ports {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use std::time::SystemTime;
            use super::*;

#[async_trait]
pub trait WeatherSoap {
	/// Gets Information for each WeatherID
	async fn get_weather_information (&mut self, get_weather_information_soap_in: GetWeatherInformationSoapIn) -> GetWeatherInformationSoapOut;
	/// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_soap_in: GetCityForecastByZIPSoapIn) -> GetCityForecastByZIPSoapOut;
	/// Allows you to get your City's Weather, which is updated hourly. U.S. Only
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_soap_in: GetCityWeatherByZIPSoapIn) -> GetCityWeatherByZIPSoapOut;
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
	async fn get_weather_information (&mut self, get_weather_information_http_get_in: GetWeatherInformationHttpGetIn) -> GetWeatherInformationHttpGetOut;
	/// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_http_get_in: GetCityForecastByZIPHttpGetIn) -> GetCityForecastByZIPHttpGetOut;
	/// Allows you to get your City's Weather, which is updated hourly. U.S. Only
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_http_get_in: GetCityWeatherByZIPHttpGetIn) -> GetCityWeatherByZIPHttpGetOut;
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
	async fn get_weather_information (&mut self, get_weather_information_http_post_in: GetWeatherInformationHttpPostIn) -> GetWeatherInformationHttpPostOut;
	/// Allows you to get your City Forecast Over the Next 7 Days, which is updated hourly. U.S. Only
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_http_post_in: GetCityForecastByZIPHttpPostIn) -> GetCityForecastByZIPHttpPostOut;
	/// Allows you to get your City's Weather, which is updated hourly. U.S. Only
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_http_post_in: GetCityWeatherByZIPHttpPostIn) -> GetCityWeatherByZIPHttpPostOut;
}

pub type GetWeatherInformationHttpPostIn = messages::GetWeatherInformationHttpPostIn;
pub type GetWeatherInformationHttpPostOut = messages::GetWeatherInformationHttpPostOut;
pub type GetCityForecastByZIPHttpPostIn = messages::GetCityForecastByZIPHttpPostIn;
pub type GetCityForecastByZIPHttpPostOut = messages::GetCityForecastByZIPHttpPostOut;
pub type GetCityWeatherByZIPHttpPostIn = messages::GetCityWeatherByZIPHttpPostIn;
pub type GetCityWeatherByZIPHttpPostOut = messages::GetCityWeatherByZIPHttpPostOut;
}

pub mod messages {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use std::time::SystemTime;
            use super::*;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationSoapIn", default)]
pub struct GetWeatherInformationSoapIn {
	#[yaserde(flatten)]
	pub parameters: types::GetWeatherInformation,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationSoapOut", default)]
pub struct GetWeatherInformationSoapOut {
	#[yaserde(flatten)]
	pub parameters: types::GetWeatherInformationResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPSoapIn", default)]
pub struct GetCityForecastByZIPSoapIn {
	#[yaserde(flatten)]
	pub parameters: types::GetCityForecastByZIP,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPSoapOut", default)]
pub struct GetCityForecastByZIPSoapOut {
	#[yaserde(flatten)]
	pub parameters: types::GetCityForecastByZIPResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPSoapIn", default)]
pub struct GetCityWeatherByZIPSoapIn {
	#[yaserde(flatten)]
	pub parameters: types::GetCityWeatherByZIP,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPSoapOut", default)]
pub struct GetCityWeatherByZIPSoapOut {
	#[yaserde(flatten)]
	pub parameters: types::GetCityWeatherByZIPResponse,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationHttpGetIn", default)]
pub struct GetWeatherInformationHttpGetIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationHttpGetOut", default)]
pub struct GetWeatherInformationHttpGetOut {
	#[yaserde(flatten)]
	pub body: types::ArrayOfWeatherDescription,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPHttpGetIn", default)]
pub struct GetCityForecastByZIPHttpGetIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPHttpGetOut", default)]
pub struct GetCityForecastByZIPHttpGetOut {
	#[yaserde(flatten)]
	pub body: types::ForecastReturn,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPHttpGetIn", default)]
pub struct GetCityWeatherByZIPHttpGetIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPHttpGetOut", default)]
pub struct GetCityWeatherByZIPHttpGetOut {
	#[yaserde(flatten)]
	pub body: types::WeatherReturn,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationHttpPostIn", default)]
pub struct GetWeatherInformationHttpPostIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetWeatherInformationHttpPostOut", default)]
pub struct GetWeatherInformationHttpPostOut {
	#[yaserde(flatten)]
	pub body: types::ArrayOfWeatherDescription,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPHttpPostIn", default)]
pub struct GetCityForecastByZIPHttpPostIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityForecastByZIPHttpPostOut", default)]
pub struct GetCityForecastByZIPHttpPostOut {
	#[yaserde(flatten)]
	pub body: types::ForecastReturn,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPHttpPostIn", default)]
pub struct GetCityWeatherByZIPHttpPostIn {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GetCityWeatherByZIPHttpPostOut", default)]
pub struct GetCityWeatherByZIPHttpPostOut {
	#[yaserde(flatten)]
	pub body: types::WeatherReturn,
}

}

use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct Header {}
    pub mod types {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use std::time::SystemTime;
            use super::*;

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetWeatherInformation", default)]
pub struct GetWeatherInformation {
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetWeatherInformationResponse", default)]
pub struct GetWeatherInformationResponse {
	#[yaserde(prefix = "ns", rename = "GetWeatherInformationResult", default)]
	pub get_weather_information_result: Vec<ArrayOfWeatherDescription>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "ArrayOfWeatherDescription", default)]
pub struct ArrayOfWeatherDescription {
	#[yaserde(prefix = "ns", rename = "WeatherDescription", default)]
	pub weather_description: Vec<WeatherDescription>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "WeatherDescription", default)]
pub struct WeatherDescription {
	#[yaserde(prefix = "ns", rename = "WeatherID", default)]
	pub weather_id: Vec<u8>,
	#[yaserde(prefix = "ns", rename = "Description", default)]
	pub description: Vec<String>,
	#[yaserde(prefix = "ns", rename = "PictureURL", default)]
	pub picture_url: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetCityForecastByZIP", default)]
pub struct GetCityForecastByZIP {
	#[yaserde(prefix = "ns", rename = "ZIP", default)]
	pub zip: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetCityForecastByZIPResponse", default)]
pub struct GetCityForecastByZIPResponse {
	#[yaserde(prefix = "ns", rename = "GetCityForecastByZIPResult", default)]
	pub get_city_forecast_by_zip_result: Vec<ForecastReturn>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "ForecastReturn", default)]
pub struct ForecastReturn {
	#[yaserde(prefix = "ns", rename = "Success", default)]
	pub success: Vec<bool>,
	#[yaserde(prefix = "ns", rename = "ResponseText", default)]
	pub response_text: Vec<String>,
	#[yaserde(prefix = "ns", rename = "State", default)]
	pub state: Vec<String>,
	#[yaserde(prefix = "ns", rename = "City", default)]
	pub city: Vec<String>,
	#[yaserde(prefix = "ns", rename = "WeatherStationCity", default)]
	pub weather_station_city: Vec<String>,
	#[yaserde(prefix = "ns", rename = "ForecastResult", default)]
	pub forecast_result: Vec<ArrayOfForecast>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "ArrayOfForecast", default)]
pub struct ArrayOfForecast {
	#[yaserde(prefix = "ns", rename = "Forecast", default)]
	pub forecast: Vec<Forecast>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "Forecast", default)]
pub struct Forecast {
	#[yaserde(prefix = "ns", rename = "Date", default)]
	pub date: Vec<String>,
	#[yaserde(prefix = "ns", rename = "WeatherID", default)]
	pub weather_id: Vec<u8>,
	#[yaserde(prefix = "ns", rename = "Desciption", default)]
	pub desciption: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Temperatures", default)]
	pub temperatures: Vec<Temp>,
	#[yaserde(prefix = "ns", rename = "ProbabilityOfPrecipiation", default)]
	pub probability_of_precipiation: Vec<Pop>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "temp", default)]
pub struct Temp {
	#[yaserde(prefix = "ns", rename = "MorningLow", default)]
	pub morning_low: Vec<String>,
	#[yaserde(prefix = "ns", rename = "DaytimeHigh", default)]
	pub daytime_high: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "POP", default)]
pub struct Pop {
	#[yaserde(prefix = "ns", rename = "Nighttime", default)]
	pub nighttime: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Daytime", default)]
	pub daytime: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetCityWeatherByZIP", default)]
pub struct GetCityWeatherByZIP {
	#[yaserde(prefix = "ns", rename = "ZIP", default)]
	pub zip: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "GetCityWeatherByZIPResponse", default)]
pub struct GetCityWeatherByZIPResponse {
	#[yaserde(prefix = "ns", rename = "GetCityWeatherByZIPResult", default)]
	pub get_city_weather_by_zip_result: Vec<WeatherReturn>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "ns", namespace = "ns: http://ws.cdyne.com/WeatherWS/", rename = "WeatherReturn", default)]
pub struct WeatherReturn {
	#[yaserde(prefix = "ns", rename = "Success", default)]
	pub success: Vec<bool>,
	#[yaserde(prefix = "ns", rename = "ResponseText", default)]
	pub response_text: Vec<String>,
	#[yaserde(prefix = "ns", rename = "State", default)]
	pub state: Vec<String>,
	#[yaserde(prefix = "ns", rename = "City", default)]
	pub city: Vec<String>,
	#[yaserde(prefix = "ns", rename = "WeatherStationCity", default)]
	pub weather_station_city: Vec<String>,
	#[yaserde(prefix = "ns", rename = "WeatherID", default)]
	pub weather_id: Vec<u8>,
	#[yaserde(prefix = "ns", rename = "Description", default)]
	pub description: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Temperature", default)]
	pub temperature: Vec<String>,
	#[yaserde(prefix = "ns", rename = "RelativeHumidity", default)]
	pub relative_humidity: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Wind", default)]
	pub wind: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Pressure", default)]
	pub pressure: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Visibility", default)]
	pub visibility: Vec<String>,
	#[yaserde(prefix = "ns", rename = "WindChill", default)]
	pub wind_chill: Vec<String>,
	#[yaserde(prefix = "ns", rename = "Remarks", default)]
	pub remarks: Vec<String>,
}

}

pub mod bindings {
use yaserde::{{YaSerialize, YaDeserialize}};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use std::time::SystemTime;
            use super::*;

pub struct WeatherSoap {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                
                #[async_trait]
                impl ports::WeatherSoap for WeatherSoap {
                	async fn get_weather_information (&mut self, get_weather_information_soap_in: ports::GetWeatherInformationSoapIn) -> ports::GetWeatherInformationSoapOut {

        let __request = GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
            body: get_weather_information_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetWeatherInformation",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetWeatherInformationSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn) -> ports::GetCityForecastByZIPSoapOut {

        let __request = GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
            body: get_city_forecast_by_zip_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityForecastByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityForecastByZIPSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn) -> ports::GetCityWeatherByZIPSoapOut {

        let __request = GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
            body: get_city_weather_by_zip_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityWeatherByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
}

impl Default for WeatherSoap {
                fn default() -> Self {
                    WeatherSoap {
                        client: reqwest::Client::new(),
                        url: String::new(),
                        credentials: Option::None,
                     }
                }
            }
            impl WeatherSoap {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
pub struct WeatherSoap12 {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                
                #[async_trait]
                impl ports::WeatherSoap for WeatherSoap12 {
                	async fn get_weather_information (&mut self, get_weather_information_soap_in: ports::GetWeatherInformationSoapIn) -> ports::GetWeatherInformationSoapOut {

        let __request = GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
            body: get_weather_information_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetWeatherInformation",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetWeatherInformationSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn) -> ports::GetCityForecastByZIPSoapOut {

        let __request = GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
            body: get_city_forecast_by_zip_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityForecastByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityForecastByZIPSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn) -> ports::GetCityWeatherByZIPSoapOut {

        let __request = GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
            body: get_city_weather_by_zip_soap_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityWeatherByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
}

impl Default for WeatherSoap12 {
                fn default() -> Self {
                    WeatherSoap12 {
                        client: reqwest::Client::new(),
                        url: String::new(),
                        credentials: Option::None,
                     }
                }
            }
            impl WeatherSoap12 {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
                    WeatherSoap12 {
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }
                }
        }
        pub struct WeatherHttpGet {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                
                #[async_trait]
                impl ports::WeatherHttpGet for WeatherHttpGet {
                	async fn get_weather_information (&mut self, get_weather_information_http_get_in: ports::GetWeatherInformationHttpGetIn) -> ports::GetWeatherInformationHttpGetOut {

        let __request = GetWeatherInformationHttpGetInSoapEnvelope::new(SoapGetWeatherInformationHttpGetIn {
            body: get_weather_information_http_get_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetWeatherInformation",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetWeatherInformationHttpGetOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_http_get_in: ports::GetCityForecastByZIPHttpGetIn) -> ports::GetCityForecastByZIPHttpGetOut {

        let __request = GetCityForecastByZIPHttpGetInSoapEnvelope::new(SoapGetCityForecastByZIPHttpGetIn {
            body: get_city_forecast_by_zip_http_get_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityForecastByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityForecastByZIPHttpGetOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_http_get_in: ports::GetCityWeatherByZIPHttpGetIn) -> ports::GetCityWeatherByZIPHttpGetOut {

        let __request = GetCityWeatherByZIPHttpGetInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpGetIn {
            body: get_city_weather_by_zip_http_get_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityWeatherByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityWeatherByZIPHttpGetOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
}

impl Default for WeatherHttpGet {
                fn default() -> Self {
                    WeatherHttpGet {
                        client: reqwest::Client::new(),
                        url: String::new(),
                        credentials: Option::None,
                     }
                }
            }
            impl WeatherHttpGet {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
pub struct WeatherHttpPost {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                
                #[async_trait]
                impl ports::WeatherHttpPost for WeatherHttpPost {
                	async fn get_weather_information (&mut self, get_weather_information_http_post_in: ports::GetWeatherInformationHttpPostIn) -> ports::GetWeatherInformationHttpPostOut {

        let __request = GetWeatherInformationHttpPostInSoapEnvelope::new(SoapGetWeatherInformationHttpPostIn {
            body: get_weather_information_http_post_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetWeatherInformation",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetWeatherInformationHttpPostOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_forecast_by_zip (&mut self, get_city_forecast_by_zip_http_post_in: ports::GetCityForecastByZIPHttpPostIn) -> ports::GetCityForecastByZIPHttpPostOut {

        let __request = GetCityForecastByZIPHttpPostInSoapEnvelope::new(SoapGetCityForecastByZIPHttpPostIn {
            body: get_city_forecast_by_zip_http_post_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityForecastByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityForecastByZIPHttpPostOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
	async fn get_city_weather_by_zip (&mut self, get_city_weather_by_zip_http_post_in: ports::GetCityWeatherByZIPHttpPostIn) -> ports::GetCityWeatherByZIPHttpPostOut {

        let __request = GetCityWeatherByZIPHttpPostInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpPostIn {
            body: get_city_weather_by_zip_http_post_in,
        });            
        
        let body = to_string(&__request).expect("failed to generate xml");
        
        let mut req = self.client
        .post("http://localhost:9800/webservices/services/AicAgentAdmin")
        .body(body)
        .header("Content-Type", "text/xml")
        .header(
            "Soapaction",
            "http://xml.avaya.com/ws/AgentAdmin/InteractionCenter/71/GetCityWeatherByZIP",
        );
        
        if let Some(credentials) = &self.credentials {
            req = req.basic_auth(credentials.0.to_string(), Option::from(credentials.1.to_string()));    
        }
        
        let res = req.send()
            .await
            .expect("can not send request");
        
        let txt = res.text().await.unwrap_or_default();
        
        let r: GetCityWeatherByZIPHttpPostOutSoapEnvelope = from_str(&txt).expect("can not unmarshal");
        r.body.body}
}

impl Default for WeatherHttpPost {
                fn default() -> Self {
                    WeatherHttpPost {
                        client: reqwest::Client::new(),
                        url: String::new(),
                        credentials: Option::None,
                     }
                }
            }
            impl WeatherHttpPost {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
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
                    tnsattr: Option::None,
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
}

