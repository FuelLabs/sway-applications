library interface;

dep data_structures;

use data_structures::NFTInfo;

abi FractionalNFT {
    /// Locks an NFT into this contract and sends tokens to the caller.
    ///
    /// # Arguments
    ///
    /// * `admin` - The identity which will have the ability to withdraw.
    /// * `nft` - The contract that manages the deposited NFT.
    /// * `supply` - The number of fractionalized tokens that will be minted.
    /// * `token_id` - The id of the NFT that is being deposited.
    ///
    /// # Reverts
    ///
    /// * When the contract has already been initialized.
    #[storage(read, write)]
    fn deposit(admin: Option<Identity>, nft: ContractId, supply: u64, token_id: u64);

    /// Returns the information of the NFT locked in the contract.
    #[storage(read)]
    fn nft_info() -> Option<NFTInfo>;

    /// Changes the identity which has permission to withdraw and change the admin.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The identity which will now controls the contract.
    ///
    /// # Reverts
    ///
    /// * When no NFT has been locked into the contract.
    /// * When the caller is not the admin.
    #[storage(read, write)]
    fn set_admin(new_admin: Option<Identity>);

    /// Returns the total circulating supply of fractionalized tokens.
    #[storage(read)]
    fn supply() -> u64;

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
