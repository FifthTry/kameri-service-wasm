# Kameri Service: WASM Backend

This repository contains the Rust code for the TODO app which compiles to WASM.
The generated WASM package can then be used by the FPM package to implement the
TODO application logics.

## How to build

The crate by default builds for the `wasm32-unknown-unknown` platform which 
generates a platform agnostic WASM binary to be used along with the backend.

```sh
$ rustup target add wasm32-unknown-unknown
$ cargo build
```

