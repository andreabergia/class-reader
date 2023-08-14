use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// Line number in the source code
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct LineNumber(pub u16);

impl Display for LineNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
