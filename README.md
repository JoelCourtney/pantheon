# Pantheon

Not gonna lie, this project is insanely difficult and I might never finish it.

cargo-make, trunk, libssl-dev, rustup target add wasm32-unknown-unknown

## Pre-requisites

1. [Rust](https://www.rust-lang.org/tools/install)
2. `cargo-make` and `trunk`
  - Install with `$ cargo install cargo-make trunk`.
  - Can take several minutes.
  - you may need to install `pkg-config` and `openssl` through your system's package manager to install `cargo-make`.
    See the docs for [Rust Openssl](https://docs.rs/openssl) for details.
3. `wasm32-unknown-unknown`
  - Install with `$ rustup add target wasm32-unknown-unknown`.

## Building

`$ makers build`