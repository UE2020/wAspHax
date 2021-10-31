use std::os::raw::c_char;
use std::mem::transmute;

use std::ffi::CString;
use std::ffi::CStr;

use crate::util;

#[repr(C)]
pub struct StudioHdr {
    id: i32,
    version: i32,
    checksum: i32,
    name: [c_char; 64],
    length: i32,
    
    eye_position: cgmath::Vector3<f32>,
    illumposition: cgmath::Vector3<f32>,
    hull_min: cgmath::Vector3<f32>,
    hull_max: cgmath::Vector3<f32>,
    view_bbmin: cgmath::Vector3<f32>,
    view_bbmax: cgmath::Vector3<f32>,

    flags: i32,
    numbones: i32,
    boneindex: i32,

}

#[repr(C)]
pub struct CModel {
    name: [i8; 255]
}

#[derive(Debug)]
pub struct CModelInfo {
    base: *mut usize,
}

impl CModelInfo {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn get_model_index(&self, filename: &str) -> i32 {
        unsafe {
            let raw = CString::new(filename).unwrap();
            type Fn = unsafe extern "thiscall" fn(*const usize, *const c_char) -> i32;
            transmute::<_, Fn>(util::get_virtual_function(self.base, 3))(self.base, raw.as_ptr())
        }
    }

    pub fn get_model_name(&self, model: &CModel) -> String {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(*const usize, *const CModel) -> *const i8;
            let result = transmute::<_, Fn>(util::get_virtual_function(self.base, 4))(self.base, model as _); 
            let result = CStr::from_ptr(result as *mut i8);
            result.to_str().unwrap().to_owned()
        }
    }
}

impl Default for CModelInfo {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
