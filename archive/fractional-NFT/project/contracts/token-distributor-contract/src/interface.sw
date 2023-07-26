library;

use ::data_structures::TokenDistribution;

abi TokenDistributor {
    /// Allows for the token admin to start a buyback of tokens at a set price.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    /// * `token_price` - The price for a single fractionalized NFT token.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an exising token distribution.
    /// * When the sender is not the token admin.
    /// * When the distribution is not in the distributing state.
    /// * When providing an incorrect payment asset or amount.
    #[payable, storage(read, write)]
    fn buyback(fractional_nft_id: ContractId, token_price: u64);

    /// Starts a new token distribution and takes control of the NFT.
    ///
    /// # Arguments
    ///
    /// * `nft_asset_id` - The contract that manages the NFT held by the fractionalized NFT.
    /// * `external_asset_id` - The asset which will be accepted in return for fractionalized NFT tokens.
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    /// * `reserve_price` - The price at which admin rights of the NFT may be bought outright.
    /// * `token_admin` - The user which will have the ability to withdraw both the paid tokens and the NFT.
    /// * `token_price` - The price for a single fractionalized NFT token.
    /// * `token_supply` - The total number of fractionalized NFT tokens which will be minted.
    /// * `nft_token_id` - The id of the NFT which will be fractionalized.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` maps to an already existing token distribution.
    #[storage(read, write)]
    fn create(nft_asset_id: ContractId, external_asset_id: ContractId, fractional_nft_id: ContractId, reserve_price: Option<u64>, token_admin: Option<Identity>, token_price: u64, token_supply: u64, nft_token_id: u64);

    /// Will end the token distribution and return the NFT.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When the `fractional_nft_id` does not map to an existing token distribution.
    /// * When the sender is not the admin of the distribution.
    /// * When all tokens that have been purchased haven't been sold back to the contract.
    #[storage(read, write)]
    fn end(fractional_nft_id: ContractId);

    /// Allows users to purchase fractionalized NFT tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of fractionalized NFT tokens to purchase.
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an exisiting token distribution.
    /// * When the distribution is not allowing sales.
    /// * When trying to buy more tokens than held in by contract.
    /// * When providing an incorrect payment asset or amount.
    #[payable, storage(read, write)]
    fn purchase(amount: u64, fractional_nft_id: ContractId);

    /// Allows user to purchase admin rights of the fractionalized NFT.
    ///
    /// # Arguments
    ///
    /// * `admin` - The identity which admin rights should be given to.
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    /// * `reserve` - The new reserve price at which admin rights may be sold.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an existing token distribution.
    /// * When there is no reserve price.
    /// * When the token distribution has already closed.
    /// * When providing an incorrect payment asset or amount.
    #[payable, storage(read, write)]
    fn purchase_admin(admin: Option<Identity>, fractional_nft_id: ContractId, reserve: Option<u64>);

    /// Allows for a fractionalized NFT token holder to sell their tokens.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an exising token distribution.
    /// * When the token distribution is not in the buyback state.
    /// * When not sending fractionalized NFT tokens.
    #[payable, storage(read)]
    fn sell(fractional_nft_id: ContractId);

    /// Allows for the admin to change the price at which admin rights may be bought outright.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    /// * `reserve` - The price at which admin rights may be purchased.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an existing token distribution.
    /// * When the sender is not the admin of the token distribution.
    #[storage(read, write)]
    fn set_reserve(fractional_nft_id: ContractId, reserve: Option<u64>);

    /// Allows for the admin to change the price at which fractionalize NFT tokens are sold.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    /// * `token_price` - The price at which 1 token may be purchased.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an existing token distribution.
    /// * When the sender is not the admin of the token distribution.
    /// * When the token distribution is not in the started or distributed state.
    #[storage(read, write)]
    fn set_token_price(fractional_nft_id: ContractId, token_price: u64);

    /// Allows the admin to withdraw the payments made by token purchasers.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft_id` does not map to an existing token distribution.
    /// * When the sender is not the admin of the token distribution.
    #[storage(read, write)]
    fn withdraw(fractional_nft_id: ContractId);
}

abi Info {
    /// Returns the information on a fractionalized NFT token distribution.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft_id` - The token contract that holds the NFT.
    #[storage(read)]
    fn token_distribution(fractional_nft_id: ContractId) -> Option<TokenDistribution>;
}

abi FractionalNFT {
    #[storage(read, write)]
    fn deposit(admin: Option<Identity>, asset_id: ContractId, supply: u64, token_id: u64);
    #[storage(read)]
    fn supply() -> u64;
    #[storage(read, write)]
    fn withdraw(to: Identity);
}
