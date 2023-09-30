#![no_std]

use gmeta::{
    Metadata,
    In,
    InOut,
    Out
};
use gstd::{ prelude::*, ActorId };

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
