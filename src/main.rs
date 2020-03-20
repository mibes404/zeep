use crate::writer::NodeWriter;
use clap::{App, Arg};
use log::warn;
use std::fs::File;

mod writer;

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config. {}", err);
    }

    let matches = App::new("XSD Generator")
        .version("0.1.0")
        .author("Marcel Ibes <mibes@avaya.com>")
        .about("Generate Serde annotated Rust structs from XSD")
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
        .get_matches();

    let to_file_name = matches.value_of("to_file");
    let from_file_name = matches
        .value_of("from_file")
        .unwrap_or("agentCommProfile.xsd");
    let base_path = matches.value_of("path").unwrap_or("resources/smgr");

    if let Some(output_file) = to_file_name {
        let file = File::create(output_file).expect("can not create file");
        let mut writer = NodeWriter::new_file(file);
        println!(
            "parsing {}/{} --> {}",
            base_path, from_file_name, output_file
        );
        writer.process_file(base_path, from_file_name);
    } else {
        let mut writer = NodeWriter::default();
        writer.process_file(base_path, from_file_name);
    }
}
