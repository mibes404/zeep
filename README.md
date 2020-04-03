# Zeep  - XSD/WSDL client code generator for Rust

Generate yaserde annotated structs for Rust based on XSD/WSDL. For WSDL input, client code is generated as well.

The generated code depends on yaserde for the XSD-generated types. The SOAP client needs tokio and reqwest in addition. 
The generated code does *not* have any dependencies on this library.
 
Check the examples and the Cargo.toml for a complete list of dependencies.

```shell script
USAGE:
    zeep [OPTIONS] --input <from_file> --path <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dns <dns>            Default namespace (URL)
    -i, --input <from_file>    Input from XSD/WSDL file
    -n, --ns <ns>              Namespace prefix
    -p, --path <path>          Base path for the XSD file(s)
    -o, --output <to_file>     Output to file
```

Example usage:

### Output to stdout:
```bash
zeep -p resources/aic -i agent_wsdl.xml
```

### Output to file:
```bash
zeep -p resources/hello -i hello.wsdl -o examples/hello/hello.rs
```

### Format the output
```bash
rustfmt --color=always --edition=2018 examples/hello/hello.rs
```

### Define a default namespace:
No prefixes will be generated for elements matching the default namespace.

```bash
zeep -p resources/hello -i hello.wsdl -o examples/hello/hello.rs -d "http://learnwebservices.com/services/hello"
```

### Define a different prefix:
Use a different prefix than "tns" for the types in the default namespace.

```bash
zeep -p resources/hello -i hello.wsdl -o examples/hello/hello.rs -n ns1
```

## TODO

* [ ] Remove the dependency on customized yaserde version, once the renaming of a vector of structs is fixed
* [ ] Allow network imports 

## DISCLAIMER

Please note: all content in this repository is released for use "AS IS" without any warranties of any kind, including, but not limited to their installation, use, or performance. We disclaim any and all warranties, either express or implied, including but not limited to any warranty of noninfringement, merchantability, and/ or fitness for a particular purpose. We do not warrant that the technology will meet your requirements, that the operation thereof will be uninterrupted or error-free, or that any errors will be corrected.

Any use of these scripts and tools is at your own risk. There is no guarantee that they have been through thorough testing in a comparable environment and we are not responsible for any damage or data loss incurred with their use.

You are responsible for reviewing and testing any generated code you run thoroughly before use in any non-testing environment.

## About Avaya

Avaya elevates communications to the next generation of engagement, connecting organizations to their customers, workforce, and communities with secure, intelligent experiences that matter.

Check us out on: https://www.avaya.com