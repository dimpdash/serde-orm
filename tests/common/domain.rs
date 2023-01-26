use std::mem::MaybeUninit;
use std::rc::Weak;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use serde_orm::{common::KeyLink, ser::serialize_data};
use serde_orm::{common::Linkable, de::deserialize_data};

use serde::Deserialize;

use serde_orm::common::{ForeignKey, Links};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Config {
    pub persons: Vec<Rc<RefCell<Person>>>,
    pub pets: Vec<Rc<RefCell<Dog>>>,
}

impl Config {
    fn get_items_needing_links(&self) -> Vec<Rc<RefCell<dyn Links<Self>>>> {
        let mut linked: Vec<Rc<RefCell<dyn Links<Self>>>> = vec![];

        for p in &self.persons {
            linked.push(p.clone());
        }

        linked
    }

    pub fn link_items(&self) {
        let linked = self.get_items_needing_links();

        for obj_with_links in &linked {
            obj_with_links.borrow_mut().convert_fks_to_objs(&self);
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RoommateConfig {
    pub persons: Vec<Rc<RefCell<Roomate>>>,
    pub pets: Vec<Rc<RefCell<Dog>>>,
}

impl RoommateConfig {
    fn get_items_needing_links(&self) -> Vec<Rc<RefCell<dyn Links<Self>>>> {
        let mut linked: Vec<Rc<RefCell<dyn Links<Self>>>> = vec![];

        for p in &self.persons {
            linked.push(p.clone());
        }

        linked
    }

    pub fn link_items(&self) {
        let linked = self.get_items_needing_links();

        for obj_with_links in &linked {
            obj_with_links.borrow_mut().convert_fks_to_objs(&self);
        }
    }
}

#[derive(Debug, serde::Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Default, Debug, serde::Serialize, Deserialize, PartialEq, Eq)]
pub struct Dog {
    pub name: String,
}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
pub struct Partner {
    pub name: String,
    #[serde(deserialize_with = "deserialize_data")]
    #[serde(serialize_with = "serialize_data")]
    pub partner: Weak<RefCell<Partner>>,
}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
pub struct PartnerConfig {
    pub partners: Vec<Rc<RefCell<Partner>>>,
}

impl PartnerConfig {
    fn get_items_needing_links(&self) -> Vec<Rc<RefCell<dyn Links<Self>>>> {
        let mut linked: Vec<Rc<RefCell<dyn Links<Self>>>> = vec![];

        for p in &self.partners {
            linked.push(p.clone());
        }

        linked
    }

    pub fn link_items(&self) {
        let linked = self.get_items_needing_links();

        for obj_with_links in &linked {
            obj_with_links.borrow_mut().convert_fks_to_objs(&self);
        }
    }
}

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

impl Links<PartnerConfig> for Partner {
    fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let key = self.partner.upgrade().unwrap().borrow().name.clone();
        vec![("pet".to_string(), key)]
    }
    fn convert_fks_to_objs(&mut self, config: &PartnerConfig) {
        for pet in config.partners.iter() {
            self.partner = Rc::downgrade(&pet);
        }
    }
}

impl Linkable<String, Self> for Dog {
    fn get_fake(key: String) -> Self {
        Dog { name: key }
    }
}

impl Linkable<String, Self> for Partner {
    fn get_fake(key: String) -> Self {
        let partner = unsafe {
            let partner: MaybeUninit<Weak<RefCell<Partner>>> = MaybeUninit::uninit();
            Partner {
                name: key,
                partner: partner.assume_init(),
            }
        };
        partner
    }
}

impl KeyLink<String> for Dog {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

impl KeyLink<String> for Partner {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}
