#[derive(Debug)]
pub struct CEngineVGui {
    pub base: *mut usize,
}

impl CEngineVGui {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }
}

impl Default for CEngineVGui {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
