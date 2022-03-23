# Pantheon

A framework and server for running a suite of character sheet editors/viewers in role-playing games.

The server is built for Unix systems, but the websites it serves should run anywhere.

## Pre-requisites

1. [Rust and Cargo](https://www.rust-lang.org/tools/install)
2. [wasm32-unknown-unknown](https://doc.rust-lang.org/nightly/rustc/platform-support.html) compile target
    - `rustup target add wasm32-unknown-unknown`
3. [Trunk](https://trunkrs.dev/)
    - `cargo install trunk`
4. [cargo-script](https://lib.rs/crates/cargo-script)
    - `cargo install cargo-script`

## Installation

1. First, clone the repo:
   ```
   git clone https://github.com/JoelCourtney/pantheon.git
   ```
2. In the root of the repo, run `./rmake install`. Yes, it is that sketchy.
3. Then set the `PANTHEON_ROOT` environment variable to the path where you cloned the repo. This needs to be set whenever pantheon is run, so you should probably put it in you `.bashrc` or `.zshrc`.
    ```
    export PANTHEON_ROOT=/path/to/pantheon
    ```

## Running

Run `pantheon` wherever you want. It will ~~erase~~ scan your files (recursively into subdirectories) and serve any files with the extension `.panth`.

Open up your browser to 

## Important Considerations

**Pros of installing my code on your computer:**

- You get a cool character sheet viewer.*
  - if it runs*
- Its free.

**Cons of installing my code on your computer:**

- It could be any of these wonderful things instead!
  - key logger
  - ransomware
  - crypto miner
  - broken
- If you don't like spaghetti, you won't like my code.
    - Your characters will like it even less.
- It uses http, not https.

I wouldn't install my code on my machine if I was me.