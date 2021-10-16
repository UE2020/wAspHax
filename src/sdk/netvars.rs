use std::collections::HashMap;
use std::sync::Mutex;
use maplit::hashmap;

#[derive(PartialEq, Eq, Hash)]
pub struct NetvarPair {
    first: String,
    second: String
}

impl NetvarPair {
    pub fn new(first: &str, second: &str) -> Self {
        Self {
            first: first.to_owned(),
            second: second.to_owned()
        }
    }
}

lazy_static::lazy_static! {
    pub static ref OFFSETS: Mutex<HashMap<NetvarPair, usize>> = Mutex::new(hashmap!{
        // Entity
        NetvarPair::new("DT_BaseEntity", "m_Collision") => 0,
        NetvarPair::new("DT_BaseEntity", "m_iTeamNum") => 0,
        NetvarPair::new("DT_BaseEntity", "m_bSpotted") => 0,

        // Player
        NetvarPair::new("DT_BasePlayer", "m_vecVelocity[0]") => 0,
        NetvarPair::new("DT_BasePlayer", "m_nTickBase") => 0,
        NetvarPair::new("DT_CSPlayer", "m_iAccount") => 0,
        NetvarPair::new("DT_BasePlayer", "m_iHealth") => 0,
        NetvarPair::new("DT_CSPlayer", "m_fFlags") => 0,
        NetvarPair::new("DT_BasePlayer", "m_aimPunchAngle") => 0,
        NetvarPair::new("DT_BasePlayer", "m_viewPunchAngle") => 0,
        NetvarPair::new("DT_CSPlayer", "m_hActiveWeapon") => 0,
    });
}