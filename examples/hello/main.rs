use hello::{HelloEndpointService, SayHelloInputEnvelope, SayHelloInputEnvelopeBody, mod_hel::HelloRequest};

mod hello;

#[tokio::main]
async fn main() {
    env_logger::init();

    let h = HelloEndpointService::new(None);
    let request = SayHelloInputEnvelope {
        body: SayHelloInputEnvelopeBody {
            hello_request: HelloRequest {
                name: "John".to_string(),
            },
        },
    };
    let hi = h.say_hello(request).await.expect("can not greet");

    println!("{hi:?}");
}
