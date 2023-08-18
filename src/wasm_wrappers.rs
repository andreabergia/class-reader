use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    class_access_flags::ClassAccessFlags, class_file::ClassFile,
    class_file_version::ClassFileVersion, read_buffer,
};

#[derive(Debug, Serialize)]
struct WasmClassFile {
    pub version: ClassFileVersion,
    pub flags: Vec<WasmClassAccessFlags>,
    pub name: String,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    // pub fields: Vec<ClassFileField>,
    // pub methods: Vec<ClassFileMethod>,
    pub deprecated: bool,
    pub source_file: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum WasmClassAccessFlags {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
}

impl WasmClassAccessFlags {
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

impl WasmClassFile {
    fn new(class: ClassFile) -> Self {
        Self {
            version: class.version,
            flags: class.flags.iter().map(WasmClassAccessFlags::from).collect(),
            name: class.name,
            superclass: class.superclass,
            interfaces: class.interfaces,
            deprecated: class.deprecated,
            source_file: class.source_file,
        }
    }
}

#[wasm_bindgen]
pub fn wasm_read_buffer(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let class_file = read_buffer(buffer).map(WasmClassFile::new);
    match class_file {
        Ok(class_file) => Ok(serde_wasm_bindgen::to_value(&class_file)?),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err)?),
    }
}
