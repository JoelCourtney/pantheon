use serde::Serialize;
use std::fmt::Debug;

pub trait Featured {
    fn receive_choice(&mut self, _choice: &str, _feature_index: usize, _choice_index: usize) {}
    fn write_features(&self) -> Vec<FeatureSerial> { vec![] }

    fn receive_feat_choice(&mut self, _choice: &str, _feat_index: usize, _feature_index: usize, _choice_index: usize) {}
    fn write_feats(&self) -> Vec<Vec<FeatureSerial>> { vec! [] }
}

#[derive(Debug, Serialize, Default)]
pub struct FeatureSerial {
    pub text: &'static str,
    pub choose: ChooseSerial
}

#[allow(dead_code)]
pub struct FeatSerial {
    pub text: &'static str,
    pub choose: ChooseSerial,
    pub generator: FeatGenerator
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

#[allow(dead_code)]
pub enum FeatGenerator {
    Race,
    Class
}