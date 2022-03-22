use seed::{*, prelude::*};
use reqwest::Body;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use anyhow::*;

struct Model {
    characters: Option<Vec<CharacterEntry>>
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct CharacterEntry {
    system: String,
    prefix: String,
    name: String
}

impl TryFrom<PathBuf> for CharacterEntry {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> Result<Self> {
        let full_path = path.to_str().ok_or(anyhow!("couldn't covert full path"))?;
        let file_name = path.file_name().ok_or(anyhow!("no file name"))?
            .to_str().ok_or(anyhow!("could not convert OS string"))?
            .to_owned();
        let prefix_end = full_path.find(&file_name).unwrap();
        let first_dot = file_name.find(".").ok_or(anyhow!("no first dot"))?;
        let second_dot = &file_name[first_dot+1..].find(".").ok_or(anyhow!("no second dot"))? + first_dot + 1;
        Ok(
            CharacterEntry {
                system: file_name[first_dot+1..second_dot].to_string(),
                prefix: full_path[..prefix_end].to_string(),
                name: file_name[..first_dot].to_string()
            }
        )
    }
}

#[derive(Debug)]
enum Msg {
    CharactersReceived(Vec<PathBuf>)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let list: Vec<PathBuf> = query("list_characters", "").await.unwrap();
        Msg::CharactersReceived(list)
    });
    Model {
        characters: None
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::CharactersReceived(list) => {
            let mut list: Vec<CharacterEntry> = list.into_iter().map(|path| path.try_into().unwrap()).collect();
            list.sort();
            model.characters = Some(list);
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.characters {
        None => section! {
            C!["section"],
            h1! {
                C!("title"),
                "Pantheon"
            },
            p! {
                C!("subtitle"),
                "Loading characters..."
            },
            progress! {
                C!("progress is-small is-primary"),
                attrs! {
                    At::Max => "100"
                }
            }
        },
        Some(list) => {
            let mut menu_lists = Vec::new();
            let mut system_name = String::new();
            let mut system_characters = Vec::new();
            for character in list.iter() {
                if system_name != character.system {
                    menu_lists.push((system_name, system_characters));
                    system_name = character.system.clone();
                    system_characters = Vec::new();
                }
                system_characters.push(character);
            }
            menu_lists.remove(0);
            menu_lists.push((system_name, system_characters));
            section! {
                C!["section"],
                h1! {
                    C!("title"),
                    "Pantheon"
                },
                div! {
                    C!("container"),
                    aside! {
                        C!("menu"),
                        menu_lists.into_iter().map(|(name, characters)| vec![
                            p! {
                                C!("menu-label"),
                                name
                            },
                            ul! {
                                C!("menu-list"),
                                characters.iter().map(|character| li! {
                                    a!(character.name.clone())
                                })
                            }
                        ])
                    }
                }
            }
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
