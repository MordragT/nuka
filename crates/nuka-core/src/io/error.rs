use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoError {}

pub type IoResult<T> = Result<T, IoError>;
