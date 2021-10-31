use crate::util::get_virtual_function;
use std::mem::transmute;

#[repr(C)]
pub struct CPlayerInfoDataID {
    lower: i32,
    upper: i32,
}

#[repr(C)]
pub union CPlayerInfoData {
    xuid: i64,
    data: CPlayerInfoDataID,
}

#[repr(C)]
pub struct CPlayerInfo {
    _pad0: i64,
    xuid: CPlayerInfoData,
    name: [i8; 128],
    userid: i32,
    guid: [i8; 33],
    friendsid: u32,
    friendsname: [i8; 128],
    fakeplayer: bool,
    ishltv: bool,
    custom_files: [u32; 4],
    files_downloaded: u8,
}

#[derive(Debug)]
pub struct CEngineClient {
    pub base: *mut usize,
}

impl CEngineClient {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn get_screen_size(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            type Fn = unsafe extern "thiscall" fn(thisptr: *mut usize, x: *mut i32, y: *mut i32);
            transmute::<_, Fn>(get_virtual_function(self.base, 5))(
                self.base,
                &mut x as _,
                &mut y as _,
            );
            (x, y)
        }
    }

    pub fn get_player_info(&self, idx: i32) -> (bool, CPlayerInfo) {
        unsafe {
            let mut info = std::mem::zeroed();
            type Fn = unsafe extern "thiscall" fn(
                thisptr: *mut usize,
                index: i32,
                info: *mut CPlayerInfo,
            ) -> bool;
            let bool = transmute::<_, Fn>(get_virtual_function(self.base, 8))(
                self.base,
                idx,
                &mut info as _,
            );
            (bool, info)
        }
    }

    pub fn get_player_for_user_id(&self, uid: i32) -> i32 {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(thisptr: *mut usize, index: i32) -> i32;
            transmute::<_, Fn>(get_virtual_function(self.base, 9))(self.base, uid)
        }
    }

    pub fn get_local_player(&self) -> i32 {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;
            transmute::<_, Fn>(get_virtual_function(self.base, 12))(self.base)
        }
    }

    pub fn get_max_clients(&self) -> i32 {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;
            transmute::<_, Fn>(get_virtual_function(self.base, 20))(self.base)
        }
    }

    pub fn is_in_game(&self) -> bool {
        unsafe {
            type Fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;
            transmute::<_, Fn>(get_virtual_function(self.base, 26))(self.base)
        }
    }
}

unsafe impl Send for CEngineClient {}

impl Default for CEngineClient {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
