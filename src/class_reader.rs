use log::warn;
use result::prelude::*;

use crate::{
    attribute::Attribute,
    buffer::Buffer,
    class_access_flags::ClassAccessFlags,
    class_file::ClassFile,
    class_file_field::{ClassFileField, FieldConstantValue},
    class_file_method::{ClassFileMethod, ClassFileMethodCode},
    class_file_version::ClassFileVersion,
    class_reader_error::{ClassReaderError, Result},
    constant_pool::{ConstantPool, ConstantPoolEntry},
    exception_table::{ExceptionTable, ExceptionTableEntry},
    field_flags::FieldFlags,
    field_type::FieldType,
    line_number::LineNumber,
    line_number_table::{LineNumberTable, LineNumberTableEntry},
    method_descriptor::MethodDescriptor,
    method_flags::MethodFlags,
    program_counter::ProgramCounter,
    type_conversion::ToUsizeSafe,
};

/// A reader of a byte array representing a class. Supports only a subset of Java 7 class format,
/// in particular it does not support generics.
struct ClassFileReader<'a> {
    buffer: Buffer<'a>,
    /// The class being read, created empty and updated in place
    class_file: ClassFile,
}

/// Reference: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
impl<'a> ClassFileReader<'a> {
    fn new(data: &[u8]) -> ClassFileReader {
        ClassFileReader {
            buffer: Buffer::new(data),
            class_file: Default::default(),
        }
    }

    fn read(mut self) -> Result<ClassFile> {
        self.check_magic_number()?;
        self.read_version()?;
        self.read_constants()?;
        self.read_access_flags()?;
        self.class_file.name = self.read_class_reference()?;
        self.class_file.superclass = self.read_class_reference_optional()?;
        self.read_interfaces()?;
        self.read_fields()?;
        self.read_methods()?;
        self.read_class_attributes()?;

        Ok(self.class_file)
    }

    fn check_magic_number(&mut self) -> Result<()> {
        match self.buffer.read_u32() {
            Ok(0xCAFEBABE) => Ok(()),
            Ok(_) => Err(ClassReaderError::invalid_class_data(
                "invalid magic number".to_owned(),
            )),
            Err(err) => Err(err.into()),
        }
    }

    fn read_version(&mut self) -> Result<()> {
        let minor_version = self.buffer.read_u16()?;
        let major_version = self.buffer.read_u16()?;

        self.class_file.version = ClassFileVersion::from(major_version, minor_version)?;
        Ok(())
    }

    fn read_constants(&mut self) -> Result<()> {
        let constants_count = self.buffer.read_u16()? - 1;
        let mut i = 0;
        while i < constants_count {
            let tag = self.buffer.read_u8()?;
            let constant = match tag {
                1 => self.read_utf8_constant()?,
                3 => self.read_int_constant()?,
                4 => self.read_float_constant()?,
                5 => {
                    i += 1; // long constants takes up two slots in the pool
                    self.read_long_constant()?
                }
                6 => {
                    i += 1; // double constants takes up two slots in the pool
                    self.read_double_constant()?
                }
                7 => self.read_class_reference_constant()?,
                8 => self.read_string_reference_constant()?,
                9 => self.read_field_reference_constant()?,
                10 => self.read_method_reference_constant()?,
                11 => self.read_interface_method_reference_constant()?,
                12 => self.read_name_and_type_constant()?,
                // For newer versions of java, there are more constant types
                _ => {
                    warn!("invalid entry in constant pool at index {} tag {}", i, tag);
                    return Err(ClassReaderError::invalid_class_data(format!(
                        "Unknown constant type: 0x{tag:X}"
                    )));
                }
            };
            self.class_file.constants.add(constant);

            i += 1;
        }

        Ok(())
    }

    fn read_utf8_constant(&mut self) -> Result<ConstantPoolEntry> {
        let len = self.buffer.read_u16()?;
        self.buffer
            .read_utf8(len as usize)
            .map(ConstantPoolEntry::Utf8)
            .map_err(|err| err.into())
    }

    fn read_int_constant(&mut self) -> Result<ConstantPoolEntry> {
        self.buffer
            .read_i32()
            .map(ConstantPoolEntry::Integer)
            .map_err(|err| err.into())
    }

    fn read_float_constant(&mut self) -> Result<ConstantPoolEntry> {
        self.buffer
            .read_f32()
            .map(ConstantPoolEntry::Float)
            .map_err(|err| err.into())
    }

    fn read_long_constant(&mut self) -> Result<ConstantPoolEntry> {
        self.buffer
            .read_i64()
            .map(ConstantPoolEntry::Long)
            .map_err(|err| err.into())
    }

