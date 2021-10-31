use std::os::raw::c_char;

use crate::util;

type GetNameFn = unsafe extern "thiscall" fn(*const usize, u64) -> *const c_char;

#[derive(Debug)]
pub struct CPanel {
    pub base: *mut usize,
}

impl CPanel {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn get_panel_name(&self, panel: u64) -> *const c_char {
        let vfunc = unsafe {
            std::mem::transmute::<_, GetNameFn>(util::get_virtual_function(self.base, 37))
        };

        unsafe { vfunc(self.base, panel) }
    }
}

impl Default for CPanel {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
