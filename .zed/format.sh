#!/usr/bin/env bash

# if the formatters return an error, the original file will be kept

input=$(cat)
output=$(echo "$input" | rustfmt --edition 2024 | leptosfmt --stdin)

if [ -n "$output" ]; then
    echo "$output"
else
    echo "$input"
fi
