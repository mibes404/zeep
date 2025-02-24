use crate::services::{
    GetUserAvailabilityInputEnvelope, GetUserAvailabilityInputEnvelopeBody, GetUserAvailabilityInputEnvelopeHeader,
    mod_mes::GetUserAvailabilityRequest,
    mod_typ::{ArrayOfMailboxData, EmailAddress, ExchangeVersionType, MailboxData, RequestServerVersion},
};
use yaserde::ser::Config;

mod services;

#[tokio::main]
async fn main() {
    env_logger::init();

    let sample_user_availability_request = GetUserAvailabilityInputEnvelope {
        header: GetUserAvailabilityInputEnvelopeHeader {
            request_version: Some(RequestServerVersion {
                version: ExchangeVersionType {
                    inner: "Exchange2010".to_string(),
                },
            }),
            ..Default::default()
        },
        body: GetUserAvailabilityInputEnvelopeBody {
            get_user_availability_request: GetUserAvailabilityRequest {
                mailbox_data_array: ArrayOfMailboxData {
                    mailbox_data: vec![MailboxData {
                        email: EmailAddress {
                            address: "user2@domain.com".to_string(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                },
                ..Default::default()
            },
        },
    };

    // print the XML:
    println!(
        "{}",
        yaserde::ser::to_string_with_config(
            &sample_user_availability_request,
            &Config {
                perform_indent: true,
                write_document_declaration: true,
                indent_string: None
            }
        )
        .unwrap()
    );
}
