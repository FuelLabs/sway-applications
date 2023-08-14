library;

abi SimpleAsset {
    /// An example constructor which implements an airdrop distributor contract.
    ///
    /// # Arguments
    ///
    /// * `asset_supply`: [u64] - The total qualntity of the asset that may ever be minted.
    /// * `minter`: [Identity] - The Address or Contract which will be permissioned to mint the asset.
    ///
    /// # Reverts
    ///
    /// * When the constructor has already been called.
    /// * When the provided `asset_supply` is zero.
    #[storage(read, write)]
    fn constructor(asset_supply: u64, minter: Identity);

    /// An example function that is to be called by the airdrop distributor contract.
    ///
    /// The mint function is authorized to be called only by the airdrop contract.
    ///
    /// # Arguments
    ///
    /// * `amount`: [u64] - The quantity of the asset that is to be minted.
    /// * `to`: [Identity] - The user which should recieve the minted asset.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the airdrop contract.
    /// * When the amount of the asset to be minted is greater than the total supply.
    #[storage(read, write)]
    fn mint_to(amount: u64, to: Identity);
}
