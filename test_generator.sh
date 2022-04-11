#!/bin/bash
cargo build

# Output to file:
./target/debug/zeep -p resources/aic -i agent_wsdl.xml -o zeep-lib/examples/aic/aic.rs
./target/debug/zeep -p resources/hello -i hello.wsdl -o zeep-lib/examples/hello/hello.rs
./target/debug/zeep -p resources/temp_converter -i tempconverter.wsdl -o zeep-lib/examples/temperature/tempconverter.rs
./target/debug/zeep -p resources/weather -i weather.wsdl -o zeep-lib/examples/weather/weather.rs
./target/debug/zeep -p resources/smgr -i agentCommProfile.xsd -o zeep-lib/examples/smgr/smgr_agent.rs
./target/debug/zeep -p resources/blz_service -i blz.wsdl -o zeep-lib/examples/blz_service/blz.rs
./target/debug/zeep -p resources/number_services -i number_services.wsdl -o zeep-lib/examples/number_services/number_services.rs -n 'm'
./target/debug/zeep -p resources/broadband_forum -i cwmp-1-2.xsd -o zeep-lib/examples/broadband_forum/cwmp.rs

# Format the output
rustfmt --color=always --edition=2021 zeep-lib/examples/aic/aic.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/hello/hello.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/weather/weather.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/smgr/smgr.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/blz_service/blz.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/number_services/number_services.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/broadband_forum/cwmp.rs