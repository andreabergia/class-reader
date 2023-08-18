mod attribute;
mod buffer;
pub mod class_access_flags;
pub mod class_file;
pub mod class_file_field;
pub mod class_file_method;
pub mod class_file_version;
mod class_reader;
pub mod class_reader_error;
pub mod constant_pool;
pub mod exception_table;
pub mod field_flags;
pub mod field_type;
pub mod instruction;
pub mod line_number;
pub mod line_number_table;
pub mod method_descriptor;
pub mod method_flags;
pub mod program_counter;
pub mod type_conversion;

#[cfg(feature = "wasm")]
pub mod wasm_wrappers;

pub use class_reader::read_buffer;
