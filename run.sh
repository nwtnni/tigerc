#!/bin/bash

if [[ "$#" -ne "1" ]]; then
    echo "Usage: ./run.sh <FILE>"
    exit 1
fi

# Compile runtime if it doesn't exist
pushd runtime >/dev/null 2>&1
make >/dev/null 2>&1
popd >/dev/null 2>&1

# Compile file
cargo run $1 && gcc "${1%.tig}.s" runtime/libtiger.a

printf "\n------------------------------------"
printf "\nRunning assembly..."
printf "\n------------------------------------\n\n"

time ./a.out
EXIT_CODE="$?"

printf "\n------------------------------------"
printf "\nTerminated with exit code $EXIT_CODE"
printf "\n------------------------------------\n"

rm -f a.out "${1%.tig}.s"
