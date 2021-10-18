pub fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    unsafe {
        let vt = *base as *mut usize;
        let vfn = vt.offset(idx).read() as *mut usize;

        return vfn;
    }
}