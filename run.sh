#!/usr/bin/env bash

RUST_BACKTRACE=1 cargo run -- ./program.c && (
    ./program
    echo $?
)
