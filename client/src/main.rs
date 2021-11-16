use seed::{*, prelude::*};

struct Model {
    character: Option<Character>,
    content: Option<u32>,
    page: Page
}

enum Character{}

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

fn main() {
    App::start("app", init, update, view);
}
