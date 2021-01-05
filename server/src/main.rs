mod character;
mod modifier;
mod content;

use character::{StoredCharacter};

fn main() {
    let json = std::fs::read_to_string("test.json").unwrap();
    println!("{}", json);
    let char: StoredCharacter = serde_json::from_str(&json).unwrap();
    println!("{:?}", char);

    let res = char.resolve();
    println!("{:?}", res);
}
