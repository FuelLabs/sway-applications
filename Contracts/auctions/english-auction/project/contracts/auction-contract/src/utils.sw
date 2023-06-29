library;

use ::data_structures::{auction_asset::AuctionAsset, nft_asset::NFTAsset};
use ::errors::AccessError;
use std::token::transfer;
use ::nft::NFT;

/// Transfers assets out of the auction contract to the specified user.
///
/// # Arguments
///
/// * `asset` - The asset that is to be transfered.
/// * `to` - The user which will recieve the asset.
pub fn transfer_asset(asset: AuctionAsset, to: Identity) {
    match asset {
        AuctionAsset::NFTAsset(asset) => {
            transfer_nft(asset, to)
        },
        AuctionAsset::TokenAsset(asset) => {
            transfer(asset.amount(), asset.asset_id(), to)
        },
    }
}

/// Transfers an NFT from one `Identity` to another.
///
/// # Arguments
///
/// * `asset` - The struct which contains the NFT data.
/// * `to` - The user which the NFTs should be transfered to.
///
/// # Reverts
///
/// * The NFT transfer failed.
pub fn transfer_nft(asset: NFTAsset, to: Identity) {
    let nft_abi = abi(NFT, asset.asset_id().value);

    nft_abi.transfer(to, asset.token_id());

    let owner = nft_abi.owner_of(asset.token_id());
    require(owner.is_some() && owner.unwrap() == to, AccessError::NFTTransferNotApproved);
}
