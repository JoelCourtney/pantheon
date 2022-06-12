pub mod elements;

use crate::{
    requests::{send_query, QueryError},
    shared::Query,
    system::{CharacterError, SetName, System},
};
use seed::{prelude::*, *};
use std::path::PathBuf;

use self::elements::UiError;

struct State<S: System + 'static> {
    character: CharacterRequest<S>,
    system_state: S::State,
}

#[derive(Clone)]
pub enum Message<S: System + 'static> {
    CharacterRequest(CharacterRequest<S>),
    Menu(MenuOption),
    Custom(S::Message),
}

impl<S: System + 'static> Message<S> {
    pub fn from(custom: S::Message) -> Self {
        Message::Custom(custom)
    }
}

#[derive(Clone)]
pub enum CharacterRequest<S: System + 'static> {
    Success(S::MinCharacter),
    Failure(UiError<Message<S>>),
    None,
}

#[derive(Clone)]
pub enum MenuOption {
    Play,
    Build,
    Browse,
}

pub fn run<S: System + 'static>() {
    App::start(
        "app",
        // *vomit*
        // Future Joel here. WTF is this shit
        &init
            as &dyn Fn(
                Url,
                &mut seed::app::OrdersContainer<Message<S>, State<S>, Vec<Node<Message<S>>>>,
            ) -> State<S>,
        update,
        view,
    );
}

fn init<S: System>(_url: Url, orders: &mut impl Orders<Message<S>>) -> State<S> {
    orders.perform_cmd(async {
        let url = Url::current();
        let character_name = url.search().get("c");
        if let Some(v) = character_name {
            if v.len() == 1 {
                let mut path = PathBuf::from(v.first().unwrap());
                path.set_extension(format!("{}.panth", S::NAME));
                let query = send_query::<S::MinCharacter>(Query::ReadCharacter(path.clone())).await;
                match query {
                    Ok(min) => Message::<S>::CharacterRequest(CharacterRequest::Success(min)),
                    Err(QueryError::Bincode(_)) => {
                        let string_query = send_query::<String>(Query::ReadCharacter(path)).await;
                        match string_query {
                            Ok(name) => {
                                let mut default = S::MinCharacter::default();
                                default.set_name(name);
                                Message::CharacterRequest(CharacterRequest::Success(default))
                            }
                            Err(e) => {
                                Message::CharacterRequest(CharacterRequest::Failure(UiError {
                                    title: "Could not deserialize character.".to_string(),
                                    body: format!(
                                        "Serialiation failed, attempted to get name: {e}"
                                    ),
                                    message: Box::new(Message::CharacterRequest(
                                        CharacterRequest::None,
                                    )),
                                }))
                            }
                        }
                    }
                    Err(e) => Message::CharacterRequest(CharacterRequest::Failure(UiError {
                        title: "Could not get character.".to_string(),
                        body: e.to_string(),
                        message: Box::new(Message::CharacterRequest(CharacterRequest::None)),
                    })),
                }
            } else {
                Message::CharacterRequest(CharacterRequest::Failure(UiError {
                    title: "whomst'd've".to_string(),
                    body: format!(
                        "Something is wrong with the `c` GET parameter: {:?}",
                        url.search().get("c")
                    ),
                    message: Box::new(Message::CharacterRequest(CharacterRequest::None)),
                }))
            }
        } else {
            Message::CharacterRequest(CharacterRequest::Failure(UiError {
                title: "whomst'd've".to_string(),
                body: format!(
                    "Something is wrong with the `c` GET parameter: {:?}",
                    url.search().get("c")
                ),
                message: Box::new(Message::CharacterRequest(CharacterRequest::None)),
            }))
        }
    });
    State {
        character: CharacterRequest::None,
        system_state: S::State::default(),
    }
}

fn update<S: System>(msg: Message<S>, state: &mut State<S>, _orders: &mut impl Orders<Message<S>>) {
    use Message::*;
    match msg {
        CharacterRequest(r) => state.character = r,
        _ => todo!(),
    }
}

fn view<S: System>(state: &State<S>) -> Vec<Node<Message<S>>> {
    nodes! {
        elements::MenuBar(vec![
            ("Play".to_string(), Message::Menu(MenuOption::Play)),
            ("Build".to_string(), Message::Menu(MenuOption::Build)),
            ("Browse".to_string(), Message::Menu(MenuOption::Browse))
        ]),
        match &state.character {
            CharacterRequest::Failure(fail) => fail.clone().into_nodes(),
            CharacterRequest::Success(min) => {
                let character = min.clone().into();
                match S::view(&state.system_state, character) {
                    Ok(nodes) => nodes,
                    Err(CharacterError::Deadlock) => UiError {
                        title: "Character Evaluation Deadlock".to_string(),
                        body: "Encountered a dependency cycle when evaluating character.".to_string(),
                        message: Box::new(Message::CharacterRequest(CharacterRequest::None))
                    }.into_nodes(),
                    Err(CharacterError::SystemError(error)) => UiError {
                        title: format!("Custom {} error encountered", S::NAME),
                        body: error.to_string(),
                        message: Box::new(Message::CharacterRequest(CharacterRequest::None))
                    }.into_nodes()
                }
            }
            _ => vec![]
        }
    }
}
