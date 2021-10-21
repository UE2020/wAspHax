use std::mem::{self, transmute};

lazy_static::lazy_static! {
    pub static ref PAGESIZE: i64 = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) };
    pub static ref PAGEMASK: i64 = !(*PAGESIZE-1);
}

pub unsafe fn unprotect(region: *mut usize) -> i32 {
    libc::mprotect(
        transmute::<i64, *mut libc::c_void>(region as i64 & *PAGEMASK),
        *PAGESIZE as usize,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    );

    libc::PROT_READ | libc::PROT_EXEC
}

pub unsafe fn protect(region: *mut usize, protection: i32) {
    libc::mprotect(
        transmute::<i64, *mut libc::c_void>(region as i64 & *PAGEMASK),
        *PAGESIZE as usize,
        protection,
    );
}

pub unsafe fn hook(instance: *mut isize, hook: *mut usize, offset: isize) -> *mut usize {
    let vtable = *instance;
    let entry = vtable + mem::size_of::<isize>() as isize * offset;
    let original = *(entry as *mut isize);

    let original_protection = unprotect(entry as *mut usize);
    *(entry as *mut isize) = hook as isize;
    protect(entry as *mut usize, original_protection);

    original as *mut usize
}
