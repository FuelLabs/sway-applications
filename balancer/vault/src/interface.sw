library interface;

dep data_structures;

use data_structures::{
    BatchSwapStep,
    ExitPoolRequest,
    FundManagement,
    JoinPoolRequest,
    PoolBalanceOp,
    PoolSpecialization,
    SingleSwap,
    SwapKind,
    UserBalanceOp,
    UserData,
};

use std::{address::Address, contract_id::ContractId, vec::Vec};

abi Vault {
    /// Performs a series of swaps with one or multiple Pools. In each individual swap, the caller determines either
    /// the amount of tokens sent to or received from the Pool, depending on the `kind` value.
    ///
    /// Returns an array with the net Vault asset balance deltas. Positive amounts represent tokens (or ETH) sent to the
    /// Vault, and negative amounts represent tokens (or ETH) sent by the Vault. Each delta corresponds to the asset at
    /// the same index in the `assets` array.
    ///
    /// Swaps are executed sequentially, in the order specified by the `swaps` array. Each array element describes a
    /// Pool, the token to be sent to this Pool, the token to receive from it, and an amount that is either `amountIn` or
    /// `amountOut` depending on the swap kind.
    ///
    /// Multihop swaps can be executed by passing an `amount` value of zero for a swap. This will cause the amount in/out
    /// of the previous swap to be used as the amount in for the current one. In a 'given in' swap, 'tokenIn' must equal
    /// the previous swap's `tokenOut`. For a 'given out' swap, `tokenOut` must equal the previous swap's `tokenIn`.
    ///
    /// `batchSwap` can be used to make a single swap, like `swap` does, but doing so requires more gas than the
    /// equivalent `swap` call.
    ///
    /// Emits `Swap` events.
    ///
    /// # Arguments
    ///
    /// * `kind` - Type of swap, given_in or givin_out
    /// * `swaps` - Array of the swaps. Each array element describes a Pool, the token to be sent to this Pool, the token to receive from it, and an amount that is either `amountIn` or `amountOut` depending on the swap kind.
    /// * `assets` - Array contains the addresses of all assets involved in the swaps
    /// * `funds` - Struct of funds mangament that contains the sender, fromInternalBalance, recipient, toInternalBalance.
    /// * `limits` - Array specifies the minimum or maximum amount of each token the vault is allowed to transfer.
    /// * `deadline` - deadline of transaction
    ///
    /// # Returns
    ///
    /// * The net Vault token deltas
    ///
    /// # Reverts
    ///
    /// * Pool run of the funds
    /// * Wrong Assests
    #[storage(read, write)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64,
    ) -> Vec<u64>;

    /// Emitted for each individual swap performed by `swap` or `batchSwap`.
    ///
    /// # Arguments
    ///
    /// * `singleSwap` - Struct that contains the information of pool_id, SwapKind, assetIn, assetOut, amount, user_data.
    /// * `funds` - Struct of funds mangament that contains the sender, fromInternalBalance, recipient, toInternalBalance.
    /// * `limits` - Array specifies the minimum or maximum amount of each token the vault is allowed to transfer.
    /// * `deadline` - deadline of transaction
    ///
    /// # Returns
    ///
    /// The amount of the token after swap
    ///
    /// # Reverts
    ///
    /// * Pool run of the funds
    /// * Wrong Assests
    /// * Tokens are not registered
    #[storage(read, write)]
    fn swap(
        single_swap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64,
    ) -> u64;

    /// Simulates a call to `batchSwap`, returning an array of Vault asset deltas. Calls to `swap` cannot be
    /// simulated directly, but an equivalent `batchSwap` call can and will yield the exact same result.
    ///
    /// Each element in the array corresponds to the asset at the same index, and indicates the number of tokens (or ETH)
    /// the Vault would take from the sender (if positive) or send to the recipient (if negative). The arguments it
    /// receives are the same that an equivalent `batchSwap` call would receive.
    ///
    /// Unlike `batchSwap`, this function performs no checks on the sender or recipient field in the `funds` struct.
    /// This makes it suitable to be called by off-chain applications via eth_call without needing to hold tokens,
    /// approve them for the Vault, or even know a user's address.
    ///
    /// Note that this function is not 'view' (due to implementation details): the client code must explicitly execute
    /// eth_call instead of eth_sendTransaction.
    ///
    /// # Arguments
    ///
    /// * `kind` - Type of swap, given_in or givin_out
    /// * `swaps` - Array of the swaps. Each array element describes a Pool, the token to be sent to this Pool, the token to receive from it, and an amount that is either `amountIn` or `amountOut` depending on the swap kind.
    /// * `assets` - Array contains the addresses of all assets involved in the swaps
    /// * `funds` - Struct of funds mangament that contains the sender, fromInternalBalance, recipient, toInternalBalance.
    /// * `limits` - Array specifies the minimum or maximum amount of each token the vault is allowed to transfer.
    /// * `deadline` - deadline of transaction
    ///
    /// # Returns
    ///
    /// * The net Vault token deltas
    #[storage(write, read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
    ) -> Vec<u64>;

    /// Deregisters `tokens` for the `pool_id` Pool. Must be called by the Pool's contract.
    ///
    /// Only registered tokens (via `registerTokens`) can be deregistered. Additionally, they must have zero total
    /// balance. For Pools with the Two Token specialization, `tokens` must have a length of two, that is, both tokens
    /// must be deregistered in the same `deregisterTokens` call.
    ///
    /// A deregistered token can be re-registered later on, possibly with a different Asset Manager.
    ///
    /// Emits a `TokensDeregistered` event.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the pool
    /// * `tokens` - Array of contract_ids of the tokens
    ///
    /// # Reverts
    ///
    /// * Pool is not registered
    /// * Tokens are not registered
    #[storage(read, write)]
    fn deregister_tokens(pool_id: b256, tokens: Vec<ContractId>);

    /// Registers `tokens` for the `pool_id` Pool. Must be called by the Pool's contract.
    ///
    /// Pools can only interact with tokens they have registered. Users join a Pool by transferring registered tokens,
    /// exit by receiving registered tokens, and can only swap registered tokens.
    ///
    /// Each token can only be registered once. For Pools with the Two Token specialization, `tokens` must have a length
    /// of two, that is, both tokens must be registered in the same `registerTokens` call, and they must be sorted in
    /// ascending order.
    ///
    /// The `tokens` and `asset_managers` arrays must have the same length, and each entry in these indicates the Asset
    /// Manager for the corresponding token. Asset Managers can manage a Pool's tokens via `managePoolBalance`,
    /// depositing and withdrawing them directly, and can even set their balance to arbitrary amounts. They are therefore
    /// expected to be highly secured smart contracts with sound design principles, and the decision to register an
    /// Asset Manager should not be made lightly.
    ///
    /// Pools can choose not to assign an Asset Manager to a given token by passing in the zero address. Once an Asset
    /// Manager is set, it cannot be changed except by deregistering the associated token and registering again with a
    /// different Asset Manager.
    ///
    /// Emits a `TokensRegistered` event.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the token's pool
    /// * `tokens` - Array of the contract ids of the tokens
    /// * `asset_managers` - Array of the asset managers of the tokens
    ///
    /// # Reverts
    ///
    /// * Pool is not registered
    /// * Tokens are not registered
    /// * Not enough numbers tokens
    /// * More than limit tokens
    #[storage(read, write)]
    fn register_tokens(
        pool_id: b256,
        tokens: Vec<ContractId>,
        asset_managers: Vec<Address>,
    );

    /// Called by users to join a Pool, which transfers tokens from `sender` into the Pool's balance. This will
    /// trigger custom Pool behavior, which will typically grant something in return to `recipient` - often tokenized
    /// Pool shares.
    ///
    /// If the caller is not `sender`, it must be an authorized relayer for them.
    ///
    /// The `assets` and `maxAmountsIn` arrays must have the same length, and each entry indicates the maximum amount
    /// to send for each asset. The amounts to send are decided by the Pool and not the Vault: it just enforces
    /// these maximums.
    ///
    /// If joining a Pool that holds WETH, it is possible to send ETH directly: the Vault will do the wrapping. To enable
    /// this mechanism, the IAsset sentinel value (the zero address) must be passed in the `assets` array instead of the
    /// WETH address. Note that it is not possible to combine ETH and WETH in the same join. Any excess ETH will be sent
    /// back to the caller (not the sender, which is important for relayers).
    ///
    /// `assets` must have the same length and order as the array returned by `getPoolTokens`. This prevents issues when
    /// interacting with Pools that register and deregister tokens frequently. If sending ETH however, the array must be
    /// sorted *before* replacing the WETH address with the ETH sentinel value (the zero address), which means the final
    /// `assets` array might not be sorted. Pools with no registered tokens cannot be joined.
    ///
    /// If `fromInternalBalance` is true, the caller's Internal Balance will be preferred: ERC20 transfers will only
    /// be made for the difference between the requested amount and Internal Balance (if any). Note that ETH cannot be
    /// withdrawn from Internal Balance: attempting to do so will trigger a revert.
    ///
    /// This causes the Vault to call the `IBasePool.onJoinPool` hook on the Pool's contract, where Pools implement
    /// their own custom logic. This typically requires additional information from the user (such as the expected number
    /// of Pool shares). This can be encoded in the `user_data` argument, which is ignored by the Vault and passed
    /// directly to the Pool's contract, as is `recipient`.
    ///
    /// Emits a `PoolBalanceChanged` event.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the token's pool
    /// * `sender` - Address of the sender
    /// * `recipient` - Address of the recipient
    /// * `request` - struct that contains the info of assets, maxAmountsIn, user_data, fromInternalBalance
    ///
    /// # Reverts
    ///
    /// * Token is not registered
    #[storage(read, write)]
    fn join_pool(
        pool_id: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest,
    );

    /// Called by users to exit a Pool, which transfers tokens from the Pool's balance to `recipient`. This will
    /// trigger custom Pool behavior, which will typically ask for something in return from `sender` - often tokenized
    /// Pool shares. The amount of tokens that can be withdrawn is limited by the Pool's `cash` balance (see
    /// `getPoolTokenInfo`).
    ///
    /// If the caller is not `sender`, it must be an authorized relayer for them.
    ///
    /// The `tokens` and `minAmountsOut` arrays must have the same length, and each entry in these indicates the minimum
    /// token amount to receive for each token contract. The amounts to send are decided by the Pool and not the Vault:
    /// it just enforces these minimums.
    ///
    /// If exiting a Pool that holds WETH, it is possible to receive ETH directly: the Vault will do the unwrapping. To
    /// enable this mechanism, the IAsset sentinel value (the zero address) must be passed in the `assets` array instead
    /// of the WETH address. Note that it is not possible to combine ETH and WETH in the same exit.
    ///
    /// `assets` must have the same length and order as the array returned by `getPoolTokens`. This prevents issues when
    /// interacting with Pools that register and deregister tokens frequently. If receiving ETH however, the array must
    /// be sorted *before* replacing the WETH address with the ETH sentinel value (the zero address), which means the
    /// final `assets` array might not be sorted. Pools with no registered tokens cannot be exited.
    ///
    /// If `toInternalBalance` is true, the tokens will be deposited to `recipient`'s Internal Balance. Otherwise,
    /// an ERC20 transfer will be performed. Note that ETH cannot be deposited to Internal Balance: attempting to
    /// do so will trigger a revert.
    ///
    /// `minAmountsOut` is the minimum amount of tokens the user expects to get out of the Pool, for each token in the
    /// `tokens` array. This array must match the Pool's registered tokens.
    ///
    /// This causes the Vault to call the `IBasePool.onExitPool` hook on the Pool's contract, where Pools implement
    /// their own custom logic. This typically requires additional information from the user (such as the expected number
    /// of Pool shares to return). This can be encoded in the `user_data` argument, which is ignored by the Vault and
    /// passed directly to the Pool's contract.
    ///
    /// Emits a `PoolBalanceChanged` event.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the token's pool
    /// * `sender` - Address of the sender
    /// * `recipient` - Address of the recipient
    /// * `request` - struct that contains the info of assets, minAmountsOut, user_data, toInternalBalance
    ///
    /// # Reverts
    ///
    /// * Token is not registered
    #[storage(read, write)]
    fn exit_pool(
        pool_id: b256,
        sender: Address,
        recipient: Address,
        request: ExitPoolRequest,
    );

    /// Performs a 'flash loan', sending tokens to `recipient`, executing the `receiveFlashLoan` hook on it,
    /// and then reverting unless the tokens plus a proportional protocol fee have been returned.
    ///
    /// The `tokens` and `amounts` arrays must have the same length, and each entry in these indicates the loan amount
    /// for each token contract. `tokens` must be sorted in ascending order.
    ///
    /// The 'user_data' field is ignored by the Vault, and forwarded as-is to `recipient` as part of the
    /// `receiveFlashLoan` call.
    ///
    /// Emits `FlashLoan` events.
    ///
    /// # Arguments
    ///
    /// * `recipient` - The contract id  of the recipient
    /// * `tokens` - The Array of tokens contract IDs
    /// * `amounts` - The Array of tokens contract amounts
    /// * `user_data` - The of the user_data in
    ///
    /// # Reverts
    ///
    /// * Token is not registered
    #[storage(read, write)]
    fn flash_loan(
        recipient: ContractId,
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        user_data: Vec<b256>,
    );

    /// Performs a set of Pool balance operations, which may be either withdrawals, deposits or updates.
    ///
    /// Pool Balance management features batching, which means a single contract call can be used to perform multiple
    /// operations of different kinds, with different Pools and tokens, at once.
    ///
    /// For each operation, the caller must be registered as the Asset Manager for `token` in `pool_id`.
    ///
    /// # Arguments
    ///
    /// * `ops` - The array of PoolBalanceOp enum that contains info the transaction
    ///
    /// # Reverts
    ///
    /// * msg_sender is not the owner of the pool
    #[storage(read, write)]
    fn manage_pool_balance(ops: Vec<PoolBalanceOp>);

    /// Performs a set of user balance operations, which involve Internal Balance (deposit, withdraw or transfer)
    /// and plain ERC20 transfers using the Vault's allowance. This last feature is particularly useful for relayers, as
    /// it lets integrators reuse a user's Vault allowance.
    ///
    /// For each operation, if the caller is not `sender`, it must be an authorized relayer for them.
    ///
    /// # Arguments
    ///
    /// * `ops` - The array of UserBalanceOp struct that contains info of amount, asset, kind, recipient, sender,
    /// # Reverts
    ///
    /// * msg_sender is not the owner
    #[storage(read, write)]
    fn manage_user_balance(ops: Vec<UserBalanceOp>);

    /// Registers the caller account as a Pool with a given specialization setting. Returns the Pool's ID, which
    /// is used in all Pool-related functions. Pools cannot be deregistered, nor can the Pool's specialization be
    /// changed.
    ///
    /// The caller is expected to be a smart contract that implements either `IGeneralPool` or `IMinimalSwapInfoPool`,
    /// depending on the chosen specialization setting. This contract is known as the Pool's contract.
    ///
    /// Note that the same contract may register itself as multiple Pools with unique Pool IDs, or in other words,
    /// multiple Pools may share the same contract.
    ///
    /// Emits a `PoolRegistered` event
    ///
    /// # Arguments
    ///
    /// * `specialization` - Struct that contains the info about the pool type
    ///
    /// # Reverts
    ///
    /// * Pool is already registered
    #[storage(read, write)]
    fn register_pool(pool_id: b256, specialization: PoolSpecialization) -> b256;

    /// Sets a new Authorizer for the Vault. The caller must be allowed by the current Authorizer to do this.
    ///
    /// Emits an `AuthorizerChanged` event.
    ///
    /// # Arguments
    ///
    /// * `new_authorizer` - The of the authorizer
    #[storage(read, write)]
    fn set_authorizer(new_authorizer: ContractId);

    /// Allows `relayer` to act as a relayer for `sender` if `approved` is true, and disallows it otherwise.
    ///
    /// Emits a `RelayerApprovalChanged` event.
    ///
    /// # Arguments
    ///
    /// * `sender` - The address of the sender
    /// * `relayer` - The address of the relayer
    /// * The state of the relayer
    /// * `approved` - The state of the relayer
    #[storage(read, write)]
    fn set_relayer_approval(sender: Address, relayer: Address, approved: bool);

    /// read functions
    /// # Returns
    ///
    /// * The Vault's WETH instance.
    #[storage(read)]
    fn weth() -> ContractId;

    /// # Returns
    ///
    /// * The address of the authorizer
    #[storage(read)]
    fn get_authorizer() -> ContractId;

    /// # Arguments
    ///
    /// * `user` - The address of the user
    /// * `tokens` - The array of tokens contract IDs
    ///
    /// # Returns
    ///
    /// * User's Internal Balance for a set of tokens.
    #[storage(read)]
    fn get_internal_balance(user: Address, tokens: Vec<ContractId>) -> Vec<u64>;

    /// # Arguments
    ///
    /// * `user` - The address of the user
    ///
    /// # Returns
    ///
    /// * The next nonce used by an address to sign messages.
    #[storage(read)]
    fn get_next_nonce(user: Address) -> u64;

    /// # Returns
    ///
    /// * The current paused state.
    /// * The end times of the Pause Window and Buffer
    /// * The end times of the Buffer period
    #[storage(read)]
    fn get_paused_state() -> (bool, u64, u64);

    /// # Arguments
    ///
    /// * `pool_id` - The id of the pool
    ///
    /// # Returns
    ///
    /// * The Id of the pool
    /// * Info of the pool
    #[storage(read)]
    fn get_pool(pool_id: b256) -> (ContractId, PoolSpecialization);

    /// # Arguments
    ///
    /// * `pool_id` - The id of the pool
    /// * `token` - The contract id of the token
    ///
    /// # Returns
    ///
    /// * The number of tokens the Vault currently holds for the Pool.
    /// * The number of tokens withdrawn and held outside the Vault by the Pool's token Asset Manager.
    /// * The number of the block in which `token`'s total balance was last modified (via either a join, exit, swap, or Asset Manager update).
    /// * The Pool's token Asset Manager.
    #[storage(read)]
    fn get_pool_token_info(pool_id: b256, token: ContractId) -> (u64, u64, u64, Address);

    /// # Arguments
    ///
    /// * `poolId` - The id of the pool
    /// 
    /// # Returns
    ///
    /// * The contract addresss of the pool's tokens 
    /// * Balance of respicative tokens
    /// * Timestamp of the last changed block
    ///
    /// # Reverts
    ///
    /// * The pool is not registered
    #[storage(read)]
    fn get_pool_tokens(poolId: b256) -> (Vec<ContractId>, Vec<u64>, u64);

    /// # Arguments
    ///
    /// * `user` - The address of the user
    /// * `relayer` - The address of the relayer
    ///
    /// # Returns
    ///
    /// * The state of the relayer
    #[storage(read)]
    fn has_approved_relayer(user: Address, relayer: Address) -> bool;

    /// # Returns
    ///
    /// * The swap fee percentage
    #[storage(read)]fn get_swap_fee_percentage() -> u64;
}