    fn read_double_constant(&mut self) -> Result<ConstantPoolEntry> {
        self.buffer
            .read_f64()
            .map(ConstantPoolEntry::Double)
            .map_err(|err| err.into())
    }

    fn read_class_reference_constant(&mut self) -> Result<ConstantPoolEntry> {
        let fqn_string_index = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::ClassReference(fqn_string_index))
    }

    fn read_string_reference_constant(&mut self) -> Result<ConstantPoolEntry> {
        let string_index = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::StringReference(string_index))
    }

    fn read_method_reference_constant(&mut self) -> Result<ConstantPoolEntry> {
        let class_reference = self.buffer.read_u16()?;
        let name_and_type = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::MethodReference(
            class_reference,
            name_and_type,
        ))
    }

    fn read_interface_method_reference_constant(&mut self) -> Result<ConstantPoolEntry> {
        let class_reference = self.buffer.read_u16()?;
        let name_and_type = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::InterfaceMethodReference(
            class_reference,
            name_and_type,
        ))
    }

    fn read_field_reference_constant(&mut self) -> Result<ConstantPoolEntry> {
        let class_reference = self.buffer.read_u16()?;
        let name_and_type = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::FieldReference(
            class_reference,
            name_and_type,
        ))
    }

    fn read_name_and_type_constant(&mut self) -> Result<ConstantPoolEntry> {
        let name = self.buffer.read_u16()?;
        let type_descriptor = self.buffer.read_u16()?;
        Ok(ConstantPoolEntry::NameAndTypeDescriptor(
            name,
            type_descriptor,
        ))
    }

    fn read_access_flags(&mut self) -> Result<()> {
        let num = self.buffer.read_u16()?;
        match ClassAccessFlags::from_bits(num) {
            Some(flags) => {
                self.class_file.flags = flags;
                Ok(())
            }
            None => Err(ClassReaderError::invalid_class_data(format!(
                "invalid class flags: {num}"
            ))),
        }
    }

    fn read_class_reference(&mut self) -> Result<String> {
        let class_constant_idx = self.buffer.read_u16()?;
        self.read_string_reference(class_constant_idx)
    }

    fn read_class_reference_optional(&mut self) -> Result<Option<String>> {
        let super_constant_idx = self.buffer.read_u16()?;
        if super_constant_idx == 0 {
            Ok(None)
        } else {
            Ok(Some(self.read_string_reference(super_constant_idx)?))
        }
    }

    fn read_string_reference(&self, index: u16) -> Result<String> {
        Self::read_string_reference_from(&self.class_file.constants, index)
    }

    fn read_string_reference_from(constants_pool: &ConstantPool, index: u16) -> Result<String> {
        constants_pool.text_of(index).map_err(|err| err.into())
    }

    fn read_interfaces(&mut self) -> Result<()> {
        let interfaces_count = self.buffer.read_u16()?;
        self.class_file.interfaces = (0..interfaces_count)
            .map(|_| self.read_class_reference())
            .collect::<Result<Vec<String>>>()?;
        Ok(())
    }

    fn read_fields(&mut self) -> Result<()> {
        let fields_count = self.buffer.read_u16()?;
        self.class_file.fields = (0..fields_count)
            .map(|_| self.read_field())
            .collect::<Result<Vec<ClassFileField>>>()?;
        Ok(())
    }

    fn read_field(&mut self) -> Result<ClassFileField> {
        let flags = self.read_field_flags()?;
        let name_constant_index = self.buffer.read_u16()?;
        let name = self.read_string_reference(name_constant_index)?;
        let type_constant_index = self.buffer.read_u16()?;
        let type_descriptor_raw = self.read_string_reference(type_constant_index)?;
        let type_descriptor = FieldType::parse(&type_descriptor_raw)?;

        let raw_attributes = self.read_raw_attributes()?;
        let constant_value = self.extract_constant_value(&raw_attributes)?;
        let deprecated = self.search_deprecated_attribute(&raw_attributes);

        Ok(ClassFileField {
            flags,
            name,
            type_descriptor,
            constant_value,
            deprecated,
        })
    }

    fn read_field_flags(&mut self) -> Result<FieldFlags> {
        let field_flags_bits = self.buffer.read_u16()?;
        match FieldFlags::from_bits(field_flags_bits) {
            Some(flags) => Ok(flags),
            None => Err(ClassReaderError::invalid_class_data(format!(
                "invalid field flags: {field_flags_bits:#0x}"
            ))),
        }
    }

    fn extract_constant_value(
        &self,
        raw_attributes: &[Attribute],
    ) -> Result<Option<FieldConstantValue>> {
        raw_attributes
            .iter()
            .filter(|attr| attr.name == "ConstantValue")
            .map(|attr| {
                if attr.bytes.len() != std::mem::size_of::<u16>() {
                    Err(ClassReaderError::invalid_class_data(
                        "invalid attribute of type ConstantValue".to_string(),
                    ))
                } else {
                    let attribute_bytes: &[u8] = &attr.bytes;
                    let constant_index = u16::from_be_bytes(attribute_bytes.try_into().unwrap());
                    self.class_file
                        .constants
                        .get(constant_index)
                        .map_err(|err| err.into())
                        .and_then(|entry| match entry {
                            ConstantPoolEntry::StringReference(v) => {
                                let referred_string = self.read_string_reference(*v)?;
                                Ok(FieldConstantValue::String(referred_string))
                            }
                            ConstantPoolEntry::Integer(v) => Ok(FieldConstantValue::Int(*v)),
                            ConstantPoolEntry::Float(v) => Ok(FieldConstantValue::Float(*v)),
                            ConstantPoolEntry::Long(v) => Ok(FieldConstantValue::Long(*v)),
                            ConstantPoolEntry::Double(v) => Ok(FieldConstantValue::Double(*v)),
                            v => Err(ClassReaderError::invalid_class_data(format!(
                                "invalid type for ConstantValue: {v:?}"
                            ))),
                        })
                }
            })
            .next()
            .invert()
    }

    fn search_deprecated_attribute(&self, raw_attributes: &[Attribute]) -> bool {
        raw_attributes.iter().any(|attr| attr.name == "Deprecated")
    }

    fn read_methods(&mut self) -> Result<()> {
        let methods_count = self.buffer.read_u16()?;
        self.class_file.methods = (0..methods_count)
            .map(|_| self.read_method())
            .collect::<Result<Vec<ClassFileMethod>>>()?;
        Ok(())
    }

    fn read_method(&mut self) -> Result<ClassFileMethod> {
        let flags = self.read_method_flags()?;
        let name_constant_index = self.buffer.read_u16()?;
        let name = self.read_string_reference(name_constant_index)?;
        let type_constant_index = self.buffer.read_u16()?;
        let type_descriptor = self.read_string_reference(type_constant_index)?;
        let parsed_type_descriptor = MethodDescriptor::parse(&type_descriptor)?;
        let raw_attributes = self.read_raw_attributes()?;
        let code = if flags.contains(MethodFlags::NATIVE) || flags.contains(MethodFlags::ABSTRACT) {
            None
        } else {
            Some(self.extract_code(&raw_attributes, &name)?)
        };
        let deprecated = self.search_deprecated_attribute(&raw_attributes);
        let thrown_exceptions = self.extract_thrown_exceptions(&raw_attributes)?;

        Ok(ClassFileMethod {
            flags,
            name,
            type_descriptor,
            parsed_type_descriptor,
            attributes: raw_attributes,
            code,
            deprecated,
            thrown_exceptions,
        })
    }

    fn read_method_flags(&mut self) -> Result<MethodFlags> {
        let method_flags_bits = self.buffer.read_u16()?;
        match MethodFlags::from_bits(method_flags_bits) {
            Some(flags) => Ok(flags),
            None => Err(ClassReaderError::invalid_class_data(format!(
                "invalid method flags: {method_flags_bits:#0x}"
            ))),
        }
    }

    fn extract_code(
        &self,
        raw_attributes: &[Attribute],
        name: &str,
    ) -> Result<ClassFileMethodCode> {
        raw_attributes
            .iter()
            .filter(|attr| attr.name == "Code")
            .map(|attr| {
                let mut buf = Buffer::new(&attr.bytes);
                let max_stack = buf.read_u16()?;
                let max_locals = buf.read_u16()?;
                let code_length = buf.read_u32()?.into_usize_safe();
                let code = Vec::from(buf.read_bytes(code_length)?);
                let exception_table = self.read_exception_table(&mut buf)?;
                let attributes =
                    Self::read_raw_attributes_from(&self.class_file.constants, &mut buf)?;
                let line_number_table = self.extract_line_number_table(&attributes)?;

                Result::<ClassFileMethodCode>::Ok(ClassFileMethodCode {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    line_number_table,
                    attributes,
                })
            })
            .next()
            .invert()?
            .ok_or_else(|| {
                ClassReaderError::invalid_class_data(format!(
                    "method {name} is missing code attribute"
                ))
            })
    }

    fn read_exception_table(&self, buf: &mut Buffer) -> Result<ExceptionTable> {
        let exception_table_length = buf.read_u16()?.into_usize_safe();
        let mut entries: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length / 8);
        for _ in 0..exception_table_length {
            let start_pc = buf.read_u16()?;
            let end_pc = buf.read_u16()?;
            let handler_pc = buf.read_u16()?;
            let catch_class_constant = buf.read_u16()?;
            let catch_class = if catch_class_constant == 0 {
                None
            } else {
                Some(self.read_string_reference(catch_class_constant)?)
            };
            entries.push(ExceptionTableEntry {
                range: ProgramCounter(start_pc)..ProgramCounter(end_pc),
                handler_pc: ProgramCounter(handler_pc),
                catch_class,
            })
        }
        Ok(ExceptionTable::new(entries))
    }

    fn extract_line_number_table(
        &self,
        raw_attributes: &[Attribute],
    ) -> Result<Option<LineNumberTable>> {
        raw_attributes
            .iter()
            .find(|attr| attr.name == "LineNumberTable")
            .map(|attr| {
                let mut buf = Buffer::new(&attr.bytes);
                let num_entries = buf.read_u16()?.into_usize_safe();
                let mut entries = Vec::with_capacity(num_entries);
                for _ in 0..num_entries {
                    let program_counter = buf.read_u16()?;
                    let line_number = buf.read_u16()?;
                    entries.push(LineNumberTableEntry::new(
                        ProgramCounter(program_counter),
                        LineNumber(line_number),
                    ));
                }
                Ok(LineNumberTable::new(entries))
            })
            .invert()
    }

    fn extract_thrown_exceptions(&self, raw_attributes: &[Attribute]) -> Result<Vec<String>> {
        raw_attributes
            .iter()
            .find(|attr| attr.name == "Exceptions")
            .map(|attr| {
                let mut buf = Buffer::new(&attr.bytes);
                let num_entries = buf.read_u16()?.into_usize_safe();
                let mut exceptions = Vec::with_capacity(num_entries);
                for _ in 0..num_entries {
                    let class_name = buf.read_u16()?;
                    let exception_class_name = self.read_string_reference(class_name)?;
                    exceptions.push(exception_class_name);
                }
                Ok(exceptions)
            })
            .unwrap_or(Ok(Vec::new()))
    }

    fn read_class_attributes(&mut self) -> Result<()> {
        let raw_attributes = self.read_raw_attributes()?;
        self.class_file.deprecated = self.search_deprecated_attribute(&raw_attributes);
        self.class_file.source_file = self.search_source_file_attribute(&raw_attributes)?;
        Ok(())
    }

    fn search_source_file_attribute(&self, raw_attributes: &[Attribute]) -> Result<Option<String>> {
        raw_attributes
            .iter()
            .find(|attr| attr.name == "SourceFile")
            .map(|attr| {
                let attribute_bytes: &[u8] = &attr.bytes;
                let constant_index = u16::from_be_bytes(attribute_bytes.try_into().unwrap());
                self.class_file
                    .constants
                    .get(constant_index)
                    .map_err(|err| err.into())
                    .and_then(|entry| match entry {
                        ConstantPoolEntry::Utf8(file_name) => Ok(file_name.clone()),
                        _ => Err(ClassReaderError::invalid_class_data(
                            "invalid SourceFile attribute".to_string(),
                        )),
                    })
            })
            .invert()
    }

    fn read_raw_attributes(&mut self) -> Result<Vec<Attribute>> {
        Self::read_raw_attributes_from(&self.class_file.constants, &mut self.buffer)
    }

    fn read_raw_attributes_from(
        constants_pool: &ConstantPool,
        buffer: &mut Buffer,
    ) -> Result<Vec<Attribute>> {
        let attributes_count = buffer.read_u16()?;
        (0..attributes_count)
            .map(|_| Self::read_raw_attribute(constants_pool, buffer))
            .collect::<Result<Vec<Attribute>>>()
    }

    fn read_raw_attribute(constants_pool: &ConstantPool, buffer: &mut Buffer) -> Result<Attribute> {
        let name_constant_index = buffer.read_u16()?;
        let name = Self::read_string_reference_from(constants_pool, name_constant_index)?;
        let len = buffer.read_u32()?;
        let bytes = buffer.read_bytes(len.into_usize_safe())?;
        Ok(Attribute {
            name,
            bytes: Vec::from(bytes),
        })
    }
}

/// Reads a class from a byte slice.
pub fn read_buffer(buf: &[u8]) -> Result<ClassFile> {
    ClassFileReader::new(buf).read()
}

#[cfg(test)]
mod tests {
    use crate::{class_reader::read_buffer, class_reader_error::ClassReaderError};

    #[test]
    fn magic_number_is_required() {
        let data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(matches!(
            read_buffer(&data),
            Err(ClassReaderError::InvalidClassData(s, None)) if s == "invalid magic number"
        ));
    }
}
