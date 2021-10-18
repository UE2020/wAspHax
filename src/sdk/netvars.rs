use maplit::hashmap;
use std::collections::HashMap;
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
}

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
