library utils;

dep abi;
dep data_structures;
dep errors;

use abi::NFT;
use data_structures::{Asset, Auction};
use errors::{AccessError, InputError};
use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::*,
    option::*,
    result::*,
    token::{force_transfer_to_contract, transfer_to_output}
};

/// This function takes a seller, an identity, and NFT data and returns true if the identity is
/// approved to transfer or owns the NFT
pub fn approved_for_nft_transfer(identity: Identity, seller: Identity, nft_contract: ContractId, nft_id: u64) -> bool {
    let nft_abi = abi(NFT, nft_contract.value);
    let approved: Option<Identity> = nft_abi.get_approved(nft_id);
    let owner: Option<Identity> = nft_abi.owner_of(nft_id);
    let approved_for_all = nft_abi.is_approved_for_all(seller, identity);

    approved_for_all ||
        (approved.is_none() && identity == approved.unwrap()) ||
        (owner.is_none() && identity == owner.unwrap())
}

/// This function gets called when the reserve price is met and transfers the sell assets.
/// If an amount greater than the reserve is provided, the remainder is returned
pub fn reserve_met(auction: Auction, balance: u64, reserve: u64) -> Auction {
    // Set variables
    let mut mut_auction = auction;
    let sender = sender_identity();
    let sell_nft_id: Option<u64> = auction.sell_asset.nft_id;

    // Update the auction state
    mut_auction.state = 2;
    mut_auction.bidder = Option::Some(sender);
    mut_auction.buy_asset.amount = reserve;

    // Transfer selling asset to sender
    match sell_nft_id {
        Option::Some(u64) => transfer_nft(Identity::ContractId(contract_id()), sender, auction.sell_asset),
        Option::None(u64) => send_tokens(sender, auction.sell_asset),
    };

    // Return any amount overpaid
    let overpaid_balance = balance - reserve;
    if (overpaid_balance > 0) {
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

/// This function will return the identity of the sender
pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

/// This function will send tokens to the idenitity provided
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

/// This function will transfer a NFT from one identity to another
///
/// # Panics
///
/// This function will panic when:
/// - The NFT transfer failed
pub fn transfer_nft(from: Identity, to: Identity, asset: Asset) {
    let nft_abi = abi(NFT, asset.contract_id.value);
    let nft_id: Option<u64> = asset.nft_id;
    nft_abi.transfer_from(from, to, nft_id.unwrap());

    let owner: Option<Identity> = nft_abi.owner_of(nft_id.unwrap());
    require(owner.is_some() && owner.unwrap() == to, AccessError::NFTTransferNotApproved);
}

/// This function will panic when the recieving assets in a tansaction are incorrect
///
/// # Panics
/// 
/// This function will panic when:
/// - The contract IDs in the buy_asset and recieved_asset are different
/// - The auction contract is not approved to transfer the NFT specified in recieved_asset struct
/// - The transaction asset is not the same as the buy_asset
/// - The transaction asset is not the same as the recieved_asset 
pub fn validate_corrent_asset(buy_asset: Asset, recieved_asset: Asset) {
    let nft_id = recieved_asset.nft_id;
    let sender = sender_identity();

    match nft_id {
        // Depositing a NFT
        Option::Some(u64) => {
            // This is the correct NFT and the auction contract can transfer 
            require(recieved_asset.contract_id == buy_asset.contract_id, InputError::IncorrectAssetProvided);
            require(
                approved_for_nft_transfer(
                    Identity::ContractId(contract_id()), 
                    sender, 
                    recieved_asset.contract_id, 
                    nft_id.unwrap()
                ), 
                AccessError::NFTTransferNotApproved
            );
        },
        // Depositing a token
        Option::None(u64) => {
            require(
                msg_asset_id() == buy_asset.contract_id &&
                msg_asset_id() == recieved_asset.contract_id,
                InputError::IncorrectAssetProvided
            );
        }
    };
}
