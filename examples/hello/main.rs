use crate::hello::bindings::HelloEndpointServiceSoapBinding;
use crate::hello::messages::SayHello;
use crate::hello::ports::HelloEndpoint;
use crate::hello::services::HelloEndpointService;
use crate::hello::types;
use crate::hello::types::HelloRequest;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod hello;

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let h = HelloEndpointService::new_client(Option::None);

    let hi = h
        .say_hello(SayHello {
            parameters: types::SayHello {
                hello_request: HelloRequest {
                    name: "Claire".to_string(),
                },
            },
        })
        .await
        .expect("can not greet");

    println!("{:?}", hi);
}
