use rocket::{Rocket, State};
use std::sync::RwLock;
use crate::character::{StoredCharacter,FinalCharacter};
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::Deserialize;

struct SharedData {
    path: String,
    stored_char: RwLock<StoredCharacter>,
    final_char: RwLock<FinalCharacter>
}

pub(crate) fn ignite(path: String) -> Rocket {
    let mut stored_char = StoredCharacter::read(&*path);
    let final_char = stored_char.resolve().expect("ignite character resolve failed");
    let state = SharedData {
        path,
        stored_char: RwLock::new(stored_char),
        final_char: RwLock::new(final_char)
    };
    rocket::ignite()
        .manage(state)
        .mount("/", routes![serve_root, serve_static_file, get_character, edit_character])
}

#[post("/")]
fn get_character(state: State<SharedData>) -> content::Json<String> {
    let final_char = state.inner().final_char.read()
        .expect("could not get get final read lock");
    content::Json(serde_json::to_string(&*final_char).expect("SERIALIZATION FAILED"))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "field", content = "value")]
enum EditRequest<'a> {
    Name(String),
    Health(u32),
    TempHealth(u32),
    Feature {
        container: &'a str,
        feature_index: usize,
        choice_index: usize,
        choice: &'a str
    }
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
        Health(u) => (*stored_char).health = u,
        TempHealth(u) => (*stored_char).temp_health = u,
        Feature {
            container,
            feature_index,
            choice_index,
            choice
        } => {
            match match container {
                "race" => &mut (*final_char).race_traits,
                "class" => &mut (*final_char).class_features,
                "background" => &mut (*final_char).background_features,
                "feat" => &mut (*final_char).feat_features,
                _ => panic!(format!("no container found: {}", container))
            }.get_mut(feature_index)
                .expect(&format!("feature index out of bounds: {}", feature_index)).1 {
                Some(c) => unsafe {
                    (*c).choose(choice, choice_index);
                }
                None => panic!(format!("no choice here to choose from: {:?}", feature_index))
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

    Content(ContentType::HTML, include_bytes!("../../www/index.html"))
}

#[get("/<path..>")]
fn serve_static_file(path: std::path::PathBuf) -> Option<rocket::response::Content<&'static [u8]>> {
    use rocket::response::content::Content;
    use rocket::http::ContentType;

    let content_type = ContentType::from_extension(path.extension().unwrap().to_str().unwrap()).unwrap();
    let bytes = macros::match_raw_files!([
        "modules",
        "images/icon/icon.png",
        "scripts",
        "uikit/dist/css/uikit.dndcent-theme.min.css",
        "uikit/dist/js/uikit.min.js",
        "uikit/dist/js/uikit-icons.min.js",
        "fonts/Rajdhani/Rajdhani-Regular.ttf",
        "fonts/Montserrat/Montserrat-Regular.ttf",
        "fonts/Montserrat/Montserrat-Bold.ttf",
        "fonts/Montserrat/Montserrat-BlackItalic.ttf"
    ]);
    Some(Content(content_type, bytes))
}