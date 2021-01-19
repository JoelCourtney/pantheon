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
    let in_json = std::fs::read_to_string("test_variant_human.json").unwrap();
    let mut stored_char: StoredCharacter = serde_json::from_str(&in_json).expect("DESERIALIZATION FAILED");
    let char = stored_char.resolve().unwrap();
    let mut final_char = char.finalize();
    match final_char.race_traits[0].1 {
        Some(ref mut c) => {
            dbg!(&c);
            c.choose("Constitution", 1);
        }
        None => {}
    }
    let out_json = serde_json::to_string(&final_char).expect("SERIALIZATION FAILED");
    let back_json = serde_json::to_string(&stored_char).expect("SERIALIZATION FAILED");
    println!("{}",&back_json);
    println!("{}",&out_json);
    // final_char.race_traits[0].1.as_deref_mut().unwrap().choose("Terran", 0);
    // dbg!(&final_char);
    // dbg!(&stored_char);
    // let json = serde_json::to_string(&final_char).unwrap();
    // dbg!(json);

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
