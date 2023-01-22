use std::{
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::de::deserialize_data;

use serde::{de, Deserialize};

use crate::{
    common::{ForeignKey, Links},
    Config,
};

#[derive(Debug, Default)]
pub enum Link<F, O>
where
    O: Debug,
    F: Debug + Default,
{
    FK(F),
    OBJ(O),
    #[default]
    NONE,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: i32,
    #[serde(deserialize_with = "deserialize_data")]
    // #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub pet: Link<String, Rc<Dog>>,
}

#[derive(Default, Debug, serde::Serialize, Deserialize)]
pub struct Dog {
    pub name: String,
}

impl Links<Config> for Person {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let key = match &self.pet {
            Link::FK(key) => key.to_owned(),
            Link::OBJ(pet) => pet.name.to_string(),
            Link::NONE => todo!(),
        };

        vec![("pet".to_string(), key)]
    }

    fn convert_fks_to_objs(&mut self, config: &Config) {
        for pet in config.pets.iter() {
            let pet = Link::OBJ(Rc::clone(pet));
            self.pet = pet;
        }
    }
}
