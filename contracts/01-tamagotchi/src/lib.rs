#![no_std]
use gstd::{msg, prelude::*, exec};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Failed to decode Tamagotchi name");
    let age = exec::block_timestamp();

    let tmg = Tamagotchi {
        name, 
        age,
    };

    unsafe {
        TAMAGOTCHI = Some(tmg);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.age;
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in a reply `TmgEvent::Age`");
        }
    };
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Error in taking current state") };
    msg::reply(tmg, 0).expect("Failed to reply state");
}