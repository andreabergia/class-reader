use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Models the program counter, i.e. the address of an instruction in the bytecode of a method
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub struct ProgramCounter(pub u16);

impl Display for ProgramCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
