library interface;

use std::{contract_id::ContractId, identity::Identity};

abi SimpleAsset {
    /// An example function that is to be called by the airdrop distributor contract.
    /// 
    /// The mint function is authorized to be called only by the airdrop contract.
    /// 
    /// # Arguments
    /// 
    /// * `amount` - The quantity of the asset that is to be minted.
    /// * `to` - The user which should recieve the minted asset.
    /// 
    /// # Reverts
    /// 
    /// * When the sender is not the airdrop contract.
    /// * When the amount of the asset to be minted is greater than the total supply.
    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity);

    /// An example constructor which implements an airdrop distributor contract.
    /// 
    /// # Arguments
    /// 
    /// * `minter` - The Address or Contract which will be permissioned to mint the asset.
    /// * `asset_supply` - The total qualntity of the asset that may ever be minted.
    /// 
    /// # Reverts
    /// 
    /// * When the constructor has already been called.
    /// * When the provided `asset_supply` is zero.
    #[storage(read, write)]
    fn constructor(minter: Identity, asset_supply: u64);
}
