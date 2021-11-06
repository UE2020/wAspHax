use maplit::hashmap;
use std::collections::HashMap;
use std::ffi::CStr;
use std::mem::transmute;
use std::sync::Mutex;

#[derive(PartialEq, Eq, Hash)]
pub struct NetvarPair {
    first: String,
    second: String,
}

impl NetvarPair {
    pub fn new(first: &str, second: &str) -> Self {
        Self {
            first: first.to_owned(),
            second: second.to_owned(),
        }
    }
}

lazy_static::lazy_static! {
    /// Global netvars map
    pub static ref OFFSETS: Mutex<HashMap<NetvarPair, usize>> = Mutex::new(hashmap!{
        /* Entity */
        NetvarPair::new("DT_BaseEntity", "m_Collision") => 0,
        NetvarPair::new("DT_BaseEntity", "m_iTeamNum") => 0,
        NetvarPair::new("DT_BaseEntity", "m_bSpotted") => 0,

        /* Player */
        NetvarPair::new("DT_BasePlayer", "m_vecVelocity[0]") => 0,
        NetvarPair::new("DT_BasePlayer", "m_nTickBase") => 0,
        NetvarPair::new("DT_CSPlayer", "m_iAccount") => 0,
        NetvarPair::new("DT_BasePlayer", "m_iHealth") => 0,
        NetvarPair::new("DT_CSPlayer", "m_fFlags") => 0,
        NetvarPair::new("DT_BasePlayer", "m_aimPunchAngle") => 0,
        NetvarPair::new("DT_BasePlayer", "m_viewPunchAngle") => 0,
        NetvarPair::new("DT_CSPlayer", "m_hActiveWeapon") => 0,
        NetvarPair::new("DT_CSPlayer", "m_hObserverTarget") => 0,
        NetvarPair::new("DT_CSPlayer", "m_bHasDefuser") => 0,
        NetvarPair::new("DT_BasePlayer", "m_vecViewOffset[0]") => 0,
        NetvarPair::new("DT_CSPlayer", "m_angEyeAngles[0]") => 0,
        NetvarPair::new("DT_CSPlayer", "m_flLowerBodyYawTarget") => 0,
        NetvarPair::new("DT_CSPlayer", "m_flFlashDuration") => 0,
        NetvarPair::new("DT_CSPlayer", "m_flFlashMaxAlpha") => 0,
        NetvarPair::new("DT_CSPlayer", "m_bIsScoped") => 0,
        NetvarPair::new("DT_BasePlayer", "deadflag") => 0,
        NetvarPair::new("DT_CSPlayer", "m_bHasHelmet") => 0,
        NetvarPair::new("DT_CSPlayer", "m_ArmorValue") => 0,
        NetvarPair::new("DT_BaseEntity", "m_nRenderMode") => 0, // Used for movetype
        NetvarPair::new("DT_CSPlayer", "m_nSurvivalTeam") => 0,
        NetvarPair::new("DT_BasePlayer", "m_lifeState") => 0,
        NetvarPair::new("DT_BaseEntity", "m_vecOrigin") => 0,

        /* Item */
        NetvarPair::new("DT_BaseAttributableItem", "m_iItemDefinitionIndex") => 0,

        /* Weapon */
        NetvarPair::new("DT_BaseCombatWeapon", "m_hOwner") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_hOwnerEntity") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_iItemIDHigh") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_iAccountID") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_nFallbackPaintKit") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_flFallbackWear") => 0,
        NetvarPair::new("DT_BaseCombatWeapon", "m_nFallbackStatTrak") => 0,

        /* Bomb */
        NetvarPair::new("DT_PlantedC4", "m_flC4Blow") => 0,

        /* Tonemap Controller */
        NetvarPair::new("DT_EnvTonemapController", "m_bUseCustomAutoExposureMin") => 0,
        NetvarPair::new("DT_EnvTonemapController", "m_bUseCustomAutoExposureMax") => 0,
        NetvarPair::new("DT_EnvTonemapController", "m_flCustomAutoExposureMin") => 0,
        NetvarPair::new("DT_EnvTonemapController", "m_flCustomAutoExposureMax") => 0,

        /* Player Resource */
        NetvarPair::new("DT_PlayerResource", "m_iPing") => 0,

        /* Fog Controller */
        NetvarPair::new("DT_FogController", "m_fog.enable") => 0,
        NetvarPair::new("DT_FogController", "m_fog.start") => 0,
        NetvarPair::new("DT_FogController", "m_fog.end") => 0,
        NetvarPair::new("DT_FogController", "m_fog.maxdensity") => 0,
        NetvarPair::new("DT_FogController", "m_fog.colorPrimary") => 0,
    });

    pub static ref DATA: Offsets = unsafe {
        Offsets {
            move_data: *transmute::<_, *mut *mut usize>(crate::util::get_abs_addr(
                crate::scan::find_first_in_module(
                    "/client_client.so",
                    "48 8B 0D ? ? ? ? 4C 89 EA"
                ).unwrap() as usize,
                3, 7
            )),
            move_helper: *transmute::<_, *mut *mut usize>(crate::util::get_abs_addr(
                crate::scan::find_first_in_module(
                    "/client_client.so",
                    "00 48 89 3D ? ? ? ? C3"
                ).unwrap() as usize + 1,
                3, 7
            ))
        }
    };
}

