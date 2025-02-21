use clap::{Arg, Command};
use std::{fs::File, path::Path};
use zeep_lib::{
    reader::{WriteXml, XmlReader},
    utils::read_input_file_and_xsd_files_at_path,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();

    let matches = Command::new("Zeep - XSD/WSDL client generator for Rust")
        .version(VERSION)
        .author("Marcel Ibes <mibes@avaya.com>")
        .about("Generate Yaserde annotated Rust structs from XSD or WSDL")
        .arg(Arg::new("to_file").short('o').long("output").help("Output to file"))
        .arg(
            Arg::new("from_file")
                .short('i')
                .long("input")
                .required(true)
                .help("Input from XSD/WSDL file"),
        )
        .get_matches();

    let to_file_name = matches.get_one::<String>("to_file");
    let from_file_name = matches
        .get_one::<String>("from_file")
        .map(ToString::to_string)
        .unwrap_or_default();

    let from_file_path = Path::new(&from_file_name);
    let files = read_input_file_and_xsd_files_at_path(from_file_path).expect("can not read input file");

    let output_file = to_file_name.map_or_else(|| from_file_path.with_extension("rs"), |f| Path::new(f).to_path_buf());

    let mut file = File::create(output_file).expect("can not create file");
    let document = XmlReader.read_xml(&files).expect("can not read xml");
    document.write_xml(&mut file).expect("can not write xml");
}
