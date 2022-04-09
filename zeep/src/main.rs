use clap::{Arg, Command};
use log::warn;
use std::fs::File;
use zeep_lib::writer::FileWriter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let matches = Command::new("Zeep - XSD/WSDL client generator for Rust")
        .version(VERSION)
        .author("Marcel Ibes <mibes@avaya.com>")
        .about("Generate Yaserde annotated Rust structs from XSD or WSDL")
        .arg(
            Arg::new("to_file")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Output to file"),
        )
        .arg(
            Arg::new("from_file")
                .short('i')
                .long("input")
                .takes_value(true)
                .required(true)
                .help("Input from XSD/WSDL file"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .required(true)
                .help("Base path for the XSD file(s)"),
        )
        .arg(
            Arg::new("ns")
                .short('n')
                .long("ns")
                .takes_value(true)
                .help("Namespace prefix"),
        )
        .arg(
            Arg::new("dns")
                .short('d')
                .long("dns")
                .takes_value(true)
                .help("Default namespace (URL)"),
        )
        .get_matches();

    let to_file_name = matches.value_of("to_file");
    let from_file_name = matches.value_of("from_file").unwrap_or_default();
    let base_path = matches.value_of("path").unwrap_or_default();
    let ns_prefix = matches.value_of("ns").map(|ns| ns.to_string());
    let default_namespace = matches.value_of("dns").map(|dns| dns.to_string());

    if let Some(output_file) = to_file_name {
        let file = File::create(output_file).expect("can not create file");
        let mut writer = FileWriter::new_file(file, ns_prefix, default_namespace);
        println!(
            "parsing {}/{} --> {}",
            base_path, from_file_name, output_file
        );
        if let Err(err) = writer.process_file(base_path, from_file_name) {
            println!("Failed to process {from_file_name}: {err}")
        }
    } else {
        let mut writer = FileWriter::new(ns_prefix, default_namespace);
        if let Err(err) = writer.process_file(base_path, from_file_name) {
            println!("Failed to process {from_file_name}: {err}")
        }
    }
}
