library utils;

dep abi;
dep data_structures;
dep errors;

use abi::NFT;
use data_structures::{Asset, Auction};
use errors::AccessError;
use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    option::*,
    result::*,
    token::{force_transfer_to_contract, transfer_to_output}
};

/// This function takes a seller, an identity, and NFT data and returns true if the identity is
/// approved to transfer or owns the NFT
pub fn approved_for_nft_transfer(identity: Identity, seller: Identity, contract_id: ContractId, nft_id: u64) -> bool {
    let contract_b256 = contract_id().value;
    let nft_abi = abi(NFT, contract_b256);

    let approved: Option<Identity> = nft_abi.get_approved(nft_id);
    let owner: Option<Identity> = nft_abi.owner_of(nft_id);
    let approved_for_all = nft_abi.is_approved_for_all(seller, identity);

    approved_for_all ||
        (approved.is_none() && identities_equal(identity, approved.unwrap())) ||
        (owner.is_none() && identities_equal(identity, owner.unwrap()))
}

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
    let sell_nft_id: Option<u64> = auction.sell_asset.nft_id;

    mut_auction.state = 2;
    mut_auction.bidder = Option::Some(sender);
    mut_auction.buy_asset.amount = reserve;

    match sell_nft_id {
        Option::Some(u64) => transfer_nft(Identity::ContractId(contract_id()), sender, auction.sell_asset),
        Option::None(u64) => send_tokens(sender, auction.sell_asset),
    };

    let overpaid_balance = balance - reserve;
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

pub fn send_tokens(identity: Identity, asset: Asset) {
    match identity {
        Identity::Address(identity) => {
            transfer_to_output(asset.amount, asset.contract_id, identity);    
        },
        Identity::ContractId(identity) => {
            force_transfer_to_contract(asset.amount, asset.contract_id, identity);
        },
    };
}

pub fn transfer_nft(from: Identity, to: Identity, asset: Asset) {
    let nft_abi = abi(NFT, asset.contract_id.value);
    let nft_id: Option<u64> = asset.nft_id;
    nft_abi.transfer_from(from, to, nft_id.unwrap());

    let owner: Option<Identity> = nft_abi.owner_of(nft_id.unwrap());
    require(owner.is_some() && identities_equal(owner.unwrap(), to), AccessError::NFTTransferNotApproved);
}
