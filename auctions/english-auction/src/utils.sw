library utils;

dep data_structures;
dep errors;
dep interface;

use data_structures::{Asset, NFTAsset};
use errors::AccessError;
use interface::NFT;
use std::{context::call_frames::contract_id, token::transfer};

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
