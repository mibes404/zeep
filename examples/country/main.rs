use country::{messages::GetCountryByCountryCodeSoapIn, ports::CountrySoap, services::Country};

use crate::country::types;

mod country;

#[tokio::main]
async fn main() {
    env_logger::init();

    let country_soap_service = Country::new_client(Option::None);

    let country_details = country_soap_service
        .get_country_by_country_code(GetCountryByCountryCodeSoapIn {
            parameters: types::GetCountryByCountryCode {
                country_code: Some("US".to_string()),
            },
        })
        .await
        .expect("can not get country");

    println!("{:?}, belongs to the country code DE", country_details);
}
