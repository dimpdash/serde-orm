use crate::domain::Person;

pub type ForeignKey = (String, String);

pub trait Links {
    fn get_foreign_keys(&self) -> Vec<ForeignKey>;
}

#[derive(Debug)]
pub struct Wrapper {
    pub me: Person,
}
