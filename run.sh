#!/bin/sh
exec cargo run --bin $1 src/bin/$1/${2:-input}.txt
