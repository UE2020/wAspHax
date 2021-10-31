use std::ffi::CString;
use std::os::raw::c_int;

use crate::util;

pub enum EFontFlags {
    FontflagNone,
    FontflagItalic = 0x001,
    FontflagUnderline = 0x002,
    FontflagStrikeout = 0x004,
    FontflagSymbol = 0x008,
    FontflagAntialias = 0x010,
    FontflagGaussianblur = 0x020,
    FontflagRotary = 0x040,
    FontflagDropshadow = 0x080,
    FontflagAdditive = 0x100,
    FontflagOutline = 0x200,
    FontflagCustom = 0x400,
    FontflagBitmap = 0x800,
}

#[repr(C)]
pub struct Color {
    r: c_int,
    g: c_int,
    b: c_int,
    a: c_int,
}

impl Color {
    pub fn new_rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new_rgb(r: i32, g: i32, b: i32) -> Self {
        Self { r, g, b, a: 255 }
    }
}

type SetDrawColorFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, r: i32, g: i32, b: i32, a: i32);
type DrawLineFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, x0: i32, y0: i32, x1: i32, y1: i32);
type DrawFilledRectFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, x: i32, y: i32, x1: i32, y1: i32);
type SetTextPosFn = unsafe extern "thiscall" fn(thisptr: *mut usize, x: i32, y: i32);
type SetTextFontFn = unsafe extern "thiscall" fn(thisptr: *mut usize, font: u64);
type SetTextColorFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, r: i32, g: i32, b: i32, a: i32);
type PrintTextFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, text: *const u32, length: i32, unk: i32);
type CreateFontFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> u64;
type SetFontGlyphsetFn = unsafe extern "thiscall" fn(
    thisptr: *mut usize,
    font: u64,
    name: *const libc::c_char,
    tall: i32,
    weight: i32,
    blur: i32,
    scanlines: i32,
    flags: i32,
    unk1: i32,
    unk2: i32,
) -> u64;
type DrawOutlinedRectFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, x: i32, y: i32, x1: i32, y1: i32);

#[derive(Debug)]
pub struct CSurface {
    pub base: *mut usize,
}

impl CSurface {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    pub fn set_draw_color(&self, color: Color) {
        let vfunc = unsafe {
            std::mem::transmute::<_, SetDrawColorFn>(util::get_virtual_function(self.base, 14))
        };
        unsafe {
            vfunc(self.base, color.r, color.g, color.b, color.a);
        }
    }

    pub fn draw_line(&self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let vfunc = unsafe {
            std::mem::transmute::<_, DrawLineFn>(util::get_virtual_function(self.base, 19))
        };
        unsafe {
            vfunc(self.base, x0, y0, x1, y1);
        }
    }

    pub fn set_text_font(&self, font: u64) {
        let vfunc = unsafe {
            std::mem::transmute::<_, SetTextFontFn>(util::get_virtual_function(self.base, 23))
        };
        unsafe {
            vfunc(self.base, font);
        }
    }

    pub fn set_text_color(&self, color: Color) {
        let vfunc = unsafe {
            std::mem::transmute::<_, SetTextColorFn>(util::get_virtual_function(self.base, 24))
        };
        unsafe {
            vfunc(self.base, color.r, color.g, color.b, color.a);
        }
    }

    pub fn print_text(&self, text: &str) {
        let wide_text = widestring::U32CString::from_str(text).unwrap();
        let len = wide_text.len();
        let vfunc = unsafe {
            std::mem::transmute::<_, PrintTextFn>(util::get_virtual_function(self.base, 28))
        };
        unsafe {
            let raw = wide_text.into_raw();
            vfunc(self.base, raw, len as i32, 0);
            drop(widestring::U32CString::from_raw(raw))
        }
    }

    pub fn draw_filled_rect(&self, x: i32, y: i32, x1: i32, y1: i32) {
        let vfunc = unsafe {
            std::mem::transmute::<_, DrawFilledRectFn>(util::get_virtual_function(self.base, 16))
        };
        unsafe {
            vfunc(self.base, x, y, x1, y1);
        }
    }

    pub fn set_text_pos(&self, x: i32, y: i32) {
        let vfunc = unsafe {
            std::mem::transmute::<_, SetTextPosFn>(util::get_virtual_function(self.base, 26))
        };
        unsafe {
            vfunc(self.base, x, y);
        }
    }

    pub fn create_font(&self) -> u64 {
        let vfunc = unsafe {
            std::mem::transmute::<_, CreateFontFn>(util::get_virtual_function(self.base, 71))
        };
        unsafe { vfunc(self.base) }
    }

    pub fn set_font_glyphset(
        &self,
        font: u64,
        font_name: &str,
        tall: i32,
        weight: i32,
        blur: i32,
        scanlines: i32,
        flags: i32,
    ) -> u64 {
        let vfunc = unsafe {
            std::mem::transmute::<_, SetFontGlyphsetFn>(util::get_virtual_function(self.base, 72))
        };
        unsafe {
            let font_name: CString = CString::new(font_name).unwrap();
            vfunc(
                self.base,
                font,
                font_name.as_ptr(),
                tall,
                weight,
                blur,
                scanlines,
                flags,
                0,
                0,
            )
        }
    }

    pub fn draw_outlined_rect(&self, x: i32, y: i32, x1: i32, y1: i32) {
        let vfunc = unsafe {
            std::mem::transmute::<_, DrawOutlinedRectFn>(util::get_virtual_function(self.base, 18))
        };
        unsafe {
            vfunc(self.base, x, y, x1, y1);
        }
    }
}

impl Default for CSurface {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}

pub fn draw_box(x: i32, y: i32, w: i32, h: i32, clr: Color) {
    let interfaces = &super::INTERFACES;

    interfaces.surface.set_draw_color(clr);
    interfaces.surface.draw_outlined_rect(x, y, x + w, y + h);

    interfaces.surface.set_draw_color(Color::new_rgb(0, 0, 0));
    interfaces
        .surface
        .draw_outlined_rect(x - 1, y - 1, x + w + 1, y + h + 1);
    interfaces
        .surface
        .draw_outlined_rect(x + 1, y + 1, x + w - 1, y + h - 1);
}

pub fn draw_text(x: i32, y: i32, text: &str, font: u64, clr: Color) {
    let interfaces = &super::INTERFACES;

    interfaces.surface.set_text_pos(x, y);
    interfaces.surface.set_text_font(font);
    interfaces.surface.set_text_color(clr);
    interfaces.surface.print_text(text);
}

pub fn create_font(name: &str, size: i32, weight: i32, flag: i32) -> u64 {
    let interfaces = &super::INTERFACES;

    let font = interfaces.surface.create_font();
    interfaces
        .surface
        .set_font_glyphset(font, name, size, weight, 0, 0, flag);
    font
}
