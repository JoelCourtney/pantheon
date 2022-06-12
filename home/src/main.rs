use heck::*;
use pantheon::reexports::seed::{prelude::*, *};
use pantheon::ui::elements::UiError;
use pantheon::{
    requests::send_query,
    shared::{CharacterFile, Query},
};
use sanitise_file_name::{sanitise_with_options, Options};

struct Model {
    characters: Option<Vec<CharacterFile>>,
    systems: Option<Vec<String>>,
    character_name: String,
    system: Option<String>,
    character_error: Option<UiError<Msg>>,
}

#[derive(Debug, Clone)]
enum Msg {
    CharactersReceived(Vec<CharacterFile>),
    SystemsReceived(Vec<String>),
    NameChanged(String),
    SystemChanged(String),
    CreateCharacter,
    CreateSuccess(String),
    ErrorDeleted,
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let list: Vec<CharacterFile> = send_query(Query::ListCharacters).await.unwrap();
        Msg::CharactersReceived(list)
    });
    orders.perform_cmd(async {
        let list: Vec<String> = send_query(Query::ListSystems).await.unwrap();
        Msg::SystemsReceived(list)
    });
    Model {
        characters: None,
        systems: None,
        character_name: "".to_string(),
        system: None,
        character_error: None,
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CharactersReceived(mut list) => {
            list.sort();
            model.characters = Some(list);
        }
        Msg::SystemsReceived(mut list) => {
            list.sort();
            model.systems = Some(list);
        }
        Msg::NameChanged(s) => {
            model.character_name = s;
        }
        Msg::SystemChanged(s) => {
            model.system = Some(s);
        }
        Msg::CreateCharacter => {
            let name_info = process_name(&model.character_name);
            if let Some(system) = &model.system {
                let without_extension = format!("{}{}", &name_info.prefix, &name_info.file_stem);
                let full_path = format!("{without_extension}.{}.panth", system);
                let contents =
                    pantheon::reexports::bincode::serialize(&name_info.pretty_name).unwrap();
                orders.perform_cmd(async move {
                    let _: () = send_query(Query::WriteCharacter(full_path.into(), contents))
                        .await
                        .unwrap();
                    Msg::CreateSuccess(without_extension)
                });
            } else {
                model.character_error = Some(UiError {
                    title: "Creation failed.".to_string(),
                    body: "Please select an RPG system.".to_string(),
                    message: Box::new(Msg::ErrorDeleted),
                })
            }
        }
        Msg::CreateSuccess(path) => {
            let full_path = format!("systems/{}/?c={path}", model.system.clone().unwrap());
            orders.perform_cmd(async move {
                Url::go_and_load_with_str(full_path);
            });
        }
        Msg::ErrorDeleted => {
            model.character_error = None;
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.characters {
        None => nodes![
            pantheon::ui::elements::MenuBar(vec![]),
            view_center_box(vec![p! {
                C!("title"),
                "Loading characters..."
            }])
        ],
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
            nodes![
                pantheon::ui::elements::MenuBar(vec![]),
                view_center_box(vec![
                    p! {
                        C!("subtitle"),
                        "Create a character:"
                    },
                    div! {
                        C!("block"),
                        IF!(model.character_error.is_some() => model.character_error.clone().unwrap().into_nodes()),
                    },
                    div! {
                        C!("box"),
                        form! {
                            div! {
                                C!("field is-grouped"),
                                div! {
                                    C!("control is-expanded"),
                                    input! {
                                        C!("input"),
                                        attrs! {
                                            At::Type => "text"
                                            At::Placeholder => "Name"
                                        },
                                        input_ev(Ev::Input, Msg::NameChanged)
                                    }
                                },
                                div! {
                                    C!("control"),
                                    div! {
                                        C!("select"),
                                        select! {
                                            input_ev(Ev::Input, Msg::SystemChanged),
                                            option! {
                                                attrs! {
                                                    At::Selected => model.system.is_none().as_at_value(),
                                                    At::Disabled => true.as_at_value()
                                                },
                                                "system"
                                            },
                                            {
                                                if let Some(list) = &model.systems {
                                                    list.iter().map(|sys| option! {
                                                        attrs! {
                                                            At::Selected => (model.system.as_ref() == Some(sys)).as_at_value()
                                                        },
                                                        sys
                                                    }).collect()
                                                } else {
                                                    vec![]
                                                }
                                            }
                                        }
                                    }
                                },
                                p! {
                                    C!("control"),
                                    a! {
                                        C!("button is-primary"),
                                        attrs! {
                                            At::Type => "submit",
                                        },
                                        "Create",
                                        ev(Ev::Click, |_| Msg::CreateCharacter)
                                    }
                                },
                            },
                            IF!(!model.character_name.is_empty() =>
                                p! {
                                    C!("help"),
                                    "Actual file name: ",
                                    span! {
                                        C!("has-text-weight-bold"),
                                        {
                                            let name_info = process_name(&model.character_name);
                                            format!("{}{}.{}.panth",
                                                name_info.prefix,
                                                name_info.file_stem,
                                                model.system.as_ref().unwrap_or(&"<system>".to_string())
                                            )
                                        }
                                    }
                                }
                            )
                        }
                    },
                    p! {
                        C!("subtitle"),
                        "Choose a character:"
                    },
                    div! {
                        C!("box"),
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
                                        a! {
                                            attrs! {
                                                At::Href => format!(
                                                    "{}/systems/{}/?c={}{}",
                                                    window().location().origin().unwrap(),
                                                    &character.system,
                                                    &character.prefix,
                                                    &character.name
                                                )
                                            },
                                            span! {
                                                C!("has-text-grey"),
                                                character.prefix.clone()
                                            },
                                            span! {
                                                C!("has-text-weight-bold"),
                                                character.name.clone().to_title_case()
                                            },
                                        }
                                    })
                                }
                            ])
                        }
                    }
                ])
            ]
        }
    }
}

fn process_name(raw: &str) -> NameInfo {
    let last_slash = raw.rfind('/');
    let (prefix, pretty_name) = if let Some(i) = last_slash {
        (raw[..=i].to_string(), raw[i + 1..].to_string())
    } else {
        ("".to_string(), raw.to_string())
    };
    let file_stem = format_file_stem(&pretty_name);
    NameInfo {
        pretty_name,
        file_stem,
        prefix,
    }
}

fn sanitize_path(raw: &str) -> String {
    sanitise_with_options(
        raw,
        &Options {
            url_safe: true,
            ..Options::DEFAULT
        },
    )
}

fn format_file_stem(raw: &str) -> String {
    sanitize_path(&raw.to_snek_case())
}

struct NameInfo {
    pretty_name: String,
    file_stem: String,
    prefix: String,
}

fn view_center_box(content: Vec<Node<Msg>>) -> impl IntoNodes<Msg> {
    section! {
        C!["section"],
        div! {
            C!("container is-max-desktop"),
            content
        }
    }
}

fn main() {
    App::start("app", init, update, view);
}
