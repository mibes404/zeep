use log::warn;

use crate::blz::{messages::GetBank, ports::BlzservicePortType, services::Blzservice, types};

mod blz;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let blz_service = Blzservice::new_client(Option::None);

    let bank = blz_service
        .get_bank(GetBank {
            parameters: types::GetBank {
                blz: "50070010".to_string(),
            },
        })
        .await
        .expect("can not obtain Bank from Bankleitzahl (BLZ)");

    println!("Found bank: {:?}", bank.parameters.details);
}
