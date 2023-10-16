#![no_std]

use gmeta::{Metadata, In, Out, InOut};
use gstd::{prelude::*, msg, ActorId, exec};

use sharded_fungible_token_io::{ FTokenAction, FTokenEvent, LogicAction };
use store_io::{ StoreAction, StoreEvent };

pub type TransactionId = u64;
pub type AttributeId = u32;

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
    pub approved_account: Option<ActorId>,    
        
    // TODO: 2️⃣ Add new fields
    pub ft_contract_id: ActorId,
    pub transaction_id: TransactionId,
    // Check pending transaction
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
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
    
    pub fn is_owner_or_approved(&self, user: &ActorId) -> bool {
        if self.owner == *user {
            return true;
        }
        if self.approved_account == Some(*user) {
            return true;
        }
        false
    }
    
    pub async fn buy_attribute(&mut self, store_id: ActorId, attribute_id: AttributeId) {
        let response = msg::send_for_reply_as::<_, StoreEvent>(
            store_id,
            StoreAction::BuyAttribute {
                attribute_id
            },
            0,
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await
        .expect("Error in decoding 'FTokenEvent'");
        msg::reply(response, 0)
            .expect("Error in sending reply 'StoreEvent' event");
    }
    
    pub async fn approve_tokens(&mut self, account: ActorId, amount: u128) {
        let (transaction_id, account, amount) = if let Some((prev_transaction_id, prev_account, prev_amount)) = self.approve_transaction {
            if prev_account != account && prev_amount != amount {
                msg::reply(TmgEvent::ApprovalError, 0)
                    .expect("Error in sending a reply `TmgEvent::ApprovalError`");
                return;
            }
            (prev_transaction_id, prev_account, prev_amount)
        } else {
            let current_transaction_id = self.transaction_id;
            self.transaction_id = self.transaction_id.wrapping_add(1);
            self.approve_transaction = Some((current_transaction_id, account, amount));
            (current_transaction_id, account, amount)
        };
        
        let result_transaction = msg::send_for_reply_as::<_, FTokenEvent>(
            self.ft_contract_id,
            FTokenAction::Message {
                transaction_id: transaction_id,
                payload: LogicAction::Approve {
                    approved_account: account,
                    amount,
                },
            },
            0,
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await
        .expect("Error in decoding 'FTokenEvent'");
        
        if result_transaction != FTokenEvent::Ok {
            msg::reply(TmgEvent::ApprovalError, 0)
                .expect("Error in sending a reply `TmgEvent::ApprovalError`");
            return;
        }
        
        let response = TmgEvent::TokensApproved { 
            account, 
            amount
        };
        msg::reply(response, 0)
            .expect("Error in sending a reply `TmgEvent::ApprovalError`");
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
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    // TODO: 3️⃣ Add new actions
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },  
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
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    // TODO: 4️⃣ Add new events
    FTokenContractSet, //
    TokensApproved { account: ActorId, amount: u128 }, //
    ApprovalError, //
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
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

