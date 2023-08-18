use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    class_access_flags::ClassAccessFlags,
    class_file::ClassFile,
    class_file_field::{ClassFileField, FieldConstantValue},
    class_file_version::ClassFileVersion,
    field_flags::FieldFlags,
    read_buffer,
};

#[derive(Debug, Serialize)]
struct WasmClass {
    pub version: ClassFileVersion,
    pub flags: Vec<WasmClassFlag>,
    pub name: String,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    // pub methods: Vec<ClassFileMethod>,
    pub deprecated: bool,
    pub source_file: Option<String>,
    pub fields: Vec<WasmClassField>,
}

// TODO: not sure if there is some better way to do this via bitflags
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum WasmClassFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
}

#[derive(Debug, Serialize)]
struct WasmClassField {
    pub flags: Vec<WasmFieldFlag>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_descriptor: String,
    pub constant_value: Option<WasmFieldConstantValue>,
    pub deprecated: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum WasmFieldFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum WasmFieldConstantValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}

impl From<ClassFile> for WasmClass {
    fn from(class: ClassFile) -> Self {
        Self {
            version: class.version,
            flags: class.flags.iter().map(|f| f.into()).collect(),
            name: class.name,
            superclass: class.superclass,
            interfaces: class.interfaces,
            deprecated: class.deprecated,
            source_file: class.source_file,
            fields: class.fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl From<ClassAccessFlags> for WasmClassFlag {
    fn from(flag: ClassAccessFlags) -> Self {
        match flag {
            ClassAccessFlags::PUBLIC => Self::Public,
            ClassAccessFlags::FINAL => Self::Final,
            ClassAccessFlags::SUPER => Self::Super,
            ClassAccessFlags::INTERFACE => Self::Interface,
            ClassAccessFlags::ABSTRACT => Self::Abstract,
            ClassAccessFlags::SYNTHETIC => Self::Synthetic,
            ClassAccessFlags::ANNOTATION => Self::Annotation,
            ClassAccessFlags::ENUM => Self::Enum,
            _ => panic!("Unknown flag: {:?}", flag),
        }
    }
}

impl From<ClassFileField> for WasmClassField {
    fn from(value: ClassFileField) -> Self {
        Self {
            flags: value.flags.iter().map(|f| f.into()).collect(),
            name: value.name,
            type_descriptor: value.type_descriptor.to_string(),
            constant_value: value.constant_value.map(|c| c.into()),
            deprecated: value.deprecated,
        }
    }
}

impl From<FieldFlags> for WasmFieldFlag {
    fn from(flag: FieldFlags) -> Self {
        match flag {
            FieldFlags::PUBLIC => Self::Public,
            FieldFlags::PRIVATE => Self::Private,
            FieldFlags::PROTECTED => Self::Protected,
            FieldFlags::STATIC => Self::Static,
            FieldFlags::FINAL => Self::Final,
            FieldFlags::VOLATILE => Self::Volatile,
            FieldFlags::TRANSIENT => Self::Transient,
            FieldFlags::SYNTHETIC => Self::Synthetic,
            FieldFlags::ENUM => Self::Enum,
            _ => panic!("Unknown flag: {:?}", flag),
        }
    }
}

impl From<FieldConstantValue> for WasmFieldConstantValue {
    fn from(value: FieldConstantValue) -> Self {
        match value {
            FieldConstantValue::Int(value) => Self::Int(value),
            FieldConstantValue::Float(value) => Self::Float(value),
            FieldConstantValue::Long(value) => Self::Long(value),
            FieldConstantValue::Double(value) => Self::Double(value),
            FieldConstantValue::String(value) => Self::String(value),
        }
    }
}

#[wasm_bindgen]
pub fn wasm_read_buffer(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let class_file = read_buffer(buffer).map(WasmClass::from);
    match class_file {
        Ok(class_file) => Ok(serde_wasm_bindgen::to_value(&class_file)?),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err)?),
    }
}
