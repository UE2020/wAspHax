use libc::c_void;
use std::mem::{self, transmute};

use super::interfaces;
use super::interfaces::surface::Color;

use super::classes;
use super::entity;

#[allow(non_camel_case_types)]
type intptr = libc::intptr_t;

lazy_static::lazy_static! {
    pub static ref PAGESIZE: i64 = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) };
    pub static ref PAGEMASK: i64 = !(*PAGESIZE-1);
}

/// Unprotect a memory region - the old protection is returned.
pub unsafe fn unprotect(region: *mut c_void) -> i32 {
    libc::mprotect(
        transmute::<intptr, *mut libc::c_void>(region as intptr & (*PAGEMASK) as intptr),
        *PAGESIZE as usize,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    );

    libc::PROT_READ | libc::PROT_EXEC
}

/// Set the protection of a memory region.
pub unsafe fn protect(region: *mut c_void, protection: i32) {
    libc::mprotect(
        transmute::<intptr, *mut libc::c_void>(region as intptr & (*PAGEMASK) as intptr),
        *PAGESIZE as usize,
        protection,
    );
}

/// Replace an offset in a vtable with a new func.
/// To unhook, just hook again, but replace the new func with the original -
/// as the original is returned when you call hook.
pub unsafe fn hook(instance: *mut c_void, hook: *mut c_void, offset: i32) -> *mut c_void {
    let vtable = *(instance as *mut intptr);
    let entry = vtable + mem::size_of::<intptr>() as intptr * offset as intptr;
    let original = *(entry as *mut isize);

    let original_protection = unprotect(entry as *mut c_void);
    *(entry as *mut isize) = hook as isize;
    protect(entry as *mut c_void, original_protection); // reprotect the unprotected region

    original as *mut c_void
}

pub struct Hook {
    pub original: *mut c_void,
    pub hook: *mut c_void,
    pub offset: i32,
    pub instance: *mut c_void,
}

impl Hook {
    pub const fn new(hook: *mut c_void, instance: *mut c_void, offset: i32) -> Self {
        Self {
            original: std::ptr::null_mut(),
            hook: hook as *mut c_void,
            offset,
            instance,
        }
    }

    pub fn hook(&mut self) {
        unsafe {
            self.original = hook(self.instance, self.hook, self.offset);
            log::debug!(
                "hook({:p}, {:p}, 0x{:X})",
                self.instance,
                self.hook,
                self.offset
            );
            log::debug!("self.original = {:p}", self.original);
        }
    }

    /// Unhooking a hook that has not been hooked is UB.
    pub fn unhook(&mut self) {
        unsafe {
            hook(self.instance, self.original, self.offset);
            log::debug!(
                "unhook({:p}, {:p}, 0x{:X})",
                self.instance,
                self.original,
                self.offset
            );
        }
    }
}

unsafe impl Send for Hook {}
unsafe impl Sync for Hook {}

pub struct UnsafeHook(*mut Hook);

unsafe impl Send for UnsafeHook {}
unsafe impl Sync for UnsafeHook {}

lazy_static::lazy_static! {
    pub static ref PAINT_HOOK: UnsafeHook = UnsafeHook(Box::into_raw(Box::new(Hook::new(paint as _, super::interfaces::INTERFACES.vgui.base as *mut c_void, 15))));
    pub static ref ESP_FONT: u64 = super::interfaces::surface::create_font(
        "Andale Mono",
        40,
        0,
        0x80,
    );
}

pub fn init() {
    log::info!("Initializing hooks...");

    unsafe { (*PAINT_HOOK.0).hook() }
}

pub fn cleanup() {
    log::info!("Cleaning up hooks...");
    unsafe { (*PAINT_HOOK.0).unhook() }
}

#[derive(Clone, Copy)]
#[repr(C)]
enum PaintMode {
    PAINT_UIPANELS = (1 << 0),
    PAINT_INGAMEPANELS = (1 << 1),
    PAINT_CURSOR = (1 << 2),
}

