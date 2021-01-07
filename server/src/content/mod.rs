macros::register!("/");
use std::collections::HashMap;
use crate::character::*;
use maplit::hashmap;

#[derive(Debug)]
pub struct Registry {
    races: HashMap<Registration, fn() -> Box<dyn Race>>,
    feats: HashMap<Registration, fn() -> Box<dyn Feat>>
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            races: hashmap! {
                Registration {
                    source: "official",
                    book: "Player's Handbook",
                    name: "Human"
                } => crate::content::official::players_handbook::races::human::new as fn() -> Box<dyn Race>
            },
            feats: hashmap! {
                Registration {
                    source: "official",
                    book: "Player's Handbook",
                    name: "Alert"
                } => crate::content::official::players_handbook::feats::alert::new as fn() -> Box<dyn Feat>
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    source: &'static str,
    book: &'static str,
    name: &'static str
}
