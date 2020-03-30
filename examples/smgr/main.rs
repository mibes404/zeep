use crate::smgr::types::XmlUser;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod smgr;
mod smgr_station;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_unmarshall() {
        let sample_response =
            read_to_string("resources/smgr/smgr_get_response.xml").expect("file not found");
        let users: crate::smgr_station::types::Users =
            from_str(&sample_response).expect("problems unmarshalling");

        println!("user {:?}", users);
    }
}
#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    // smgr

    let xml_user = XmlUser {
        user_organization_details: None,
        user_provision_rules: None,
        authentication_type: "".to_string(),
        description: None,
        display_name: None,
        display_name_ascii: None,
        dn: None,
        is_duplicated_login_allowed: None,
        is_enabled: None,
        is_virtual_user: None,
        given_name: "".to_string(),
        given_name_ascii: None,
        honorific: None,
        login_name: "".to_string(),
        new_login_name: None,
        employee_no: None,
        department: None,
        organization: None,
        middle_name: None,
        manager_name: None,
        preferred_given_name: None,
        preferred_language: None,
        source: None,
        source_user_key: None,
        status: None,
        suffix: None,
        surname: "".to_string(),
        surname_ascii: None,
        time_zone: None,
        title: None,
        user_name: None,
        user_password: None,
        comm_password: None,
        user_type: vec![],
        roles: None,
        localized_names: None,
        address: vec![],
        security_identity: vec![],
        owned_contact_lists: None,
        owned_contacts: None,
        presence_user_default: None,
        presence_user_acl: vec![],
        presence_user_cl_default: None,
        comm_profile_set: vec![],
    };

    println!("-------");
    println!("{}", to_string(&xml_user).expect("failed to generate xml"));
}
