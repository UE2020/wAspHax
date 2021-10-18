use super::netvars;
use crate::util::get_virtual_function;

use cgmath::{vec3, Vector3};
use std::mem::transmute;

#[derive(Copy, Clone)]
pub struct CEntity {
    base: *mut usize,
}

type IsPlayerFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;
type OriginFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> *mut Vector3<f32>;
type DormantFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;

impl CEntity {
    pub fn get_value<T>(&self, offset: usize) -> T {
        unsafe {
           ((self.base as usize + offset) as *mut T).read()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_null()
    }

    pub unsafe fn from_raw(base: *mut usize) -> Self {
        Self {
            base,
        }
    }

    pub fn networkable(&self) -> *mut usize {
        self.get_value(16)
    }

    pub fn renderable(&self) -> *mut usize {
        self.get_value(0x8)
    }

    pub fn get_health(&self) -> i32 {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_iHealth"))
    }

    pub fn get_armor(&self) -> i32 {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_ArmorValue"))
    }

    pub fn get_aim_punch(&self) -> Vector3<f32> {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_aimPunchAngle"))
    }

    pub fn is_scoped(&self) -> bool {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_bIsScoped"))
    }

    #[deprecated]
    pub fn is_defusing(&self) -> bool {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_bIsDefusing"))
    }

    pub fn get_team_num(&self) -> i32 {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_iTeamNum"))
    }

    pub fn get_origin(&self) -> *mut Vector3<f32> {
        unsafe {
            transmute::<_, OriginFn>(get_virtual_function(self.base, 12))(self.base)
        }
    }

    pub fn get_velocity(&self) -> Vector3<f32> {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_vecVelocity[0]"))
    }

    pub fn is_dormant(&self) -> bool {
        unsafe {
            transmute::<_, DormantFn>(get_virtual_function(self.base, 9))(self.base)
        }
    }

    pub fn is_player(&self) -> bool {
        unsafe {
            transmute::<_, IsPlayerFn>(get_virtual_function(self.base, 157))(self.base)
        }
    }

    #[deprecated]
    pub fn get_life_state(&self) -> i32 {
        self.get_value(netvars::get_netvar_offset!("DT_BasePlayer", "m_lifeState"))
    }

    pub fn is_alive(&self) -> bool {
        self.get_life_state() == 0
    }

    pub fn get_bone_pos(&self, bone: i32) -> Vector3<f32> {
        let ptr: *mut usize = self.get_value(0x26a8);

        let x = unsafe { *((ptr as usize + (bone as usize * 48 + 12)) as *mut f32) };
        let y = unsafe { *((ptr as usize + (bone as usize * 48 + 28)) as *mut f32) };
        let z = unsafe { *((ptr as usize + (bone as usize * 48 + 44)) as *mut f32) };

        vec3(x, y, z)
    }
}