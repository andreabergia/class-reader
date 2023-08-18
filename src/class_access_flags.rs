use bitflags::bitflags;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
/// Class flags
pub struct ClassAccessFlags(u16);

bitflags! {
    impl ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

impl Default for ClassAccessFlags {
    fn default() -> ClassAccessFlags {
        ClassAccessFlags::empty()
    }
}
