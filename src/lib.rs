#![feature(abi_thiscall)]

use ctor::*;
use std::time::Duration;
use std::{mem::transmute, thread};

use std::ffi::CString;

pub mod scan;
pub mod sdk;
pub mod util;

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
    let path = CString::new("./bin/linux64/serverbrowser_client.so").unwrap();
    while unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_NOLOAD | libc::RTLD_NOW).is_null() } {
        thread::sleep(Duration::from_millis(100));
    }

    debug!("Game loaded");

    // Initialize netvars
    sdk::netvars::init();

    info!("Getting entitylist interface");
    let interface_ptr: *mut usize = sdk::interfaces::get_interface(
        "./csgo/bin/linux64/client_client.so",
        "VClientEntityList",
        false,
    );
    info!("Got entitylist {:p}", interface_ptr);

    unsafe {
        let entitylist = sdk::interfaces::entitylist::CEntityList::from_raw(interface_ptr);
        let entity = entitylist.get_client_entity(3);
        info!("Got entity: {:p}", entity);
    
        let entity = sdk::entity::CEntity::from_raw(entity);
        info!("entity.get_position() = {:?}", *entity.get_origin());
        info!("entity.is_player() = {}", entity.is_player());
        info!("entity.get_health() = {}", entity.get_health());
        info!("entity.get_armor() = {}", entity.get_armor());
        info!("entity.get_armor() = {}", entity.get_armor());
    }
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
    debug!("Unloaded");
}