type PaintFn = unsafe extern "C" fn(thisptr: *mut usize, paint_mode: PaintMode);

unsafe extern "C" fn paint(thisptr: *mut usize, paint_mode: PaintMode) {
    (transmute::<*mut c_void, PaintFn>((*PAINT_HOOK.0).original))(thisptr, paint_mode);

    if !interfaces::INTERFACES.engine.is_in_game() {
        return;
    }

    if (paint_mode as i32 & PaintMode::PAINT_UIPANELS as i32) != 0 {
        let local_player = interfaces::INTERFACES
            .entitylist
            .get_client_entity(interfaces::INTERFACES.engine.get_local_player());
        if local_player.is_null() {
            return;
        }

        interfaces::surface::draw_text(
            50,
            50,
            "wAspHax v1.58-nightly",
            *ESP_FONT,
            Color::new_rgb(255, 0, 0),
        );
        let max_clients = interfaces::INTERFACES.engine.get_max_clients();
        for i in 1..interfaces::entitylist::get_highest_entity_index() {
            if i < max_clients {
                let entity = entity::CEntity::from_raw(
                    interfaces::INTERFACES.entitylist.get_client_entity(i),
                );
                if entity.is_empty() || entity.base == local_player {
                    continue;
                }

                if entity.get_health() > 0 && !entity.is_dormant() {
                    let head_w2s = interfaces::debugoverlay::world_to_screen(&entity.get_bone_pos(8));
                    let origin_w2s = interfaces::debugoverlay::world_to_screen(&entity.get_origin());
            
                    if !head_w2s.is_some() || !origin_w2s.is_some() {
                        continue;
                    }
            
                    let height: i32 = (origin_w2s.unwrap().y - head_w2s.unwrap().y) as i32;
                    let width = height / 2;
            
                    let x1: i32 = (head_w2s.unwrap().x - (width / 2) as f32) as i32;
                    let y1: i32 = head_w2s.unwrap().y as i32;
                    let w: i32 = width;
                    let h: i32 = height;
            
                    interfaces::surface::draw_box(x1, y1, w, h, Color::new_rgb(255, 0, 0));
                }   
            }
        }
    }
}

/*type PaintTraverseFn = unsafe extern "C" fn(thisptr: *mut usize, panel: u64, force_repaint: bool, allow_force: bool);

unsafe extern "C" fn paint_traverse(thisptr: *mut usize, panel: u64, force_repaint: bool, allow_force: bool) {
    use std::ffi::CStr;

    static mut PANEL_ID: u64 = 0;
    static mut PANEL_HUD_ID: u64 = 0;

    let interfaces = &super::interfaces::INTERFACES;

    if PANEL_HUD_ID == 0 {
        let panel_name = interfaces.panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("HudZoom") {
            PANEL_HUD_ID = panel;
        }
    }

    if PANEL_ID == 0 {
        let panel_name = interfaces.panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("MatSystemTopPanel") {
            PANEL_ID = panel;
        }
    }

    (transmute::<*mut c_void, PaintTraverseFn>((*PAINT_TRAVERSE_HOOK.0).original))(thisptr, panel, force_repaint, allow_force);

    if PANEL_ID == panel {
        super::interfaces::surface::draw_text(50, 50, "wAspHax v1.58-nightly", *ESP_FONT, Color::new_rgb(255, 0, 0));

        for entity in super::interfaces::entitylist::get_all_players() {
            let origin_w2s = super::interfaces::debugoverlay::world_to_screen(&entity.get_origin());

            if !origin_w2s.is_some() {
                continue;
            }

            let height = 200;
            let width = 200;

            let x1: i32 = origin_w2s.unwrap().x as i32;
            let y1: i32 = origin_w2s.unwrap().y as i32;
            let w: i32 = width;
            let h: i32 = height;

            super::interfaces::surface::draw_box(x1, y1, w, h, Color::new_rgb(255, 0, 0));
        }
    }
}*/
