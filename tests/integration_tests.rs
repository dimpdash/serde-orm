mod common;

use std::{cell::RefCell, rc::Rc};

use self::common::domain::{Dog, Person};

#[cfg(test)]
mod simple {
    use std::{
        borrow::{Borrow, BorrowMut},
        cell::Ref,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        rc::Weak,
        vec,
    };

    use crate::common::domain::{Partner, PartnerConfig, Roomate, RoommateConfig};

    use self::common::domain::Config;
    use super::*;
    #[test]
    fn simple() {
        let pet = Rc::new(RefCell::new(Dog {
            name: "buddy".to_string(),
        }));

        let person = Person {
            id: 0,
            name: "dan".to_string(),
            data: 10,
            pet: pet.clone(),
        };
        let person2 = Person {
            id: 1,
            name: "matthew".to_string(),
            data: 10,
            pet: pet.clone(),
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

        let deserialised_config: Config = serde_yaml::from_str(&yaml).unwrap();

        deserialised_config.link_items();

        println!("{:?}", deserialised_config);
        println!("{:?}", config);

        assert_eq!(deserialised_config, config);
    }

    #[test]
    fn weak() {
        let pet = Rc::new(RefCell::new(Dog {
            name: "buddy".to_string(),
        }));

        let person = Roomate {
            id: 0,
            name: "dan".to_string(),
            data: 10,
            pet: Rc::downgrade(&pet),
        };

        let config = RoommateConfig {
            persons: vec![Rc::new(RefCell::new(person))],
            pets: vec![pet],
        };

        let yaml = serde_yaml::to_string(&config).unwrap();

        let deserialised_config: RoommateConfig = serde_yaml::from_str(&yaml).unwrap();

        deserialised_config.link_items();

        println!("{}", &yaml);

        // let roommate_pet = &deserialised_config.persons[0]
        //     .borrow_mut()
        //     .pet
        //     .upgrade()
        //     .unwrap();
        // let config_pet = &deserialised_config.pets[0];
        // assert_eq!(roommate_pet, config_pet);

        // println!("{:?}", roommate_pet);
        // println!("{:?}", config);
    }

    #[test]
    fn circular() {
        let p1 = Rc::new(RefCell::new(Partner {
            name: "Daniel".to_string(),
            partner: Weak::new(),
        }));

        let p2 = Rc::new(RefCell::new(Partner {
            name: "Jess".to_string(),
            partner: Weak::new(),
        }));

        p1.as_ref().borrow_mut().partner = Rc::downgrade(&p2);
        p2.as_ref().borrow_mut().partner = Rc::downgrade(&p1);

        let config = PartnerConfig {
            partners: vec![p1, p2],
        };

        let yaml = serde_yaml::to_string(&config).unwrap();

        let config_deserilized: PartnerConfig = serde_yaml::from_str(yaml.as_str()).unwrap();

        config_deserilized.link_items();

        let p1 = config_deserilized.partners[0].as_ref();

        let p2_name_1 = p1
            .borrow()
            .partner
            .upgrade()
            .unwrap()
            .as_ref()
            .borrow()
            .name
            .clone();
        let p2 = config_deserilized.partners[1].as_ref();
        let p2_name_2 = p2.borrow().name.clone();

        assert_eq!(p2_name_1, p2_name_2);
    }
}

// #[cfg(test)]
// mod circular {
//     use std::mem::MaybeUninit;

//     use crate::common::domain::Parent;

//     use super::*;
//     pub struct Config {
//         pub partners: Vec<Rc<RefCell<Parent>>>,
//     }

//     #[test]
//     fn circular_ref() {
//         let pet = Rc::new(RefCell::new(Dog {
//             name: "buddy".to_string(),
//         }));

//         let person: Parent;
//         let person2: Parent;

//         person = Parent {
//             name: "dan".to_string(),
//             child: Rc::new(RefCell::new(person2)),
//         };

//         person2 = Parent {
//             name: "matthew".to_string(),
//             child: Rc::new(RefCell::new(person)),
//         };

//         let config = Config {
//             partners: vec![
//                 Rc::new(RefCell::new(person)),
//                 Rc::new(RefCell::new(person2)),
//             ],
//         };

//         let mut linked: Vec<Rc<RefCell<dyn Links<Config>>>> = vec![];

//         for p in &config.partners {
//             linked.push(p.clone());
//         }

//         let yaml = serde_yaml::to_string(&config).unwrap();

//         println!("{}", &yaml);

//         let wrapper: Config = serde_yaml::from_str(&yaml).unwrap();

//         println!("{:?}", wrapper);

//         for obj_with_links in &linked {
//             obj_with_links.borrow_mut().convert_fks_to_objs(&config);
//         }

//         config.pets[0].borrow_mut().name = "Joe".to_string();

//         println!("{:?}", config);
//     }
// }
