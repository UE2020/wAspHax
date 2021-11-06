#[repr(C)]
pub struct CUserCmd {
    pub vptr: *mut usize,
    pub command_number: i32,
    pub tick_count: i32,
    pub viewangles: cgmath::Vector3<f32>,
    pub aimdirection: cgmath::Vector3<f32>,
    pub forwardmove: f32,
    pub sidemove: f32,
    pub upmove: f32,
    pub buttons: i32,
    pub impulse: u8,
    pub weaponselect: i32,
    pub weaponsubtype: i32,
    pub random_seed: i32,
    pub mousedx: i16,
    pub mousedy: i16,
    pub hasbeenpredicted: bool,
    pub headangles: cgmath::Vector3<f32>,
    pub headoffset: cgmath::Vector3<f32>,
}

#[derive(Debug)]
pub struct CClientMode {
    pub base: *mut usize,
}

impl CClientMode {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }
}

impl Default for CClientMode {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
