use std::mem::{self, transmute};

lazy_static::lazy_static! {
    pub static ref PAGESIZE: i64 = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) };
    pub static ref PAGEMASK: i64 = !(*PAGESIZE-1);
}

/// Unprotect a memory region - the old protection is returned.
pub unsafe fn unprotect(region: *mut usize) -> i32 {
    libc::mprotect(
        transmute::<i64, *mut libc::c_void>(region as i64 & *PAGEMASK),
        *PAGESIZE as usize,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    );

    libc::PROT_READ | libc::PROT_EXEC
}

/// Set the protection of a memory region.
pub unsafe fn protect(region: *mut usize, protection: i32) {
    libc::mprotect(
        transmute::<i64, *mut libc::c_void>(region as i64 & *PAGEMASK),
        *PAGESIZE as usize,
        protection,
    );
}

/// Replace an offset in a vtable with a new func.
/// To unhook, just hook again, but replace the new func with the original -
/// as the original is returned when you call hook.
pub unsafe fn hook(instance: *mut isize, hook: *mut usize, offset: isize) -> *mut usize {
    let vtable = *instance;
    let entry = vtable + mem::size_of::<isize>() as isize * offset;
    let original = *(entry as *mut isize);

    let original_protection = unprotect(entry as *mut usize);
    *(entry as *mut isize) = hook as isize;
    protect(entry as *mut usize, original_protection); // reprotect the unprotected region

    original as *mut usize
}
