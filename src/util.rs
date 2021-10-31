pub unsafe fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    let vt = *base as *mut usize;
    vt.offset(idx).read() as *mut usize
}

macro_rules! c_str {
    ($lit:expr) => {
        std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr() as *const libc::c_char)
    };
}

pub(crate) use c_str;
