use crate::writer::FileWriter;
use clap::{App, Arg};
use log::warn;
use std::fs::File;

mod error;
mod writer;

#[macro_use]
extern crate log;

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let matches = App::new("XSD Generator")
        .version("0.1.0")
        .author("Marcel Ibes <mibes@avaya.com>")
        .about("Generate Yaserde annotated Rust structs from XSD or WSDL")
        .arg(
            Arg::with_name("to_file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output to file"),
        )
        .arg(
            Arg::with_name("from_file")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input from XSD/WSDL file"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Base path for the XSD file(s)"),
        )
        .arg(
            Arg::with_name("ns")
                .short("n")
                .long("ns")
                .takes_value(true)
                .help("Namespace prefix"),
        )
        .get_matches();

    let to_file_name = matches.value_of("to_file");
    let from_file_name = matches
        .value_of("from_file")
        .unwrap_or("agentCommProfile.xsd");

    let base_path = matches.value_of("path").unwrap_or("resources/smgr");
    let ns_prefix = matches.value_of("ns").map(|ns| ns.to_string());

    if let Some(output_file) = to_file_name {
        let file = File::create(output_file).expect("can not create file");
        let mut writer = FileWriter::new_file(file, ns_prefix);
        println!(
            "parsing {}/{} --> {}",
            base_path, from_file_name, output_file
        );
        if let Err(err) = writer.process_file(base_path, from_file_name) {
            println!("Failed to process {}: {}", from_file_name, err.to_string())
        }
    } else {
        let mut writer = FileWriter::default();
        if let Err(err) = writer.process_file(base_path, from_file_name) {
            println!("Failed to process {}: {}", from_file_name, err.to_string())
        }
    }
}
