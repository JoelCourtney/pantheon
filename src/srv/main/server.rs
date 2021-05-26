use rocket::{Rocket, State, config::{Environment, Config}};
use std::sync::RwLock;
use crate::character::{StoredCharacter,FinalCharacter};
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::Deserialize;
use rocket_contrib::serve::StaticFiles;
use rocket::config::LoggingLevel;
use crate::misc::Ability;
use crate::ui::Element;
use crate::moves::Move;
use crate::content::Registration;

struct SharedData {
    path: String,
    stored_char: RwLock<StoredCharacter>,
    final_char: RwLock<FinalCharacter>
}

pub(crate) fn ignite(path: String) -> Rocket {
    let dev = cfg!(debug_assertions);
    let config = Config::build(Environment::active()
                               .expect("active rocket env not found"))
        .address("localhost")
        .port(8000)
        .log_level(if cfg!(debug_assertions) {LoggingLevel::Normal} else {LoggingLevel::Off})
        .secret_key(make_secret_key())
        .finalize()
        .expect("failed to config rocket");
    let mut stored_char = StoredCharacter::read(&*path);
    let final_char = stored_char.resolve().expect("ignite character resolve failed");
    let state = SharedData {
        path,
        stored_char: RwLock::new(stored_char),
        final_char: RwLock::new(final_char)
    };
    let rocket = rocket::custom(config)
        .manage(state)
        .mount("/", routes![get_character, edit_character, serve_registry, serve_description]);
    return if dev {
        rocket.mount("/", StaticFiles::from("src/www/build"))
    } else {
        rocket.mount("/", routes![serve_root, serve_static_file])
    }
}

/// Generates a 256 bit secret key for rocket's cookies.
///
/// I know this makes cookies unusable between sessions.
/// Good thing I don't use cookies.
fn make_secret_key() -> String {
    let bytes = rand::random::<[u8; 32]>();
    base64::encode(&bytes)
}

#[post("/")]
fn get_character(state: State<SharedData>) -> content::Json<String> {
    let final_char = state.inner().final_char.read()
        .expect("could not get get final read lock");
    content::Json(serde_json::to_string(&*final_char).expect("SERIALIZATION FAILED"))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum EditRequest<'a> {
    Name(String),
    Description(String),
    Health(u32),
    TempHealth(u32),
    Choice {
        container: Container,
        element_index: usize,
        choice_index: usize,
        choice: &'a str
    },
    Toggle {
        container: Container,
        element_index: usize,
        toggle_index: usize
    },
    Race(&'a str),
    Class {
        index: usize,
        name: &'a str,
    },
    Level {
        index: usize,
        level: u32
    },
    Background(&'a str),
    AbilityScore(Ability, u32)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Container {
    Race,
    Class(usize),
    Background,
    Feat,
    Moves
}

#[post("/edit", format="json", data="<data>")]
fn edit_character(data: Json<EditRequest>, state: State<SharedData>) -> content::Json<String> {
    let mut final_char = state.inner().final_char.write()
        .expect("could not get edit final write lock");
    let mut stored_char = state.inner().stored_char.write()
        .expect("could not get edit stored write lock");
    use EditRequest::*;
    match data.into_inner() {
        Name(s) => (*stored_char).name = s,
        Description(s) => (*stored_char).description = s,
        Health(u) => (*stored_char).health = u,
        TempHealth(u) => (*stored_char).temp_health = u,
        Race(r) => (*stored_char).race = crate::content::race(r).unwrap(),
        Class { index, name} => {
            if index == (*stored_char).classes.len() {
                (*stored_char).classes.push(
                    (crate::content::class(name).unwrap(), 1)
                );
            } else {
                (*stored_char).classes[index] = (crate::content::class(name).unwrap(), 1);
            }
        }
        Level { index, level } => {
            if level == 0 {
                (*stored_char).classes.remove(index);
            } else if level > 20 {
                panic!("levels can't go above 20");
            } else {
                (*stored_char).classes[index].1 = level;
            }
        }
        Background(b) => (*stored_char).background = crate::content::background(b).unwrap(),
        Choice {
            container,
            element_index,
            choice_index,
            choice
        } => {
            match match container {
                Container::Race => &mut (*final_char).race_traits,
                Container::Class(index) => &mut (*final_char).class_features[index],
                Container::Background => &mut (*final_char).background_features,
                Container::Feat => &mut (*final_char).feats,
                _ => panic!("no choice container found: {:?}", container)
            }.get_mut(element_index)
                .expect(&format!("element index out of bounds: {}", element_index)) {
                Element::Choice {
                    data, ..
                } => unsafe { (**data).choose(choice, choice_index) }
                _ => panic!("element must be a choice")
            }
        }
        Toggle {
            container,
            element_index,
            toggle_index
        } => {
            match match container {
                Container::Moves => &mut (*final_char).moves,
                _ => panic!("no toggle container found: {:?}", container)
            }.get_mut(element_index)
                .expect(&format!("element index out of bounds: {}", element_index)) {
                Move::Other {
                    element: Element::Toggle {
                        data, ..
                    }, ..
                } => unsafe { (**data).toggle(toggle_index) }
                _ => panic!("element must be a toggle")
            }
        }
        AbilityScore(a, n) => {
            match a {
                Ability::Strength => (*stored_char).base_abilities.strength = n,
                Ability::Dexterity => (*stored_char).base_abilities.dexterity = n,
                Ability::Constitution => (*stored_char).base_abilities.constitution = n,
                Ability::Intelligence => (*stored_char).base_abilities.intelligence = n,
                Ability::Wisdom => (*stored_char).base_abilities.wisdom = n,
                Ability::Charisma => (*stored_char).base_abilities.charisma = n,
                Ability::Unknown => panic!("cannot edit unknown base score")
            }
        }
    }
    *final_char = stored_char.resolve().expect("edit character resolve failed");
    std::mem::drop(final_char);
    stored_char.write(&*state.path);
    get_character(state)
}

#[get("/")]
fn serve_root() -> rocket::response::Content<&'static [u8]> {
    use rocket::response::content::Content;
    use rocket::http::ContentType;

    Content(ContentType::HTML, include_bytes!("../../www/build/index.html"))
}

#[get("/<path..>")]
fn serve_static_file(path: std::path::PathBuf) -> Option<rocket::response::Content<&'static [u8]>> {
    use rocket::response::content::Content;
    use rocket::http::ContentType;

    let content_type = ContentType::from_extension(path.extension().unwrap().to_str().unwrap()).unwrap();
    let bytes = proc_macros::match_raw_files!([
        ""
    ]);
    Some(Content(content_type, bytes))
}

#[post("/registry")]
fn serve_registry() -> content::Json<String> {
    content::Json(
        serde_json::to_string(&crate::content::get_registrations())
            .expect("could not convert registrations to json")
    )
}

#[post("/description", format="json", data="<data>")]
fn serve_description(data: Json<Registration>) -> content::Json<String> {
    content::Json(
        serde_json::to_string(&crate::content::get_description(data.into_inner()))
            .expect("could not convert description to json")
    )
}
