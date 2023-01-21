use std::collections::BTreeMap;

use crate::{
    common::Wrapper,
    domain::{Dog, Person},
};

pub mod common;
pub mod de;
pub mod domain;
pub mod ser;

fn main() -> Result<(), ()> {
    let person = Person {
        id: 0,
        name: "dan".to_string(),
        data: 10,
        pet: Dog {
            name: "buddy".to_string(),
        },
    };

    let wrapper = Wrapper { me: person };
    let yaml = serde_yaml::to_string(&wrapper).unwrap();

    println!("{}", &yaml);

    let wrapper: Wrapper = serde_yaml::from_str(&yaml).unwrap();

    println!("{:?}", wrapper);

    Ok(())
}
