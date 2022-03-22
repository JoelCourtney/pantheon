use seed::{*, prelude::*};
use reqwest::Body;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use anyhow::Result;

struct Model {
}

#[derive(Debug)]
enum Msg {
    Hello
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let list: Vec<PathBuf> = query("list_characters", "").await.unwrap();
        log!(&list);
        // let _: () = query("write_character/asdf.bin", bincode::serialize("Hello World!").unwrap()).await.unwrap();
        // let character: String = query("read_character/asdf.bin", "").await.unwrap();
        // log(character);
        Msg::Hello
    });
    Model {
    }
}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
}

fn view(_model: &Model) -> impl IntoNodes<Msg> {
    section! {
        C!["section"],
        h1! {
            C!["title"],
            "Hello World!"
        },
        p! {
            C!["subtitle"],
            "My first website with ",
            strong!("Bulma!")
        }
    }
}

fn main() {
    App::start("app", init, update, view);
}

async fn query<T: DeserializeOwned>(path: impl AsRef<str>, body: impl Into<Body>) -> Result<T> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/{}", seed::window().location().origin().unwrap(), path.as_ref()))
        .body(body).send()
        .await?;
    let bytes = res.bytes().await?;
    Ok(bincode::deserialize(&bytes)?)
}