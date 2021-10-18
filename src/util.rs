pub unsafe fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    let vt = *base as *mut usize;
    vt.offset(idx).read() as *mut usize
}
