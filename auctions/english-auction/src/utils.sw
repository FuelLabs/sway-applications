library utils;

dep abi;
dep data_structures;
dep errors;

use abi::NFT;
use data_structures::*;
use errors::{AccessError, InputError};
use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::*,
    option::*,
    result::*,
    revert::revert,
    token::{force_transfer_to_contract, transfer_to_output}
};

/// This function takes a seller, an identity, and NFT data and returns true if the identity is
/// approved to transfer or owns the NFT
pub fn approved_for_nft_transfer(identity: Identity, seller: Identity, nft_contract: ContractId, nft_id: u64) -> bool {
    let nft_abi = abi(NFT, nft_contract.value);
    let approved: Option<Identity> = nft_abi.get_approved(nft_id);
    let owner: Option<Identity> = nft_abi.owner_of(nft_id);
    let approved_for_all = nft_abi.is_approved_for_all(seller, identity);

    approved_for_all || (approved.is_none() && identity == approved.unwrap()) || (owner.is_none() && identity == owner.unwrap())
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
            transfer_to_output(asset.amount(), asset.contract_id(), identity);
        },
        Identity::ContractId(identity) => {
            force_transfer_to_contract(asset.amount(), asset.contract_id(), identity);
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
    let asset_contract_id = asset.contract_id();
    let nft_abi = abi(NFT, asset_contract_id.value);

    let token_id = match asset {
        Asset::NFTAsset(asset) => {
            asset.token_ids
        },
        _ => {
            revert(0)
        }
    };

    nft_abi.transfer_from(from, to, token_id);

    let owner: Option<Identity> = nft_abi.owner_of(token_id);
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
    let sender = sender_identity();

    require(buy_asset == recieved_asset, InputError::IncorrectAssetProvided);

    match recieved_asset {
        Asset::NFTAsset(asset) => {
            require(approved_for_nft_transfer(Identity::ContractId(contract_id()), sender, asset.contract_id, asset.token_ids), AccessError::NFTTransferNotApproved);
        },
        Asset::TokenAsset(asset) => {
            require(msg_amount() == asset.amount, InputError::IncorrectAmountProvided);
        }
    }
}
