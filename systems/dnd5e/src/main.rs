use pantheon::reexports::seed::{prelude::*, *};
use pantheon::reexports::serde::{Deserialize, Serialize};
use pantheon::reexports::thiserror::Error;
use pantheon::system::*;
use pantheon::ui::Message;
use pantheon::{eval, ops};

#[derive(Default, Clone)]
struct Dnd5e;

impl System for Dnd5e {
    type MinCharacter = MinCharacter;
    type Character = Character;

    type SystemError = EvalError;

    type State = ();
    type Message = ();

    const NAME: &'static str = "dnd5e";

    fn view(
        _state: &(),
        character: Character,
    ) -> CharacterResult<Vec<Node<Message<Self>>>, EvalError> {
        Ok(nodes! {
            h1! {
                C!("title"),
                eval!(character.name)
            }
        })
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(crate = "pantheon::reexports::serde")]
struct MinCharacter {
    name: String,
}

impl SetName for MinCharacter {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

type L<T> = Lazy<T, Dnd5e, EvalError>;

#[derive(Default)]
struct Character {
    name: L<String>,
}

impl TryFrom<MinCharacter> for Character {
    type Error = EvalError;

    fn try_from(min: MinCharacter) -> Result<Character, EvalError> {
        let c = Character::default();
        ops! {
            c;
            name 0 => *name = min.name;
        }
        Ok(c)
    }
}

#[derive(Error, Debug)]
enum EvalError {}

fn main() {
    pantheon::ui::run::<Dnd5e>();
}
