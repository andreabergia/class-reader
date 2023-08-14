use std::fmt;

use crate::{
    class_access_flags::ClassAccessFlags, class_file_field::ClassFileField,
    class_file_method::ClassFileMethod, class_file_version::ClassFileVersion,
    constant_pool::ConstantPool,
};

/// Represents the content of a .class file.
#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constants: ConstantPool,
    pub flags: ClassAccessFlags,
    pub name: String,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<ClassFileField>,
    pub methods: Vec<ClassFileMethod>,
    pub deprecated: bool,
    pub source_file: Option<String>,
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {} ", self.name,)?;
        if let Some(superclass) = self.superclass.as_ref() {
            write!(f, "(extends {}) ", superclass)?;
        }
        writeln!(f, "version: {}", self.version)?;
        write!(f, "{}", self.constants)?;
        writeln!(
            f,
            "flags: {:?}, deprecated: {}",
            self.flags, self.deprecated
        )?;
        writeln!(f, "interfaces: {:?}", self.interfaces)?;
        writeln!(f, "fields:")?;
        for field in self.fields.iter() {
            writeln!(f, "  - {field}")?;
        }
        writeln!(f, "methods:")?;
        for method in self.methods.iter() {
            writeln!(f, "  - {method}")?;
        }
        Ok(())
    }
}
