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
        user_organization_details: Default::default(),
        user_provision_rules: Default::default(),
        authentication_type: "".to_string(),
        description: "".to_string(),
        display_name: "".to_string(),
        display_name_ascii: "".to_string(),
        dn: "".to_string(),
        is_duplicated_login_allowed: false,
        is_enabled: false,
        is_virtual_user: false,
        given_name: "".to_string(),
        given_name_ascii: "".to_string(),
        honorific: "".to_string(),
        login_name: "".to_string(),
        new_login_name: "".to_string(),
        employee_no: "".to_string(),
        department: "".to_string(),
        organization: "".to_string(),
        middle_name: "".to_string(),
        manager_name: "".to_string(),
        preferred_given_name: "".to_string(),
        preferred_language: "".to_string(),
        source: "".to_string(),
        source_user_key: "".to_string(),
        status: "".to_string(),
        suffix: "".to_string(),
        surname: "".to_string(),
        surname_ascii: "".to_string(),
        time_zone: "".to_string(),
        title: "".to_string(),
        user_name: "".to_string(),
        user_password: "".to_string(),
        comm_password: "".to_string(),
        user_type: vec![],
        roles: Default::default(),
        localized_names: Default::default(),
        address: vec![],
        security_identity: vec![],
        owned_contact_lists: Default::default(),
        owned_contacts: Default::default(),
        presence_user_default: Default::default(),
        presence_user_acl: vec![],
        presence_user_cl_default: Default::default(),
        comm_profile_set: vec![],
    };

    println!("-------");
    println!("{}", to_string(&xml_user).expect("failed to generate xml"));
}
