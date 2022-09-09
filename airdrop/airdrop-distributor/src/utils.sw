library utils;

dep interface;

use interface::SimpleAsset;
use std::{contract_id::ContractId, identity::Identity};

/// Calls the `mint_to` function in another contract.
/// 
/// # Arguments
/// 
/// `amount` - The quantity of an asset to be minted.
/// `to` - The user which the asset should be given to.
/// `asset` - The external asset contract which has an implemented `mint_to` function.
pub fn mint_to(amount: u64, to: Identity, asset: ContractId) {
    let asset_abi = abi(SimpleAsset, asset.value);
    asset_abi.mint_to(amount, to);
}
