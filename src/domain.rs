use std::{
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::ser::serialize_data;
use crate::{common::Linkable, de::deserialize_data};

use serde::{de, Deserialize};

use crate::{
    common::{ForeignKey, Links},
    Config,
};

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: i32,
    #[serde(deserialize_with = "deserialize_data")]
    #[serde(serialize_with = "serialize_data")]
    pub pet: Rc<Dog>,
}

#[derive(Default, Debug, serde::Serialize, Deserialize)]
pub struct Dog {
    pub name: String,
}

impl Links<Config> for Person {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let key = self.pet.get_key();
        vec![("pet".to_string(), key)]
    }

    fn convert_fks_to_objs(&mut self, config: &Config) {
        for pet in config.pets.iter() {
            let pet = Rc::clone(pet);
            self.pet = pet;
        }
    }
}

impl Linkable<String, Self> for Dog {
    fn get_key(&self) -> String {
        self.name.clone()
    }

    fn get_fake(key: String) -> Dog {
        Dog { name: key }
    }
}
