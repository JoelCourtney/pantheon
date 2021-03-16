//! This is the server backend for DnDCent. It both serves the pages and responds to requests about
//! the character object.
//!
//! # Overview
//!
//! The general process of serving the content is as follows:
//!
//! 1. Startup and load the character file given as command line arg.
//! 2. Open a browser to localhost.
//! 3. Respond to requests.
//!
//! The server is responsible for resolving [StoredCharacters](StoredCharacter) into [Characters](character::Character).
//! That is a very (very) involved process and is the whole reason why this project is insane.
//! More deats can be found in the documentation for other files.

#![feature(proc_macro_hygiene, decl_macro)]
#[deny(missing_docs)]

#[macro_use] extern crate rocket;

extern crate macros;

mod server;
mod character;
mod content;
mod feature;
mod moves;
mod misc;

fn main() {
    let mut args = std::env::args();
    {
        if args.len() != 2 {
            server::ignite("test_character.json".to_string(), true)
        } else {
            server::ignite(args.nth(1).expect("get 1st arg failed"), false)
        }
    }.launch();
}