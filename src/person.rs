use std::fmt::Display;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Person", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.end()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = serde_json::to_string_pretty(&self);
        match serialized {
            Ok(serialized_person) => write!(f, "{}", &serialized_person),
            Err(_e) => write!(
                f,
                "id: {}, name: {}, age: {}",
                &self.id, &self.name, &self.age
            ),
        }
    }
}