Generate yaserde annotated structs for Rust based on XSD/WSDL. For WSDL input, client code is generated as well.

The generated code depends on yaserde for the XSD-types. The SOAP client needs tokio and reqwest.  

Usage:

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
