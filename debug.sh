#!/bin/bash

if [ $1 == thumbv4t-nintendo-gba ]; then
    cargo build --target config/thumbv4t-nintendo-gba.json                              \
        -Zbuild-std=core,alloc                                                          \
        -Zbuild-std-features=compiler-builtins-mem                                      \
        -p emurs_loader_gba
    # mkdir -pv dist
    # llvm-objcopy -O binary target/thumbv4t-nintendo-gba/debug/emurs_loader_gba dist/emurs_loader_gba.gba
    # gbafix dist/emurs_loader_gba.gba
fi

if [ $1 == x86_64-unknown-linux-gnu ]; then
    cargo build --target x86_64-unknown-linux-gnu -p emurs_loader_desktop
fi