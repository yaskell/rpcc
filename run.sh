#!/usr/bin/env bash

cargo run -- --validate ./return.c && (
    ./return
    echo $?
)
