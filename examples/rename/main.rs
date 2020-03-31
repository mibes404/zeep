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
    root = "users",
    prefix = "tns",
    namespace = "tns: http://my.namespace.com/test",
    default
)]
pub struct Users {
    #[yaserde(rename = "userList", prefix = "tns", default)]
    pub user: Vec<User>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    root = "user",
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
        to_string(&Users {
            user: vec![User {
                user_id: "myid".to_string()
            }]
        })
        .unwrap()
    )
}
