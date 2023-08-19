use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    class_access_flags::ClassAccessFlags,
    class_file::ClassFile,
    class_file_field::{ClassFileField, FieldConstantValue},
    class_file_method::{ClassFileMethod, ClassFileMethodCode},
    class_file_version::ClassFileVersion,
    exception_table::ExceptionTable,
    field_flags::FieldFlags,
    instruction::Instruction,
    line_number_table::LineNumberTable,
    method_descriptor::MethodDescriptor,
    method_flags::MethodFlags,
    read_buffer,
};

#[derive(Debug, Serialize)]
struct WasmClass {
    pub version: ClassFileVersion,
    pub flags: Vec<WasmClassFlag>,
    pub name: String,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub deprecated: bool,
    pub source_file: Option<String>,
    pub fields: Vec<WasmField>,
    pub methods: Vec<WasmMethod>,
}

// TODO: not sure if there is some better way to do this with bitflags
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
struct WasmField {
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

#[derive(Debug, Serialize)]
struct WasmMethod {
    pub flags: Vec<WasmMethodFlag>,
    pub name: String,
    #[serde(rename = "internal_type")]
    pub type_descriptor: String,
    #[serde(rename = "type")]
    pub parsed_type_descriptor: WasmMethodDescriptor,
    pub deprecated: bool,
    pub thrown_exceptions: Vec<String>,
    pub code: Option<WasmMethodCode>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum WasmMethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Vargargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

#[derive(Debug, Serialize)]
struct WasmMethodDescriptor {
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct WasmMethodCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: Vec<WasmInstruction>,
    pub raw_bytecode: Vec<u8>,
    #[serde(flatten)]
    pub exception_table: ExceptionTable,
    #[serde(flatten)]
    pub line_number_table: Option<LineNumberTable>,
}

#[derive(Debug, Serialize)]
struct WasmInstruction {
    pub address: usize,
    pub instruction: Instruction,
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
            methods: class.methods.into_iter().map(|f| f.into()).collect(),
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

impl From<ClassFileField> for WasmField {
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

impl From<ClassFileMethod> for WasmMethod {
    fn from(method: ClassFileMethod) -> Self {
        Self {
            flags: method.flags.iter().map(|f| f.into()).collect(),
            name: method.name,
            type_descriptor: method.type_descriptor.to_string(),
            parsed_type_descriptor: method.parsed_type_descriptor.into(),
            deprecated: method.deprecated,
            thrown_exceptions: method.thrown_exceptions,
            code: method.code.map(|c| c.into()),
        }
    }
}

impl From<MethodFlags> for WasmMethodFlag {
    fn from(flag: MethodFlags) -> Self {
        match flag {
            MethodFlags::PUBLIC => Self::Public,
            MethodFlags::PRIVATE => Self::Private,
            MethodFlags::PROTECTED => Self::Protected,
            MethodFlags::STATIC => Self::Static,
            MethodFlags::FINAL => Self::Final,
            MethodFlags::SYNCHRONIZED => Self::Synchronized,
            MethodFlags::BRIDGE => Self::Bridge,
            MethodFlags::VARARGS => Self::Vargargs,
            MethodFlags::NATIVE => Self::Native,
            MethodFlags::ABSTRACT => Self::Abstract,
            MethodFlags::STRICT => Self::Strict,
            MethodFlags::SYNTHETIC => Self::Synthetic,
            _ => panic!("Unknown flag: {:?}", flag),
        }
    }
}

impl From<MethodDescriptor> for WasmMethodDescriptor {
    fn from(value: MethodDescriptor) -> Self {
        Self {
            parameters: value.parameters.iter().map(|p| p.to_string()).collect(),
            return_type: value.return_type.map(|t| t.to_string()),
        }
    }
}

impl From<ClassFileMethodCode> for WasmMethodCode {
    fn from(value: ClassFileMethodCode) -> Self {
        Self {
            max_stack: value.max_stack,
            max_locals: value.max_locals,
            instructions: Instruction::parse_instructions(&value.code)
                .unwrap()
                .iter()
                .map(|i| i.into())
                .collect(),
            raw_bytecode: value.code,
            exception_table: value.exception_table,
            line_number_table: value.line_number_table,
        }
    }
}

impl From<&(usize, Instruction)> for WasmInstruction {
    fn from(value: &(usize, Instruction)) -> Self {
        Self {
            address: value.0,
            instruction: value.1.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn wasm_read_buffer(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new()
        .serialize_maps_as_objects(true)
        .serialize_missing_as_null(true);

    let class_file = read_buffer(buffer).map(WasmClass::from);
    match class_file {
        Ok(class_file) => Ok(class_file.serialize(&serializer)?),
        Err(err) => Err(err.serialize(&serializer)?),
    }
}
