#![no_std]

use codec::{Decode, Encode};
use gmeta::Metadata;
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 1️⃣ Add `name` and `age` fields
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 2️⃣ Add `Name` and `Age` actions that set the name and age
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 3️⃣ Add `Name` and `Age` events that return the name and age
}

pub struct ProgramMetadata;

// TODO: 4️⃣ Fill `Init`, `Handle`, and `State` types
impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = ();
    type State = ();
    type Reply = ();
    type Others = ();
    type Signal = ();
}
