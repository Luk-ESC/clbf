#!/bin/bash

mkdir tests/$1 || exit 1
wl-paste > tests/$1/$1.bf || exit 1

./run_tests.sh -b $1
