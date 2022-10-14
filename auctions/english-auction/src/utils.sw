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
    token::transfer,
};

/// Returns true if the `to` `Identity` is approved to transfer the token.
///
/// # Arguments
///
/// * `asset` - The struct which contains the NFT data.
/// * `from` - The owner the NFTs.
/// * `to` - The user which the NFTs should be transfered to.
pub fn approved_for_nft_transfer(asset: NFTAsset, from: Identity, to: Identity) -> bool {
    let nft_contract = asset.contract_id;
    let nft_abi = abi(NFT, nft_contract.value);

    let approved_for_all = nft_abi.is_approved_for_all(from, to);
    let approved_for_token = nft_abi.approved(asset.token_id);

    // The to `Identity` either needs to be approved for all or approved for this token id
    approved_for_all || to == approved_for_token
}

/// Returns true if the `owner` `Identity` owns the NFT token.
///
/// # Arguments
///
/// * `asset` - The struct which contains the NFT data.
/// * `owner` - The user which should be checked for ownership.
pub fn owns_nft(asset: NFTAsset, owner: Identity) -> bool {
    let nft_contract = asset.contract_id;
    let nft_abi = abi(NFT, nft_contract.value);

    let token_owner = nft_abi.owner_of(asset.token_id);

    owner == token_owner
}

/// Transfers assets out of the auction contract to the specified user.
///
/// # Arguments
///
/// * `asset` - The asset that is to be transfered.
/// * `to` - The user which will recieve the asset.
pub fn transfer_asset(asset: Asset, to: Identity) {
    match asset {
        Asset::NFTAsset(asset) => {
            transfer_nft(asset, Identity::ContractId(contract_id()), to)
        },
        Asset::TokenAsset(asset) => {
            transfer(asset.amount, asset.contract_id, to)
        },
    }
}

/// Transfers an NFT from one `Identity` to another.
///
/// # Arguments
///
/// * `asset` - The struct which contains the NFT data.
/// * `from` - The owner of the NFT.
/// * `to` - The user which the NFTs should be transfered to.
///
/// # Reverts
///
/// * The NFT transfer failed.
pub fn transfer_nft(asset: NFTAsset, from: Identity, to: Identity) {
    let nft_contract = asset.contract_id;
    let nft_abi = abi(NFT, nft_contract.value);

    nft_abi.transfer_from(from, to, asset.token_id);

    let owner = nft_abi.owner_of(asset.token_id);
    require(owner == to, AccessError::NFTTransferNotApproved);
}

/// Ensures the assets provided match, NFTs must be permissioned for transfer, and token 
/// amounts match.
//
/// # Arguments
///
/// * `bid_asset` - The struct containing information on the asset is accepted for bids.
/// * `recieved_asset` - The struct containing information on the asset that was bid.
///
/// # Reverts
///
/// * When the asset types are different.
/// * When the sender does not own the NFT tokens to be transfered.
/// * When the auction contract is not permissioned to transfer the NFT tokens.
/// * When the transaction's token amount does not match the amount specified in the struct.
pub fn validate_asset(bid_asset: Asset, recieved_asset: Asset) {
    let sender = msg_sender().unwrap();

    require(bid_asset == recieved_asset, InputError::IncorrectAssetProvided);

    match recieved_asset {
        Asset::NFTAsset(asset) => {
            require(owns_nft(asset, sender), AccessError::NFTTransferNotApproved);
            require(approved_for_nft_transfer(asset, sender, Identity::ContractId(contract_id())), AccessError::NFTTransferNotApproved);
        },
        Asset::TokenAsset(asset) => {
            require(msg_amount() == asset.amount, InputError::IncorrectAmountProvided);
        }
    }
}
