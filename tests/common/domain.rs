use std::rc::Weak;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use serde_orm::{common::KeyLink, ser::serialize_data};
use serde_orm::{common::Linkable, de::deserialize_data};

use serde::{Deserialize, Serialize};

use serde_orm::common::{ForeignKey, Links};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub persons: Vec<Rc<RefCell<Person>>>,
    pub pets: Vec<Rc<RefCell<Dog>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RoommateConfig {
    pub persons: Vec<Rc<RefCell<Roomate>>>,
    pub pets: Vec<Rc<RefCell<Dog>>>,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: i32,
    #[serde(deserialize_with = "deserialize_data")]
    #[serde(serialize_with = "serialize_data")]
    pub pet: Rc<RefCell<Dog>>,
}
#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Roomate {
    pub id: i32,
    pub name: String,
    pub data: i32,
    #[serde(deserialize_with = "deserialize_data")]
    #[serde(serialize_with = "serialize_data")]
    pub pet: Weak<RefCell<Dog>>,
}

#[derive(Default, Debug, serde::Serialize, Deserialize)]
pub struct Dog {
    pub name: String,
}

// #[derive(Debug)]
// pub struct Parent {
//     pub name: String,
//     #[serde(deserialize_with = "deserialize_data")]
//     #[serde(serialize_with = "serialize_data")]
//     pub child: Rc<RefCell<Child>,
// }

// #[derive(Debug)]
// pub struct Child {
//     pub name: String,
//     #[serde(deserialize_with = "deserialize_data")]
//     #[serde(serialize_with = "serialize_data")]
//     pub parent: Rc<RefCell<Parent>,
// }

impl Links<Config> for Person {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let key = self.pet.get_key();
        vec![("pet".to_string(), key)]
    }

    fn convert_fks_to_objs(&mut self, config: &Config) {
        for pet in config.pets.iter() {
            self.pet = pet.clone();
        }
    }
}

impl Links<RoommateConfig> for Roomate {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let key = self.pet.get_key();
        vec![("pet".to_string(), key)]
    }

    fn convert_fks_to_objs(&mut self, config: &RoommateConfig) {
        for pet in config.pets.iter() {
            self.pet = Rc::downgrade(&pet);
        }
    }
}

// impl Links<Config> for Parent {
//     fn get_foreign_keys(&self) -> Vec<ForeignKey> {
//         let key = self.child.borrow().name;
//         vec![("pet".to_string(), key)]
//     }

//     fn convert_fks_to_objs(&mut self, config: &Config) {
//         for pet in config.pets.iter() {
//             let pet = Rc::clone(pet);
//             self.pet = pet;
//         }
//     }
// }

impl Linkable<String, Self> for Dog {
    fn get_fake(key: String) -> Self {
        Dog { name: key }
    }
}

impl KeyLink<String> for Dog {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}
