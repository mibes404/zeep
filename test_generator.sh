#!/bin/bash
if [ ! -f ./target/release/xsd_gen ]; then
  cargo build --release
fi

./target/release/xsd_gen -p resources/smgr -i agentCommProfile.xsd -o examples/generated/smgr.rs
./target/release/xsd_gen -p resources/aic -i agent_wsdl.xml -o examples/generated/aic.rs