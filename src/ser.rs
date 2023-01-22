use std::rc::Rc;

use serde::ser::Serializer;

use crate::{common::Linkable, domain::Dog};

pub fn serialize_data<S>(dog: &Dog, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let fk = dog.get_key();
    s.serialize_str(fk.as_str())
}
