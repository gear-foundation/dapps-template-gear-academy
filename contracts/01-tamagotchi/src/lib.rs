#![no_std]

#[allow(unused_imports)]
use gstd::msg;
use gstd::exec;
use gstd::prelude::*;
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load().expect("Failed to load init config");

    let tamagotchi_init = Tamagotchi {
        name: name.clone(),
        date_of_birth: exec::block_timestamp(),
    };

    unsafe {
        TAMAGOTCHI = Some(tamagotchi_init);
    };

}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let action: TmgAction = msg::load().expect("Failed to load action");

    let tamagotchi = unsafe {
        TAMAGOTCHI.get_or_insert(Default::default())
    };

    match action {
        TmgAction::Name => {
            let name = tamagotchi.name.clone();
            msg::reply(TmgEvent::Name(name), 0).expect("Failed to reply name");
        }

        TmgAction::Age => {
            let age = exec::block_timestamp() - tamagotchi.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Failed to reply age");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tamagotchi = unsafe {
        TAMAGOTCHI.take().expect("Unexpected error in taking state")
    };

    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
