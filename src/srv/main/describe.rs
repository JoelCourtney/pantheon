pub trait Describe {
    fn description_with_title() -> &'static str;
    fn description_without_title() -> &'static str {
        let text: &'static str = Self::description_with_title();
        let newline = text.find('\n')
            .expect("first first newline failed");
        let newline = newline + 1 + &text[newline+1..].find('\n')
            .expect("find second newline failed");
        &text[newline+1..]
    }
}