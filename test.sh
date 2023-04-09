#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/debug/rsc "$input" > ./output/tmp.s
  cc -o ./output/tmp ./output/tmp.s
  ./output/tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "1+2-3"

echo OK