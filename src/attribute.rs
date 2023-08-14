use std::{fmt, fmt::Formatter};

/// An attribute in the class file, which can belong to a class, field, method, or code block.
#[derive(Debug, Default, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub bytes: Vec<u8>,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (data = {} bytes)", self.name, self.bytes.len())
    }
}
