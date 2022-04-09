use crate::weather::messages::GetWeatherInformationSoapIn;
use crate::weather::ports::WeatherSoap;
use crate::weather::services::Weather;
use crate::weather::types::GetWeatherInformation;
use log::warn;

mod weather;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    // -- this is not giving a response at the moment; SQL error...
    let w = Weather::new_client(Option::None);
    let w_info = w
        .get_weather_information(GetWeatherInformationSoapIn {
            parameters: GetWeatherInformation {},
        })
        .await;

    match w_info {
        Ok(w_info) => println!("Succeeded: {:?}", w_info),
        Err(Some(err)) => println!("Failed: {:?}", err.fault_string),
        Err(None) => println!("Failed: no info"),
    }
}
