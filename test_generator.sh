#!/bin/bash
if [ ! -f ./target/release/zeep ]; then
  cargo build --release
fi

# Output to file:
./target/release/zeep -p resources/aic -i agent_wsdl.xml -o examples/aic/aic.rs
./target/release/zeep -p resources/hello -i hello.wsdl -o examples/hello/hello.rs
./target/release/zeep -p resources/temp_converter -i tempconverter.wsdl -o examples/temperature/tempconverter.rs
./target/release/zeep -p resources/weather -i weather.wsdl -o examples/weather/weather.rs
./target/release/zeep -p resources/smgr -i agentCommProfile.xsd -o examples/smgr/smgr_agent.rs

# Format the output
rustfmt --color=always --edition=2018 examples/aic/aic.rs
rustfmt --color=always --edition=2018 examples/hello/hello.rs
rustfmt --color=always --edition=2018 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2018 examples/weather/weather.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr.rs