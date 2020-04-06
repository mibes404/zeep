use crate::smgr::types::*;
use crate::smgr_presence::types::XmlPsCommProfile;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

mod smgr;
mod smgr_agent;
mod smgr_presence;
mod smgr_sm;
mod smgr_station;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smgr_sm::types::SessionManagerCommProfXML;
    use crate::smgr_station::types::XmlStationProfile;
    use std::fs::read_to_string;

    #[test]
    fn test_unmarshall() {
        let sample_response =
            read_to_string("resources/smgr/smgr_get_response.xml").expect("file not found");
        let users: Users = from_str(&sample_response).expect("problems unmarshalling");

        let users = resolve_comm_profiles(users);
        println!("user {:?}", users);

        let xml = to_string(&users).expect("problems marshalling");
        println!("xml {}", xml);
    }

    #[test]
    fn test_marshall() {
        let users = Users {
            secure_store: None,
            user: vec![XmlUser {
                user_organization_details: Some(UserOrganizationDetailsType {
                    tenant: Tenant {
                        name: "I-Hessen".to_string(),
                        create_tenant_if_not_already_present: false,
                    },
                    organization_unit_level_one: Some("Hessen".to_string()),
                    organization_unit_level_two: None,
                    organization_unit_level_three: None,
                }),
                user_provision_rules: None,
                authentication_type: "BASIC".to_string(),
                description: None,
                display_name: Some("karijes, zubni".to_string()),
                display_name_ascii: Some("karijes, zubni".to_string()),
                dn: None,
                is_duplicated_login_allowed: None,
                is_enabled: None,
                is_virtual_user: None,
                given_name: "zubni".to_string(),
                given_name_ascii: Some("zubni".to_string()),
                honorific: None,
                login_name: "goTest11118@sip.avayacloud.com".to_string(),
                new_login_name: None,
                employee_no: None,
                department: None,
                organization: None,
                middle_name: None,
                manager_name: None,
                preferred_given_name: None,
                preferred_language: Some("de_DE".to_string()),
                source: None,
                source_user_key: None,
                status: None,
                suffix: None,
                surname: "karijes".to_string(),
                surname_ascii: Some("karijes".to_string()),
                time_zone: Some(
                    "(+2:0)Amsterdam, Berlin, Rome, Belgrade, Prague, Brussels, Sarajevo"
                        .to_string(),
                ),
                title: None,
                user_name: None,
                user_password: Some("KP_pw_65801".to_string()),
                comm_password: Some("65801".to_string()),
                user_type: vec![],
                roles: None,
                localized_names: Some(XmLocalizedNames {
                    localized_name: vec![XmlLocalizedName {
                        locale: "German".to_string(),
                        name: "karijes, zubni".to_string(),
                    }],
                }),
                address: vec![],
                security_identity: vec![],
                owned_contact_lists: None,
                owned_contacts: None,
                presence_user_default: None,
                presence_user_acl: vec![],
                presence_user_cl_default: None,
                comm_profile_set: vec![XmlCommProfileSetType {
                    comm_profile_set_name: "Primary".to_string(),
                    is_primary: true,
                    handle_list: Some(HandleList {
                        handle: vec![
                            XmlHandle {
                                handle_name: "+4980033322300602".to_string(),
                                handle_type: "sip".to_string(),
                                handle_sub_type: Some("e164".to_string()),
                                domain_name: Some("sip.avayacloud.com".to_string()),
                            },
                            XmlHandle {
                                handle_name: "300602".to_string(),
                                handle_type: "sip".to_string(),
                                handle_sub_type: Some("username".to_string()),
                                domain_name: Some("sip.avayacloud.com".to_string()),
                            },
                            XmlHandle {
                                handle_name: "goTest11117@simulator.amazonses.com".to_string(),
                                handle_type: "smtp".to_string(),
                                handle_sub_type: Some("msexchange".to_string()),
                                domain_name: None,
                            },
                        ],
                    }),
                    comm_profile_list: Some(CommProfileList {
                        comm_profile: vec![
                            XmlCommProfileType {
                                comm_profile_type: "SessionManager".to_string(),
                                comm_profile_sub_type: None,
                                xsi_type: "ns7: SessionManagerCommProfXML".to_string(),
                                job_id: None,
                                station: None,
                                ps: None,
                                sm: Some(SessionManagerCommProfXML {
                                    primary_sm: "SM01a".to_string(),
                                    secondary_sm: None,
                                    termination_app_sequence: Some("CMAppSeq".to_string()),
                                    origination_app_sequence: Some("CMAppSeq".to_string()),
                                    conf_factory_set: None,
                                    survivability_server: None,
                                    home_location: "Neubrandenburg".to_string(),
                                    max_simultaneous_devices: Some(2),
                                    block_new_registration_when_max_active: Some(false),
                                    enabledisablecalllog: Some(true),
                                    emergency_termination_app_sequence: None,
                                    emergency_origination_app_sequence: None,
                                }),
                            },
                            XmlCommProfileType {
                                comm_profile_type: "CM".to_string(),
                                comm_profile_sub_type: None,
                                xsi_type: "ns2: xmlStationProfile".to_string(),
                                job_id: None,
                                station: Some(XmlStationProfile {
                                    cm_name: "cm1".to_string(),
                                    pref_handle_id: Some("300602@sip.avayacloud.com".to_string()),
                                    use_existing_extension: Some(false),
                                    extension_range: None,
                                    extension: "300602".to_string(),
                                    template: Some("BVT_9608_STD".to_string()),
                                    set_type: None,
                                    security_code: None,
                                    port: None,
                                    delete_on_unassign: Some(true),
                                    over_ride_endpoint_name: None,
                                    dual_registration: None,
                                    enh_callr_infodisplay: None,
                                    lock_messages: Some(true),
                                    coverage_path_1: Some("".to_string()),
                                    coverage_path_2: None,
                                    hunt_to_station: None,
                                    tn: None,
                                    cor: Some(3),
                                    cos: Some(1),
                                    xmobile_type: None,
                                    mapping_mode: None,
                                    configuration_set: None,
                                    mobility_trunk_group: None,
                                    dial_prefix: None,
                                    cell_phone_number: None,
                                    music_source: None,
                                    tests: Some(false),
                                    data_module: Some(false),
                                    speakerphone: None,
                                    display_language: Some("unicode".to_string()),
                                    personalized_ringing_pattern: Some(1),
                                    message_lamp_ext: Some("300602".to_string()),
                                    mute_button_enabled: Some(true),
                                    media_complex_ext: None,
                                    ip_softphone: Some(true),
                                    survivable_gk_node_name: None,
                                    survivable_cor: Some("internal".to_string()),
                                    survivable_trunk_dest: Some(true),
                                    voice_mail_number: Some("9879800".to_string()),
                                    off_premises_station: Some(false),
                                    data_option: None,
                                    display_module: Some(false),
                                    message_waiting_indicator: None,
                                    remote_office_phone: Some(false),
                                    lwc_reception: Some("spe".to_string()),
                                    lwc_activation: Some(true),
                                    lwc_log_external_calls: Some(false),
                                    cdr_privacy: Some(false),
                                    redirect_notification: Some(true),
                                    per_button_ring_control: Some(true),
                                    bridged_call_alerting: Some(false),
                                    bridged_idle_line_preference: Some(false),
                                    conf_trans_on_primary_appearance: None,
                                    customizable_labels: Some(true),
                                    expansion_module: Some(false),
                                    ip_video_softphone: Some(false),
                                    active_station_ringing: Some("single".to_string()),
                                    idle_active_ringing: None,
                                    switchhook_flash: Some(false),
                                    ignore_rotary_digits: Some(false),
                                    h_320_conversion: Some(false),
                                    service_link_mode: None,
                                    multimedia_mode: None,
                                    mwi_served_user_type: Some("sip-adjunct".to_string()),
                                    audix_name: None,
                                    automatic_moves: None,
                                    remote_softphone_emergency_calls: Some(
                                        "as-on-local".to_string(),
                                    ),
                                    emergency_location_ext: Some("300602".to_string()),
                                    always_use: None,
                                    precedence_call_waiting: None,
                                    auto_select_any_idle_appearance: None,
                                    coverage_msg_retrieval: None,
                                    auto_answer: None,
                                    data_restriction: None,
                                    idle_appearance_preference: None,
                                    call_waiting_indication: None,
                                    att_call_waiting_indication: None,
                                    distinctive_audible_alert: None,
                                    restrict_last_appearance: None,
                                    adjunct_supervision: None,
                                    per_station_cpn_send_calling_number: None,
                                    busy_auto_callback_without_flash: None,
                                    audible_message_waiting: None,
                                    extended_local_calls: None,
                                    ims_feature_sequencing: None,
                                    display_client_redirection: None,
                                    select_last_used_appearance: None,
                                    coverage_after_forwarding: None,
                                    direct_ip_ip_audio_connections: None,
                                    ip_audio_hairpinning: None,
                                    prime_appearance_preference: None,
                                    station_site_data: None,
                                    abbr_list: vec![],
                                    buttons: vec![],
                                    feature_buttons: vec![],
                                    expansion_module_buttons: vec![],
                                    soft_keys: vec![],
                                    display_buttons: vec![],
                                    station_data_module: None,
                                    hot_line_data: None,
                                    native_name: None,
                                    button_modules: None,
                                    unconditional_internal_dest: None,
                                    unconditional_internal_active: None,
                                    unconditional_external_dest: None,
                                    unconditional_external_active: None,
                                    busy_internal_dest: None,
                                    busy_internal_active: None,
                                    busy_external_dest: None,
                                    busy_external_active: None,
                                    no_reply_internal_dest: None,
                                    no_reply_internal_active: None,
                                    no_reply_external_dest: None,
                                    no_reply_external_active: None,
                                    sac_cf_override: None,
                                    loss_group: None,
                                    time_of_day_lock_table: None,
                                    emu_login_allowed: None,
                                    ec_500_state: None,
                                    mute_on_off_hook_in_sc_mode: None,
                                    type_3pcc_enabled: None,
                                    calculate_route_pattern: None,
                                    sip_trunk: None,
                                    enable_reach_sta_domain_control: None,
                                    multimedia_early_answer: None,
                                    bridged_appr_orig_restr: None,
                                    call_appr_disp_format: None,
                                    ip_phone_group_id: None,
                                    xoip_end_point_type: None,
                                    xid: None,
                                    step_clearing: None,
                                    fixed_tei: None,
                                    tei: None,
                                    country_protocol: None,
                                    endpt_init: None,
                                    spid: None,
                                    endpt_id: None,
                                    is_mct_signalling: None,
                                    is_short_calling_party_display: None,
                                    passage_way: None,
                                    dtmf_over_ip: None,
                                    location: None,
                                    display_caller_id: None,
                                    caller_id_msg_waiting_indication: None,
                                    recall_rotary_digit: None,
                                    profile_settings_data: None,
                                }),
                                ps: None,
                                sm: None,
                            },
                        ],
                    }),
                }],
            }],
        };
        let users = resolve_comm_profiles(users);
        let xml = to_string(&users).expect("problems marshalling");
        println!("xml {}", xml);
    }
}

