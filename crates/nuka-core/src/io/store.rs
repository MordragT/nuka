// A store of packages, should also be able to be served by a server so that building and executing packages is handled by remote servers
// for use cases like collaboration similar to https://flox.dev/

use std::path::Path;

use cap_async_std::{async_std::stream::Stream, fs::Dir};

use super::{entry::Entry, error::IoResult, id::Id};

pub struct Store {
    dir: Dir,
}

impl Store {
    pub async fn init<P: AsRef<Path>>(path: P) -> Self {
        todo!()
    }

    pub async fn open<P: AsRef<Path>>(path: P) -> Self {
        todo!()
    }

    pub async fn get(&self, id: &Id) -> Option<&Entry> {
        todo!()
    }

    pub async fn import(&mut self, path: impl AsRef<Path>) -> IoResult<Id> {
        todo!()
    }

    // pub async fn import_file(&mut self, path: impl AsRef<Path>) -> IoResult<Id> {
    //     todo!()
    // }

    // pub async fn import_dir(&mut self, path: impl AsRef<Path>) -> IoResult<Id> {
    //     todo!()
    // }

    pub async fn import_bytes(&mut self, bytes: &[u8]) -> IoResult<Id> {
        todo!()
    }

    pub async fn import_stream(&mut self, stream: impl Stream<Item = u8>) -> IoResult<Id> {
        todo!()
    }
}
