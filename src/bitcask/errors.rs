use std::error::Error;

#[derive(Debug)]
pub enum BitcaskError {
    InternalError(Box<dyn Error>),
    NonExistentKey(),
}

impl From<bincode::ErrorKind> for BitcaskError {
    fn from(err: bincode::ErrorKind) -> Self {
        BitcaskError::InternalError(Box::new(err))
    }
}

impl From<Box<bincode::ErrorKind>> for BitcaskError {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        BitcaskError::InternalError(err)
    }
}

impl From<std::io::Error> for BitcaskError {
    fn from(err: std::io::Error) -> Self {
        BitcaskError::InternalError(Box::new(err))
    }
}
