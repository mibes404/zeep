use crate::weather::{
    messages::GetWeatherInformationSoapIn, ports::WeatherSoap, services::Weather,
    types::GetWeatherInformation,
};

mod weather;

#[tokio::main]
async fn main() {
    env_logger::init();

    // -- this is not giving a response at the moment; SQL error...
    let w = Weather::new_client(Option::None);
    let w_info = w
        .get_weather_information(GetWeatherInformationSoapIn {
            parameters: GetWeatherInformation {},
        })
        .await;

    match w_info {
        Ok(w_info) => println!("Succeeded: {w_info:?}"),
        Err(Some(err)) => println!("Failed: {:?}", err.fault_string),
        Err(None) => println!("Failed: no info"),
    }
}
