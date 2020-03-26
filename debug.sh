#!/bin/bash

# Output to stdout:
# cargo run -- -p resources/aic -i agent_wsdl.xml

# Output to file:
cargo run -- -p resources/aic -i agent_wsdl.xml -o code_generator/examples/generated/aic.rs
cargo run -- -p resources/hello -i hello.wsdl -o code_generator/examples/generated/hello.rs
cargo run -- -p resources/temp_converter -i tempconverter.wsdl -o code_generator/examples/generated/tempconverter.rs
cargo run -- -p resources/weather -i weather.wsdl -o code_generator/examples/generated/weather.rs