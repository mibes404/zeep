#!/bin/bash
cargo build

# Output to file:
./target/debug/zeep --input ./resources/aic/agent_wsdl.xml --output ./examples/aic/aic.rs
./target/debug/zeep --input ./resources/temp_converter/tempconverter.wsdl --output ./examples/temperature/tempconverter.rs
./target/debug/zeep --input ./resources/hello/hello.wsdl --output ./examples/hello/hello.rs
./target/debug/zeep --input ./resources/exchange/services.wsdl --output ./examples/exchange/services.rs
./target/debug/zeep --input ./resources/aacc/CustomerWS.wsdl --output ./examples/aacc/customer.rs

# Format the output
rustfmt --color=always --edition=2024 examples/aic/aic.rs
rustfmt --color=always --edition=2024 examples/hello/hello.rs
rustfmt --color=always --edition=2024 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2024 examples/exchange/services.rs
rustfmt --color=always --edition=2024 examples/aacc/customer.rs