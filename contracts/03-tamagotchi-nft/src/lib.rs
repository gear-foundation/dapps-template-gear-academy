#![no_std]

#[allow(unused_imports)]
use gstd::{
    prelude::*,
    msg,
    exec
};
use tamagotchi_nft_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let tamagotchi_name: String = msg::load().expect("Error in init message");
    let block_height = blocks_height();
    let new_tamagotchi: Tamagotchi = Tamagotchi {
        name: tamagotchi_name,
        date_of_birth: block_height,
        owner: msg::source(),
        fed: 5000,
        fed_block: block_height,
        entertained: 5000,
        entertained_block: block_height,
        rested: 5000,
        rested_block: block_height,
        approved_account: None
    };
    unsafe {
        TAMAGOTCHI = Some(new_tamagotchi);
    };
    msg::reply("successful initialization!", 0)
        .expect("error in reply");
}

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let type_message: TmgAction = msg::load()
        .expect("error in load message");
    // In each call all tamagotchi fields are updated
    update_tamagotchi_fields();
    let tamagotchi = state_mut();

    match type_message {
        TmgAction::Name => {
            let tamagotchi_name = TmgEvent::Name(String::from(&tamagotchi.name));
            msg::reply(tamagotchi_name, 0)
                .expect("Error in sending tamagotchi name");
        },
        TmgAction::Age => {
            let tamagotchi_age = TmgEvent::Age(tamagotchi.date_of_birth);
            msg::reply(tamagotchi_age, 0)
                .expect("Errorin sending tamagotchi age");
        },
        TmgAction::Feed => {
            update_field(&mut tamagotchi.fed, FILL_PER_FEED);
            msg::reply(TmgEvent::Fed, 0)
                .expect("Error sending tamagotchi variant 'Fed'");
        },
        TmgAction::Play => {
            update_field(&mut tamagotchi.entertained, FILL_PER_ENTERTAINMENT);
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error sending tamagotchi variant 'Entertained'");  
        },
        TmgAction::Sleep => {
            update_field(&mut tamagotchi.rested, FILL_PER_SLEEP);
            msg::reply(TmgEvent::Slept, 0)
                .expect("Error sending tamagotchi variant 'Slept'");  
        },
        TmgAction::Transfer(actor_id) => {
            let source_id = msg::source();
            let mut owner_transfered = false;
            if tamagotchi.owner == source_id {
                tamagotchi.owner = actor_id;
                owner_transfered = true;
            }
            if let Some(approved_account) = tamagotchi.approved_account {
                if approved_account == source_id {
                    tamagotchi.owner = actor_id;
                    owner_transfered = true;
                }
            }
            if owner_transfered {
                msg::reply(TmgEvent::Transferred(actor_id), 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::Approve(actor_id) => {
            let source_id = msg::source();
            if tamagotchi.owner == source_id {
                tamagotchi.approved_account = Some(actor_id);
                msg::reply(TmgEvent::Approved(actor_id), 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::RevokeApproval => {
            let source_id = msg::source();
            if tamagotchi.owner == source_id {
                tamagotchi.approved_account = None;
                msg::reply(TmgEvent::ApprovalRevoked, 0)
                    .expect("Error in sending reply");
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    msg::reply(state_ref(), 0)
        .expect("Failed to share state");
}


fn blocks_height() -> u64 {
    exec::block_height() as u64
}

fn state_ref() -> &'static Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_ref() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_mut() -> &'static mut Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_mut() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() } 
}

fn update_tamagotchi_fields() {
    let state = state_mut();
    let blocks_height = blocks_height();
    state.fed = updated_field_value(
        state.fed,
        state.fed_block,
        HUNGER_PER_BLOCK,
        blocks_height
    );
    state.fed_block = blocks_height;
    
    state.entertained = updated_field_value(
        state.entertained,
        state.entertained_block,
        BOREDOM_PER_BLOCK,
        blocks_height
    );
    state.entertained_block = blocks_height;  
      
    state.rested = updated_field_value(
        state.rested,
        state.rested_block,
        ENERGY_PER_BLOCK,
        blocks_height
    );
    state.rested_block = blocks_height;    
}

fn updated_field_value(field: u64, field_block: u64, value_per_block: u64, blocks_height: u64) -> u64 {
    let total_value_to_rest = (blocks_height - field_block) * value_per_block;
    if field >= total_value_to_rest {
        // If the given value of the tamagotchi is greater than the value to be 
        // subtracted after a certain number of blocks, the update value is
        // returned
        field - total_value_to_rest
    } else {
        // If not, the given value is smaller, causing a negative result, zero
        // is returned instead. 
        0
    }
}

fn update_field(field: &mut u64, increase_value: u64) {
    *field += increase_value;
    *field = *field.min(&mut 10_000);
} 