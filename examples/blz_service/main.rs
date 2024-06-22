use crate::blz::{messages::GetBank, ports::BlzservicePortType, services::Blzservice, types};

mod blz;

#[tokio::main]
async fn main() {
    env_logger::init();

    let blz_service = Blzservice::new_client(None);

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
