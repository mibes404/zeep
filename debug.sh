#!/bin/bash

# Output to stdout:
# cargo run -- -p resources/aic -i agent_wsdl.xml

# Output to file:
cargo run -- -p resources/aic -i agent_wsdl.xml -o examples/aic/aic_agent.rs
cargo run -- -p resources/aic -i version_wsdl.xml -o examples/aic/aic_version.rs
cargo run -- -p resources/aic -i workflow_wsdl.xml -o examples/aic/aic_workflow.rs
cargo run -- -p resources/hello -i hello.wsdl -o examples/hello/hello.rs
cargo run -- -p resources/temp_converter -i tempconverter.wsdl -o examples/temperature/tempconverter.rs
cargo run -- -p resources/weather -i weather.wsdl -o examples/weather/weather.rs
cargo run -- -p resources/smgr -i agentCommProfile.xsd -o examples/smgr/smgr.rs

# Format the output
rustfmt --color=always --edition=2018 examples/aic/aic_agent.rs
rustfmt --color=always --edition=2018 examples/aic/aic_version.rs
rustfmt --color=always --edition=2018 examples/aic/aic_workflow.rs
rustfmt --color=always --edition=2018 examples/hello/hello.rs
rustfmt --color=always --edition=2018 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2018 examples/weather/weather.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr.rs