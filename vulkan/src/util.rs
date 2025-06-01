pub trait AsPtr<Output> {
    fn as_ptr(val: Self) -> Output;
}

impl<'a, T: AsRef<std::ffi::CStr>> AsPtr<&'a [*const i8]> for &'a [T] {
    fn as_ptr(val: Self) -> &'a [*const i8] {
        todo!()
        // val.as_ptr()
        // val.iter().map(|v| v.as_ref().as_ptr()).collect()
    }
}
