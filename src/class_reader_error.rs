use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::{buffer::BufferError, constant_pool::InvalidConstantPoolIndexError};

/// Models the possible errors returned when reading a .class file
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub enum ClassReaderError {
    /// Generic error meaning that the class file is invalid
    InvalidClassData(String, Option<InvalidConstantPoolIndexError>),
    UnsupportedVersion(u16, u16),
    /// Error while parsing a given type descriptor in the file
    InvalidTypeDescriptor(String),
}

impl ClassReaderError {
    pub fn invalid_class_data(message: String) -> Self {
        ClassReaderError::InvalidClassData(message, None)
    }
}

impl Display for ClassReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassReaderError::InvalidClassData(details, _) => {
                write!(f, "invalid class file: {details}")
            }
            ClassReaderError::UnsupportedVersion(major, minor) => {
                write!(f, "unsupported class file version {major}.{minor}")
            }
            ClassReaderError::InvalidTypeDescriptor(descriptor) => {
                write!(f, "invalid type descriptor: {descriptor}")
            }
        }
    }
}

impl Error for ClassReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClassReaderError::InvalidClassData(_, Some(source)) => Some(source),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, ClassReaderError>;

impl From<InvalidConstantPoolIndexError> for ClassReaderError {
    fn from(err: InvalidConstantPoolIndexError) -> Self {
        Self::InvalidClassData(err.to_string(), Some(err))
    }
}

impl From<BufferError> for ClassReaderError {
    fn from(err: BufferError) -> Self {
        match err {
            BufferError::UnexpectedEndOfData => {
                Self::invalid_class_data("unexpected end of class file".to_string())
            }
            BufferError::InvalidCesu8String => {
                Self::invalid_class_data("invalid cesu8 string".to_string())
            }
        }
    }
}
