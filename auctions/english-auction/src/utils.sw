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

/// This function returns true if the `to` `Identity` is approved to transfer the token.
///
/// # Arguments
///
/// * `from` - The `Identity` which owns the NFTs.
/// * `to` - The `Identity` which the NFTs should be transfered to.
/// * `asset` - The `NFTAsset` struct which contains the NFT data.
pub fn approved_for_nft_transfer(from: Identity, to: Identity, asset: NFTAsset) -> bool {
    let nft_contract = asset.contract_id;
    // TODO: This will be a Vec
    let nft_id = asset.token_ids;

    let nft_abi = abi(NFT, nft_contract.value);
    let approved_for_all = nft_abi.is_approved_for_all(from, to);
    // TODO: This needs to loop over a Vec of token_ids
    let approved: Option<Identity> = nft_abi.get_approved(nft_id);

    // The to address either needs to be approved for all or approved for this token id
    approved_for_all || (approved.is_some() && to == approved.unwrap())
}

/// This function returns true if the `owner` `Identity` owns the NFT token.
///
/// # Arguments
///
/// * `owner` - The `Identity` which owns the tokens.
/// * `asset` - The `NFTAsset` struct which contains the NFT data.
pub fn owns_nft(owner: Identity, asset: NFTAsset) -> bool {
    let nft_contract = asset.contract_id;
    // TODO: This will be a Vec
    let token_id = asset.token_ids;

    let nft_abi = abi(NFT, nft_contract.value);
    // TODO: This will need to loop over a Vec of token_ids
    let token_owner: Option<Identity> = nft_abi.owner_of(token_id);
    token_owner.is_some() && owner == token_owner.unwrap()
}

/// This function will return the identity of the sender.
pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

/// This function will send tokens to the `Idenitity`.
///
/// # Arguments
///
/// * `identity` - The `Identity` which the tokens should be sent to.
/// * `asset` - The `TokenAsset` which is to be sent.
pub fn send_tokens(identity: Identity, asset: TokenAsset) {
    match identity {
        Identity::Address(identity) => {
            transfer_to_output(asset.amount, asset.contract_id, identity);
        },
        Identity::ContractId(identity) => {
            force_transfer_to_contract(asset.amount, asset.contract_id, identity);
        },
    };
}

/// This function will transfer a NFT from one `Identity` to another.
///
/// # Reverts
///
/// This function will panic when:
/// * The NFT transfer failed
pub fn transfer_nft(from: Identity, to: Identity, asset: NFTAsset) {
    let nft_contract = asset.contract_id;
    // TODO: This will be a Vec
    let token_id = asset.token_ids;

    let nft_abi = abi(NFT, nft_contract.value);
    // TODO: This will need to itterate over a Vec of token IDs
    nft_abi.transfer_from(from, to, token_id);

    // Make sure that the transfer worked
    // TODO: This may be removed in the future
    let owner: Option<Identity> = nft_abi.owner_of(token_id);
    require(owner.is_some() && owner.unwrap() == to, AccessError::NFTTransferNotApproved);
}

/// This function will panic when the `recieved_asset` and `buy_asset` `contract_id`s do not match.
/// If `received_asset` is of type `TokenAsset` the function also ensures that the amount provided
/// in the transaction and `recieved_asset` struct match. If the `received_asset` is of type
/// `NFTAsset` the function will ensure that the auction contract is permissioned to transfer the
/// NFT tokens.
///
/// # Reverts
///
/// * When the `contract_id`s in the `buy_asset` and `recieved_asset` are different.
/// * When the `sender` does not own the NFT tokens to be transfered
/// * When the auction contract is not approved to transfer the NFT tokens specified in the
///   `recieved_asset` struct.
/// * When the transaction asset amount is not the same as the amount specified in the
///   `recieved_asset` struct.
pub fn validate_asset(buy_asset: Asset, recieved_asset: Asset) {
    let sender = sender_identity();

    require(buy_asset == recieved_asset, InputError::IncorrectAssetProvided);

    match recieved_asset {
        Asset::NFTAsset(asset) => {
            require(owns_nft(sender, asset), AccessError::NFTTransferNotApproved);
            require(approved_for_nft_transfer(sender, Identity::ContractId(contract_id()), asset), AccessError::NFTTransferNotApproved);
        },
        Asset::TokenAsset(asset) => {
            require(msg_amount() == asset.amount, InputError::IncorrectAmountProvided);
        }
    }
}