abi ExternalInterface {
    /// Called by the Vault when a user calls `joinPool` to add liquidity to this Pool. Returns how many of
    /// each registered token the user should provide, as well as the amount of protocol fees the Pool owes to the Vault.
    /// The Vault will then take tokens from `sender` and add them to the Pool's balances, as well as collect
    /// the reported amount in protocol fees, which the pool should calculate based on `protocol_swap_fee_percentage`.
    ///
    /// Protocol fees are reported and charged on join events so that the Pool is free of debt whenever new users join.
    ///
    /// `user_data` contains any pool-specific instructions needed to perform the calculations, such as the type of
    /// join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    ///
    /// Contracts implementing this function should check that the caller is indeed the Vault before performing any
    /// state-changing operations, such as minting pool shares.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the pool
    /// * `sender` - the account performing the join (from which tokens will be withdrawn)
    /// * `recipient` - The account performing the join (from which tokens will be withdrawn)
    /// * `balances` - The account designated to receive any benefits (typically pool shares)
    /// * `last_change_block` - The last block in which *any* of the Pool's registered tokens last changed its total balance.
    /// * `protocol_swap_fee_percentage` - The percentage of the total balance that
    /// * `user_data` - contains any pool-specific instructions needed to perform the calculations, such as the type of join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    ///
    /// # Returns
    ///
    /// * The array of deposit amount
    /// * The array of fee of the due protocol fee amounts
    #[storage(read, write)]
    fn on_join_pool(
        pool_id: b256,
        sender: ContractId,
        recipient: ContractId,
        balances: Vec<u64>,
        last_change_block: u64,
        protocol_swap_fee_percentage: u64,
        user_data: UserData,
    ) -> (Vec<u64>, Vec<u64>);

    /// Called by the Vault when a user calls `IVault.exitPool` to remove liquidity from this Pool. Returns how many
    /// tokens the Vault should deduct from the Pool's balances, as well as the amount of protocol fees the Pool owes
    /// to the Vault. The Vault will then take tokens from the Pool's balances and send them to `recipient`,
    /// as well as collect the reported amount in protocol fees, which the Pool should calculate based on
    /// `protocol_swap_fee_percentage`.
    ///
    /// Protocol fees are charged on exit events to guarantee that users exiting the Pool have paid their share.
    ///
    /// Contracts implementing this function should check that the caller is indeed the Vault before performing any
    /// state-changing operations, such as burning pool shares.
    ///
    /// # Arguments
    ///
    /// * `pool_id` - The id of the pool
    /// * `sender` - the account performing the join (from which tokens will be withdrawn)
    /// * `recipient` - The account performing the join (from which tokens will be withdrawn)
    /// * `balances` - The account designated to receive any benefits (typically pool shares)
    /// * `last_change_block` - The last block in which *any* of the Pool's registered tokens last changed its total balance.
    /// * `protocol_swap_fee_percentage` - The percentage of the total balance that
    /// * `user_data` - contains any pool-specific instructions needed to perform the calculations, such as the type of join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    ///
    /// # Returns
    ///
    /// * The array of deposit amount
    /// * The array of fee of the due protocol fee amounts
    #[storage(read, write)]
    fn on_exit_pool(
        pool_id: b256,
        sender: ContractId,
        recipient: ContractId,
        balances: Vec<u64>,
        last_change_block: u64,
        protocol_swap_fee_percentage: u64,
        user_data: UserData,
    ) -> (Vec<u64>, Vec<u64>);
}
