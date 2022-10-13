library utils;

dep data_structures;
dep errors;
dep interface;

use data_structures::{Asset, NFTAsset, TokenAsset};
use errors::{AccessError, InputError};
use interface::NFT;
use std::{
    chain::auth::{
        AuthError,
        msg_sender,
    },
    context::{
        call_frames::{
            contract_id,
            msg_asset_id,
        },
        msg_amount,
    },
    identity::Identity,
    option::Option,
    result::Result,
    revert::require,
    token::transfer,
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
    let nft_id = asset.token_id;

    let nft_abi = abi(NFT, nft_contract.value);
    let approved_for_all = nft_abi.is_approved_for_all(from, to);
    let approved_for_token: Option<Identity> = nft_abi.approved(nft_id);

    // The to address either needs to be approved for all or approved for this token id
    approved_for_all || (approved_for_token.is_some() && to == approved_for_token.unwrap())
}

/// This function returns true if the `owner` `Identity` owns the NFT token.
///
/// # Arguments
///
/// * `owner` - The `Identity` which owns the tokens.
/// * `asset` - The `NFTAsset` struct which contains the NFT data.
pub fn owns_nft(owner: Identity, asset: NFTAsset) -> bool {
    let nft_contract = asset.contract_id;
    let token_id = asset.token_id;

    let nft_abi = abi(NFT, nft_contract.value);
    let token_owner: Option<Identity> = nft_abi.owner_of(token_id);

    token_owner.is_some() && owner == token_owner.unwrap()
}

/// This function will transfer the `Asset` given to the specified `Identity`.
pub fn transfer_asset(to: Identity, asset: Asset) {
    match asset {
        Asset::NFTAsset(asset) => {
            transfer_nft(Identity::ContractId(contract_id()), to, asset)
        },
        Asset::TokenAsset(asset) => {
            transfer(asset.amount, asset.contract_id, to)
        },
    }
}

/// This function will transfer a NFT from one `Identity` to another.
///
/// # Reverts
///
/// This function will panic when:
/// * The NFT transfer failed
pub fn transfer_nft(from: Identity, to: Identity, asset: NFTAsset) {
    let nft_contract = asset.contract_id;

    let nft_abi = abi(NFT, nft_contract.value);
    nft_abi.transfer_from(from, to, asset.token_id);

    // Make sure that the transfer worked
    // TODO: This may be removed in the future
    let owner: Option<Identity> = nft_abi.owner_of(asset.token_id);
    require(owner.is_some() && owner.unwrap() == to, AccessError::NFTTransferNotApproved);
}

/// This function will panic when the `recieved_asset` and `bid_asset` `ContractId`s do not match.
/// If `received_asset` is of type `TokenAsset`, the function ensures that the amount provided
/// in the transaction and `recieved_asset` struct match. If the `received_asset` is of type
/// `NFTAsset`, the function will ensure that the auction contract is permissioned to transfer the
/// NFT tokens and the sender owns the NFT tokens.
///
/// # Reverts
///
/// * When the `contract_id`s in the `bid_asset` and `recieved_asset` are different.
/// * When the `sender` does not own the NFT tokens to be transfered
/// * When the auction contract is not approved to transfer the NFT tokens specified in the
///   `recieved_asset` struct.
/// * When the transaction asset amount is not the same as the amount specified in the
///   `recieved_asset` struct.
pub fn validate_asset(bid_asset: Asset, recieved_asset: Asset) {
    let sender = msg_sender().unwrap();

    require(bid_asset == recieved_asset, InputError::IncorrectAssetProvided);

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
