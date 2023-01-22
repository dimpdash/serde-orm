use std::{
    cell::RefCell,
    collections::BTreeMap,
    default,
    rc::{Rc, Weak},
    vec,
};

use crate::{
    common::{Links, Wrapper},
    domain::{Dog, Link, Person},
};

pub mod common;
pub mod de;
pub mod domain;
pub mod ser;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    persons: Vec<Rc<RefCell<Wrapper<Config>>>>,
    pets: Vec<Rc<Dog>>,
}

fn main() -> Result<(), ()> {
    let pet = Rc::new(Dog {
        name: "buddy".to_string(),
    });

    let person = Person {
        id: 0,
        name: "dan".to_string(),
        data: 10,
        pet: Link::OBJ(Rc::clone(&pet)),
    };

    let wrapper = Wrapper {
        me: person,
        obj_list: vec![],
    };

    let mut config = Config {
        persons: vec![Rc::new(RefCell::new(wrapper))],
        pets: vec![pet],
    };

    let yaml = serde_yaml::to_string(&config).unwrap();

    println!("{}", &yaml);

    let wrapper: Config = serde_yaml::from_str(&yaml).unwrap();

    println!("{:?}", wrapper);

    for person in config.persons.iter() {
        person.borrow_mut().me.convert_fks_to_objs(&config);
    }

    println!("{:?}", config);

    Ok(())
}
