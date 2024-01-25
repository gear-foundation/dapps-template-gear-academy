#![no_std]


#[allow(unused_imports)]
use gstd::{ActorId, msg, prelude::*, debug, exec};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};


static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let tamagochi: Tamagotchi = Tamagotchi {
        name: String::from("Ivan"),
        date_of_birth: 45,
    };
    let init_msg: String = msg::load().expect("Can't decode an init message");

    let tamagotchi = Tamagotchi {
        name: "Ivan".to_string(),
        date_of_birth: exec::block_timestamp(),
    };
    debug!(
        "The Tamagotchi Program was initialized with name {:?} and birth date {:?}",
        tamagotchi.name, tamagotchi.date_of_birth
    );
    unsafe { TAMAGOTCHI = Some(tamagotchi) };

   
    debug!("Program was initialized with message {:?}",
    init_msg);
    let block = exec::block_timestamp();
    debug!("Current block timestamp is {}", block);
   
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
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
    };
    
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
