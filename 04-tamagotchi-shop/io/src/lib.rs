#![no_std]

use codec::{Decode, Encode};
use gmeta::Metadata;
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    // TODO: 1️⃣ Add new fields
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    // TODO: 2️⃣ Add new actions
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    // TODO: 3️⃣ Add new events
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = ();
    type State = ();
    type Reply = ();
    type Others = ();
    type Signal = ();
}
