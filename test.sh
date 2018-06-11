#!/bin/bash

successes=0

for i in $(seq 1 49); do
  ./target/debug/tc "../notes/mci/testcases/test$i.tig" > /dev/null 2>&1
  if [[ $? -eq 0 ]]; then
    successes=$((successes + 1))
  else
    echo "Test case failed: test$i.tig"
  fi
done

echo "$successes / 49 succeeded"
