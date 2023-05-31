use crate::hello::{
    messages::{SayHello},
    ports::{ HelloEndpoint},
    services::HelloEndpointService,
    types,
};

mod hello;

#[tokio::main]
async fn main() {
    env_logger::init();

    let h = HelloEndpointService::new_client(Option::None);

    let hi = h
        .say_hello(SayHello {
            hello_request: types::HelloRequest {
                name: "Claire".to_string(),
            },
        })
        .await
        .expect("can not greet");

    println!("{:?}", hi);
}
