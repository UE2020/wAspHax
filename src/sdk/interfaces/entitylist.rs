use std::mem::*;

use crate::util::get_virtual_function;

/// A wrapper over the IEntityList interface.
#[derive(Debug)]
pub struct CEntityList {
    base: *mut usize,
}

type GetEntityByIdFn = unsafe extern "thiscall" fn(thisptr: *mut usize, id: i32) -> *mut usize;
type GetHighestEntityIndexFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;
type GetEntityFromHandleFn =
    unsafe extern "thiscall" fn(thisptr: *mut usize, unknown: *mut usize) -> *mut usize;

impl CEntityList {
    /// Creates a new CEntityList wrapper.
    /// # Safety
    /// The pointer must be a valid IEntityList pointer.
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self { base: addr }
    }

    /// Gets the entity with the specified index.
    pub fn get_client_entity(&self, id: i32) -> *mut usize {
        unsafe {
            let func = transmute::<_, GetEntityByIdFn>(get_virtual_function(self.base, 3));
            func(self.base, id)
        }
    }

    /// Returns the highest entity index.
    pub fn get_highest_entity_idx(&self) -> i32 {
        unsafe {
            transmute::<_, GetHighestEntityIndexFn>(get_virtual_function(self.base, 6))(self.base)
        }
    }

    /// Get an entity from the given handle.
    /// # Safety
    /// This function is unsafe because it does not check if the handle is valid.
    pub unsafe fn get_client_entity_from_handle(&self, handle: *mut usize) -> *mut usize {
        transmute::<_, GetEntityFromHandleFn>(get_virtual_function(self.base, 4))(self.base, handle)
    }
}

impl Default for CEntityList {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}
