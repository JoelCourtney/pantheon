# DnDCent

Its like DnDBeyond, but worse, and probably illegal.

## Setup

You need [yarn](https://yarnpkg.com/) and [Cargo](https://doc.rust-lang.org/cargo/index.html) nightly installed. If you are on Linux and run into an error that says `'rustc' is not installed for the tollchain nightly-...` you might need to run:

```bash
$ rustup toolchain uninstall nightly
$ rustup toolchain install nightly
```

Nightly is needed for the min_const_generics feature, which will be added to stable in March, so at least this mess is temporary. Once this is done, run

```bash
$ ./setup.py
```

in the root of the repo.

## Building

Currently there isn't a single build script that handles both client and server. That will happen eventually. In the meantime, build the client theme with:

```bash
$ cd client/uikit
$ yarn compile
```

and build the server with:

```bash
$ cd server
$ cargo build --release
```

## Running

You can view the client page by opening `client/src/index.html` in a server like that provided by Intellij. Otherwise, viewing the client is currently impossible until I add the actual server functionality to the server half.

You can run the server with:

```bash
$ cd server
$ target/release/dndcent
```

if you're morbidly curious :)
