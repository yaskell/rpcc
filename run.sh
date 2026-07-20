#!/usr/bin/env bash

cargo run -- --tacky ./return.c && (
    ./return
    echo $?
)
