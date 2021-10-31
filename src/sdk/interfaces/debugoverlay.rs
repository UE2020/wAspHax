use std::mem::transmute;

use cgmath::Vector3;
use crate::util::get_virtual_function;

type WorldToScreenFn = unsafe extern "thiscall" fn(thisptr: *mut usize, input: *const Vector3<f32>, out: *mut Vector3<f32>) -> u8;

#[derive(Debug)]
pub struct CDebugOverlay {
    base: *mut usize,
}

impl CDebugOverlay {

    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn world_to_screen(&self, position: &Vector3<f32>) -> Option<Vector3<f32>> {
        let mut return_vec = unsafe { std::mem::zeroed() };
        let return_code = unsafe {
            transmute::<_, WorldToScreenFn>(get_virtual_function(self.base, 11))(self.base, position as *const _, &mut return_vec as *mut _)
        };

        if return_code == 1 {
            return None;
        }

        Some(return_vec)
    }

}

impl Default for CDebugOverlay {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}

pub fn world_to_screen(position: &Vector3<f32>) -> Option<Vector3<f32>> {
    let interfaces = &super::INTERFACES;
    interfaces.debug_overlay.world_to_screen(position)
}