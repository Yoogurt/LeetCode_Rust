pub trait Vec2OwnedString {
    fn to_owned_string(self) -> Vec<String>;
}

impl<'a, F> Vec2OwnedString for F where F: IntoIterator<Item=&'a str> {
    fn to_owned_string(self) -> Vec<String> {
        self.into_iter().map(|value| (*value).to_owned()).collect::<Vec<_>>()
    }
}