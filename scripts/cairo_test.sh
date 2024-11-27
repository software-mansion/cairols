#!/bin/bash

cargo run --profile=ci --bin cairo-test -- corelib/ &&
    cargo run --profile=ci --bin cairo-test -- tests/bug_samples/ --starknet
