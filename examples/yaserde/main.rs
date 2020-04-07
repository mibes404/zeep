use std::io::{Read, Write};
use yaserde::de::from_str;
use yaserde::ser::to_string;

use yaserde::{YaDeserialize, YaSerialize};

#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "project",
    prefix = "tns",
    namespace = "tns: http://my.namespace.com/test",
    default
)]
pub struct Project {
    #[yaserde(rename = "userList", prefix = "tns", default)]
    pub user: Vec<User>,
    #[yaserde(rename = "topic", prefix = "tns", default)]
    pub topics: Vec<String>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "project2",
    prefix = "tns",
    namespace = "tns: http://my.namespace.com/test",
    default
)]
pub struct Project2 {
    #[yaserde(rename = "userList", prefix = "tns", default)]
    pub users: Vec<User>,
    #[yaserde(rename = "adminList", prefix = "tns", default)]
    pub admins: Vec<User>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    root = "rootUser",
    rename = "renameUser",
    prefix = "tns",
    namespace = "tns: http://my.namespace.com/test",
    default
)]
pub struct User {
    #[yaserde(rename = "id", prefix = "tns", default)]
    pub user_id: String,
}
fn main() {
    println!(
        "{}",
        to_string(&Project {
            user: vec![User {
                user_id: "myid".to_string()
            }],
            topics: vec!["topic1".to_string(), "topic2".to_string()]
        })
        .unwrap()
    );

    println!(
        "{}",
        to_string(&Project2 {
            admins: vec![User {
                user_id: "root".to_string()
            }],
            users: vec![User {
                user_id: "user".to_string()
            }],
        })
        .unwrap()
    )
}
