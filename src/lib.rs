use ctor::*;
use std::{thread, mem};
use std::time::Duration;

use std::ffi::CString;

pub mod sdk;

pub struct ApplicationState {
    world_to_screen_matrix: sdk::vector::VMatrix,
    screen_size_x: i32,
    screen_size_y: i32,
    send_packet: *mut bool,
    fov: f32,
}

fn main_thread() {
    let path = CString::new("./bin/linux64/serverbrowser_client.so").unwrap();
    while unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOLOAD | libc::RTLD_NOW).as_ref().is_none() } {
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("Game loaded");

}

#[ctor]
fn main() {
  println!("Injected");
  thread::spawn(main_thread);
}

#[dtor]
fn unload() {
  println!("Unloaded");
}
