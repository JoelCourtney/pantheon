# DnDCent

Its like DnDBeyond, but worse, and probably illegal.

Currently under development, doesn't do anything useful yet.

## Setup

### Server

You need [Cargo](https://doc.rust-lang.org/cargo/index.html) with nightly rustc installed. If you are on Linux and run into an error that says `'rustc' is not installed for the toolchain nightly-...` you might need to run:

```bash
$ rustup toolchain uninstall nightly
$ rustup toolchain install nightly
```

### UI

The frontend uses [Svelte](https://svelte.dev/), [UIkit](https://getuikit.com/), [TypeScript](https://www.typescriptlang.org/), and [Snowpack](https://www.snowpack.dev/), through [npm](https://www.npmjs.com/).

```
$ cd src/www
$ npm install
```

#### UI Style (optional)

If you want to make changes to the css style, you will need [yarn](https://yarnpkg.com/) installed. I know it sucks using npm and yarn in the same project...deal with it? Who am I talking to? Anyway, you will need to run:

```bash
$ cd src/www/uikit
$ git submodule update --init
$ yarn install
```

The UIkit theme is already included in the repo, so if you aren't making *changes* to the style, you don't need to do this (and you don't need yarn).

## Building

### Server

Run in the root of the repo:

```bash
$ cargo build
```

If you make any changes to the client-side code, you do need to rebuild the server, because the contents of those files are bundled directly into the binary to avoid relying on environment variables.

### UI

Any changes to the frontend will require you to rebuild it with `npm run build --prefix src/www`. Or, you can use `npm run watch --prefix src/www` instead to watch for changes and rebuild automatically. Then, you will need to rebuild the server if you are not in dev mode (see end).

### UI Style

If you make any changes in `src/www/uikit-custom`, you will need to rebuild the uikit theme. Do that with:

```bash
$ cd src/www
$ ./compile_style.py
$ npm run build
```

## Running

In the root of the repo, you can run the server with `cargo run [FILE]` for debug build or `cargo run --release [FILE]` for release build. Or, you can run the binary directly with `target/debug/dndcent [FILE]` or `target/release/dndcent [FILE]`.

### Dev mode

If [FILE] is not provided, it defaults to "test_human.json", which will be an error if run anywhere other than the root of the repo. Also, it will serve pages dynamically, so changes to the frontend files will be automatically be incorporated without rebuilding/restarting.
