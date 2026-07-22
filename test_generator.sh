#!/bin/bash
cargo build

# Output to file:
./target/debug/zeep --input ./resources/temp_converter/tempconverter.wsdl --output ./examples/temperature/tempconverter.rs
./target/debug/zeep --input ./resources/hello/hello.wsdl --output ./examples/hello/hello.rs
./target/debug/zeep --input ./resources/exchange/services.wsdl --output ./examples/exchange/services.rs
./target/debug/zeep --input ./resources/claim_service/claim_service.wsdl --output ./examples/claim_service/claim_service.rs

# Use cargo +nightly fmt when available, otherwise use stable
if cargo +nightly fmt --version > /dev/null 2>&1; then
    FMT_CMD="cargo +nightly fmt --"
else
    FMT_CMD="cargo fmt --"
fi

# Format all generated files
for file in examples/hello/hello.rs \
            examples/temperature/tempconverter.rs \
            examples/exchange/services.rs \
            examples/claim_service/claim_service.rs; do
    $FMT_CMD "$file"
done