mod common;

use std::{cell::RefCell, rc::Rc};

use serde_orm::common::Links;

use self::common::domain::{Dog, Person};
#[cfg(test)]
mod simple {
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

        let mut linked: Vec<Rc<RefCell<dyn Links<Config>>>> = vec![];

        for p in &config.persons {
            linked.push(p.clone());
        }

        let yaml = serde_yaml::to_string(&config).unwrap();

        println!("{}", &yaml);

        let wrapper: Config = serde_yaml::from_str(&yaml).unwrap();

        println!("{:?}", wrapper);

        for obj_with_links in &linked {
            obj_with_links.borrow_mut().convert_fks_to_objs(&config);
        }

        config.pets[0].borrow_mut().name = "Joe".to_string();

        println!("{:?}", config);
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
