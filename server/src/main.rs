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

#[deny(missing_docs)]

extern crate macros;

mod character;
mod modify;
mod content;
mod feature;

use character::StoredCharacter;

fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    // println!("{}", json);
    let char: StoredCharacter = serde_json::from_str(&json).unwrap();
    // println!("{:?}", char);

    let mut res = char.copy_to_full_character();
    res.resolve();
    dbg!(&res);
}

