library interface;

dep data_structures;

use data_structures::TokenDistribution;

abi TokenDistributor {
    /// Allows for the NFT owner to start a buyback of tokens at a set price.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    /// * `token_price` - The price for a single fractionalized NFT token.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an exising token distribution.
    /// * When the sender is not the NFT owner.
    /// * When the distribution is not in the distributing state.
    /// * When providing an incorrect payment asset or amount.
    #[storage(read, write)]
    fn buyback(fractional_nft: ContractId, token_price: u64);

    /// Starts a new token distribution and takes control of the NFT.
    ///
    /// # Arguments
    ///
    /// * `external_asset` - The asset which will be accepted in return for fractionalized NFT tokens.
    /// * `fractional_nft` - The token contract that holds the NFT.
    /// * `nft` - The contract that manages the NFT held by the fractionalized NFT.
    /// * `owner` - The user which will have the ability to withdraw both the paid tokens and the NFT.
    /// * `reserve_price` - The price at which ownership of the NFT may be bought outright.
    /// * `token_price` - The price for a single fractionalized NFT token.
    /// * `token_supply` - The total number of fractionalized NFT tokens which will be minted.
    /// * `nft_token_id` - The id of the NFT which will be fractionalized.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` maps to an already existing token distribution.
    #[storage(read, write)]
    fn create(external_asset: ContractId, fractional_nft: ContractId, nft: ContractId, owner: Option<Identity>, reserve_price: Option<u64>, token_price: u64, token_supply: u64, nft_token_id: u64);

    /// Will end the token distribution and return the NFT.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When the `fractional_nft` does not map to an existing token distribution.
    /// * When the sender is not the owner of the distribution.
    /// * When not all tokens can be returned to the fractionalized NFT contract.
    #[storage(read, write)]
    fn end(fractional_nft: ContractId);

    /// Allows users to purchase fractionalized NFT tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of fractionalized NFT tokens to purchase.
    /// * `fractional_nft` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an exisiting token distribution.
    /// * When the distribution is not allowing sales.
    /// * When trying to buy more tokens than held in by contract.
    /// * When providing an incorrect payment asset or amount.
    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft: ContractId);

    /// Allows user to purchase ownership of the fractionalized NFT.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    /// * `owner` - The identity which ownership should be given to.
    /// * `reserve` - The new reserve price at which ownership may be sold.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an existing token distribution.
    /// * When there is no reserve price.
    /// * When the token distribution has already closed.
    /// * When providing an incorrect payment asset or amount.
    #[storage(read, write)]
    fn purchase_ownership(fractional_nft: ContractId, owner: Option<Identity>, reserve: Option<u64>);

    /// Allows for a fractionalized NFT token holder to sell their tokens.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an exising token distribution.
    /// * When the token distribution is not in the buyback state.
    /// * When not sending fractionalized NFT tokens.
    #[storage(read)]
    fn sell(fractional_nft: ContractId);

    /// Allows for the owner to change the price at which fractionalize NFT tokens are sold.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    /// * `token_price` - The price at which ownership may be purchased.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an existing token distribution.
    /// * When the sender is not the owner of the token distribution.
    /// * When the token distribution is not in the started or distributed state.
    #[storage(read, write)]
    fn set_token_price(fractional_nft: ContractId, token_price: u64);

    /// Allows for the owner to change the price at which ownership may be bought outright.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    /// * `reserve` - The price at which ownership may be purchased.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an existing token distribution.
    /// * When the sender is not the owner of the token distribution.
    #[storage(read, write)]
    fn set_reserve(fractional_nft: ContractId, reserve: Option<u64>);

    /// Returns the information on a fractionalized NFT token distribution.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    #[storage(read)]
    fn token_distribution(fractional_nft: ContractId) -> Option<TokenDistribution>;

    /// Allows the owner to withdraw the payments made by token purchasers.
    ///
    /// # Arguments
    ///
    /// * `fractional_nft` - The token contract that holds the NFT.
    ///
    /// # Reverts
    ///
    /// * When `fractional_nft` does not map to an existing token distribution.
    #[storage(read, write)]
    fn withdraw(fractional_nft: ContractId);
}

abi FractionalNFT {
    #[storage(read, write)]
    fn deposit(nft: ContractId, owner: Option<Identity>, supply: u64, token_id: u64);
    #[storage(read)]
    fn supply() -> u64;
    #[storage(read, write)]
    fn withdraw(to: Identity);
}
