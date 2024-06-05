#!/bin/bash
cargo build

# Output to file:
./target/debug/zeep -p resources/aic -i agent_wsdl.xml -o examples/aic/aic.rs
./target/debug/zeep -p resources/hello -i hello.wsdl -o examples/hello/hello.rs
./target/debug/zeep -p resources/temp_converter -i tempconverter.wsdl -o examples/temperature/tempconverter.rs
./target/debug/zeep -p resources/weather -i weather.wsdl -o examples/weather/weather.rs
./target/debug/zeep -p resources/smgr -i agentCommProfile.xsd -o examples/smgr/smgr_agent.rs
./target/debug/zeep -p resources/blz_service -i blz.wsdl -o examples/blz_service/blz.rs
./target/debug/zeep -p resources/number_services -i number_services.wsdl -o examples/number_services/number_services.rs -n 'm'
./target/debug/zeep -p resources/broadband_forum -i cwmp-1-2.xsd -o examples/broadband_forum/cwmp.rs
./target/debug/zeep -p resources/aacc -i CustomerWS.wsdl -o examples/aacc/customer.rs

# Format the output
rustfmt --color=always --edition=2021 examples/aic/aic.rs
rustfmt --color=always --edition=2021 examples/hello/hello.rs
rustfmt --color=always --edition=2021 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2021 examples/weather/weather.rs
rustfmt --color=always --edition=2021 examples/smgr/smgr.rs
rustfmt --color=always --edition=2021 examples/blz_service/blz.rs
rustfmt --color=always --edition=2021 examples/number_services/number_services.rs
rustfmt --color=always --edition=2021 examples/broadband_forum/cwmp.rs
rustfmt --color=always --edition=2021 examples/aacc/customer.rs