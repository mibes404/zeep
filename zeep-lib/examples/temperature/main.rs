use crate::tempconverter::messages::CelsiusToFahrenheit;
use crate::tempconverter::ports::TempConverterEndpoint;
use crate::tempconverter::services::TempConverterEndpointService;
use crate::tempconverter::types::CelsiusToFahrenheitRequest;
use log::warn;

mod tempconverter;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let tc = TempConverterEndpointService::new_client(Option::None);

    let fahrenheit = tc
        .celsius_to_fahrenheit(CelsiusToFahrenheit {
            celsius_to_fahrenheit_request: CelsiusToFahrenheitRequest {
                temperature_in_celsius: 30.0,
            },
        })
        .await
        .expect("can not obtain temperature");

    println!(
        "{:?}",
        fahrenheit
            .celsius_to_fahrenheit_response
            .temperature_in_fahrenheit
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tempconverter::bindings::{
        CelsiusToFahrenheitResponseSoapEnvelope, CelsiusToFahrenheitSoapEnvelope,
        SoapCelsiusToFahrenheit,
    };
    use yaserde::ser::to_string;

    #[test]
    fn test_celsius_to_fahrenheit_req() {
        let request = CelsiusToFahrenheitSoapEnvelope {
            encoding_style: "http://www.w3.org/2003/05/soap-encoding".to_string(),
            tnsattr: Option::from("http://learnwebservices.com/services/tempconverter".to_string()),
            urnattr: None,
            xsiattr: None,
            header: None,
            body: SoapCelsiusToFahrenheit {
                body: CelsiusToFahrenheit {
                    celsius_to_fahrenheit_request: CelsiusToFahrenheitRequest {
                        temperature_in_celsius: 30.0,
                    },
                },
                xmlns: Option::from(
                    "http://learnwebservices.com/services/tempconverter".to_string(),
                ),
            },
        };

        let request_body = to_string(&request).expect("can not parse request");
        let expected = r#"<?xml version="1.0" encoding="utf-8"?><soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" soapenv:encodingStyle="http://www.w3.org/2003/05/soap-encoding" xmlns:tns="http://learnwebservices.com/services/tempconverter"><soapenv:Body xmlns="http://learnwebservices.com/services/tempconverter"><CelsiusToFahrenheitRequest xmlns:tns="http://learnwebservices.com/services/tempconverter"><tns:TemperatureInCelsius>30</tns:TemperatureInCelsius></CelsiusToFahrenheitRequest></soapenv:Body></soapenv:Envelope>"#.to_string();
        assert_eq!(request_body, expected);
    }

    #[test]
    fn test_celcius_to_fahrenheit_resp() {
        const TEMP_RESPONSE: &str = r#"
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
            <soap:Body>
                <CelsiusToFahrenheitResponse xmlns="http://learnwebservices.com/services/tempconverter">
                    <TemperatureInFahrenheit>86.0</TemperatureInFahrenheit>
                </CelsiusToFahrenheitResponse>
            </soap:Body>
        </soap:Envelope>"#;

        let response: CelsiusToFahrenheitResponseSoapEnvelope =
            yaserde::de::from_str(TEMP_RESPONSE).expect("can not parse SOAP response");

        assert_eq!(
            86.0,
            response
                .body
                .body
                .celsius_to_fahrenheit_response
                .temperature_in_fahrenheit
        );
    }
}
