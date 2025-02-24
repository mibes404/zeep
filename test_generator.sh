#!/bin/bash
cargo build

# Output to file:
./target/debug/zeep --input ./resources/aic/agent_wsdl.xml --output ./examples/aic/aic.rs
./target/debug/zeep --input ./resources/temp_converter/tempconverter.wsdl --output ./examples/temperature/tempconverter.rs
./target/debug/zeep --input ./resources/hello/hello.wsdl --output ./examples/hello/hello.rs
./target/debug/zeep --input ./resources/exchange/services.wsdl --output ./examples/exchange/services.rs
./target/debug/zeep --input ./resources/weather/weather.wsdl -o examples/weather/weather.rs
./target/debug/zeep -p resources/smgr -i agentCommProfile.xsd -o examples/smgr/smgr_agent.rs
./target/debug/zeep -p resources/blz_service -i blz.wsdl -o examples/blz_service/blz.rs
./target/debug/zeep -p resources/number_services -i number_services.wsdl -o examples/number_services/number_services.rs -n 'm'
./target/debug/zeep -p resources/broadband_forum -i cwmp-1-2.xsd -o examples/broadband_forum/cwmp.rs
./target/debug/zeep --input ./resources/aacc/CustomerWS.wsdl --output ./examples/aacc/customer.rs

# Format the output
rustfmt --color=always --edition=2024 examples/aic/aic.rs
rustfmt --color=always --edition=2024 examples/hello/hello.rs
rustfmt --color=always --edition=2024 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2024 examples/exchange/services.rs
rustfmt --color=always --edition=2024 examples/weather/weather.rs
rustfmt --color=always --edition=2024 examples/smgr/smgr_agent.rs
rustfmt --color=always --edition=2024 examples/blz_service/blz.rs
rustfmt --color=always --edition=2024 examples/number_services/number_services.rs
rustfmt --color=always --edition=2024 examples/broadband_forum/cwmp.rs
rustfmt --color=always --edition=2024 examples/aacc/customer.rs