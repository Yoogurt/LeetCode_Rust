pub trait Vec2OwnedString {
    fn to_owned_string(&self) -> Vec<String>;
}

impl Vec2OwnedString for Vec<&str> {
    fn to_owned_string(&self) -> Vec<String> {
        return self.iter().map(|value| (*value).to_owned()).collect::<Vec<_>>()
    }
}