fn resolve_comm_profiles(mut input: Users) -> Users {
    let new_users: Vec<XmlUser> = input
        .user
        .iter()
        .cloned()
        .map(|mut new_user| {
            new_user.comm_profile_set = new_user
                .comm_profile_set
                .iter()
                .cloned()
                .map(|mut comm_profile_set: XmlCommProfileSetType| {
                    if let Some(profile_list) = &comm_profile_set.comm_profile_list {
                        let mut new_profile_list = profile_list.clone();
                        let new_list = profile_list
                            .comm_profile
                            .iter()
                            .cloned()
                            .map(|mut comm_profile_type: XmlCommProfileType| {
                                let profile_type_str = &comm_profile_type.comm_profile_type;
                                match profile_type_str.as_str() {
                                    "PS" => {
                                        comm_profile_type.station = None;
                                        comm_profile_type.sm = None;
                                        comm_profile_type.xsi_type =
                                            "ns3:XmlPsCommProfile".to_string();
                                    }
                                    "CM" => {
                                        comm_profile_type.ps = None;
                                        comm_profile_type.sm = None;
                                        comm_profile_type.xsi_type =
                                            "ns2:xmlStationProfile".to_string();
                                    }
                                    "SessionManager" => {
                                        comm_profile_type.station = None;
                                        comm_profile_type.ps = None;
                                        comm_profile_type.xsi_type =
                                            "ns7:SessionManagerCommProfXML".to_string();
                                    }
                                    _ => println!("Unknown comm profile type {}", profile_type_str),
                                };

                                comm_profile_type
                            })
                            .collect();
                        new_profile_list.comm_profile = new_list;
                        comm_profile_set.comm_profile_list = Some(new_profile_list);
                    }
                    comm_profile_set
                })
                .collect();
            new_user
        })
        .collect();

    input.user = new_users;
    input
}

#[tokio::main]
async fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    // smgr

    let xml_users = Users {
        secure_store: None,
        user: vec![XmlUser {
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
            comm_profile_set: vec![XmlCommProfileSetType {
                comm_profile_set_name: "".to_string(),
                is_primary: false,
                handle_list: None,
                comm_profile_list: Some(CommProfileList {
                    comm_profile: vec![XmlCommProfileType {
                        comm_profile_type: "PS".to_string(),
                        comm_profile_sub_type: None,
                        xsi_type: "".to_string(),
                        job_id: None,
                        station: None,
                        ps: Some(XmlPsCommProfile {
                            system: "".to_string(),
                            im_gateway_sip_entity: None,
                            publish_via_aes_collector: "".to_string(),
                        }),
                        sm: None,
                    }],
                }),
            }],
        }],
    };
    let xml_users = resolve_comm_profiles(xml_users);

    println!("-------");
    println!("{}", to_string(&xml_users).expect("failed to generate xml"));
}
