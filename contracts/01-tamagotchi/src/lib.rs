#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let initname = msg::load().expect("unable to load name");
    let birthdate = exec::block_timestamp();
    let tmg = Tamagotchi {
        name: initname,
        date_of_birth: birthdate,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let action: TmgAction = msg::load().expect("unable to load action");
    let tmg = unsafe { TAMAGOTCHI.as_mut().expect("TAMAGOTCHI is not initialized") };

    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply'tamagotchi::name'");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply'tamagotchi::age'");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
