use crate::common::Linkable;
use core::fmt;
use serde::de::{self, Visitor};

pub fn deserialize_data<'de, D, F>(deserializer: D) -> Result<F, D::Error>
where
    D: de::Deserializer<'de>,
    F: Linkable<String, F>,
{
    struct KeyValueVisitor {}

    impl<'de> Visitor<'de> for KeyValueVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string containing describing key to an object")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(s.to_owned())
        }
    }

    let visitor = KeyValueVisitor {};

    let fk = deserializer.deserialize_str(visitor)?;
    let dog = F::get_fake(fk);
    Ok(dog)
}
