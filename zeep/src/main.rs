use clap::{Arg, Command};
use std::fs::File;
use zeep_lib::writer::FileWriter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();

    let matches = Command::new("Zeep - XSD/WSDL client generator for Rust")
        .version(VERSION)
        .author("Marcel Ibes <mibes@avaya.com>")
        .about("Generate Yaserde annotated Rust structs from XSD or WSDL")
        .arg(
            Arg::new("to_file")
                .short('o')
                .long("output")
                .help("Output to file"),
        )
        .arg(
            Arg::new("from_file")
                .short('i')
                .long("input")
                .required(true)
                .help("Input from XSD/WSDL file"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .required(true)
                .help("Base path for the XSD file(s)"),
        )
        .arg(
            Arg::new("ns")
                .short('n')
                .long("ns")
                .help("Namespace prefix"),
        )
        .arg(
            Arg::new("dns")
                .short('d')
                .long("dns")
                .help("Default namespace (URL)"),
        )
        .get_matches();

    let to_file_name = matches.get_one::<String>("to_file");
    let from_file_name = matches
        .get_one::<String>("from_file")
        .map(|ff| ff.to_string())
        .unwrap_or_default();
    let base_path = matches
        .get_one::<String>("path")
        .map(|ns| ns.to_string())
        .unwrap_or_default();
    let ns_prefix = matches.get_one::<String>("ns").map(|ns| ns.to_string());
    let default_namespace = matches.get_one::<String>("dns").map(|dns| dns.to_string());

    if let Some(output_file) = to_file_name {
        let file = File::create(output_file).expect("can not create file");
        let mut writer = FileWriter::new_file(file, ns_prefix, default_namespace);
        println!(
            "parsing {}/{} --> {}",
            base_path, from_file_name, output_file
        );
        if let Err(err) = writer.process_file(&base_path, &from_file_name) {
            println!("Failed to process {from_file_name}: {err}")
        }
    } else {
        let mut writer = FileWriter::new(ns_prefix, default_namespace);
        if let Err(err) = writer.process_file(&base_path, &from_file_name) {
            println!("Failed to process {from_file_name}: {err}")
        }
    }
}
