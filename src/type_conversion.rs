/// Simple utility trait that models the conversion of various integer types into usize.
/// Assumes that these types are all smaller than usize, or panics.
pub trait ToUsizeSafe {
    fn into_usize_safe(self) -> usize;
}

impl ToUsizeSafe for u8 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("usize should have at least 8 bits")
    }
}

impl ToUsizeSafe for u16 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("usize should have at least 16 bits")
    }
}

impl ToUsizeSafe for u32 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("usize should have at least 32 bits")
    }
}

impl ToUsizeSafe for i32 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("usize should have at least 64 bits")
    }
}
