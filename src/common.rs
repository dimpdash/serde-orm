use std::fmt::Debug;

use crate::domain::Person;
pub type ForeignKey = (String, String);

pub trait Links<Config>: Debug {
    fn get_foreign_keys(&self) -> Vec<ForeignKey>;
    fn convert_fks_to_objs(&mut self, config: &Config);
}

#[derive(Debug)]
pub struct Wrapper<Config> {
    pub me: Person,
    pub obj_list: Vec<Box<dyn Links<Config>>>,
}
