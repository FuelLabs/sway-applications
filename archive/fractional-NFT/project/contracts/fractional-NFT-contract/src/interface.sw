library;

use ::data_structures::NFTInfo;

abi FractionalNFT {
    /// Locks an NFT into this contract and sends tokens to the caller.
    ///
    /// # Arguments
    ///
    /// * `admin` - The identity which will have the ability to withdraw.
    /// * `asset_id` - The contract that manages the deposited NFT.
    /// * `supply` - The number of fractionalized tokens that will be minted.
    /// * `token_id` - The id of the NFT that is being deposited.
    ///
    /// # Reverts
    ///
    /// * When there has already been a deposit.
    #[storage(read, write)]
    fn deposit(admin: Option<Identity>, asset_id: ContractId, supply: u64, token_id: u64);

    /// Changes the identity which has permission to withdraw and change the admin.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The identity which will take control over the contract.
    ///
    /// # Reverts
    ///
    /// * When no NFT has been locked into the contract.
    /// * When the caller is not the admin.
    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>);

    /// Unlocks and relinquishes control of the NFT when all tokens have been returned.
    ///
    /// # Arguments
    ///
    /// * `to` - The identity to whom the ownership of the NFT will be transferred to.
    ///
    /// # Reverts
    ///
    /// * When no NFT has been locked into the contract.
    /// * When the sender is not the admin.
    /// * When all tokens have not been returned.
    #[storage(read, write)]
    fn withdraw(to: Identity);
}

abi Info {
    /// Returns the information of the NFT locked in the contract.
    #[storage(read)]
    fn nft_info() -> Option<NFTInfo>;

    /// Returns the total supply of fractionalized tokens.
    #[storage(read)]
    fn supply() -> u64;
}
