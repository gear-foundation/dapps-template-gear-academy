#![no_std]

use gstd::{debug, exec, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_interaction_io::{Tamagotchi, TmgAction, TmgEvent};

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
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load()
        .expect("Can't decode the init message");

    debug!("Program was initialized with message {:?}",
        name);

    let tamagotchi = Tamagotchi {
        name: name.clone(),
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 1,
        fed_block: exec::block_height() as u64,
        entertained: 1,
        entertained_block: exec::block_height() as u64,
        slept: 1,
        slept_block: exec::block_height() as u64
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
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
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
        TmgAction::Feed => {
            tmg.fed -= (exec::block_height() as u64 - tmg.fed_block) * HUNGER_PER_BLOCK;
            tmg.fed += FILL_PER_FEED;
            tmg.fed_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Fed, 0).expect("Not fed correctly");
        }
        TmgAction::Entertain => {
            tmg.entertained -= (exec::block_height() as u64 - tmg.entertained_block) * BOREDOM_PER_BLOCK;
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            tmg.entertained_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Entertained, 0).expect("Not entertained correctly");
        }
        TmgAction::Sleep => {
            tmg.slept -= (tmg.slept_block - exec::block_height() as u64) * ENERGY_PER_BLOCK;
            tmg.slept_block += FILL_PER_SLEEP;
            tmg.slept_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Slept, 0).expect("Not slept correctly");
        }
    }
    // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tmg = unsafe {
        TAMAGOTCHI.as_ref().expect("The contract is not initialized")
    };
    msg::reply(tmg, 0).expect("Failed to share state");
}
