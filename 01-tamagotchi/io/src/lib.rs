#![no_std]

use codec::{Decode, Encode};
use gmeta::Metadata;
use gstd::prelude::*;
use scale_info::TypeInfo;
use gmeta::{In, InOut};

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
