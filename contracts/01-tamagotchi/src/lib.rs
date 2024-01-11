#![no_std]

use gstd::{debug, exec, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load()
        .expect("Can't decode the init message");

    debug!("Program was initialized with message {:?}",
        name);

    let tamagotchi = Tamagotchi {
        name: name.clone(),
        date_of_birth: exec::block_timestamp()
    };

    unsafe {
        TAMAGOTCHI = Some(tamagotchi)
    }

    msg::reply(
        TmgEvent::Name(name),
        0
    ).unwrap();
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let input_msg = msg::load().expect("Error in loading Tmg Input Message");
    let tmg = unsafe {
        TAMAGOTCHI.as_mut().expect("The contract is not initialized")
    };
    match input_msg {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0).expect("Name not loaded correctly");
        }
        TmgAction::Age => {
            msg::reply(TmgEvent::Age(tmg.date_of_birth.clone()), 0).expect("Age not loaded correctly");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tmg = unsafe {
        TAMAGOTCHI.as_ref().expect("The contract is not initialized")
    };
    msg::reply(tmg, 0).expect("Failed to share state");
}
