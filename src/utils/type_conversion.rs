pub trait ToUsizeSafe {
    fn to_usize_safe(&self) -> usize;
}

impl ToUsizeSafe for u32 {
    fn to_usize_safe(&self) -> usize {
        usize::try_from(*self).expect("usize should have at least 32 bits")
    }
}

impl ToUsizeSafe for u16 {
    fn to_usize_safe(&self) -> usize {
        usize::try_from(*self).expect("usize should have at least 16 bits")
    }
}