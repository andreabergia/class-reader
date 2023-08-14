use log::info;

use class_reader::class_file::ClassFile;

pub fn read_class_from_bytes(bytes: &[u8]) -> ClassFile {
    let class = class_reader::read_buffer(bytes).unwrap();
    info!("read class file: {}", class);
    class
}
