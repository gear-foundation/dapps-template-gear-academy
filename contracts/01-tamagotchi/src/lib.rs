#![no_std]

use gstd::{msg, exec, debug, prelude::*};
use tamagotchi_io::{Tamagotchi, TmgAction};

static mut TAMAGOTCHI:Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load()
        .expect("Can't decode an init message");
    debug!("Program was initialized with message {:?}", name);
    unsafe { TAMAGOTCHI = Some(Tamagotchi { name:name,
                                             date_of_birth:exec::block_timestamp()
                                            });
            };
    msg::reply("Initialization OK", 0)
                .expect("Error in sending reply");
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let input_message: TmgAction = msg::load()
        .expect("Error in loading messages");
    let tamagotchi = unsafe { 
            TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    match input_message {
        TmgAction::Name => {
            debug!("Message name: {:?}", &tamagotchi.name);
            msg::reply(&tamagotchi.name, 0)
                .expect("Error in sending reply");
        }
        TmgAction::Age => {
            debug!("Message age: {:?}", &tamagotchi.date_of_birth);
            msg::reply(tamagotchi.date_of_birth, 0)
                .expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
