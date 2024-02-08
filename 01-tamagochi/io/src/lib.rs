#![no_std]

use gstd::{String, codec::*};
use gmeta::{In, InOut, Metadata, Out};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagochi {
    pub name: String,
    pub date_of_birth: u64
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmAction {
    Name,
    Age
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmEvent {
    Name(String),
    Age(u64)
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmAction, TmEvent>;
    type State = Out<Tamagochi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}