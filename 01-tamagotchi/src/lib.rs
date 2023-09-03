#![no_std]

use gstd::{exec, msg, prelude::*};
use tmg1_io::{Tamagotchi, TmgAction};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Can't load init message");

    unsafe { TAMAGOTCHI = Some(
        Tamagotchi {
            name,
            date_of_birth: exec::block_timestamp() 
        })
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Error in loading Tamagotchi Action");
    let tmg = unsafe { TAMAGOTCHI.as_ref().unwrap() };

    let _ = match action {
        TmgAction::Name => msg::reply(tmg.name.clone(), 0),
        TmgAction::Age => msg::reply(exec::block_timestamp() - tmg.date_of_birth, 0),
    };
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(unsafe { TAMAGOTCHI.clone().unwrap() }, 0).expect("Unable to return Tamagotchi instance");
}
