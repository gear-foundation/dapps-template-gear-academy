#![no_std]

use gmeta::{Metadata, In, Out, InOut};
use gstd::{prelude::*, msg, ActorId, exec, ReservationId};

use sharded_fungible_token_io::{ FTokenAction, FTokenEvent, LogicAction };
use store_io::{ StoreAction, StoreEvent };

pub type TransactionId = u64;
pub type AttributeId = u32;

pub const DELAY_OF_ONE_MINUTE: u32 = 20;
pub const HUNGER_PER_BLOCK: u64 = 1;
pub const ENERGY_PER_BLOCK: u64 = 2;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const FILL_PER_SLEEP: u64 = 1000;
pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;

pub struct GasReservationHandlers {
    pub contract_send_a_delayed_message: bool,
    pub can_send_delayed_message: bool
}

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
    pub ft_contract_id: ActorId,
    pub transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
    // TODO: 1️⃣ Add new fields
    pub reservations: Vec<ReservationId>,
}

impl Tamagotchi {   
    pub fn sleep(&mut self) {
        let blocks_height = blocks_height();
        let updated_rested = self.updated_sleep_value(blocks_height);
        self.rested = update_field(updated_rested, FILL_PER_SLEEP);
        self.rested_block = blocks_height;  
    }
    
    pub fn feed(&mut self) {
        let blocks_height = blocks_height();
        let updated_feed = self.updated_feed_value(blocks_height);
        self.fed = update_field(updated_feed, FILL_PER_FEED);
        self.fed_block = blocks_height;
    }
    
    pub fn play(&mut self) {
        let blocks_height = blocks_height();
        let updated_entertainer = self.updated_play_value(blocks_height);
        self.entertained = update_field(updated_entertainer, FILL_PER_ENTERTAINMENT);
        self.entertained_block = blocks_height;  
    }
    
    pub fn updated_sleep_value(&self, blocks_height: u64) -> u64 {
        updated_field_value(
            self.rested,
            self.rested_block,
            ENERGY_PER_BLOCK,
            blocks_height
        )
    }
    
    pub fn updated_feed_value(&self, blocks_height: u64) -> u64 {
        updated_field_value(
            self.fed,
            self.fed_block,
            HUNGER_PER_BLOCK,
            blocks_height
        )
    }
    
    pub fn updated_play_value(&self, blocks_height: u64) -> u64 {
        updated_field_value(
            self.entertained,
            self.entertained_block,
            BOREDOM_PER_BLOCK,
            blocks_height
        )
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
        let (
            transaction_id, 
            account, 
            amount
        ) = if let Some((prev_transaction_id, prev_account, prev_amount)) = self.approve_transaction {
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
        
        let result_transaction: FTokenEvent = msg::send_for_reply_as(
            self.ft_contract_id,
            FTokenAction::Message {
                transaction_id,
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
    
    pub fn make_reservation(&mut self, reservation_amount: u64, reservation_duration: u32) {
        let reservation_id = ReservationId::reserve(
            reservation_amount,
            reservation_duration
        ).expect("reservation across executions");
        
        self.reservations.push(reservation_id);
    }
    
    pub fn check_state_of_tamagotchi(&mut self) {
        let Some(reservation_id) = self.reservations.pop() else {
            panic!("Error getting reservation id"); 
        };
        
        msg::send_delayed_from_reservation(
            reservation_id, 
            exec::program_id(), 
            TmgAction::CheckState, 
            0,
            DELAY_OF_ONE_MINUTE
        ).expect("Error sending message from reservation");
    }
    
    pub fn send_delayed_message_with_reservation_to_owner(&mut self, payload: TmgEvent) {
        self.send_delayed_message_from_reservation(
            self.owner, 
            payload,
            DELAY_OF_ONE_MINUTE
        );
    }
    
    pub fn send_delayed_make_reservation_message_to_owner(&mut self) {
        self.send_delayed_message_from_reservation(
            self.owner, 
            TmgEvent::MakeReservation,
            DELAY_OF_ONE_MINUTE
        );
    }
    
    pub fn send_delayed_message_from_reservation(&mut self, to: ActorId, payload: TmgEvent, delay: u32) {
        let Some(reservation_id) = self.reservations.pop() else {
            panic!("Error getting reservation id"); 
        };
        
        msg::send_delayed_from_reservation(
            reservation_id, 
            to, 
            payload, 
            0,
            delay
        ).expect("Error sending message from reservation");
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
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },  
    // TODO: 2️⃣ Add new actions
    CheckState,
    ReserveGas {
        reservation_amount: u64,
        duration: u32,
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
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
    // TODO: 3️⃣ Add new events
    FeedMe,
    PlayWithMe,
    WantToSleep,
    AllGood, // extra field to return if the user check state
    MakeReservation,
    GasReserved,
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Reply = ();
    type Others = InOut<TmgAction, TmgEvent>;
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
}

pub fn blocks_height() -> u64 {
    exec::block_height() as u64
}

pub fn updated_field_value(field: u64, field_block: u64, value_per_block: u64, blocks_height: u64) -> u64 {
    let total_value_to_rest = (blocks_height - field_block) * value_per_block;
    if field > total_value_to_rest {
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