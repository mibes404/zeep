use crate::number_services::{
    messages::{NumberToDollarsSoapRequest, NumberToWordsSoapRequest},
    ports::NumberConversionSoapType,
    services::NumberConversion,
    types,
};

mod number_services;

#[tokio::main]
async fn main() {
    env_logger::init();

    let converter_service = NumberConversion::new_client(Option::None);
    let result = converter_service
        .number_to_words(NumberToWordsSoapRequest {
            parameters: types::NumberToWords { ubi_num: 100 },
        })
        .await
        .expect("can not obtain word for number");

    println!(
        "Converted 100 to: {}",
        result.parameters.number_to_words_result
    );

    let result = converter_service
        .number_to_dollars(NumberToDollarsSoapRequest {
            parameters: types::NumberToDollars { d_num: 100.0 },
        })
        .await
        .expect("can not obtain Dollars for number");

    println!(
        "Converted 100,0 to: {}",
        result.parameters.number_to_dollars_result
    );
}

#[cfg(test)]
mod tests {
    use crate::number_services::bindings::NumberToWordsResponseSoapEnvelope;

    #[test]
    fn test_soap_response() {
        const RESPONSE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
          <soap:Body>
            <m:NumberToWordsResponse xmlns:m="http://www.dataaccess.com/webservicesserver/">
              <m:NumberToWordsResult>one hundred </m:NumberToWordsResult>
            </m:NumberToWordsResponse>
          </soap:Body>
        </soap:Envelope>"#;

        let response: NumberToWordsResponseSoapEnvelope =
            yaserde::de::from_str(RESPONSE).expect("can not parse SOAP response");

        assert_eq!(
            "one hundred",
            response.body.body.parameters.number_to_words_result
        );
    }
}
