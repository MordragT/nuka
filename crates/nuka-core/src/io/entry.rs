//! An entry is a directory containing a collection of objects inside the store

use cap_async_std::fs::{Dir, DirEntry};

// pub enum EntryKind {
//     Package,
//     Source,
//     Derivation,
//     Runner,
// }

pub struct Entry {
    entry: DirEntry,
}

// impl Entry {
//     pub async fn kind(&self) -> EntryKind {
//         todo!()
//     }
// }
