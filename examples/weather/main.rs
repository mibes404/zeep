use crate::weather::bindings;
use crate::weather::messages::GetWeatherInformationSoapIn;
use crate::weather::ports::WeatherSoap;
use crate::weather::types::GetWeatherInformation;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod weather;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    // -- this is not giving a response at the moment; SQL error...
    let mut w =
        bindings::WeatherSoap::new("http://wsf.cdyne.com/WeatherWS/Weather.asmx", Option::None);
    let w_info = w
        .get_weather_information(GetWeatherInformationSoapIn {
            parameters: GetWeatherInformation {},
        })
        .await;

    println!("{:?}", w_info);
}
