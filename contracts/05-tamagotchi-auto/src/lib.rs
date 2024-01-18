#![no_std]

#[allow(unused_imports)]
use gstd::{prelude::*, msg, ActorId, exec, Reservation, ReservationId};
use tamagotchi_auto_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// extra state to handle backup gas and delayed messages, 
//separated from the main state so as not to affect the flow of tasks
static mut GAS_RESERVATIONS_HANDLERS: Option<GasReservationHandlers> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let tamagotchi_name: String = msg::load().expect("Error in init message");
    let block_height = blocks_height();
    let new_tamagotchi: Tamagotchi = Tamagotchi {
        name: tamagotchi_name,
        date_of_birth: block_height,
        owner: msg::source(),
        fed: 5000,
        fed_block: block_height,
        entertained: 5000,
        entertained_block: block_height,
        rested: 5000,
        rested_block: block_height,
        approved_account: None,
        ..Tamagotchi::default()
    };
    unsafe {
        TAMAGOTCHI = Some(new_tamagotchi);
        GAS_RESERVATIONS_HANDLERS = Some(
            GasReservationHandlers {
                contract_send_a_delayed_message: false,
                can_send_delayed_message: false
            }
        );
    };
    msg::reply("successful initialization!", 0)
        .expect("error in reply");
}

