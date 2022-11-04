library interface;
/*
 * Reference implementation for the base layer of a Pool contract
 * Manages a single Pool with optional Asset Managers, an admin-controlled swap fee percentage, a temporary
 * emergency pause mechanism that disables the pool, and a permanent Recovery Mode option that ensures LPs can
 * always proportionally exit the pool, even if it's in a pathological state.
 *
 * Note that neither swap fees nor the pause mechanism are used by this contract. They are passed through so that
 * derived contracts can use them via the `_addSwapFeeAmount` and `_subtractSwapFeeAmount` functions, and the
 * `whenNotPaused` modifier.
 *
 * No admin permissions are checked here: instead, this contract delegates that to the Vault's own Authorizer.
 *
 * Because this contract doesn't implement the swap hooks, derived contracts should generally inherit from
 * BaseGeneralPool or BaseMinimalSwapInfoPool. Otherwise, subclasses must inherit from the corresponding interfaces
 * and implement the swap callbacks themselves.
 */

abi LegacyBasePool {
    /// Base Pools are expected to be deployed using factories. By using the factory address as the action
    /// disambiguator, we make all Pools deployed by the same factory share action identifiers. This allows for
    /// simpler management of permissions (such as being able to manage granting the 'set fee percentage' action in
    /// any Pool created by the same factory), while still making action identifiers unique among different factories
    /// if the selectors match, preventing accidental errors.
    /// Authentication(bytes32(uint256(msg.sender)))
    /// BalancerPoolToken(name, symbol, vault)
    /// BasePoolAuthorization(owner)
    /// TemporarilyPausable(pauseWindowDuration, bufferPeriodDuration)
    
     #[storage(read, write)]
    fn init_legacy_base_pool(
        specialization: PoolSpecialization,
        tokens: Vec<ContractId>,
        assetManagers: Vec<ContractId>,
        swapFeePercentage: u64,
        // pauseWindowDuration: u64,
        // bufferPeriodDuration: u64,
        owner: Identity    
    );

    /*
     * Set the swap fee percentage.
     * This is a permissioned fn, and disabled if the pool is paused. The swap fee must be within the
     * bounds set by MIN_SWAP_FEE_PERCENTAGE/MAX_SWAP_FEE_PERCENTAGE. Emits the SwapFeePercentageChanged event.
     */
    #[storage(read, write)]fn set_swap_fee_percentage(swapFeePercentage: u64);
    #[storage(read, write)]fn _set_swap_fee_percentage(swapFeePercentage: u64);
    fn set_asset_manager_pool_config(
        token: ContractId,
        //poolConfig: bytes
    );

    /*
     * Set the asset manager parameters for the given token.
     * This is a permissioned fn, unavailable when the pool is paused.
     * The details of the configuration data are set by each Asset Manager. (For an example, see
     * `RewardsAssetManager`.)
     */
    //Todo when Bytes operations are added
    fn _set_asset_manager_pool_config( 
        token: ContractId, 
        //poolConfig: bytes
    )
    /*
     * Pause the pool: an emergency action which disables all pool functions.
     * This is a permissioned fn that will only work during the Pause Window set during pool factory
     * deployment (see `TemporarilyPausable`).
     */
    #[storage(write)] fn pause();

    /*
     * Reverse a `pause` operation, and restore a pool to normal functionality.
     * This is a permissioned fn that will only work on a paused pool within the Buffer Period set during
     * pool factory deployment (see `TemporarilyPausable`). Note that any paused pools will automatically unpause after
     * the Buffer Period expires.
     */
    #[storage(write)] fn unpause();

    /*
     * Inserts data into the least-significant 192 bits of the misc data storage slot.
     * Note that the remaining 64 bits are used for the swap fee percentage and cannot be overloaded.
     */

    #[storage(read, write)]fn _set_misc_data(newData: b256);

    #[storage(read, write)]fn set_swap_fee_percentage(swapFeePercentage: u64);
    
    #[storage(read, write)]fn _set_swap_fee_percentage(swapFeePercentage: u64);
}
