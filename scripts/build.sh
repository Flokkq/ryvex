#!/usr/bin/env bash

args_for_binary=()
clear_terminal=false

for arg in "$@"
do 
    if [ "$arg" = "--clear" ]; then
        clear_terminal=true
    else 
        args_for_binary+=("$arg")
    fi 
done

if $clear_terminal; then
    cargo build > /dev/null 2>&1
else 
    cargo build
fi 

if [ $? -eq 0 ]; then
    echo "Build succeeded"
    clear 
    ./target/debug/ryvex "${args_for_binary[@]}"
else 
    echo "Build failed"
fi 
