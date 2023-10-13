#![no_std]

use gmeta::{
    Metadata,
    In,
    InOut,
    Out
};
use gstd::{ prelude::*, ActorId, exec };

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const ENERGY_PER_BLOCK: u64 = 2;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const FILL_PER_SLEEP: u64 = 1000;
pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
    // TODO: 1️⃣ Add new fields
    pub approved_account: Option<ActorId>,    
}

impl Tamagotchi {   
    pub fn sleep(&mut self) {
        let blocks_height = blocks_height();
        let updated_rested = updated_field_value(
            self.rested,
            self.rested_block,
            ENERGY_PER_BLOCK,
            blocks_height
        );
        self.rested = update_field(updated_rested, FILL_PER_SLEEP);
        self.rested_block = blocks_height;  
    }
    
    pub fn feed(&mut self) {
        let blocks_height = blocks_height();
        let updated_feed = updated_field_value(
            self.fed,
            self.fed_block,
            HUNGER_PER_BLOCK,
            blocks_height
        );
        self.fed = update_field(updated_feed, FILL_PER_FEED);
        self.fed_block = blocks_height;
    }
    
    pub fn play(&mut self) {
        let blocks_height = blocks_height();
        let updated_entertainer = updated_field_value(
            self.entertained,
            self.entertained_block,
            BOREDOM_PER_BLOCK,
            blocks_height
        );
        self.entertained = update_field(updated_entertainer, FILL_PER_ENTERTAINMENT);
        self.entertained_block = blocks_height;  
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    // TODO: 2️⃣ Add new actions
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    // TODO: 3️⃣ Add new events
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
}

pub fn blocks_height() -> u64 {
    exec::block_height() as u64
}

pub fn updated_field_value(field: u64, field_block: u64, value_per_block: u64, blocks_height: u64) -> u64 {
    let total_value_to_rest = (blocks_height - field_block) * value_per_block;
    if field >= total_value_to_rest {
        // If the given value of the tamagotchi is greater than the value to be 
        // subtracted after a certain number of blocks, the update value is
        // returned
        field - total_value_to_rest
    } else {
        // If not, the given value is smaller, causing a negative result, one
        // is returned instead. 
        1
    }
}

pub fn update_field(field: u64, increase_value: u64) -> u64 {
    let field = field + increase_value;
    field.min(10_000)
} 

