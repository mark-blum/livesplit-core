#![feature(link_args)]
#![no_main]

#[link_args = "-Oz -s TOTAL_MEMORY=33554432 -s ALLOW_MEMORY_GROWTH=1 -s BINARYEN_METHOD='native-wasm'"]
extern "C" {}

extern crate livesplit_core_capi;
