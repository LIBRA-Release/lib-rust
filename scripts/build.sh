#!/bin/bash
#
#  author: apolo_yasuda@yahoo.co.jp
#

[[ -f ./~libra ]] && rm ./~libra
cargo run -- --test
