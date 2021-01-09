pub trait Describe {
    fn description_with_title() -> &'static str;
    fn description_without_title() -> &'static str {
        let text: &'static str = Self::description_with_title();
        let newline = text.find('\n').unwrap();
        let newline = newline + 1 + &text[newline+1..].find('\n').unwrap();
        &text[newline+1..]
    }
}