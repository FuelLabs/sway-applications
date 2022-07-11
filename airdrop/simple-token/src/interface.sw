library interface;

use std::{contract_id::ContractId, identity::Identity, option::Option};

abi SimpleToken {
    /// An example function that is to be called by the airdrop distributor contract.
    ///
    /// This mint function is permission such that only the airdrop contract may mint tokens.
    ///
    /// # Arguments
    ///
    /// * `amount` - The quantity of tokens that is to be minted.
    /// * `to` - The user which should recieve the minted tokens.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the airdrop contract.
    /// * When the amount of tokens to be minted is greater than the total supply.
    #[storage(read, write)]fn mint_to(amount: u64, to: Identity);

    /// An example constructor which implements an airdrop distributor contract.
    ///
    /// # Arguments
    ///
    /// * `airdrop_contract` - The airdrop distributor contract which will be permissioned to mint tokens.
    /// * `token_supply` - The total number of tokens that may ever be minted.
    ///
    /// # Reverts
    ///
    /// * When the constructor has already been called.
    /// * When the provided `token_supply` is zero.
    #[storage(read, write)]fn constructor(airdrop_contract: Option<ContractId>, token_supply: u64);
}
