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
mod content;
mod feature;
mod describe;
mod misc;

use character::StoredCharacter;

fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    // dbg!(&json);
    let char: StoredCharacter = serde_json::from_str(&json).expect("DESERIALIZATION FAILED");
    // dbg!(&char);
    let final_char = char.resolve().unwrap();
    dbg!(&final_char);
    // let mut class = content::class("Rogue").unwrap();
    // class.receive_choice("Assassin",0, 0);
    // dbg!(serde_json::to_string(&class).unwrap());


    // let mut race = content::race("Halfling").unwrap();
    // dbg!(&race);
    // dbg!(race.features());
    // race.receive_choice("Lightfoot", 9, 0);
    // dbg!(&race);
    // dbg!(race.features());
}
