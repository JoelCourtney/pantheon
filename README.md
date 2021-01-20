# DnDCent

Its like DnDBeyond, but worse, and probably illegal.

Currently under development, doesn't do anything useful yet.

## Setup

You need [yarn](https://yarnpkg.com/) and [Cargo](https://doc.rust-lang.org/cargo/index.html) nightly installed. If you are on Linux and run into an error that says `'rustc' is not installed for the toolchain nightly-...` you might need to run:

```bash
$ rustup toolchain uninstall nightly
$ rustup toolchain install nightly
```

Once this is done, run

```bash
$ ./setup.py
```

in the root of the repo.

## Building

Run in the root of the repo:

```bash
$ cargo build --release
```

This will handle both front and back end.

## Running

Currently it can only be run from the root of the repo.

```bash
$ target/release/dndcent <FILE>
```

I use currently `test_human.json` or `test_variant_human.json` for FILE.
