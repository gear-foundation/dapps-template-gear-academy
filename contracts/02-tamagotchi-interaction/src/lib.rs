#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*};
use tamagotchi_interaction_io::*;

// TODO: 4️⃣ Define constants
const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let initname = msg::load().expect("unable to load name");
    let birthdate = exec::block_timestamp();
    let tmg = Tamagotchi {
        name: initname,
        date_of_birth: birthdate,
        owner: Default::default(),
        fed: 1,
        fed_block: 0,
        entertained: 1,
        entertained_block: 0,
        slept: 1,
        slept_block: 0,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
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
        TmgAction::Feed => {
            tmg.fed_block = exec::block_timestamp();
            tmg.fed -= exec::block_height() as u64 * HUNGER_PER_BLOCK;
            tmg.fed += FILL_PER_FEED;
            msg::reply(TmgEvent::Fed, 0).expect("Not fed correctly");
        }
        TmgAction::Entertain => {
            tmg.entertained_block = exec::block_timestamp();
            tmg.entertained -= exec::block_height() as u64 * BOREDOM_PER_BLOCK;
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            msg::reply(TmgEvent::Entertained, 0).expect("Not entertained correctly");
        }
        TmgAction::Sleep => {
            tmg.slept_block = exec::block_timestamp();
            tmg.slept -= exec::block_height() as u64 * ENERGY_PER_BLOCK;
            tmg.slept_block += FILL_PER_SLEEP;
            msg::reply(TmgEvent::Slept, 0).expect("Not slept correctly");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
