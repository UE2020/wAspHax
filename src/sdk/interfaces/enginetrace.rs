use std::mem::transmute;
use crate::util;

use std::ffi::CStr;

#[repr(C)]
pub struct Plane {
    normal: cgmath::Vector3<f32>,
    dist: f32,
    ty: u8,
    signbits: u8,
    pad: [u8; 2],
}

#[repr(C)]
enum HitGroups {
	HITGROUP_GENERIC = 0,
	HITGROUP_HEAD,
	HITGROUP_CHEST,
	HITGROUP_STOMACH,
	HITGROUP_LEFTARM,
	HITGROUP_RIGHTARM,
	HITGROUP_LEFTLEG,
	HITGROUP_RIGHTLEG,
	HITGROUP_GEAR
}

#[repr(C)]
pub struct Surface {
    name: *const i8,
    surface_props: i16,
    flags: u16,
}

#[repr(C)]
pub struct Trace {
    startpos: cgmath::Vector3<f32>,
    endpos: cgmath::Vector3<f32>,
    plane: Plane,

    fraction: f32,

    contents: i32,
    dispflags: u32,

    allsolid: bool,
    startsolid: bool,

    fractionleftsolid: f32,

    surface: Surface,

    hitgroup: HitGroups,
    physicsbone: i16,

    world_surface_index: u16,
    m_pEntityHit: *mut usize,
    hitbox: i32,
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct AlignedVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AlignedVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        AlignedVector {
            x,
            y,
            z,
        }
    }
}

impl Into<cgmath::Vector3<f32>> for AlignedVector {
    fn into(self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(self.x, self.y, self.z)
    }
}

impl From<cgmath::Vector3<f32>> for AlignedVector {
    fn from(v: cgmath::Vector3<f32>) -> Self {
        AlignedVector {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

#[repr(C)]
pub struct Ray {
    m_Start: AlignedVector,
    m_Delta: AlignedVector,
    m_StartOffset: AlignedVector,
    m_Extents: AlignedVector,

    m_pWorldAxisTransform: *const vecmath::Matrix3x4<f32>,

    m_IsRay: bool,
    m_IsSwept: bool,
}

impl Ray {
    pub fn new(vecstart: cgmath::Vector3<f32>, vecend: cgmath::Vector3<f32>) -> Self {
        let m_Delta = AlignedVector::new(
            vecend.x - vecstart.x,
            vecend.y - vecstart.y,
            vecend.z - vecstart.z,
        );
        Self {
            m_Delta,
            m_IsSwept: (m_Delta.x * m_Delta.x + m_Delta.y * m_Delta.y + m_Delta.z * m_Delta.z) != 0.0,
            m_Extents: cgmath::vec3(0.0, 0.0, 0.0).into(),
            m_pWorldAxisTransform: std::ptr::null_mut(),
            m_IsRay: true,
            m_StartOffset: cgmath::vec3(0.0, 0.0, 0.0).into(),
            m_Start: vecstart.into(),
        }
    }
}

#[repr(C)]
pub struct CTraceFilter {
    vptr: *mut usize,
    pSkip: *mut usize,
}

#[derive(Debug)]
pub struct CEngineTrace {
    pub base: *mut usize,
}

impl CEngineTrace {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn trace_ray(&self, ray: &mut Ray, mask: u32, trace_filter: &mut CTraceFilter, trace: &mut Trace) -> String {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(*const usize, ray: *mut Ray, u32, *mut CTraceFilter, trace: *mut Trace) -> *const i8;
            let result =
                transmute::<_, Fn>(util::get_virtual_function(self.base, 5))(self.base, ray as _, mask, trace_filter as _, trace as _);
            let result = CStr::from_ptr(result as *mut i8);
            result.to_str().unwrap().to_owned()
        }
    }
}

impl Default for CEngineTrace {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
