#![no_std]

#[allow(unused_imports)]
use gstd::{prelude::*, debug, exec, msg};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};



static mut STATE: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
   let name: String = msg::load().expect("Cant decode the tamagotchi name");

   let tamagotchi = Tamagotchi {
        name,
        date_of_birth: exec::block_timestamp(),
   };

   debug!("Program was initialized with Tamagotchi {:?}", tamagotchi);
   unsafe {STATE = Some(tamagotchi)};
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode incoming TmgAction");

    let state = unsafe {
        STATE.as_ref().expect("The contract is not initialized")
    };

    match action {
        TmgAction::Name => {
            debug!("Tamagotchi: Name Requested");
            // Directly reply with the name, as TmgAction::Name does not carry data
            msg::reply(TmgEvent::Name(state.name.clone()), 0)
                .expect("Error handling Tamagotchi name");
        }
        TmgAction::Age => {
            debug!("Tamagotchi: Age Requested");
            // Directly reply with the date of birth, as TmgAction::Age does not carry data
            msg::reply(TmgEvent::Age(state.date_of_birth), 0)
                .expect("Error handling Tamagotchi age");
        }
    }
}



#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tomagotchi = unsafe {
        STATE
            .as_ref()
            .expect("The contract is not initialized")
    };
    if let Err(e) = msg::reply(tomagotchi, 0) {
        debug!("Failed to send state: {:?}", e);
    }
}
