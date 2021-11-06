use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::util::{self, c_str};

pub mod baseclient;
pub mod clientmode;
pub mod debugoverlay;
pub mod engine;
pub mod entitylist;
pub mod modelinfo;
pub mod panel;
pub mod surface;
pub mod vgui;
pub mod enginetrace;

type InstantiateInterfaceFn = unsafe extern "C" fn() -> *mut usize;

#[allow(non_snake_case)]
#[repr(C)]
struct InterfaceReg {
    m_CreateFn: InstantiateInterfaceFn,
    m_pName: *const libc::c_char,
    m_pNext: *mut InterfaceReg,
}

pub fn get_interface<T>(file: &str, name: &str, include_version: bool) -> *mut T {
    log::debug!("get_interface({}, {}, {})", file, name, include_version);
    let file = CString::new(file).unwrap();
    unsafe {
        let lib = libc::dlopen(
            file.as_ptr(),
            libc::RTLD_NOLOAD | libc::RTLD_NOW | libc::RTLD_LOCAL,
        );
        if !lib.is_null() {
            let temp = c_str!("s_pInterfaceRegs").as_ptr();
            let interface_reg =
                *transmute::<*mut libc::c_void, *mut *mut InterfaceReg>(libc::dlsym(lib, temp));

            let c_name = CString::new(name).unwrap();

            let mut cur = interface_reg;
            loop {
                if cur.is_null() {
                    break;
                }

                if (!libc::strstr((*cur).m_pName, c_name.as_ptr()).is_null()
                    && libc::strlen((*cur).m_pName) - 3 == libc::strlen(c_name.as_ptr()))
                    || (include_version
                        && (!libc::strstr((*cur).m_pName, c_name.as_ptr()).is_null()
                            && libc::strlen((*cur).m_pName) == libc::strlen(c_name.as_ptr())))
                {
                    let iface: *mut T = transmute::<*mut usize, *mut T>(((*cur).m_CreateFn)());
                    log::debug!(
                        "{} ({:?}) {:p}",
                        name,
                        CStr::from_ptr((*cur).m_pName),
                        iface
                    );
                    return iface;
                }

                cur = (*cur).m_pNext;
            }
        }
        libc::dlclose(lib);
    }

    panic!("Failed to get interface");
}

pub struct Interfaces {
    pub baseclient: baseclient::CBaseClient,
    pub entitylist: entitylist::CEntityList,
    pub surface: surface::CSurface,
    pub panel: panel::CPanel,
    pub debug_overlay: debugoverlay::CDebugOverlay,
    pub engine: engine::CEngineClient,
    pub vgui: vgui::CEngineVGui,
    pub modelinfo: modelinfo::CModelInfo,
    pub clientmode: clientmode::CClientMode,
    pub trace: enginetrace::CEngineTrace,
}

unsafe impl Send for Interfaces {}
unsafe impl Sync for Interfaces {}

lazy_static::lazy_static! {
    pub static ref INTERFACES: Interfaces = unsafe {
        log::info!("Lazy loading interfaces...");
        let client = baseclient::CBaseClient::from_raw(get_interface(
            "./csgo/bin/linux64/client_client.so",
            "VClient",
            false,
        ));
        Interfaces {
            baseclient: client,
            entitylist: entitylist::CEntityList::from_raw(get_interface(
                "./csgo/bin/linux64/client_client.so",
                "VClientEntityList",
                false,
            )),
            surface: surface::CSurface::from_raw(get_interface(
                "./bin/linux64/vguimatsurface_client.so",
                "VGUI_Surface",
                false,
            )),
            panel: panel::CPanel::from_raw(get_interface(
                "./bin/linux64/vgui2_client.so",
                "VGUI_Panel",
                false,
            )),
            debug_overlay: debugoverlay::CDebugOverlay::from_raw(get_interface(
                "./bin/linux64/engine_client.so",
                "VDebugOverlay",
                false,
            )),
            engine: engine::CEngineClient::from_raw(get_interface(
                "./bin/linux64/engine_client.so",
                "VEngineClient",
                false,
            )),
            vgui: vgui::CEngineVGui::from_raw(get_interface(
                "./bin/linux64/engine_client.so",
                "VEngineVGui",
                false,
            )),
            modelinfo: modelinfo::CModelInfo::from_raw(get_interface(
                "./bin/linux64/engine_client.so",
                "VModelInfoClient",
                false,
            )),
            clientmode: {
                let hudprocessinput = util::get_virtual_table(client.base as *mut libc::c_void, 0);
                log::debug!("hudprocessinput: {:p}", hudprocessinput);
                let hudprocessinput = transmute::<_, usize>(*hudprocessinput.offset(10));
                log::debug!("hudprocessinput after offset: 0x{:x}", hudprocessinput);

                let get_client_mode = util::get_abs_addr(hudprocessinput + 11, 1, 5);
                log::debug!("get_client_mode: 0x{:x}", get_client_mode);

                type Fn = unsafe extern "C" fn() -> *mut usize;
                let get_client_mode = transmute::<_, Fn>(get_client_mode);
                log::debug!("Calling get_client_mode()...");
                let result = clientmode::CClientMode::from_raw(get_client_mode());
                log::debug!("get_client_mode() called, resulting pointer is: {:p}", get_client_mode());
                result
            },

            trace: enginetrace::CEngineTrace::from_raw(get_interface(
                "./bin/linux64/engine_client.so",
                "EngineTraceClient",
                false,
            )),
        }
    };
}
