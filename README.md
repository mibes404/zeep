# Zeep  - XSD/WSDL client code generator for Rust

Generate yaserde annotated structs for Rust based on XSD/WSDL. For WSDL input, client code is generated as well.

The generated code depends on yaserde (0.12) and yaserde_derive for the XSD-generated types. The SOAP client needs tokio, log and reqwest in addition. 
The generated code does *not* have any dependencies on this library.

Check the examples and the Cargo.toml for a complete list of dependencies.

## Installation

```bash
$ cargo install --git https://github.com/mibes404/zeep zeep
$ zeep --version
  Zeep - XSD/WSDL client generator for Rust 0.1.11
```

## Usage

```shell script
Generate Yaserde annotated Rust structs from XSD or WSDL

Usage: zeep [OPTIONS] --input <from_file>

Options:
  -o, --output <to_file>   Output to file
  -i, --input <from_file>  Input from XSD/WSDL file
  -h, --help               Print help
  -V, --version            Print version
```

Example usage:

### Output to file:
```bash
$ zeep --input ./resources/temp_converter/tempconverter.wsdl --output ./examples/temperature/tempconverter.rs
```

### Format the output
```bash
$ rustfmt --color=always --edition=2024 examples/temperature/tempconverter.rs
```

## Examples

You can run one of the pre-built examples in the examples directory.

```bash
$ cargo run --example temperature 

or 

$ cargo run --example hello
```

## DISCLAIMER

Please note: all content in this repository is released for use "AS IS" without any warranties of any kind, including, but not limited to their installation, use, or performance. We disclaim any and all warranties, either express or implied, including but not limited to any warranty of noninfringement, merchantability, and/ or fitness for a particular purpose. We do not warrant that the technology will meet your requirements, that the operation thereof will be uninterrupted or error-free, or that any errors will be corrected.

Any use of these scripts and tools is at your own risk. There is no guarantee that they have been through thorough testing in a comparable environment and we are not responsible for any damage or data loss incurred with their use.

You are responsible for reviewing and testing any generated code you run thoroughly before use in any non-testing environment.

## About Avaya

Avaya elevates communications to the next generation of engagement, connecting organizations to their customers, workforce, and communities with secure, intelligent experiences that matter.

Check us out on: https://www.avaya.com