use rocket::{Rocket, State};
use rocket_contrib::serve::StaticFiles;
use std::sync::RwLock;
use crate::character::{StoredCharacter,FinalCharacter};
use rocket::response::content::Json;

pub(crate) fn ignite(path: &str) -> Rocket {
    let mut stored_char = StoredCharacter::from(path);
    let final_char = stored_char.resolve().unwrap();
    rocket::ignite()
        .manage(RwLock::new(stored_char))
        .manage(RwLock::new(final_char))
        .mount("/", StaticFiles::from("src/www"))
        .mount("/", routes![get_character])
}

// type StoredSync = RwLock<StoredCharacter>;
type FinalState<'a> = State<'a, RwLock<FinalCharacter>>;

#[post("/")]
fn get_character(final_state: FinalState) -> Json<String> {
    let final_char = final_state.inner().read().unwrap();
    Json(serde_json::to_string(&*final_char).expect("SERIALIZATION FAILED"))
}