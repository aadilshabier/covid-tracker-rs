#! /usr/bin/bash

# Script to install to /usr/local/bin
cargo build --release
sudo cp target/release/covid /usr/local/bin/
