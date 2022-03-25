use pantheon::system::*;
use pantheon::reexports::serde::{Serialize, Deserialize};
use pantheon::reexports::thiserror::Error;

#[derive(Default, Clone)]
struct Template;

impl System for Template {
    type MinCharacter = MinCharacter;
    type Character = Character;

    type State = ();
    type Message = ();

    const NAME: &'static str = "template";
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(crate = "pantheon::reexports::serde")]
struct MinCharacter {
    name: String
}

impl SetName for MinCharacter {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

type L<T> = Lazy<T, Template, EvalError>;

#[derive(Default)]
struct Character {
    name: L<String>
}

impl From<MinCharacter> for Character {
    fn from(min: MinCharacter) -> Character {
        let c = Character::default();
        pantheon::ops! {
            c;
            name 0 => *name = min.name;
        }
        c
    }
}

#[derive(Error, Debug)]
enum EvalError {}

fn main() {
    pantheon::ui::run::<Template>();
}
