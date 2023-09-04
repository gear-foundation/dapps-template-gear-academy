#![no_std]

use gstd::{exec, msg, prelude::*};
use tmg1_io::{Tamagotchi, TmgAction, TmgEvent};

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
    let action: TmgAction = msg::load().expect("Error on loading Tamagotchi Action");
    let tmg = unsafe { TAMAGOTCHI.as_ref().unwrap() };

    match action {
        TmgAction::Name => msg::reply(TmgEvent::Name(tmg.name.clone()), 0),
        TmgAction::Age => msg::reply(
            TmgEvent::Age(exec::block_timestamp() - tmg.date_of_birth), 0
        ),
    }
    .expect("Failed replying to sender");
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(unsafe { TAMAGOTCHI.as_ref().unwrap() }, 0).expect("Unable to return Tamagotchi instance");
}
