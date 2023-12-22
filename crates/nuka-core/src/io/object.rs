//! An object is a blob or path inside an entry of the store

use cap_async_std::fs::{Dir, File};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ObjectKind {
    Blob,
    Path,
}

pub enum Object {
    Blob(File),
    Path(Dir),
}

impl Object {
    pub fn kind(&self) -> ObjectKind {
        match &self {
            Self::Blob(_) => ObjectKind::Blob,
            Self::Path(_) => ObjectKind::Path,
        }
    }

    pub fn as_blob(&self) -> Option<&File> {
        match &self {
            Self::Blob(file) => Some(file),
            _ => None,
        }
    }

    pub fn is_blob(&self) -> bool {
        self.kind() == ObjectKind::Blob
    }

    pub fn into_blob(self) -> Option<File> {
        match self {
            Self::Blob(file) => Some(file),
            _ => None,
        }
    }

    pub fn as_path(&self) -> Option<&Dir> {
        match &self {
            Self::Path(dir) => Some(dir),
            _ => None,
        }
    }

    pub fn is_path(&self) -> bool {
        self.kind() == ObjectKind::Path
    }

    pub fn into_path(self) -> Option<Dir> {
        match self {
            Self::Path(dir) => Some(dir),
            _ => None,
        }
    }
}

impl From<File> for Object {
    fn from(value: File) -> Self {
        Self::Blob(value)
    }
}

impl From<Dir> for Object {
    fn from(value: Dir) -> Self {
        Self::Path(value)
    }
}
