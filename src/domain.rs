use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::{common::KeyLink, ser::serialize_data};
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
    pub pet: Rc<RefCell<Dog>>,
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
    fn get_fake(key: String) -> Self {
        Dog { name: key }
    }
}

impl KeyLink<String> for Dog {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

impl<T, K> KeyLink<K> for Rc<T>
where
    T: KeyLink<K>,
{
    fn get_key(&self) -> K {
        self.as_ref().get_key()
    }
}

impl<T, K> KeyLink<K> for RefCell<T>
where
    T: KeyLink<K>,
{
    fn get_key(&self) -> K {
        self.borrow().get_key()
    }
}

impl<T, K, O> Linkable<K, RefCell<O>> for RefCell<T>
where
    T: Linkable<K, O>,
{
    fn get_fake(key: String) -> RefCell<O> {
        RefCell::new(T::get_fake(key))
    }
}

impl<T, K, O> Linkable<K, Rc<O>> for Rc<T>
where
    T: Linkable<K, O>,
{
    fn get_fake(key: String) -> Rc<O> {
        Rc::new(T::get_fake(key))
    }
}