pub struct Offsets {
    pub move_data: *mut usize,
    pub move_helper: *mut usize,
}

unsafe impl Sync for Offsets {}
unsafe impl Send for Offsets {}

/// Lock the global netvars map and retreive a netvar.
#[macro_export]
macro_rules! get_netvar_offset {
    ($table: expr, $prop: expr) => {
        *crate::sdk::netvars::OFFSETS
            .lock()
            .unwrap()
            .get(&crate::sdk::netvars::NetvarPair::new($table, $prop))
            .unwrap()
    };
}

pub(crate) use get_netvar_offset;

use super::interfaces;

unsafe fn find_offset(
    table: *mut interfaces::baseclient::CRecvTable,
    table_name: &str,
    netvar_name: &str,
) -> Option<usize> {
    for i in 0..(*table).n_props {
        let prop = &*(*table).p_props.offset(i as isize);

        let cur_var_name = prop.prop_name;
        let cur_var_name = CStr::from_ptr(cur_var_name);
        let cur_var_name = cur_var_name.to_str().ok()?;

        if cur_var_name == netvar_name {
            return Some(prop.offset as usize);
        }

        if !prop.data_table.is_null() {
            let offset = find_offset(prop.data_table, table_name, netvar_name);
            if offset.is_some() {
                return offset.map(|offset| offset + prop.offset as usize);
            }
        }
    }

    None
}

unsafe fn find_netvar_offset(table_name: &str, netvar_name: &str) -> Option<usize> {
    let mut cur = interfaces::INTERFACES.baseclient.get_all_classes();
    loop {
        if cur.is_null() {
            break;
        }

        let cur_table_name = (*(*cur).recv_table).table_name;
        let cur_table_name = CStr::from_ptr(cur_table_name);
        let cur_table_name = cur_table_name.to_str().ok()?;

        if table_name == cur_table_name {
            return find_offset((*cur).recv_table, table_name, netvar_name);
        }

        cur = (*cur).next;
    }

    None
}

pub fn init() {
    log::info!("Initialising Netvars...");

    let mut netvars = OFFSETS.lock().unwrap();

    for (pair, offset) in netvars.iter_mut() {
        *offset = unsafe {
            match find_netvar_offset(pair.first.as_str(), pair.second.as_str()) {
                Some(offset) => offset,
                None => {
                    log::error!("Failed to find netvar {}::{}", pair.first, pair.second);
                    0
                }
            }
        };
        log::info!("{}::{} = 0x{:X}", pair.first, pair.second, *offset);
    }

    log::info!("Initializing DATA...");
    lazy_static::initialize(&DATA);
}