#[gstd::async_main]
async fn main() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let type_message: TmgAction = msg::load()
        .expect("error in load message");
    
    let tamagotchi = state_mut();
    let GasReservationHandlers { 
        can_send_delayed_message, 
        contract_send_a_delayed_message 
    } = handlers_state_mut();
    let caller = msg::source();
    
    match type_message {
        TmgAction::Name => {
            let tamagotchi_name = TmgEvent::Name(String::from(&tamagotchi.name));
            msg::reply(tamagotchi_name, 0)
                .expect("Error in sending tamagotchi name");
        },
        TmgAction::Age => {
            let tamagotchi_age = TmgEvent::Age(blocks_height() - tamagotchi.date_of_birth);
            msg::reply(tamagotchi_age, 0)
                .expect("Errorin sending tamagotchi age");
        },
        TmgAction::Feed => {
            tamagotchi.feed();
            msg::reply(TmgEvent::Fed, 0)
                .expect("Error sending tamagotchi variant 'Fed'");
        },
        TmgAction::Play => {
            tamagotchi.play(); 
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error sending tamagotchi variant 'Entertained'");  
        },
        TmgAction::Sleep => {
            tamagotchi.sleep();
            msg::reply(TmgEvent::Slept, 0)
                .expect("Error sending tamagotchi variant 'Slept'");  
        },
        TmgAction::Transfer(actor_id) => {
            if tamagotchi.is_owner_or_approved(&caller) {
                tamagotchi.owner = actor_id;
                msg::reply(TmgEvent::Transferred(actor_id), 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::Approve(actor_id) => {
            if tamagotchi.owner == caller {
                tamagotchi.approved_account = Some(actor_id);
                msg::reply(TmgEvent::Approved(actor_id), 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::RevokeApproval => {
            if tamagotchi.owner == caller {
                tamagotchi.approved_account = None;
                msg::reply(TmgEvent::ApprovalRevoked, 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::SetFTokenContract(contract_id) => {
            if tamagotchi.is_owner_or_approved(&caller) {
                tamagotchi.ft_contract_id = contract_id;
                msg::reply(TmgEvent::FTokenContractSet, 0)
                    .expect("Error in sending reply");
            }
        },
        TmgAction::ApproveTokens {
            account,
            amount
        } => {
            if !tamagotchi.is_owner_or_approved(&caller) {
                return;
            }
            tamagotchi.approve_tokens(account, amount).await;
        },
        TmgAction::BuyAttribute {
            store_id,
            attribute_id
        } => {
            if !tamagotchi.is_owner_or_approved(&caller) {
                return;
            }
            tamagotchi.buy_attribute(store_id, attribute_id).await;                                                   
        },  
        // TODO; 6️⃣ Add handling new actions        
        TmgAction::CheckState => {
            let payload;
            
            // this only check the state of the tamagotchi, does not change
            // the state of the contract
            
            let blocks_height = blocks_height();
            if tamagotchi.updated_feed_value(blocks_height) == 1 {
                // tamagotchi.send_message(TmgEvent::FeedMe);
                payload = TmgEvent::FeedMe;
            } else if tamagotchi.updated_play_value(blocks_height) == 1 {
                //tamagotchi.send_message(TmgEvent::PlayWithMe);
                payload = TmgEvent::PlayWithMe;
            } else if tamagotchi.updated_sleep_value(blocks_height) == 1 {
                // tamagotchi.send_message(TmgEvent::WantToSleep);
                payload = TmgEvent::WantToSleep;
            } else {
                payload = TmgEvent::AllGood;
            }
            
            // If the address is not the same as that of the contract, 
            // only the payload obtained is forwarded
            if exec::program_id() != caller {
                msg::reply(payload, 0)
                    .expect("Error in reply");
                return;
            }
            
            
            if tamagotchi.reservations.len() == 1 {
                // If there is only one gas reserve left, it is used to 
                // notify the owner to make more gas reserves.
                *can_send_delayed_message = false;
                *contract_send_a_delayed_message = false;
                tamagotchi.send_delayed_make_reservation_message_to_owner();
            } else {
                // If the tamagotchi has needs, it is sent to the user, and 
                // it calls itself again to make a new review.
                if let TmgEvent::AllGood = payload {
                    // A normal message is sent to the owner, since the 
                    // gas that was previously required with the reservation is used.
                    msg::send(tamagotchi.owner, payload, 0)
                        .expect("error sending message");
                }
                tamagotchi.check_state_of_tamagotchi();
            }
        },
        TmgAction::ReserveGas { 
            reservation_amount, 
            duration 
        } => {
            tamagotchi.make_reservation(reservation_amount, duration);
            
            // It is checked that there are three or more reservation IDs, 
            // so that the contract can send the message that it ran out of reserve gas
            if tamagotchi.reservations.len() >= 3 {
                *can_send_delayed_message = true;
            }
            
            // If the contract can already send the message, and it has not yet 
            // sent the message, the message is sent, preventing more than one 
            // message at a time by adding more gas reserves
            if *can_send_delayed_message && !(*contract_send_a_delayed_message) {
                // set to true to prevent more than one delayed message at a time
                *contract_send_a_delayed_message = true;
                tamagotchi.check_state_of_tamagotchi();
            }
            
            msg::reply(TmgEvent::GasReserved, 0)
                .expect("Error in sending a reply");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    msg::reply(state_ref(), 0)
        .expect("Failed to share state");
}

fn state_ref() -> &'static Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_ref() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_mut() -> &'static mut Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_mut() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() } 
}

fn handlers_state_mut() -> &'static mut GasReservationHandlers {
    let state = unsafe { GAS_RESERVATIONS_HANDLERS.as_mut() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() } 
}





    
    /*
    
    // It is verified if there is no longer a gas reservation ID,
    // contract has already sent a delayed message to itself (this is 
    // to prevent the contract from sending more than one message), or
    // o el que llama al contrato no es el contrato en sí (para prevenir 
    // lo mismo que lo anterior) at the same time with delay to itself), 
    // it will not allow the contract to send a message with the use of 
    // gas reserve
    
    if tamagotchi.reservations.is_empty() {
        // we reset the extra state management variables
        // of the struct
        *contract_send_a_delayed_message = false;
        *can_send_delayed_message = false;
        return;
    }
    
    // If the contract has not sent the message with a delay, the automatic 
    //contract check has not been started, and it is not processing 
    // the next call with a delay.
    // And even if the one calling the contract is not the contract itself, 
    // it will not proceed to send the message with a delay.
    if !*contract_send_a_delayed_message ||
       msg::source() != exec::program_id() {
        return;
    }
    
    let Some(reservation_id) = tamagotchi.reservations.pop() else {
        panic!("error when obtaining last reservation id");  
    };
    
    if tamagotchi.reservations.is_empty() {
        msg::send_from_reservation(
            reservation_id, 
            tamagotchi.owner, 
            TmgEvent::MakeReservation, 
            0
        ).expect("Error sending message with reservation");
        return;
    }
    
    msg::send_delayed_from_reservation(
        reservation_id, 
        exec::program_id(), 
        TmgAction::CheckState, 
        0 , 
        DELAY_OF_ONE_MINUTE
    ).expect("Error sending delayed message");
    */