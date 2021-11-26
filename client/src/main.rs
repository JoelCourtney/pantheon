mod character;
mod macros;

use seed::{*, prelude::*};
use character::StoredCharacter;
use std::path::PathBuf;

struct Model {
    base: BaseLayer,
    overlay: Overlay
}

#[derive(Debug)]
enum BaseLayer {
    List(RequestProgress<Vec<PathBuf>>),
    Character(RequestProgress<StoredCharacter>)
}

#[derive(Debug)]
enum RequestProgress<T> {
    Requested,
    Succeeded(T),
    Failed
}

#[derive(Debug)]
enum Overlay {
    Full,
    Partial,
    None
}

#[derive(Debug)]
enum Msg {
    SetBase(BaseLayer),
    SetOverlay(Overlay)
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!(url.to_string());
    character::try_it();
    orders.perform_cmd(async {
        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/list")
            .send()
            .await.unwrap();
        let bytes = res.bytes().await.unwrap();
        log!("bytes: {:?}", bytes);
        let list = bincode::deserialize(&bytes).unwrap();
        log!(&list);
        Msg::SetBase(BaseLayer::List(RequestProgress::Succeeded(list)))
    });
    Model {
        base: BaseLayer::List(RequestProgress::Requested),
        overlay: Overlay::None
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetBase(base) => model.base = base,
        Msg::SetOverlay(new_overlay) => model.overlay = new_overlay,
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    svg![
    ]
}

fn main() {
    App::start("app", init, update, view);
}
