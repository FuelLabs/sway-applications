library utils;

dep data_structures;

use data_structures::{Asset, Auction};
use std::{
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    identity::Identity,
    option::*,
    result::*,
    token::{force_transfer_to_contract, transfer_to_output}
};

/// This function will take two identities and return true if they are the same
pub fn identities_equal(identity1: Identity, identity2: Identity) -> bool {
    match identity1 {
        Identity::Address(identity1) => {
            match identity2 {
                Identity::Address(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        },
        Identity::ContractId(identity1) => {
            match identity2 {
                Identity::ContractId(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        }
    }
}

/// This function gets called when the reserve price is met and transfers the sell assets.
/// If an amount greater than the reserve is provided, the remainder is returned
pub fn reserve_met(auction: Auction, balance: u64, reserve: u64) -> Auction {
    let mut mut_auction = auction;
    let sender = sender_identity();
    mut_auction.state = 2;
    mut_auction.bidder = Option::Some(sender);
    mut_auction.buy_asset.amount = reserve;

    match sender {
        Identity::Address(sender) => {
            transfer_to_output(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);    
        },
        Identity::ContractId(sender) => {
            force_transfer_to_contract(auction.sell_asset.amount, auction.sell_asset.contract_id, sender);
        },
    };

    let overpaid_balance = (msg_amount() + balance) - reserve;
    if (overpaid_balance > 0)
    {
        match sender {
            Identity::Address(sender) => {
                transfer_to_output(overpaid_balance, auction.buy_asset.contract_id, sender);    
            },
            Identity::ContractId(sender) => {
                force_transfer_to_contract(overpaid_balance, auction.buy_asset.contract_id, sender);
            },
        };
    }

    mut_auction
}

pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}
