library interface;

use std::{address::Address, vec::Vec};

abi TwoTokenPoolsBalance {
    /// Deregisters tokens in a Two Token Pool.
    ///
    /// This function assumes `poolId` exists and corresponds to the Two Token specialization setting.
    /// Deregisters token_x and token_y from the two pool.
    ///
    /// # Requirements:
    ///
    /// - `tokenX` and `tokenY` must be registered in the Pool
    /// - both tokens must have zero balance in the Vault
    ///
    /// # Arguments
    /// 
    /// * `pool_id` - The ID of pool of the tokens
    /// * `token_x` - The Address of the token_x
    /// * `token_y` - The Address of the token_y
    #[storage(read, write)]fn deregister_two_token_pool_tokens(pool_id: b256, token_x: Address, token_y: Address);
    
    /// Returns an array with all the tokens and balances in a Two Token Pool. The order may change when
    /// tokens are registered or deregistered.
    ///
    /// This function assumes `poolId` exists and corresponds to the Two Token specialization setting.
    /// 
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    ///
    /// # Returns
    ///
    /// * The vec of addresses of the tokens
    /// * The vec of balances of the tokens
    #[storage(read)]fn get_two_token_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<b256>);

    /// This function assumes `poolId` exists and corresponds to the General specialization setting.
    /// This function is convenient but not particularly gas efficient, and should be avoided during gas-sensitive
    /// operations, such as swaps. For those, _getTwoTokenPoolSharedBalances provides a more flexible interface.
    ///
    /// # Requirements:
    ///
    /// - `token` must be registered in the Pool
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    /// * `token` - The address of the token
    ///
    /// # Returns
    ///
    /// * The balance of a token in a Two Token Pool
    #[storage(read)]fn get_two_token_pool_balance(pool_id: b256, token: Address) -> b256;

    /// Registers tokens in a Two Token Pool.
    /// 
    /// # Requirements:
    ///
    /// - `tokenX` and `tokenY` must not be the same
    /// - The tokens must be ordered: tokenX < tokenY
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    /// * `token_a` - The Address of the tokenA
    /// * `token_b` - The Address of the tokenB
    #[storage(read, write)]fn register_two_token_pool_tokens(pool_id: b256, token_x: Address, token_y: Address);
    
    /// Sets the cash balances of a Two Token Pool's tokens.
    ///
    /// WARNING: this assumes `tokenA` and `tokenB` are the Pool's two registered tokens, and are in the correct order.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    /// * `token_a` - The Address of the tokenA
    /// * `balance_a` - The amount of the tokenA 
    /// * `token_b` - The Address of the tokenB
    /// * `balance_b` - The amount of tokenB
    #[storage(read, write)]fn set_two_token_pool_cash_balances(pool_id: b256, token_a: Address, balance_a: b256, token_b: Address, balance_b: b256);
    
    /// Sets `token`'s managed balance in a Two Token Pool to `amount`.
    ///
    /// # Arguments
    /// 
    /// * `pool_id` - The Address of the pool
    /// * `token` - The Address of the token
    /// * `amount` - The amount of the token
    ///
    /// # Returns
    ///
    /// * The managed balance delta as a result of this call.
    #[storage(read, write)]fn set_two_token_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64;
    
    /// Transforms `amount` of `token`'s balance in a Two Token Pool from cash into managed.
    /// This function assumes `poolId` exists, corresponds to the Two Token specialization setting, and that `token` is
    /// registered for that Pool.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    /// * `token` - The Address of the token
    /// * `amount` - The amount of the token
    #[storage(read, write)]fn two_token_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64);
    
    /// Transforms `amount` of `token`'s balance in a Two Token Pool from managed into cash.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The Address of the pool
    /// * `token` - The Address of the token
    /// * `amount` - The amount of the token
    #[storage(read, write)]fn two_token_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64);
}
