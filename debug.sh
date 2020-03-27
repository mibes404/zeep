#!/bin/bash

# Output to stdout:
# cargo run -- -p resources/aic -i agent_wsdl.xml

# Output to file:
cargo run -- -p resources/aic -i agent_wsdl.xml -o examples/generated/aic.rs
cargo run -- -p resources/hello -i hello.wsdl -o examples/generated/hello.rs
cargo run -- -p resources/temp_converter -i tempconverter.wsdl -o examples/generated/tempconverter.rs
cargo run -- -p resources/weather -i weather.wsdl -o examples/generated/weather.rs
cargo run -- -p resources/smgr -i agentCommProfile.xsd -o examples/generated/smgr.rs