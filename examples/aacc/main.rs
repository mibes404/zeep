use customer::{GetAllCustomersInputEnvelope, get_all_customers, mod_wsd::ContactType, multi_ref};

mod customer;

#[tokio::main]
async fn main() {
    env_logger::init();
    // self-referencing struct
    let contact_type = ContactType {
        parent: Some(multi_ref::MultiRef::new(ContactType::default())),
        ..Default::default()
    };
    let xml = yaserde::ser::to_string(&contact_type).unwrap();
    println!("{xml}");

    let req = GetAllCustomersInputEnvelope::default();
    let result = get_all_customers(req, None).await.expect("Failed to get customers");
    println!("{:?}", result);
}
