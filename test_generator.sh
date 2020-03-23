#!/bin/bash
if [ ! -f ./target/release/xsd_gen ]; then
  cargo build --release
fi

./target/release/code_generator -p resources/smgr -i agentCommProfile.xsd -o code_generator/examples/generated/smgr.rs
./target/release/code_generator -p resources/aic -i agent_wsdl.xml -o code_generator/examples/generated/aic.rs