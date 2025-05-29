#!/bin/bash

cargo build --release

rustc examples/benchmark_asm.rs \
    --edition 2024 \
    --extern lightwalk=target/release/liblightwalk.rlib \
    --crate-type bin \
    --emit=asm \
    -C opt-level=3 \
    -L target/release/deps
