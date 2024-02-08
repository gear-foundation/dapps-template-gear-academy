#![no_std]
use gstd::{debug, exec, msg, prelude::*};
use io::{Tamagochi, TmAction, TmEvent};

static mut TAMAGOCHI: Option<Tamagochi> = None;

pub fn tamagochi_mut() -> &'static mut Tamagochi {
    let tamagochi = unsafe { TAMAGOCHI.as_mut() };
    return unsafe { tamagochi.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let tamagochi_name: String = msg::load().expect("Can't decode the init message");

    debug!("The program was initialized with the following tamagochi name: {:?}", tamagochi_name);

    unsafe { 
        TAMAGOCHI = Some(Tamagochi {
            name: tamagochi_name, 
            date_of_birth: exec::block_timestamp() 
        }) 
    }
}

#[no_mangle]
extern "C" fn handle() {
    let message: TmAction = msg::load().expect("Can't decode the incoming message");

    match message {
        TmAction::Name => {
            let _ = msg::reply(TmEvent::Name(tamagochi_mut().name.to_string()), 0);
        },
        TmAction::Age => {
            let _ = msg::reply(TmEvent::Age(tamagochi_mut().date_of_birth), 0);
        },
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = tamagochi_mut();
    let _ = msg::reply(state, 0);
}
