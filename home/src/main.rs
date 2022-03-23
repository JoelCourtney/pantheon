use pantheon::reexports::seed::{prelude::*, *};
use pantheon::{requests::query, shared::CharacterFile};

struct Model {
    characters: Option<Vec<CharacterFile>>,
}

#[derive(Debug)]
enum Msg {
    CharactersReceived(Vec<CharacterFile>),
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let list: Vec<CharacterFile> = query("list_characters", "").await.unwrap();
        Msg::CharactersReceived(list)
    });
    Model { characters: None }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::CharactersReceived(mut list) => {
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
            menu_lists.push((system_name, system_characters));
            menu_lists.remove(0);
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
                                characters.into_iter().map(|character| li! {
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
