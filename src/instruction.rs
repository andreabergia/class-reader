use crate::class_reader_error::ClassReaderError;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Represents a Java bytecode instruction.
//noinspection SpellCheckingInspection
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[cfg_attr(feature = "wasm", serde(tag = "opcode"))]
pub enum Instruction {
    Aaload,
    Aastore,
    Aconst_null,
    Aload { index: u8 },
    Aload_0,
    Aload_1,
    Aload_2,
    Aload_3,
    Anewarray { class: u16 },
    Areturn,
    Arraylength,
    Astore { index: u8 },
    Astore_0,
    Astore_1,
    Astore_2,
    Astore_3,
    Athrow,
    Baload,
    Bastore,
    Bipush { byte: u8 },
    Caload,
    Castore,
    Checkcast { class: u16 },
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst_0,
    Dconst_1,
    Ddiv,
    Dload { index: u8 },
    Dload_0,
    Dload_1,
    Dload_2,
    Dload_3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore { index: u8 },
    Dstore_0,
    Dstore_1,
    Dstore_2,
    Dstore_3,
    Dsub,
    Dup,
    Dup_x1,
    Dup_x2,
    Dup2,
    Dup2_x1,
    Dup2_x2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst_0,
    Fconst_1,
    Fconst_2,
    Fdiv,
    Fload { index: u8 },
    Fload_0,
    Fload_1,
    Fload_2,
    Fload_3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore { index: u8 },
    Fstore_0,
    Fstore_1,
    Fstore_2,
    Fstore_3,
    Fsub,
    Getfield { field: u16 },
    Getstatic { field: u16 },
    Goto { jump_address: u16 },
    Goto_w,
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    Iconst_m1,
    Iconst_0,
    Iconst_1,
    Iconst_2,
    Iconst_3,
    Iconst_4,
    Iconst_5,
    Idiv,
    If_acmpeq { jump_address: u16 },
    If_acmpne { jump_address: u16 },
    If_icmpeq { jump_address: u16 },
    If_icmpne { jump_address: u16 },
    If_icmplt { jump_address: u16 },
    If_icmpge { jump_address: u16 },
    If_icmpgt { jump_address: u16 },
    If_icmple { jump_address: u16 },
    Ifeq { jump_address: u16 },
    Ifne { jump_address: u16 },
    Iflt { jump_address: u16 },
    Ifge { jump_address: u16 },
    Ifgt { jump_address: u16 },
    Ifle { jump_address: u16 },
    Ifnonnull { jump_address: u16 },
    Ifnull { jump_address: u16 },
    Iinc { index: u8, constant: i8 },
    Iload { index: u8 },
    Iload_0,
    Iload_1,
    Iload_2,
    Iload_3,
    Imul,
    Ineg,
    Instanceof { class: u16 },
    Invokedynamic { call_site: u16 },
    Invokeinterface { method: u16, count: u8 },
    Invokespecial { method: u16 },
    Invokestatic { method: u16 },
    Invokevirtual { method: u16 },
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore { index: u8 },
    Istore_0,
    Istore_1,
    Istore_2,
    Istore_3,
    Isub,
    Iushr,
    Ixor,
    Jsr { jump_address: u16 },
    Jsr_w,
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst_0,
    Lconst_1,
    Ldc { index: u8 },
    Ldc_w { index: u16 },
    Ldc2_w { index: u16 },
    Ldiv,
    Lload { index: u8 },
    Lload_0,
    Lload_1,
    Lload_2,
    Lload_3,
    Lmul,
    Lneg,
    Lookupswitch,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore { index: u8 },
    Lstore_0,
    Lstore_1,
    Lstore_2,
    Lstore_3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray { class: u16, dimensions: u8 },
    New { class: u16 },
    Newarray { array_type: NewArrayType },
    Nop,
    Pop,
    Pop2,
    Putfield { field: u16 },
    Putstatic { field: u16 },
    Ret { index: u8 },
    Return,
    Saload,
    Sastore,
    Sipush { short: i16 },
    Swap,
    Tableswitch,
    Wide,
}

/// Possible arguments of instruction `newarray`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub enum NewArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

