#![no_std]

use gmeta::{In, InOut, Metadata as GMetadata, Out};
use gstd::{
    collections::{BTreeMap, BTreeSet},
    prelude::*,
    ActorId,
};

pub type AttributeId = u32;
pub type Price = u128;
pub type TamagotchiId = ActorId;
pub type TransactionId = u64;

pub struct ProgramMetadata;

impl GMetadata for ProgramMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<StoreAction, StoreEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<AttributeStore>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AttributeStore {
    pub admin: ActorId,
    pub ft_contract_id: ActorId,
    pub attributes: BTreeMap<AttributeId, (AttrMetadata, Price)>,
    pub owners: BTreeMap<TamagotchiId, BTreeSet<AttributeId>>,
    pub transaction_id: TransactionId,
    pub transactions: BTreeMap<TamagotchiId, (TransactionId, AttributeId)>,
}

#[derive(Encode, Decode, Clone, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct AttrMetadata {
    pub title: String,
    pub description: String,
    pub media: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreAction {
    CreateAttribute {
        attribute_id: AttributeId,
        attr_metadata: AttrMetadata,
        price: Price,
    },
    BuyAttribute {
        attribute_id: AttributeId,
    },
    GetAttributes {
        tamagotchi_id: TamagotchiId,
    },
    SetFtContractId {
        ft_contract_id: ActorId,
    },
    RemoveTx {
        tamagotchi_id: TamagotchiId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreEvent {
    AttributeCreated { attribute_id: AttributeId },
    AttributeSold { success: bool },
    Attributes { attributes: BTreeSet<AttributeId> },
    CompletePrevTx { attribute_id: AttributeId },
    FtContractIdSet { ft_contract_id: ActorId },
    TxRemoved { tamagotchi_id: ActorId },
}
