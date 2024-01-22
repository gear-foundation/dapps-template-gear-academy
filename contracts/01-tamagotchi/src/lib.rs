#![no_std]


#[allow(unused_imports)]
use gstd::{ActorId, msg, prelude::*, debug};
use tamagotchi_io::TmgEvent::Name;
use tamagotchi_io::TmgEvent::Age;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    Name,
    Age,
}
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 3️⃣ Add `Name` and `Age` events that return the name and age
   Name(String),
   Age(u64),
}


#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Can't decode an init message");
    debug!("Program was initialized with message {:?}",
    name);
    let date_of_birth: u64 = msg::load().expect("Can't decode age message");
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions

    let action: TmgAction = msg::load().expect("Can't decode an action message");
    // 

    let event = match action {
        TmgAction::Name => {
            msg::reply(Name(String::from("Ivan")), 0).expect("Error in sending name");
        }
        TmgAction::Age =>{
            msg::reply(Age(10), 0).expect("Error in sending age");
           
        } 
    };
    
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
}
