use tempconverter::{
    CelsiusToFahrenheitInputEnvelope, CelsiusToFahrenheitInputEnvelopeBody, TempConverterEndpointService,
    mod_tem::CelsiusToFahrenheitRequest,
};

mod tempconverter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let tc = TempConverterEndpointService::new(None);

    let fahrenheit = tc
        .celsius_to_fahrenheit(CelsiusToFahrenheitInputEnvelope {
            body: CelsiusToFahrenheitInputEnvelopeBody {
                celsius_to_fahrenheit_request: CelsiusToFahrenheitRequest {
                    temperature_in_celsius: 30.0,
                },
            },
        })
        .await
        .expect("can not obtain temperature");

    println!(
        "{:?}",
        fahrenheit.body.celsius_to_fahrenheit_response.temperature_in_fahrenheit
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tempconverter::CelsiusToFahrenheitOutputEnvelope;
    use yaserde::ser::to_string;

    #[test]
    fn test_celsius_to_fahrenheit_req() {
        let request = CelsiusToFahrenheitInputEnvelope {
            body: CelsiusToFahrenheitInputEnvelopeBody {
                celsius_to_fahrenheit_request: CelsiusToFahrenheitRequest {
                    temperature_in_celsius: 30.0,
                },
            },
        };

        let request_body = to_string(&request).expect("can not parse request");
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?><soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:tem="http://learnwebservices.com/services/tempconverter"><soapenv:Body><tem:CelsiusToFahrenheitRequest><tem:TemperatureInCelsius>30</tem:TemperatureInCelsius></tem:CelsiusToFahrenheitRequest></soapenv:Body></soapenv:Envelope>"#;
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

        let response: CelsiusToFahrenheitOutputEnvelope =
            yaserde::de::from_str(TEMP_RESPONSE).expect("can not parse SOAP response");

        assert_eq!(
            86.0,
            response.body.celsius_to_fahrenheit_response.temperature_in_fahrenheit
        );
    }
}
