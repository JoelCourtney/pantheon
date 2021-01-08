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
mod describe;
mod misc;

use character::StoredCharacter;
use crate::character::Race;

fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    // println!("{}", json);
    let mut char: StoredCharacter = serde_json::from_str(&json).expect("DESERIALIZATION FAILED");
    // println!("{:?}", char);

    // EXAMPLE OF FEATURE CHOICES
    // StoredCharacter is resolved into Character. Then the extra language choice given by human
    // is dereferenced and changed from Unspecified to Auran. This edits the value stored in
    // char.human. Note this is the ORIGINAL STOREDCHARACTER. Thus, when a request comes in
    // to change the choice for a feature kept in the RESOLVED character, you can easily edit
    // and save the value in the original StoredCharacter.
    //
    // Then you have to trash the old Character and re-resolve one from the now-edited StoredCharacter.
    // It sounds dumb, but this is actually really cool.
    dbg!(&char);
    {
        let mut res = char.resolve();
        dbg!(&res);
        res.traits.last_mut().unwrap().choose.choose("Terran", 0);
    }
    let res = char.resolve();
    dbg!(res);
    dbg!(&char);
    let human = content::race("Human");
    dbg!(human);
    dbg!(content::get_all_race());

    let mut race: Box<dyn Race> = Box::default();
    use feature::Choose;
    let mut choice = Box::<dyn Race>::choose(&mut race);
    dbg!(choice.choices());
    choice.choose("Variant Human", 0);
    dbg!(choice);
    dbg!(race);
}
