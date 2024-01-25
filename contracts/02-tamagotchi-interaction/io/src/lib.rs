#![no_std]

use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
use gmeta::{In, InOut, Metadata, Out};


#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    // TODO: 1️⃣ Add new fields
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    // TODO: 2️⃣ Add new actions
    Feed,
    Entertain,
    Sleep,
}

#[derive( Encode, Decode, TypeInfo,PartialEq, Eq,)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum TmgEvent {
    
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    // TODO: 3️⃣ Add new events
    Fed(u64),
    Entertained(u64),
    Slept(u64),
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