impl Instruction {
    /// Reads one instruction from the bytecode, and returns it along
    /// with the address of the start of the next instruction
    pub fn parse(raw_code: &[u8], address: usize) -> Result<(Self, usize), ClassReaderError> {
        let op_byte = Self::byte_at(raw_code, address)?;
        let mut address = address + 1;
        let op_code = match op_byte {
            0x32 => Instruction::Aaload,
            0x53 => Instruction::Aastore,
            0x01 => Instruction::Aconst_null,
            0x19 => Instruction::Aload {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x2a => Instruction::Aload_0,
            0x2b => Instruction::Aload_1,
            0x2c => Instruction::Aload_2,
            0x2d => Instruction::Aload_3,
            0xbd => Instruction::Anewarray {
                class: Self::read_u16(raw_code, &mut address)?,
            },
            0xb0 => Instruction::Areturn,
            0xbe => Instruction::Arraylength,
            0x3a => Instruction::Astore {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x4b => Instruction::Astore_0,
            0x4c => Instruction::Astore_1,
            0x4d => Instruction::Astore_2,
            0x4e => Instruction::Astore_3,
            0xbf => Instruction::Athrow,
            0x33 => Instruction::Baload,
            0x54 => Instruction::Bastore,
            0x10 => Instruction::Bipush {
                byte: Self::read_u8(raw_code, &mut address)?,
            },
            0x34 => Instruction::Caload,
            0x55 => Instruction::Castore,
            0xc0 => Instruction::Checkcast {
                class: Self::read_u16(raw_code, &mut address)?,
            },
            0x90 => Instruction::D2f,
            0x8e => Instruction::D2i,
            0x8f => Instruction::D2l,
            0x63 => Instruction::Dadd,
            0x31 => Instruction::Daload,
            0x52 => Instruction::Dastore,
            0x98 => Instruction::Dcmpg,
            0x97 => Instruction::Dcmpl,
            0x0e => Instruction::Dconst_0,
            0x0f => Instruction::Dconst_1,
            0x6f => Instruction::Ddiv,
            0x18 => Instruction::Dload {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x26 => Instruction::Dload_0,
            0x27 => Instruction::Dload_1,
            0x28 => Instruction::Dload_2,
            0x29 => Instruction::Dload_3,
            0x6b => Instruction::Dmul,
            0x77 => Instruction::Dneg,
            0x73 => Instruction::Drem,
            0xaf => Instruction::Dreturn,
            0x39 => Instruction::Dstore {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x47 => Instruction::Dstore_0,
            0x48 => Instruction::Dstore_1,
            0x49 => Instruction::Dstore_2,
            0x4a => Instruction::Dstore_3,
            0x67 => Instruction::Dsub,
            0x59 => Instruction::Dup,
            0x5a => Instruction::Dup_x1,
            0x5b => Instruction::Dup_x2,
            0x5c => Instruction::Dup2,
            0x5d => Instruction::Dup2_x1,
            0x5e => Instruction::Dup2_x2,
            0x8d => Instruction::F2d,
            0x8b => Instruction::F2i,
            0x8c => Instruction::F2l,
            0x62 => Instruction::Fadd,
            0x30 => Instruction::Faload,
            0x51 => Instruction::Fastore,
            0x96 => Instruction::Fcmpg,
            0x95 => Instruction::Fcmpl,
            0x0b => Instruction::Fconst_0,
            0x0c => Instruction::Fconst_1,
            0x0d => Instruction::Fconst_2,
            0x6e => Instruction::Fdiv,
            0x17 => Instruction::Fload {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x22 => Instruction::Fload_0,
            0x23 => Instruction::Fload_1,
            0x24 => Instruction::Fload_2,
            0x25 => Instruction::Fload_3,
            0x6a => Instruction::Fmul,
            0x76 => Instruction::Fneg,
            0x72 => Instruction::Frem,
            0xae => Instruction::Freturn,
            0x38 => Instruction::Fstore {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x43 => Instruction::Fstore_0,
            0x44 => Instruction::Fstore_1,
            0x45 => Instruction::Fstore_2,
            0x46 => Instruction::Fstore_3,
            0x66 => Instruction::Fsub,
            0xb4 => Instruction::Getfield {
                field: Self::read_u16(raw_code, &mut address)?,
            },
            0xb2 => Instruction::Getstatic {
                field: Self::read_u16(raw_code, &mut address)?,
            },
            0xa7 => Instruction::Goto {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xc8 => todo!("OpCode::Goto_w"),
            0x91 => Instruction::I2b,
            0x92 => Instruction::I2c,
            0x87 => Instruction::I2d,
            0x86 => Instruction::I2f,
            0x85 => Instruction::I2l,
            0x93 => Instruction::I2s,
            0x60 => Instruction::Iadd,
            0x2e => Instruction::Iaload,
            0x7e => Instruction::Iand,
            0x4f => Instruction::Iastore,
            0x02 => Instruction::Iconst_m1,
            0x03 => Instruction::Iconst_0,
            0x04 => Instruction::Iconst_1,
            0x05 => Instruction::Iconst_2,
            0x06 => Instruction::Iconst_3,
            0x07 => Instruction::Iconst_4,
            0x08 => Instruction::Iconst_5,
            0x6c => Instruction::Idiv,
            0xa5 => Instruction::If_acmpeq {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa6 => Instruction::If_acmpne {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9f => Instruction::If_icmpeq {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa0 => Instruction::If_icmpne {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa1 => Instruction::If_icmplt {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa2 => Instruction::If_icmpge {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa3 => Instruction::If_icmpgt {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xa4 => Instruction::If_icmple {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x99 => Instruction::Ifeq {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9a => Instruction::Ifne {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9b => Instruction::Iflt {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9c => Instruction::Ifge {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9d => Instruction::Ifgt {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x9e => Instruction::Ifle {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xc7 => Instruction::Ifnonnull {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xc6 => Instruction::Ifnull {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0x84 => Instruction::Iinc {
                index: Self::read_u8(raw_code, &mut address)?,
                constant: Self::read_i8(raw_code, &mut address)?,
            },
            0x15 => Instruction::Iload {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x1a => Instruction::Iload_0,
            0x1b => Instruction::Iload_1,
            0x1c => Instruction::Iload_2,
            0x1d => Instruction::Iload_3,
            0x68 => Instruction::Imul,
            0x74 => Instruction::Ineg,
            0xc1 => Instruction::Instanceof {
                class: Self::read_u16(raw_code, &mut address)?,
            },
            0xba => {
                let constant_index = Self::read_u16(raw_code, &mut address)?;
                if Self::read_u16(raw_code, &mut address)? != 0 {
                    return Err(ClassReaderError::invalid_class_data(
                        format!("expected two zero bytes after invokedynamic and the index at address {address}")
                    ));
                }
                Instruction::Invokedynamic {
                    call_site: constant_index,
                }
            }
            0xb9 => {
                let constant_index = Self::read_u16(raw_code, &mut address)?;
                let count = Self::read_u8(raw_code, &mut address)?;
                if Self::read_u8(raw_code, &mut address)? != 0 {
                    return Err(ClassReaderError::invalid_class_data(
                        format!("expected a zero byte after invokeinterface and the index at address {address}"),
                    ));
                }
                Instruction::Invokeinterface {
                    method: constant_index,
                    count,
                }
            }
            0xb7 => Instruction::Invokespecial {
                method: Self::read_u16(raw_code, &mut address)?,
            },
            0xb8 => Instruction::Invokestatic {
                method: Self::read_u16(raw_code, &mut address)?,
            },
            0xb6 => Instruction::Invokevirtual {
                method: Self::read_u16(raw_code, &mut address)?,
            },
            0x80 => Instruction::Ior,
            0x70 => Instruction::Irem,
            0xac => Instruction::Ireturn,
            0x78 => Instruction::Ishl,
            0x7a => Instruction::Ishr,
            0x36 => Instruction::Istore {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x3b => Instruction::Istore_0,
            0x3c => Instruction::Istore_1,
            0x3d => Instruction::Istore_2,
            0x3e => Instruction::Istore_3,
            0x64 => Instruction::Isub,
            0x7c => Instruction::Iushr,
            0x82 => Instruction::Ixor,
            0xa8 => Instruction::Jsr {
                jump_address: Self::read_offset(raw_code, &mut address)?,
            },
            0xc9 => todo!("OpCode::Jsr_w"),
            0x8a => Instruction::L2d,
            0x89 => Instruction::L2f,
            0x88 => Instruction::L2i,
            0x61 => Instruction::Ladd,
            0x2f => Instruction::Laload,
            0x7f => Instruction::Land,
            0x50 => Instruction::Lastore,
            0x94 => Instruction::Lcmp,
            0x09 => Instruction::Lconst_0,
            0x0a => Instruction::Lconst_1,
            0x12 => Instruction::Ldc {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x13 => Instruction::Ldc_w {
                index: Self::read_u16(raw_code, &mut address)?,
            },
            0x14 => Instruction::Ldc2_w {
                index: Self::read_u16(raw_code, &mut address)?,
            },
            0x6d => Instruction::Ldiv,
            0x16 => Instruction::Lload {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x1e => Instruction::Lload_0,
            0x1f => Instruction::Lload_1,
            0x20 => Instruction::Lload_2,
            0x21 => Instruction::Lload_3,
            0x69 => Instruction::Lmul,
            0x75 => Instruction::Lneg,
            0xab => todo!("OpCode::Lookupswitch"),
            0x81 => Instruction::Lor,
            0x71 => Instruction::Lrem,
            0xad => Instruction::Lreturn,
            0x79 => Instruction::Lshl,
            0x7b => Instruction::Lshr,
            0x37 => Instruction::Lstore {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0x3f => Instruction::Lstore_0,
            0x40 => Instruction::Lstore_1,
            0x41 => Instruction::Lstore_2,
            0x42 => Instruction::Lstore_3,
            0x65 => Instruction::Lsub,
            0x7d => Instruction::Lushr,
            0x83 => Instruction::Lxor,
            0xc2 => Instruction::Monitorenter,
            0xc3 => Instruction::Monitorexit,
            0xc5 => Instruction::Multianewarray {
                class: Self::read_u16(raw_code, &mut address)?,
                dimensions: Self::read_u8(raw_code, &mut address)?,
            },
            0xbb => Instruction::New {
                class: Self::read_u16(raw_code, &mut address)?,
            },
            0xbc => {
                let array_type_byte = Self::read_u8(raw_code, &mut address)?;
                let array_type = match array_type_byte {
                    4 => NewArrayType::Boolean,
                    5 => NewArrayType::Char,
                    6 => NewArrayType::Float,
                    7 => NewArrayType::Double,
                    8 => NewArrayType::Byte,
                    9 => NewArrayType::Short,
                    10 => NewArrayType::Int,
                    11 => NewArrayType::Long,
                    _ => {
                        return Err(ClassReaderError::invalid_class_data(format!(
                            "invalid type for newarray: {array_type_byte:#04x} at address {address}"
                        )))
                    }
                };
                Instruction::Newarray { array_type }
            }
            0x00 => Instruction::Nop,
            0x57 => Instruction::Pop,
            0x58 => Instruction::Pop2,
            0xb5 => Instruction::Putfield {
                field: Self::read_u16(raw_code, &mut address)?,
            },
            0xb3 => Instruction::Putstatic {
                field: Self::read_u16(raw_code, &mut address)?,
            },
            0xa9 => Instruction::Ret {
                index: Self::read_u8(raw_code, &mut address)?,
            },
            0xb1 => Instruction::Return,
            0x35 => Instruction::Saload,
            0x56 => Instruction::Sastore,
            0x11 => Instruction::Sipush {
                short: Self::read_i16(raw_code, &mut address)?,
            },
            0x5f => Instruction::Swap,
            0xaa => todo!("OpCode::Tableswitch"),
            0xc4 => todo!("OpCode::Wide"),
            _ => {
                return Err(ClassReaderError::invalid_class_data(format!(
                    "invalid op code: {op_byte:#04x} at address {address}"
                )))
            }
        };

        Ok((op_code, address))
    }

    /// Parses all instructions in the given raw code.
    pub(crate) fn parse_instructions(
        raw_code: &[u8],
    ) -> Result<Vec<(usize, Instruction)>, ClassReaderError> {
        let mut instructions: Vec<(usize, Self)> = Vec::new();

        let mut index = 0;
        while index < raw_code.len() {
            let (op_code, new_index) = Self::parse(raw_code, index)?;
            instructions.push((index, op_code));
            index = new_index;
        }

        Ok(instructions)
    }

    fn byte_at(raw_code: &[u8], address: usize) -> Result<u8, ClassReaderError> {
        let op_byte = *raw_code
            .get(address)
            .ok_or(ClassReaderError::invalid_class_data(format!(
                "cannot read instruction at address {address}"
            )))?;
        Ok(op_byte)
    }

    fn read_u8(raw_code: &[u8], address: &mut usize) -> Result<u8, ClassReaderError> {
        let byte = Self::byte_at(raw_code, *address).map_err(|_| {
            ClassReaderError::invalid_class_data(format!(
                "cannot find arguments for instruction at address {address}"
            ))
        })?;
        *address += 1;
        Ok(byte)
    }

    fn read_i8(raw_code: &[u8], address: &mut usize) -> Result<i8, ClassReaderError> {
        let value = Self::read_u8(raw_code, address)?;
        Ok(unsafe { std::mem::transmute(value) })
    }

    fn read_u16(raw_code: &[u8], address: &mut usize) -> Result<u16, ClassReaderError> {
        let b1 = Self::read_u8(raw_code, address)? as u16;
        let b2 = Self::read_u8(raw_code, address)? as u16;
        Ok((b1 << 8) | b2)
    }

    fn read_i16(raw_code: &[u8], address: &mut usize) -> Result<i16, ClassReaderError> {
        let value = Self::read_u16(raw_code, address)?;
        Ok(unsafe { std::mem::transmute(value) })
    }

    fn read_offset(raw_code: &[u8], address: &mut usize) -> Result<u16, ClassReaderError> {
        let instruction_address = *address - 1;
        let offset = Self::read_i16(raw_code, address)?;
        let jump_address = (instruction_address as i32) + (offset as i32);
        u16::try_from(jump_address).map_err(|_| {
            ClassReaderError::invalid_class_data(format!(
                "invalid jump offset at address {address}"
            ))
        })
    }
}
