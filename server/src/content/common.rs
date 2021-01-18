use serde::{Serialize, Deserialize};
use crate::character::Character;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommonClassContent<const HD: usize> {
    level: usize,
    first_class: bool
}

impl<const HD: usize> CommonClassContent<HD> {
    pub fn initialize(&self, _: &mut Character) {

    }
    pub fn modify(&self, _: &mut Character) {}
    pub fn finalize(&self, c: &mut Character) {
        let mut res: i32 = 0;
        if self.first_class && self.level >= 1 {
            res += (HD / 2 - 1) as i32;
        }
        res += ((HD / 2 + 1) as i32 + c.constitution_modifier) * self.level as i32;
        c.max_health += res as usize;
    }
}