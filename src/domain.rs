use serde::Deserialize;

use crate::common::{ForeignKey, Links};

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: i32,
    #[serde(skip)]
    pub pet: Dog,
}

#[derive(Default, Debug, serde::Serialize, Deserialize)]
pub struct Dog {
    pub name: String,
}

impl Links for Person {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        vec![("pet".to_string(), self.pet.name.clone())]
    }
}
