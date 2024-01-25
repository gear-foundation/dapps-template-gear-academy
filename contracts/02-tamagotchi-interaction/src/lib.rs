#![no_std]

#[allow(unused_imports)]
use gstd::prelude::*;

use tamagotchi_interaction_io::TmgAction;
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

// TODO: 4️⃣ Define constants
const HUNGER_PER_BLOCK: u32 = 1;
const BOREDOM_PER_BLOCK: u32 = 2;
const ENERGY_PER_BLOCK: u32 = 2;
const FILL_PER_FEED: u32 = 1000;
const FILL_PER_ENTERTAINMENT: u32 = 1000;
const FILL_PER_SLEEP: u32 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
 
    let init_msg: String = msg::load().expect("Can't decode an init message");

    let tamagotchi = Tamagotchi {
        name: "Ivan".to_string(),
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 1000,
        fed_block: exec::block_height(),
        entertained: 1000,
        entertained_block: exec::block_height(),
        slept: 1000,
        slept_block: exec::block_height(),
    };
    debug!(
        "The Tamagotchi Program was initialized with name {:?}, birth date {:?}, owner: {:?}",
        tamagotchi.name, tamagotchi.date_of_birth, tamagotchi.owner
    );
    unsafe { TAMAGOTCHI = Some(tamagotchi) };

}

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let _tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    
    let name = &_tamagotchi.name;
    let current_time = exec::block_timestamp();
    let age = current_time - _tamagotchi.date_of_birth;
    let action: TmgAction = msg::load().expect("Can't decode an action message");
    
    // 

    let _event = match action {
        TmgAction::Name => {
            msg::reply(name, 0).expect("Error in sending name");
        }
        TmgAction::Age =>{
            msg::reply(age, 0).expect("Error in sending age");
           
        } 
        TmgAction::Feed => {
            let fed = _tamagotchi.fed;
            let fed_block = _tamagotchi.fed_block;
            let current_block = exec::block_height();
            let time_passed = current_block - fed_block;
            let hunger = time_passed * HUNGER_PER_BLOCK;
            let current_fed = fed - hunger;
            let new_fed = current_fed + FILL_PER_FEED;
           
            let new_fed_block = current_block;
            _tamagotchi.fed = new_fed;
            _tamagotchi.fed_block = new_fed_block;
            msg::reply(new_fed, 0).expect("Error in sending fed");
        }
        TmgAction::Entertain => {
            let entertained = _tamagotchi.entertained;
           let entertained_block = _tamagotchi.entertained_block;
            let current_block = exec::block_height();
            let time_passed = current_block - entertained_block;
            let boredom = time_passed * BOREDOM_PER_BLOCK;
            let current_entertained = entertained - boredom;
            let new_entertained = current_entertained + FILL_PER_ENTERTAINMENT;

            let new_entertained_block = current_block;
            _tamagotchi.entertained = new_entertained;
            _tamagotchi.entertained_block = new_entertained_block;
            msg::reply(new_entertained, 0).expect("Error in sending entertained");
            
        }
        TmgAction::Sleep => {
            let slept = _tamagotchi.slept;
            let slept_block = _tamagotchi.slept_block;
            let current_block = exec::block_height();
            let time_passed = current_block - slept_block;
            let energy = time_passed * ENERGY_PER_BLOCK;
            let current_slept = slept - energy;
            let new_slept = current_slept + FILL_PER_SLEEP;

            let new_slept_block = current_block;
            _tamagotchi.slept = new_slept;
            _tamagotchi.slept_block = new_slept_block;
            msg::reply(new_slept, 0).expect("Error in sending slept");
        
        }
    };
    // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
