#![no_std]

use gmeta::{In, InOut, Metadata as GMetadata};
use gstd::{prelude::*, ActorId};
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
    type State = AttributeStore;
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct AttributeStore {
    admin: ActorId,
    ft_contract_id: ActorId,
    attributes: BTreeMap<AttributeId, (AttrMetadata, Price)>,
    owners: BTreeMap<TamagotchiId, BTreeSet<AttributeId>>,
    transaction_id: TransactionId,
    transactions: BTreeMap<TamagotchiId, (TransactionId, AttributeId)>,
}

#[derive(Encode, Decode, Clone, TypeInfo, Debug)]
pub struct AttrMetadata {
    pub title: String,
    pub description: String,
    pub media: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
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
pub enum StoreEvent {
    AttributeCreated { attribute_id: AttributeId },
    AttributeSold { success: bool },
    Attributes { attributes: BTreeSet<AttributeId> },
    CompletePrevTx { attribute_id: AttributeId },
    FtContractIdSet { ft_contract_id: ActorId },
    TxRemoved { tamagotchi_id: ActorId },
}
