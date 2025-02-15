use crate::hello::{messages::SayHello, ports::HelloEndpoint, services::HelloEndpointService, types};

mod hello;

#[tokio::main]
async fn main() {
    env_logger::init();

    let h = HelloEndpointService::new_client(None);
    let request = SayHello {
        hello_request: types::HelloRequest {
            name: "Claire".to_string(),
        },
    };
    let hi = h.say_hello(request).await.expect("can not greet");

    println!("{hi:?}");
}
