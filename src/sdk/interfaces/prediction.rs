#[derive(Debug)]
pub struct CPrediction {
    pub base: *mut usize,
}

impl CPrediction {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn setup_move(&self, player: *mut usize, cmd: *mut super::clientmode::CUserCmd, helper: *mut usize, mv: *mut usize) -> String {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(*mut usize, *mut usize, *mut super::clientmode::CUserCmd, *mut usize, *mut usize) -> *const i8;
            transmute::<_, Fn>(util::get_virtual_function(self.base, 21))(self.base, cmd, helper, mv);
        }
    }

    pub fn finish_move(&self, player: *mut usize, cmd: *mut super::clientmode::CUserCmd, mv: *mut usize) -> String {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(*mut usize, *mut usize, *mut super::clientmode::CUserCmd, *mut usize, *mut usize) -> *const i8;
            transmute::<_, Fn>(util::get_virtual_function(self.base, 21))(self.base, cmd, helper, mv);
        }
    }
}

impl Default for CPrediction {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
