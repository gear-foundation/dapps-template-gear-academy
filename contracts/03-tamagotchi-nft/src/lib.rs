#![no_std]

#[allow(unused_imports)]
use gstd::{debug, exec, msg, prelude::*};
use tamagotchi_nft_io::{Tamagotchi,TmgEvent, TmgAction};

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
    let init_msg: String = msg::load().expect("Can't decode an init message");

    let tamagotchi = Tamagotchi {
        name: "Ivan".to_string(),
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 1000,
        fed_block: exec::block_height().into(),
        entertained: 1000,
        entertained_block: exec::block_height().into(),
        slept: 1000,
        slept_block: exec::block_height().into(),
        approved_account: None,
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
            msg::reply(TmgEvent::Name(name.to_string()), 0).expect("Error in sending name");
        }
        TmgAction::Age =>{
            msg::reply(TmgEvent::Age(age), 0).expect("Error in sending age");
           
        } 
        TmgAction::Feed => {
            let fed: u64 = _tamagotchi.fed;
            let fed_block: u64 = _tamagotchi.fed_block;
            let current_block: u64 = exec::block_height().into();
            let time_passed: u64 = current_block - fed_block;
            let hunger: u64 = time_passed * HUNGER_PER_BLOCK;
            let current_fed:u64  = fed - hunger;
            let new_fed:u64 = current_fed + FILL_PER_FEED;
           
            let new_fed_block: u64 = current_block;
            _tamagotchi.fed = new_fed;
            _tamagotchi.fed_block = new_fed_block;
            msg::reply(TmgEvent::Fed(new_fed), 0).expect("Error in sending fed");
        }
        TmgAction::Entertain => {
            let entertained: u64 = _tamagotchi.entertained;
           let entertained_block: u64 = _tamagotchi.entertained_block;
            let current_block: u64 = exec::block_height().into();
            let time_passed: u64 = current_block - entertained_block;
            let boredom: u64 = time_passed * BOREDOM_PER_BLOCK;
            let current_entertained: u64 = entertained - boredom;
            let new_entertained: u64 = current_entertained + FILL_PER_ENTERTAINMENT;

            let new_entertained_block: u64 = current_block;
            _tamagotchi.entertained = new_entertained;
            _tamagotchi.entertained_block = new_entertained_block;
            msg::reply(TmgEvent::Entertained(new_entertained), 0).expect("Error in sending entertained");
            
        }
        TmgAction::Sleep => {
            let slept: u64 = _tamagotchi.slept;
            let slept_block: u64 = _tamagotchi.slept_block;
            let current_block: u64 = exec::block_height().into();
            let time_passed: u64 = current_block - slept_block;
            let energy: u64 = time_passed * ENERGY_PER_BLOCK;
            let current_slept: u64 = slept - energy;
            let new_slept: u64 = current_slept + FILL_PER_SLEEP;

            let new_slept_block: u64 = current_block;
            _tamagotchi.slept = new_slept;
            _tamagotchi.slept_block = new_slept_block;
            msg::reply(TmgEvent::Slept(new_slept), 0).expect("Error in sending slept");
        
        }
        TmgAction::Transfer(new_owner) => {
            if _tamagotchi.owner == msg::source() {
                _tamagotchi.owner = new_owner;
                msg::reply(TmgEvent::Transfer(new_owner), 0).expect("Error in sending transfer");
                // debug!("Tamagotchi Transfered to account: {:?}", new_owner)
            }else{
                panic!("You are not the owner of this Tamagotchi");}
         
        }
        TmgAction::Approve(account) => {
            if _tamagotchi.owner == msg::source() {
            _tamagotchi.approved_account = Some(account);
            msg::reply(TmgEvent::Approve(account), 0).expect("Error in sending approve");
            // debug!("Approved account: {:?}", account)
            }else{
                panic!("You are not the allowed to approve accounts for this Tamagotchi");}
        }
        TmgAction::RevokeApproval => {
            if _tamagotchi.owner == msg::source() {
            _tamagotchi.approved_account = None;
            msg::reply(TmgEvent::RevokeApproval, 0).expect("Error in sending revoke approval");
            // debug!("Approved account: {:?} has been revoked", _tamagotchi.approved_account)
            }else{
                panic!("You are not the allowed to revoke approval for this Tamagotchi");}
            }
    };
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
