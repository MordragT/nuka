use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Id([u8; 32]);

impl From<[u8; 32]> for Id {
    fn from(hash: [u8; 32]) -> Self {
        Id(hash)
    }
}

impl From<Id> for Vec<u8> {
    fn from(id: Id) -> Self {
        id.0.to_vec()
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = Vec<u8>;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        let hash = <[u8; 32]>::try_from(vec)?;
        Ok(Id(hash))
    }
}
