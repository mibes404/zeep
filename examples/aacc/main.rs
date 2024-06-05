use customer::types::ContactType;

mod customer;

#[tokio::main]
async fn main() {
    env_logger::init();
    // self-referencing struct
    let contact_type = ContactType::default();
    let xml = yaserde::ser::to_string(&contact_type).unwrap();
    println!("{}", xml);
}
