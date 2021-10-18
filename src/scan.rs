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

pub fn get_pattern_data(pattern: &str) -> Vec<(u8, bool)> {
    let mut buf = Vec::new();
    for pattern in pattern.split(' ') {
        buf.push(if pattern == "?" || pattern == "??" {
            (0, true)
        } else {
            (u8::from_str_radix(pattern, 16).unwrap(), false)
        });
    }
    
    buf
}

pub fn compare_bytes(addr: *mut u8, pattern_data: &[(u8, bool)]) -> bool {
    for (i, pattern) in pattern_data.iter().enumerate() {

        if pattern.1 { continue }

        unsafe {
            let value = addr.offset(i as isize);
            if *value != pattern.0 {
                return false;
            }
        }
    }
    
    true
}

pub fn find_matches(pattern: &str, addr: *mut u8, size: usize) -> Vec<*mut u8> {
    let pattern_data = get_pattern_data(pattern);
    let first_byte = pattern_data.first().unwrap();

    if first_byte.1 {
        log::error!("First pattern byte cannot be ?? or ?");
        panic!();
    }

    if size < pattern_data.len() {
        log::error!("Pattern size can't be greater than scan size");
        panic!();
    }

    let mut data = Vec::new();
    for i in 0..=(size - pattern_data.len()) {
        unsafe {
            let value = addr.offset(i as isize);
            if *value == first_byte.0 && compare_bytes(value, &pattern_data) {
                data.push(value);
            }
        }
    }

    data
}