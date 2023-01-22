

use serde::ser::Serializer;

use crate::common::{KeyLink};

pub fn serialize_data<S, F>(dog: &F, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    F: KeyLink<String>,
{
    let fk = dog.get_key();
    s.serialize_str(fk.as_str())
}
