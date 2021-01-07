pub trait Describe {
    fn describe() -> &'static str;
    fn describe_titleless() -> &'static str {
        let text: &'static str = Self::describe();
        let newline = text.find('\n').unwrap();
        let newline = &text[newline+1..].find('\n').unwrap();
        &text[newline+1..]
    }
}