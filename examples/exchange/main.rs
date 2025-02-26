use crate::services::{
    GetUserAvailabilityInputEnvelope, GetUserAvailabilityInputEnvelopeBody, GetUserAvailabilityInputEnvelopeHeader,
    mod_mes::GetUserAvailabilityRequest,
    mod_typ::{
        ArrayOfMailboxData, EmailAddress, ExchangeVersionType, MailboxData, MeetingAttendeeType, RequestServerVersion,
    },
    restrictions::CheckRestrictions,
};
use yaserde::ser::Config;

mod services;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut sample_user_availability_request = GetUserAvailabilityInputEnvelope {
        header: GetUserAvailabilityInputEnvelopeHeader {
            request_version: Some(RequestServerVersion {
                version: ExchangeVersionType {
                    value: "Exchange2010".to_string(),
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
                        attendee_type: MeetingAttendeeType {
                            value: "Optional".to_string(),
                        },
                        ..Default::default()
                    }],
                },
                ..Default::default()
            },
        },
    };

    sample_user_availability_request
        .check_restrictions(None)
        .expect("check_restrictions failed");

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

    println!("==============================");

    // if you try to use a non-defined version-type the check will fail
    sample_user_availability_request.header.request_version = Some(RequestServerVersion {
        version: ExchangeVersionType {
            value: "Exchange2011".to_string(),
        },
    });

    assert!(sample_user_availability_request.check_restrictions(None).is_err());
}
