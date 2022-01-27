#!/bin/bash
if [ ! -f ./target/release/zeep ]; then
  cargo build --release
fi

# Output to file:
./target/release/zeep -p resources/aic -i agent_wsdl.xml -o zeep-lib/examples/aic/aic.rs
./target/release/zeep -p resources/hello -i hello.wsdl -o zeep-lib/examples/hello/hello.rs
./target/release/zeep -p resources/temp_converter -i tempconverter.wsdl -o zeep-lib/examples/temperature/tempconverter.rs
./target/release/zeep -p resources/weather -i weather.wsdl -o zeep-lib/examples/weather/weather.rs
./target/release/zeep -p resources/smgr -i agentCommProfile.xsd -o zeep-lib/examples/smgr/smgr_agent.rs

# Format the output
rustfmt --color=always --edition=2021 zeep-lib/examples/aic/aic.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/hello/hello.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/weather/weather.rs
rustfmt --color=always --edition=2021 zeep-lib/examples/smgr/smgr.rs