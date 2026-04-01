#!/bin/bash

if [ ! -d opentelemetry-proto/ ]; then
    git clone https://github.com/open-telemetry/opentelemetry-proto
fi

cargo run --no-default-features --features codegen codegen