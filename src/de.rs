use crate::{
    common::Wrapper,
    domain::{Dog, Person},
};
use core::fmt;
use serde::{
    de::{self, DeserializeSeed, Deserializer, Expected, MapAccess, Visitor},
    Deserialize,
};
use std::collections::{BTreeMap, HashMap};

enum Field {
    Type,
    Pet,
    Id,
    Name,
    Data,
}

const FIELDS: &'static [&'static str] = &["id", "name", "data", "type", "pet"];

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`secs` or `nanos`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Field, E>
            where
                E: de::Error,
            {
                println!("{}", value);
                match value {
                    "type" => Ok(Field::Type),
                    "pet" => Ok(Field::Pet),
                    "id" => Ok(Field::Id),
                    "name" => Ok(Field::Name),
                    "data" => Ok(Field::Data),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> Deserialize<'de> for Wrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = PersonVisitor {};

        let person = deserializer.deserialize_map(visitor).unwrap();
        Ok(Wrapper { me: person })
    }
}

struct PersonVisitor {}

// This is the trait that Deserializers are going to be driving. There
// is one method for each type of data that our type knows how to
// deserialize from. There are many other methods that are not
// implemented here, for example deserializing from integers or strings.
// By default those methods will return an error, which makes sense
// because we cannot deserialize a MyMap from an integer or string.
impl<'de> Visitor<'de> for PersonVisitor {
    // The type that our Visitor is going to produce.
    type Value = Person;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        println!("HERE");
        let mut id = None;
        let mut name = None;
        let mut data = None;
        let mut pet = Dog {
            name: "me".to_string(),
        };
        while let Some(key) = access.next_key()? {
            match key {
                Field::Id => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    id = Some(access.next_value()?);
                }
                Field::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(access.next_value()?);
                }
                Field::Data => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("data"));
                    }
                    data = Some(access.next_value()?);
                }
                Field::Pet => {
                    println!("HERE3");
                    let dog_name: String = access.next_value()?;
                    pet = Dog { name: dog_name }
                    //do nothing
                }
                Field::Type => {
                    println!("HERE4");
                    //do nothing
                    let _: String = access.next_value()?;
                }
            }
        }
        println!("HERE2");
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
        let person = Person {
            id,
            data,
            name,
            pet,
        };

        Ok(person)
    }
}

// struct TaggedVisitor<T: ?Sized + 'static> {
//     trait_object: &'static str,
//     registry: &'static Registry<T>,
// }

// impl<T> TaggedVisitor<T> {

// }

// // impl<'de, T: ?Sized> Visitor<'de> for TaggedVisitor<T> {
// //     type Value = Box<T>;

// //     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
// //         write!(formatter, "dyn {}", self.trait_object)
// //     }

// //     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
// //     where
// //         A: MapAccess<'de>,
// //     {
// //         map.
// //         self.from_new<A>(map)
// //     }
// // }

// pub struct Registry<T: ?Sized> {
//     pub map: BTreeMap<&'static str, Option<DeserializeFn<T>>>,
//     pub names: Vec<&'static str>,
// }

// pub type DeserializeFn<T> = fn(&mut dyn erased_serde::Deserializer) -> erased_serde::Result<Box<T>>;

// pub struct MapLookupVisitor<'a, T: ?Sized + 'static> {
//     pub expected: &'a dyn Expected,
//     pub registry: &'static Registry<T>,
// }

// impl<'de, 'a, T: ?Sized + 'static> Visitor<'de> for MapLookupVisitor<'a, T> {
//     type Value = DeserializeFn<T>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         Expected::fmt(self.expected, formatter)
//     }

//     fn visit_str<E>(self, key: &str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         match self.registry.map.get(key) {
//             Some(Some(value)) => Ok(*value),
//             Some(None) => Err(de::Error::custom(format_args!(
//                 "non-unique tag of {}: {:?}",
//                 self.expected, key
//             ))),
//             None => Err(de::Error::unknown_variant(key, &self.registry.names)),
//         }
//     }
// }

// impl<'de, 'a, T: ?Sized + 'static> DeserializeSeed<'de> for MapLookupVisitor<'a, T> {
//     type Value = DeserializeFn<T>;

//     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_str(self)
//     }
// }

// pub struct FnApply<T: ?Sized> {
//     pub deserialize_fn: DeserializeFn<T>,
// }

// impl<'de, T: ?Sized> DeserializeSeed<'de> for FnApply<T> {
//     type Value = Box<T>;

//     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
//         (self.deserialize_fn)(&mut erased).map_err(de::Error::custom)
//     }
// }
