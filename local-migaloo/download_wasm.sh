#!/usr/bin/env sh

ARCH="x86_64"

WASM_VERSION=$(go list -m all | grep github.com/CosmWasm/wasmvm | awk '{print $2}')
wget -O /lib/libwasmvm_muslc.a https://github.com/CosmWasm/wasmvm/releases/download/${WASM_VERSION}/libwasmvm_muslc.${ARCH}.a; \
