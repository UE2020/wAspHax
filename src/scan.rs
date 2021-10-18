use std::sync::Mutex;
use std::ffi::CStr;

pub struct Dlinfo {
    pub library: String,
    pub address: libc::uintptr_t,
    pub size: libc::size_t,
}

impl Default for Dlinfo {
    fn default() -> Self {
        Self {
            library: String::new(),
            address: 0,
            size: 0,
        }
    }
}

unsafe impl Send for Dlinfo {}

lazy_static::lazy_static! {
    pub static ref LIBRARIES: Mutex<Vec<Dlinfo>> = Mutex::new(Vec::new());
}

#[allow(non_snake_case)]
unsafe extern "C" fn dl_iterate_phdr__fnptr(info: *mut libc::dl_phdr_info, _: usize, _: *mut libc::c_void) -> libc::c_int {
    let name = CStr::from_ptr((*info).dlpi_name);
    let name = name.to_str().unwrap();
    let name = name.to_owned();
    LIBRARIES.lock().unwrap().push(Dlinfo {
        library: name,
        address: ((*info).dlpi_addr + (*(*info).dlpi_phdr).p_vaddr) as libc::uintptr_t,
        size: (*(*info).dlpi_phdr).p_memsz as libc::size_t
    });
    
    return 0;
}

pub fn get_library_information(library: &str, address: &mut libc::uintptr_t, size: &mut usize) -> bool {
    unsafe {
        let len = LIBRARIES.lock().unwrap().len();
        if len == 0 {
            libc::dl_iterate_phdr(Some(dl_iterate_phdr__fnptr), std::ptr::null_mut());
        }
    }

    let libraries = LIBRARIES.lock().unwrap();
    let mut library = library.to_owned();
    library.make_ascii_lowercase();
    for current in libraries.iter() {
        let mut current_string = current.library.clone();
        current_string.make_ascii_lowercase();

        let find = current_string.find(library.as_str());
        if find.is_none() {
            continue;
        }

        *address = current.address;
        *size = current.size;

        return true;
    }

    false
}