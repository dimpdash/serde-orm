use std::{
    cell::RefCell,
    rc::{Rc},
    vec,
};

use crate::{
    common::{Links},
    domain::{Dog, Person},
};

pub mod common;
pub mod de;
pub mod domain;
pub mod ser;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    persons: Vec<Rc<RefCell<Person>>>,
    pets: Vec<Rc<RefCell<Dog>>>,
}

fn main() -> Result<(), ()> {
    let pet = Rc::new(RefCell::new(Dog {
        name: "buddy".to_string(),
    }));

    let person = Person {
        id: 0,
        name: "dan".to_string(),
        data: 10,
        pet: Rc::clone(&pet),
    };

    let person2 = Person {
        id: 1,
        name: "matthew".to_string(),
        data: 10,
        pet: Rc::clone(&pet),
    };

    let config = Config {
        persons: vec![
            Rc::new(RefCell::new(person)),
            Rc::new(RefCell::new(person2)),
        ],
        pets: vec![pet],
    };

    let yaml = serde_yaml::to_string(&config).unwrap();

    println!("{}", &yaml);

    let wrapper: Config = serde_yaml::from_str(&yaml).unwrap();

    println!("{:?}", wrapper);

    for person in config.persons.iter() {
        person.borrow_mut().convert_fks_to_objs(&config);
    }

    config.pets[0].borrow_mut().name = "Joe".to_string();

    println!("{:?}", config);

    Ok(())
}
