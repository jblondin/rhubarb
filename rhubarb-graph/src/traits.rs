use serde::ser::{Serialize, Serializer};

use agnes::SerializeAsVec;

#[derive(Debug, Clone)]
pub enum SingleOrMore<T: Serialize, D: SerializeAsVec> {
    Single(T),
    More(D)
}

impl<T: Serialize, D: SerializeAsVec> Serialize for SingleOrMore<T, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            SingleOrMore::Single(ref s) => {
                s.serialize(serializer)
            },
            SingleOrMore::More(ref vec) => {
                vec.serialize(serializer)
            }
        }
    }
}

impl<T: Serialize, D: SerializeAsVec> From<T> for SingleOrMore<T, D> {
    fn from(value: T) -> SingleOrMore<T, D> {
        SingleOrMore::Single(value)
    }
}


pub trait CountExistFields {
    fn count_existing_fields(&self) -> usize;
}
