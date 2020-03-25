#!/bin/bash

# Output to stdout:
# cargo run -- -p resources/aic -i agent_wsdl.xml

# Output to file:
cargo run -- -p resources/aic -i agent_wsdl.xml -o code_generator/examples/generated/aic.rs