use tempconverter::{
    CelsiusToFahrenheitInputEnvelope, TempConverterEndpointService, mod_tem::CelsiusToFahrenheitRequest,
};

mod tempconverter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let tc = TempConverterEndpointService::new(None);

    let fahrenheit = tc
        .celsius_to_fahrenheit(CelsiusToFahrenheitInputEnvelope {
            body: CelsiusToFahrenheitRequest {
                temperature_in_celsius: 30.0,
            },
        })
        .await
        .expect("can not obtain temperature");

    println!("{:?}", fahrenheit.body.temperature_in_fahrenheit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaserde::ser::to_string;

    #[test]
    fn test_celsius_to_fahrenheit_req() {
        let request = CelsiusToFahrenheitInputEnvelope {
            body: CelsiusToFahrenheitRequest {
                temperature_in_celsius: 30.0,
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
