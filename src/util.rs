use std::mem::transmute;

pub unsafe fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    let vt = *base as *mut usize;
    vt.offset(idx).read() as *mut usize
}

pub unsafe fn get_virtual_table(c: *mut libc::c_void, offset: usize) -> *mut *mut libc::c_void {
    *transmute::<usize, *mut *mut *mut libc::c_void>(c as usize + offset)
}

pub type int32_t = i32;
pub type uintptr_t = usize;

pub unsafe fn get_abs_addr(ptr: uintptr_t, offset: libc::c_int, size: libc::c_int) -> uintptr_t {
    return ptr
        .wrapping_add(*(ptr.wrapping_add(offset as usize) as *mut int32_t) as usize)
        .wrapping_add(size as usize);
}

macro_rules! c_str {
    ($lit:expr) => {
        std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr() as *const libc::c_char)
    };
}

pub(crate) use c_str;
