mod character;
mod macros;

use seed::{*, prelude::*};
use character::StoredCharacter;

struct Model {
    character: Option<StoredCharacter>,
    overlay: Overlay
}

#[derive(Debug)]
enum Overlay {
    Full,
    Partial,
    None
}

#[derive(Debug)]
enum Msg {
    SetCharacter(Option<StoredCharacter>),
    SetOverlay(Overlay)
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    character::try_it();
    Model {
        character: None,
        overlay: Overlay::None
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetCharacter(new_char) => model.character = new_char,
        Msg::SetOverlay(new_overlay) => model.overlay = new_overlay
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    svg![
        style! {
            "width" => unit!(100, %)
            "height" => unit!(100, %)
        }
    ]
}

// async fn greet() {
//     log!("hello world");
// }

fn main() {
    App::start("app", init, update, view);
}
