#!/usr/bin/env bash

cargo run -- -S ./return.c && (
    ./return
    echo $?
)
