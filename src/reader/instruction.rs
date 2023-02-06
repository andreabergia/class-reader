use std::fmt;
use std::fmt::Formatter;

use crate::reader::class_reader_error::ClassReaderError;
use crate::reader::class_reader_error::ClassReaderError::UnsupportedInstruction;
use crate::reader::opcodes::InstructionLength::Fixed;
use crate::reader::opcodes::{InstructionLength, OpCode};
use crate::utils::buffer::Buffer;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    op_code: OpCode,
    arguments: Vec<u8>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.arguments.is_empty() {
            write!(f, "{}", self.op_code)
        } else {
            write!(f, "{} {:?}", self.op_code, self.arguments)
        }
    }
}

impl Instruction {
    pub fn parse_instructions(raw_code: &[u8]) -> Result<Vec<Self>, ClassReaderError> {
        let mut reader = Buffer::new(raw_code);
        let mut instructions: Vec<Instruction> = Vec::new();

        while reader.has_more_data() {
            let op_byte = reader.read_u8()?;
            let op_code = OpCode::try_from(op_byte).map_err(|_| {
                ClassReaderError::InvalidClassData(format!("invalid op code: {:#04x}", op_byte))
            })?;
            let arguments = match op_code.instruction_length() {
                Fixed(arguments_len) => reader.read_bytes(arguments_len).map_err(|_| {
                    ClassReaderError::InvalidClassData(format!(
                        "cannot find arguments for instruction {:#04x}",
                        op_code as u8
                    ))
                }),
                InstructionLength::Variable => Err(UnsupportedInstruction(op_code)),
            }?;

            let instruction = Instruction {
                op_code,
                arguments: Vec::from(arguments),
            };
            instructions.push(instruction);
        }

        Ok(instructions)
    }
}