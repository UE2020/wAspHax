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
    pub numbones: i32,
    boneindex: i32,

    numbonecontrollers: i32,
    bonecontrollerindex: i32,
    numhitboxsets: i32,
    hitboxsetindex: i32,

    numlocalanim: i32,
    localanimindex: i32,
    numlocalseq: i32,
    localseqindex: i32,
    
    activitylistversion: i32,
    eventsindexed: i32,

    numtextures: i32,
    textureindex: i32,

    numcdtextures: i32,
    cdtextureindex: i32,

    numskinref: i32,
    numskinfamilies: i32,
    skinindex: i32,

    numbodyparts: i32,
    bodypartindex: i32,

    numlocalattachments: i32,
    localattachmentindex: i32,

    numlocalnodes: i32,
    localnodeindex: i32,
    localnodenameindex: i32,

    numflexdesc: i32,
    flexdescindex: i32,
    numflexcontrollers: i32,
    flexcontrollerindex: i32,
    numflexrules: i32,
    flexruleindex: i32,
    numikchains: i32,
    ikchainindex: i32,
    nummouths: i32,
    mouthindex: i32,

    numlocalposeparameters: i32,
    localposeparamindex: i32,

    surfacepropindex: i32,

    keyvalueindex: i32,
    keyvaluesize: i32,

    numlocalikautoplaylocks: i32,
    localikautoplaylockindex: i32,

    mass: f32,
    contents: i32,

    numincludemodels: i32,
    includemodelindex: i32,

    virtualModel: *mut usize,

    szanimblocknameindex: i32,

    numanimblocks: i32,
    animblockindex: i32,
    c: *mut usize,

    bonetablebynameindex: i32,

    pVertexBase: *mut usize,
    pIndexBase: *mut usize,

    constdirectionallightdot: u8,
    
    rootLOD: u8,

    numAllowedRootLODs: u8,
    unused: u8,
    unused4: i32,
    numflexcontrollerui: i32,
    flexcontrolleruiindex: i32,
    unused3: [i32; 2],

    studiohdr2index: i32,

    unused2: [i32; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CStudioBone {
    sznameindex: i32,
    pub parent: i32,
    bonecontroller: [i32; 6],

    pos: cgmath::Vector3<f32>,
    quat: cgmath::Quaternion<f32>,
    rot: cgmath::Vector3<f32>,

    posscale: cgmath::Vector3<f32>,
    rotscale: cgmath::Vector3<f32>,

    poseToBone: vecmath::Matrix3x4<f32>,
    qAlignment: cgmath::Quaternion<f32>,
    pub flags: i32,
    proctype: i32,
    procindex: i32,
    physicsbone: i32,
    surfacepropidx: i32,

    contents: i32,
    surfaceproplookup: i32,
    unused: [i32; 7],
}

impl StudioHdr {
    pub fn bone(&mut self, i: i32) -> *mut CStudioBone {
        unsafe { ((self as *mut StudioHdr as *mut libc::c_uchar).offset(self.boneindex as isize) as
                *mut CStudioBone).offset(i as isize) }
    }
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

    pub fn get_studio_model(&self, model: &CModel) -> *mut StudioHdr {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(*const usize, *const CModel) -> *mut StudioHdr;
            transmute::<_, Fn>(util::get_virtual_function(self.base, 31))(self.base, model as _)
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
