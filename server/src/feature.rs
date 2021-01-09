use serde::Serialize;
use std::fmt::Debug;

pub trait Featured {
    fn receive_choice(&mut self, _choice: &str, _feature_index: usize, _choice_index: usize) {}
    fn write_features(&self) -> Vec<FeatureSerial> { vec![] }
}

#[derive(Debug, Serialize, Default)]
pub struct FeatureSerial {
    pub text: &'static str,
    pub choose: ChooseSerial
}

#[derive(Debug, Serialize, Default)]
pub struct ChooseSerial {
    pub current_choices: Vec<&'static str>,
    pub all_choices: Vec<Vec<&'static str>>
}

pub trait Choose {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choose_serial(&self, unique: bool) -> ChooseSerial;
}
