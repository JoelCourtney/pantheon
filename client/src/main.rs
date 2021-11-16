mod character;

use seed::{*, prelude::*};
use character::StoredCharacter;

struct Model {
    character: Option<StoredCharacter>,
    content: Option<u32>,
    page: Page
}

#[derive(Debug)]
enum Page {
    CharacterBrowser,
    CharacterViewer,
    CharacterBuilder,
    ContentBrowser
}

#[derive(Debug)]
enum Msg {
    ChangePage(Page)
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async {
    //         let greeter = tokio::spawn(greet());
    //         let _ = tokio::join!(greeter);
    //     });
    character::try_it();
    Model {
        character: None,
        content: None,
        page: Page::CharacterBrowser
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        format!("{:?}", model.page),
        // C!["counter"],
        button![
            "Character Browser",
            ev(Ev::Click, |_| Msg::ChangePage(Page::CharacterBrowser)),
        ],
        button![
            "Character Viewer",
            ev(Ev::Click, |_| Msg::ChangePage(Page::CharacterViewer)),
        ],
    ]
}

// async fn greet() {
//     log!("hello world");
// }

fn main() {
    App::start("app", init, update, view);
}
