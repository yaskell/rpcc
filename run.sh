#!/usr/bin/env bash

cargo run ./return.c && (
    ./return
    echo $?
)
