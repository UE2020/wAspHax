#![feature(abi_thiscall)]
#![feature(const_raw_ptr_deref)]
// #![feature(untagged_unions)] Not supported anymore

use ctor::*;
use std::thread;
use std::time::Duration;

use std::ffi::CString;

pub mod scan;
pub mod sdk;
pub mod util;

pub mod aimbot;

use log::*;
use simplelog::*;

use std::fs::File;

pub struct ApplicationState {
    world_to_screen_matrix: sdk::vector::VMatrix,
    screen_size_x: i32,
    screen_size_y: i32,
    send_packet: *mut bool,
    fov: f32,
    local_player: sdk::entity::CEntity,
}

fn main_thread() {
    info!("Cheat injected");
    let path = CString::new("./bin/linux64/serverbrowser_client.so").unwrap();
    while unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOLOAD | libc::RTLD_NOW).is_null() } {
        debug!("Game not fully loaded; waiting 100 ms...");
        thread::sleep(Duration::from_millis(100));
    }

    info!("Game loaded");

    thread::sleep(Duration::from_millis(2000));

    // Initialize netvars
    sdk::netvars::init();

    // Initialize hooks
    sdk::vmt::init();
}

#[ctor]
fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("/tmp/csgocheat.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Injected");
    thread::spawn(main_thread);
}

#[dtor]
fn unload() {
    sdk::vmt::cleanup();
    debug!("Unloaded");
}
