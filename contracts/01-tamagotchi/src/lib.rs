#![no_std]

#[allow(unused_imports)]
use gstd::{debug, msg, prelude::*};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Can't load init message");
    debug!("Tamagotchi named: {:?}", name);
    let date_of_birth: u64 = gstd::exec::block_timestamp();
    let tmg = Tamagotchi {
        name,
        date_of_birth,
    };
    unsafe { TAMAGOTCHI = Some(tmg) };
}

#[no_mangle]
extern "C" fn handle() {
    let input_msg: TmgAction = msg::load().expect("Error in loading TmgAction");
    let tmg = unsafe { TAMAGOTCHI.as_ref().expect("The contract isn't initialized") };
    match input_msg {
        TmgAction::Name => {
            debug!("Message: Name");
            msg::reply(tmg.name.clone(), 0).expect("Error in sending reply");
        }
        TmgAction::Age => {
            debug!("Message: Age");
            let age = gstd::exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(age, 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let tmg = unsafe { TAMAGOTCHI.as_ref().expect("The contract isn't initialized") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
