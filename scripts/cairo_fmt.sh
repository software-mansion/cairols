#!/bin/bash

cargo run --profile=ci --bin cairo-format -- --recursive "$@"
