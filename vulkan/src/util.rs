pub fn to_pointers<T: AsRef<std::ffi::CStr>>(val: &[T]) -> Vec<*const i8> {
    val.iter().map(|v| v.as_ref().as_ptr()).collect()
}
