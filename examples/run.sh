#!/bin/bash

# Compile file
cargo run $1 && gcc "${1%.tig}.s" && ./a.out

# Display result and clean build artifacts
echo $? && rm a.out "${1%.tig}.s"
