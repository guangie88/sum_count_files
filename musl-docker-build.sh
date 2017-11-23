#!/usr/bin/env sh
docker run -u=$UID:$(id -g $USER) -v${PWD}:/volume clux/muslrust:1.23.0-nightly-2017-11-21 cargo build --release --features clippy
