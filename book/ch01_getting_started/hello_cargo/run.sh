#!/usr/bin/env bash

cargo build;
./target/debug/hello_cargo;

# equivalent to:
# $ cargo run

# Quickly check that code compiles
# $ cargo check

# Optimized build
# $ cargo build --release
