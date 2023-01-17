#!/bin/zsh

EXEC = ./target/debug/fortran-rs

function test_case {
    RESULT = $EXEC "$1"
    echo "$?"
    echo "$RESULT"
}

test_case tests/hello-word.f90 -v
