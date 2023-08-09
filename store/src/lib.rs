#![no_std]

use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use gstd::{exec, msg, prelude::*, ActorId};
use store_io::{
    AttrMetadata, AttributeId, Price, StoreAction, StoreEvent, TamagotchiId, TransactionId,
};

static mut STORE: Option<AttributeStore> = None;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct AttributeStore {
    admin: ActorId,
    ft_contract_id: ActorId,
    attributes: BTreeMap<AttributeId, (AttrMetadata, Price)>,
    owners: BTreeMap<TamagotchiId, BTreeSet<AttributeId>>,
    transaction_id: TransactionId,
    transactions: BTreeMap<TamagotchiId, (TransactionId, AttributeId)>,
}

impl AttributeStore {
    fn create_attribute(
        &mut self,
        attribute_id: AttributeId,
        metadata: &AttrMetadata,
        price: Price,
    ) {
        assert_eq!(msg::source(), self.admin, "Only admin can add attributes");

        if self
            .attributes
            .insert(attribute_id, (metadata.clone(), price))
            .is_some()
        {
            panic!("Attribute with that ID already exists");
        }

        msg::reply(StoreEvent::AttributeCreated { attribute_id }, 0)
            .expect("Error in sending a reply `StoreEvent::AttributeCreated");
    }
    async fn buy_attribute(&mut self, attribute_id: AttributeId) {
        let (transaction_id, attribute_id) = if let Some((transaction_id, prev_attribute_id)) =
            self.transactions.get(&msg::source())
        {
            // if `prev_attribute_id` is not equal to `attribute_id` then it means that transaction didn`t completed
            // we ask the tamagotchi contract to complete the previous transaction
            if attribute_id != *prev_attribute_id {
                msg::reply(
                    StoreEvent::CompletePrevTx {
                        attribute_id: *prev_attribute_id,
                    },
                    0,
                )
                .expect("Error in sending a reply `StoreEvent::CompletePrevTx`");
                return;
            }
            (*transaction_id, *prev_attribute_id)
        } else {
            let current_transaction_id = self.transaction_id;
            self.transaction_id = self.transaction_id.wrapping_add(1);
            self.transactions
                .insert(msg::source(), (current_transaction_id, attribute_id));
            (current_transaction_id, attribute_id)
        };

        let result = self.sell_attribute(transaction_id, attribute_id).await;
        self.transactions.remove(&msg::source());

        msg::reply(StoreEvent::AttributeSold { success: result }, 0)
            .expect("Error in sending a reply `StoreEvent::AttributeSold`");
    }

    async fn sell_attribute(
        &mut self,
        transaction_id: TransactionId,
        attribute_id: AttributeId,
    ) -> bool {
        let (_, price) = self
            .attributes
            .get(&attribute_id)
            .expect("Can`t get attribute_id");

        if transfer_tokens(
            transaction_id,
            &self.ft_contract_id,
            &msg::source(),
            &exec::program_id(),
            *price,
        )
        .await
        .is_ok()
        {
            self.owners
                .entry(msg::source())
                .and_modify(|attributes| {
                    attributes.insert(attribute_id);
                })
                .or_insert_with(|| [attribute_id].into());
            return true;
        }
        false
    }

    fn get_attributes(&self, tmg_id: &TamagotchiId) {
        let attributes = self.owners.get(tmg_id).unwrap_or(&BTreeSet::new()).clone();
        msg::reply(StoreEvent::Attributes { attributes }, 0)
            .expect("Error in sending a reply `StoreEvent::Attributes`");
    }

    fn set_ft_contract_id(&mut self, ft_contract_id: &ActorId) {
        assert_eq!(
            msg::source(),
            self.admin,
            "Only admin can set fungible token contract"
        );
        self.ft_contract_id = *ft_contract_id;
        msg::reply(
            StoreEvent::FtContractIdSet {
                ft_contract_id: *ft_contract_id,
            },
            0,
        )
        .expect("Error in sending a reply `StoreEvent::FtContractIdSet`");
    }

    fn remove_tx(&mut self, tmg_id: &TamagotchiId) {
        assert_eq!(
            msg::source(),
            self.admin,
            "Only admin can set remove transactions"
        );
        self.transactions.remove(tmg_id);
        msg::reply(
            StoreEvent::TxRemoved {
                tamagotchi_id: *tmg_id,
            },
            0,
        )
        .expect("Error in sending a reply `StoreEvent::TxRemoved`");
    }
}

#[gstd::async_main]
async fn main() {
    let action: StoreAction = msg::load().expect("Unable to decode `StoreAction");
    let store: &mut AttributeStore =
        unsafe { STORE.as_mut().expect("The contract is not initialized") };
    match action {
        StoreAction::CreateAttribute {
            attribute_id,
            attr_metadata,
            price,
        } => store.create_attribute(attribute_id, &attr_metadata, price),
        StoreAction::BuyAttribute { attribute_id } => store.buy_attribute(attribute_id).await,
        StoreAction::GetAttributes { tamagotchi_id } => store.get_attributes(&tamagotchi_id),
        StoreAction::SetFtContractId { ft_contract_id } => {
            store.set_ft_contract_id(&ft_contract_id)
        }
        StoreAction::RemoveTx { tamagotchi_id } => store.remove_tx(&tamagotchi_id),
    }
}

#[no_mangle]
extern "C" fn init() {
    let ft_contract_id: ActorId = msg::load().expect("Unable to decode `ActorId`");
    let store = AttributeStore {
        admin: msg::source(),
        ft_contract_id,
        ..Default::default()
    };
    unsafe { STORE = Some(store) };
}

async fn transfer_tokens(
    transaction_id: TransactionId,
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTokenEvent>(
        *token_address,
        FTokenAction::Message {
            transaction_id,
            payload: LogicAction::Transfer {
                sender: *from,
                recipient: *to,
                amount: amount_tokens,
            },
        },
        0,
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTokenEvent::Ok) => Ok(()),
        _ => Err(()),
    }
}

#[no_mangle]
extern "C" fn state() {
    let store = unsafe { STORE.as_ref().expect("The contract is not initialized") };
    msg::reply(store, 0).expect("Failed to share state");
}
