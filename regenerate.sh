#!/bin/bash

git clone https://github.com/open-telemetry/opentelemetry-proto
cargo run --no-default-features --features codegen codegen