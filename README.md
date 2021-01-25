# DnDCent

Its like DnDBeyond, but worse, and probably illegal.

Currently under development, doesn't do anything useful yet.

## Setup

### Server

You need [Cargo](https://doc.rust-lang.org/cargo/index.html) nightly installed. If you are on Linux and run into an error that says `'rustc' is not installed for the toolchain nightly-...` you might need to run:

```bash
$ rustup toolchain uninstall nightly
$ rustup toolchain install nightly
```

### UI Style

If you want to make changes to `css/style.css`, you will need [yarn](https://yarnpkg.com/) installed. You will then need to run:

```bash
$ cd src/www/uikit
$ git submodule update --init
$ yarn install
```

## Building

### Server

Run in the root of the repo:

```bash
$ cargo build
```

If you make any changes to the client-side code, you do need to rebuild the server, because the contents of those files are bundled directly into the binary to avoid relying on environment variables.

### UI Style

If you make any changes in `src/www/uikit-custom`, you will need to rebuild uikit. In the root of the repo, do that with:

```bash
$ ./build_ui.py
$ cargo build
```

## Running

In the root of the repo, you can run the server with `cargo run [FILE]` for debug build or `cargo run --release [FILE]` for release build. Or, you can run the binary directly with `target/debug/dndcent [FILE]` or `target/release/dndcent [FILE]`.

If [FILE] is not provided, it defaults to "test_human.json", which will be an error if run anywhere other than the root of the repo.
