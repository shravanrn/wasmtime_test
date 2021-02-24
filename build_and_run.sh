#/bin/bash

### Build
cd "$(dirname "$0")"
cargo build
/opt/wasi-sdk/bin/clang ./test.c -o ./test.wasm -Wl,--export=malloc_and_write

### Run
./target/debug/wasmtime_test