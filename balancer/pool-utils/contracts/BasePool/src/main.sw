contract;

use std::{
    address::*,
    assert::assert,
    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    contract_id::ContractId,
    hash::*,
    result::*,
    revert::{revert, require},
    storage::*,
    token::*,
    u128::U128,
    vec::Vec,
    identity::Identity,
};

use BalancerErrors::*; //
use InputHelpers::{ensure_input_length_match}; //
// use WordCodec::*; //
// use TemporarilyPausable::*; 
use FixedPoint::*;
// use BalancerPoolToken::*; //
// // use BasePoolAuthorization::*;
// use RecoveryMode::*; //

use WeightedPool::WeightedPool;


storage {
    WeightedPool_contract_id: ContractId = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0,
}


abi MyContract {
    // fn get_pool_id() -> u8;
    // fn _get_minimum_bpt() -> u64;
    // fn get_swap_fee_percentage() -> u64;
    fn get_protocol_fees_collector() -> u64;
    fn set_swap_fee_percentage(swapFeePercentage: u64);
    fn set_asset_manager_pool_config(token: ContractId, poolConfig: Vec<b256>);
    fn pause();
    fn unpause();
    fn _is_owner_only_action(actionId: b256) -> bool;
    // fn _get_misc_data() -> b256;
    fn _set_misc_data(newData: b256);
    fn on_join_pool(
        poolId: u8,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    ) -> (Vec<u64>, Vec<u64>);
    fn on_exit_pool(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        serData: Vec<b256>
    ) -> (Vec<u64>, Vec<u64>);
    fn query_join(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    );
    fn query_exit(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    );
    fn _pay_protocol_fees(bptAmount: u64);
    fn _add_swap_fee_amount(amount: u64) -> u64;
    fn _subtract_swap_fee_amount(amount: u64) -> u64;
    fn _compute_scaling_factor(token: ContractId) -> u64;
    fn get_scaling_factors() -> Vec<u64>;
    fn _upscale(amount: u64, scalingFactor: u64) -> u64;
    // fn _upscale_array(amounts: Vec<u64>, scalingFactors: Vec<u64>);
    fn _downscale_down(amount: u64, scalingFactor: u64) -> u64;
    // fn _downscale_down_array(amounts: Vec<u64>, scalingFactors: Vec<u64>);
    fn _downscale_up(amount: u64, scalingFactor: u64) -> u64;
    // fn _downscale_up_array(amounts: Vec<u64>, scalingFactors: Vec<u64>);
}


storage {
    _mics_data: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    _setPaused: bool = false,
}

/**
 * @notice Reference implementation for the base layer of a Pool contract.
 * @dev Reference implementation for the base layer of a Pool contract that manages a single Pool with optional
 * Asset Managers, an admin-controlled swap fee percentage, and an emergency pause mechanism.
 *
 * This Pool pays protocol fees by minting BPT directly to the ProtocolFeeCollector instead of using the
 * `dueProtocolFees` return value. This results in the underlying tokens continuing to provide liquidity
 * for traders, while still keeping gas usage to a minimum since only a single token (the BPT) is transferred.
 *
 * Note that neither swap fees nor the pause mechanism are used by this contract. They are passed through so that
 * derived contracts can use them via the `_add_swap_fee_amount` and `_subtract_swap_fee_amount` fns, and the
 * `whenNotPaused` modifier.
 *
 * No admin permissions are checked here: instead, this contract delegates that to the Vault's own Authorizer.
 *
 * Because this contract doesn't implement the swap hooks, derived contracts should generally inherit from
 * BaseGeneralPool or BaseMinimalSwapInfoPool. Otherwise, subclasses must inherit from the corresponding interfaces
 * and implement the swap callbacks themselves.
 */

const _MIN_TOKENS: u64 =  2;

const _DEFAULT_MINIMUM_BPT: u64 =  1000000;

// 1e18 corresponds to 1.0, or a 100% fee
const _MIN_SWAP_FEE_PERCENTAGE: u64 =  1000000000000; // 0.0001%
const _MAX_SWAP_FEE_PERCENTAGE: u64 =  10000000000000000; // 10% - this fits in 64 bits

// Storage slot that can be used to store unrelated pieces of information. In particular, by default is used
// to store only the swap fee percentage of a pool. But it can be extended to store some more pieces of information.
// The swap fee percentage is stored in the most-significant 64 bits, therefore the remaining 192 bits can be
// used to store any other piece of information.

// const storage._mics_data: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _SWAP_FEE_PERCENTAGE_OFFSET: u64 =  192;

// // Note that this value is immutable in the Vault, so we can make it immutable here and save gas
const _PROTOCOL_FEES_COLLECTOR = 100;

const _POOL_ID: u8 = 0;                          

// adddress of owner
const _OWNER: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _DELEGATE_OWNER: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;

const SWAP_FEE_PERCENTAGE = 10;

const TOTAL_SUPPLY = 1000000000;


const _TOTAL_TOKENS = 100000000;
// const _SCALING_FACTOR0 = 10;
// const _SCALING_FACTOR1 = 10;
// const _SCALING_FACTOR2 = 10;
// const _SCALING_FACTOR3 = 10;
// const _SCALING_FACTOR4 = 10;
// const _SCALING_FACTOR5 = 10;
// const _SCALING_FACTOR6 = 10;
// const _SCALING_FACTOR7 = 10;
// const _SCALING_FACTOR8 = 10;
// const _SCALING_FACTOR9 = 10;
// const _SCALING_FACTOR10 = 10;
// const _SCALING_FACTOR11 = 10;
// const _SCALING_FACTOR12 = 10;
// const _SCALING_FACTOR13 = 10;
// const _SCALING_FACTOR14 = 10;
// const _SCALING_FACTOR15 = 10;
// const _SCALING_FACTOR16 = 10;
// const _SCALING_FACTOR17 = 10;
// const _SCALING_FACTOR18 = 10;
// const _SCALING_FACTOR19 = 10;

// const _NORMALIZED_WEIGHT0 = 10;
// const _NORMALIZED_WEIGHT1 = 10;
// const _NORMALIZED_WEIGHT2 = 10;
// const _NORMALIZED_WEIGHT3 = 10;
// const _NORMALIZED_WEIGHT4 = 10;
// const _NORMALIZED_WEIGHT5 = 10;
// const _NORMALIZED_WEIGHT6 = 10;
// const _NORMALIZED_WEIGHT7 = 10;
// const _NORMALIZED_WEIGHT8 = 10;
// const _NORMALIZED_WEIGHT9 = 10;
// const _NORMALIZED_WEIGHT10 = 10;
// const _NORMALIZED_WEIGHT11 = 10;
// const _NORMALIZED_WEIGHT12 = 10;
// const _NORMALIZED_WEIGHT13 = 10;
// const _NORMALIZED_WEIGHT14 = 10;
// const _NORMALIZED_WEIGHT15 = 10;
// const _NORMALIZED_WEIGHT16 = 10;
// const _NORMALIZED_WEIGHT17 = 10;
// const _NORMALIZED_WEIGHT18 = 10;
// const _NORMALIZED_WEIGHT19 = 10;
// fn get_owner() -> Address {
//     ~Address::from(_OWNER)
// }

// fn get_authorizer() -> IAuthorizer {
//     _getAuthorizer()
// }

// fn _can_perform(actionId: b256, account: Address) -> bool {
//     let sender: Result<Identity, AuthError> = msg_sender();
//     let sender: Address = match sender.unwrap() {
//         Identity::Address(addr) => {
//             assert(addr == ~Address::from(MINTER));
//             addr
//         },
//         _ => {
//             revert(0);
//         },
//     };
//     if ((get_owner() != ~Address::from(_DELEGATE_OWNER)) && _is_owner_only_action(actionId)) {
//         // Only the owner can perform "owner only" actions, unless the owner is delegated.
//         return addr == get_owner();
//     } else {
//         // Non-owner actions are always processed via the Authorizer, as "owner only" ones are when delegated.
//         _getAuthorizer().canPerform(actionId, account, addr)
//     }
// }

/**
    * @notice Return the pool id.
    */
fn get_pool_id() -> u8 {
    _POOL_ID
}

fn _set_swap_fee_percentage(swapFeePercentage: u64) {
    require(swapFeePercentage >= _get_min_swap_fee_percentage(), MIN_SWAP_FEE_PERCENTAGE);
    require(swapFeePercentage <= _get_max_swap_fee_percentage(), MAX_SWAP_FEE_PERCENTAGE);

    // let mut storage._mics_data = storage._mics_data.insertUint(swapFeePercentage, _SWAP_FEE_PERCENTAGE_OFFSET, 64);
}

fn _get_min_swap_fee_percentage() -> u64 {
    _MIN_SWAP_FEE_PERCENTAGE
}

fn _get_max_swap_fee_percentage() -> u64 {
    _MAX_SWAP_FEE_PERCENTAGE
}

fn _set_asset_manager_pool_config(token: ContractId, poolConfig: Vec<b256>) {
    let poolId = get_pool_id();
    let(_, _, _, assetManager) = getVault().getPoolTokenInfo(poolId, token);

    // IAssetManager(assetManager).setConfig(poolId, poolConfig);
}

/**
    * @dev Returns the minimum BPT supply. This amount is minted to the zero address during initialization, effectively
    * locking it.
    *
    * This is useful to make sure Pool initialization happens only once, but derived Pools can change this value (even
    * to zero) by overriding this fn.
    */
fn _get_minimum_bpt() -> u64 {
    _DEFAULT_MINIMUM_BPT
}

fn _get_total_tokens() -> u64 {
    return _TOTAL_TOKENS;
}

/**
    * @dev Same as `_upscale`, but for an entire array. This fn does not return anything, but instead *mutates*
    * the `amounts` array.
    */
fn _upscale_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = ~Vec::new();
    let mut count = 0;
    count = 0;
    while count < _get_total_tokens() {
        tmp.push(mul_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}

/**
* @dev Same as `_downscale_up`, but for an entire array. This fn does not return anything, but instead
* *mutates* the `amounts` array.
*/
fn _downscale_up_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = amounts;
    let mut count = 0;
    while count < _get_total_tokens() {
        tmp.push(div_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}

/**
* @dev Same as `_downscaleDown`, but for an entire array. This fn does not return anything, but instead
* *mutates* the `amounts` array.
*/
fn _downscale_down_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) -> Vec<u64> {
    let mut tmp = amounts;
    let mut count = 0;
    while count < _get_total_tokens() {
        tmp.push(div_down(amounts.get(count).unwrap(), scalingFactors.get(count).unwrap()));
        count = count + 1;
    }
    while count < amounts.len() {
        tmp.push(amounts.get(count).unwrap());
        count = count + 1;
    }
    return tmp;
}

/**
    * @notice Return the current value of the swap fee percentage.
    * @dev This is stored in the MSB 64 bits of the `storage._mics_data`.
    */
fn get_swap_fee_percentage() -> u64 {
    storage._mics_data;
}

fn _get_misc_data() -> b256 {
    storage._mics_data
}




// fn diff_before_join_exit(
//     preBalances: Vec<u64>,
//     normalizedWeights: Vec<u64>,
//     protocolSwapFeePercentage: u64
// ) {
//     // Before joins and exits, we measure the growth of the invariant compared to the invariant after the last join
//     // or exit, which will have been caused by swap fees, and use it to mint BPT as protocol fees. This dilutes all
//     // LPs, which means that new LPs will join the pool debt-free, and exiting LPs will pay any amounts due
//     // before leaving.

//     // We return immediately if the fee percentage is zero (to avoid unnecessary computation), or when the pool is
//     // paused (to avoid complex computation during emergency withdrawals).
//     if ((protocolSwapFeePercentage == 0) || !_isNotPaused()) {
//         return;
//     }

//     let preJoinExitInvariant = WeightedMath._calculateInvariant(normalizedWeights, preBalances);

//     let toMint = WeightedMath._calcDueProtocolSwapFeeBptAmount(
//         totalSupply(),
//         _lastPostJoinExitInvariant,
//         preJoinExitInvariant,
//         protocolSwapFeePercentage
//     );

//     _payProtocolFees(toMint);
// }


// // InvariantGrowthProtocolFees
// fn _before_join_exit(
//     preBalances: Vec<u64>,
//     normalizedWeights: Vec<u64>,
//     protocolSwapFeePercentage: u64
// ) {
//     diff_before_join_exit(preBalances, normalizedWeights, protocolSwapFeePercentage);
// }

// fn _get_normalized_weights() -> Vec<u64> {
//     let totalTokens = _get_total_tokens();
//     let mut normalizedWeights = ~Vec::new();

//     normalizedWeights.push(_NORMALIZED_WEIGHT0);
//     normalizedWeights.push(_NORMALIZED_WEIGHT1);
//     if (totalTokens > 2) { normalizedWeights.push(_NORMALIZED_WEIGHT2); }
//     if (totalTokens > 3) { normalizedWeights.push(_NORMALIZED_WEIGHT3); }
//     if (totalTokens > 4) { normalizedWeights.push(_NORMALIZED_WEIGHT4); }
//     if (totalTokens > 5) { normalizedWeights.push(_NORMALIZED_WEIGHT5); }
//     if (totalTokens > 6) { normalizedWeights.push(_NORMALIZED_WEIGHT6); }
//     if (totalTokens > 7) { normalizedWeights.push(_NORMALIZED_WEIGHT7); }
//     if (totalTokens > 8) { normalizedWeights.push(_NORMALIZED_WEIGHT8); }
//     if (totalTokens > 9) { normalizedWeights.push(_NORMALIZED_WEIGHT9); }
//     if (totalTokens > 11) { normalizedWeights.push(_NORMALIZED_WEIGHT11); }
//     if (totalTokens > 10) { normalizedWeights.push(_NORMALIZED_WEIGHT10); }
//     if (totalTokens > 12) { normalizedWeights.push(_NORMALIZED_WEIGHT12); }
//     if (totalTokens > 13) { normalizedWeights.push(_NORMALIZED_WEIGHT13); }
//     if (totalTokens > 14) { normalizedWeights.push(_NORMALIZED_WEIGHT14); }
//     if (totalTokens > 15) { normalizedWeights.push(_NORMALIZED_WEIGHT15); }
//     if (totalTokens > 16) { normalizedWeights.push(_NORMALIZED_WEIGHT16); }
//     if (totalTokens > 17) { normalizedWeights.push(_NORMALIZED_WEIGHT17); }
//     if (totalTokens > 18) { normalizedWeights.push(_NORMALIZED_WEIGHT18); }
//     if (totalTokens > 19) { normalizedWeights.push(_NORMALIZED_WEIGHT19); }

//     let mut count = totalTokens;
//     while count < normalizedWeights.len() {
//         normalizedWeights.push(0);
//     }
//     return normalizedWeights;
// }



// fn _on_join_pool(
//     // poolId: u8,
//     sender: Address,
//     // recipient: Address,
//     balances: Vec<u64>,
//     // lastChangeBlock: u64,
//     protocolSwapFeePercentage: u64,
//     scalingFactors: Vec<u64>,
//     serData: Vec<b256>
// ) -> (u64, Vec<u64>) {
//     // All joins are disabled while the contract is paused.

//     let normalizedWeights: Vec<u64> = _get_normalized_weights();

//     _before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
//     let(bptAmountOut, amountsIn) = _doJoin(
//         sender,
//         balances,
//         normalizedWeights,
//         scalingFactors,
//         userData
//     );
//     _afterJoinExit(true, balances, amountsIn, normalizedWeights);

//     (bptAmountOut, amountsIn)
// }


// fn _on_exit_pool(
//     // poolId: u8,
//     sender: Address,
//     // recipient: Address,
//     balances: Vec<u64>,
//     // lastChangeBlock: u64,
//     protocolSwapFeePercentage: u64,
//     calingFactors: Vec<u64>,
//     serData: Vec<b256>
// ) -> (u64, Vec<u64>) {
//     // Exits are not disabled by default while the contract is paused, as some of them remain available to allow LPs
//     // to safely exit the Pool in case of an emergency. Other exit kinds are disabled on a case-by-case basis in
//     // their handlers.

//     let normalizedWeights = _get_normalized_weights();

//     _before_join_exit(balances, normalizedWeights, protocolSwapFeePercentage);
//     let(bptAmountIn, amountsOut) = _doExit(
//         sender,
//         balances,
//         normalizedWeights,
//         scalingFactors,
//         userData
//     );
//     _afterJoinExit(false, balances, amountsOut, normalizedWeights);

//     (bptAmountIn, amountsOut)
// }

// fn _scaling_factors() -> Vec<u64> {
//     let totalTokens = _get_total_tokens();
//     let mut scalingFactors = ~Vec::new();

//     scalingFactors.push(_SCALING_FACTOR0);
//     scalingFactors.push(_SCALING_FACTOR1);
//     if (totalTokens > 2) { scalingFactors.push(_SCALING_FACTOR2); } 
//     if (totalTokens > 3) { scalingFactors.push(_SCALING_FACTOR3); } 
//     if (totalTokens > 4) { scalingFactors.push(_SCALING_FACTOR4); } 
//     if (totalTokens > 5) { scalingFactors.push(_SCALING_FACTOR5); } 
//     if (totalTokens > 6) { scalingFactors.push(_SCALING_FACTOR6); } 
//     if (totalTokens > 7) { scalingFactors.push(_SCALING_FACTOR7); } 
//     if (totalTokens > 8) { scalingFactors.push(_SCALING_FACTOR8); } 
//     if (totalTokens > 9) { scalingFactors.push(_SCALING_FACTOR9); } 
//     if (totalTokens > 10) { scalingFactors.push(_SCALING_FACTOR10); } 
//     if (totalTokens > 11) { scalingFactors.push(_SCALING_FACTOR11); } 
//     if (totalTokens > 12) { scalingFactors.push(_SCALING_FACTOR12); } 
//     if (totalTokens > 13) { scalingFactors.push(_SCALING_FACTOR13); } 
//     if (totalTokens > 14) { scalingFactors.push(_SCALING_FACTOR14); } 
//     if (totalTokens > 15) { scalingFactors.push(_SCALING_FACTOR15); } 
//     if (totalTokens > 16) { scalingFactors.push(_SCALING_FACTOR16); } 
//     if (totalTokens > 17) { scalingFactors.push(_SCALING_FACTOR17); } 
//     if (totalTokens > 18) { scalingFactors.push(_SCALING_FACTOR18); } 
//     if (totalTokens > 19) { scalingFactors.push(_SCALING_FACTOR19); } 

//     let mut count = scalingFactors.len();
//     while count < totalTokens {
//         scalingFactors.push(0);
//         count = count + 1;
//     }

//     return scalingFactors;
// }


impl MyContract for Contract {
    // using WordCodec for bytes32;
    // using FixedPoint for uint256;


    // bytes32 private immutable _POOL_ID;

    // Getters / Setters

    // fn _get_total_tokens() internal view virtual returns (uint256);

    // fn _getMaxTokens() -> u64;


    /**
     * @notice Return the ProtocolFeesCollector contract.
     * @dev This is immutable, and retrieved from the Vault on construction. (It is also immutable in the Vault.)
     */
    fn get_protocol_fees_collector() -> u64 {
        _PROTOCOL_FEES_COLLECTOR
    }


    /**
     * @notice Set the swap fee percentage.
     * @dev This is a permissioned fn, and disabled if the pool is paused. The swap fee must be within the
     * bounds set by MIN_SWAP_FEE_PERCENTAGE/MAX_SWAP_FEE_PERCENTAGE. Emits the SwapFeePercentageChanged event.
     */
    fn set_swap_fee_percentage(swapFeePercentage: u64) {
        _set_swap_fee_percentage(swapFeePercentage);
    }

    /**
     * @notice Set the asset manager parameters for the given token.
     * @dev This is a permissioned fn, unavailable when the pool is paused.
     * The details of the configuration data are set by each Asset Manager. (For an example, see
     * `RewardsAssetManager`.)
     */
    fn set_asset_manager_pool_config(token: ContractId, poolConfig: Vec<b256>) {
        _set_asset_manager_pool_config(token, poolConfig);
    }

    /**
     * @notice Pause the pool: an emergency action which disables all pool fns.
     * @dev This is a permissioned fn that will only work during the Pause Window set during pool factory
     * deployment (see `TemporarilyPausable`).
     */
    fn pause() {
        storage._setPaused= true;
    }

    /**
     * @notice Reverse a `pause` operation, and restore a pool to normal fnality.
     * @dev This is a permissioned fn that will only work on a paused pool within the Buffer Period set during
     * pool factory deployment (see `TemporarilyPausable`). Note that any paused pools will automatically unpause
     * after the Buffer Period expires.
     */
    fn unpause() {
        storage._setPaused = false;
    }

    fn _is_owner_only_action(actionId: b256) -> bool {
        if actionId == getActionId(this.set_swap_fee_percentage.selector) || actionId == getActionId(this.set_asset_manager_pool_config.selector) {
            true
        }
        else {
            false
        }
        
    }

    /**
     * @dev Inserts data into the least-significant 192 bits of the misc data storage slot.
     * Note that the remaining 64 bits are used for the swap fee percentage and cannot be overloaded.
     */
    fn _set_misc_data(newData: b256) {
        storage._mics_data = storage._mics_data.insertBits192(newData, 0);
    }

    // Join / Exit Hooks

    // modifier onlyVault(poolId: b256) {
    //     require(msg_sender() == address(getVault()), CALLER_NOT_VAULT);
    //     require(poolId == get_pool_id(), INVALID_POOL_ID);
    //     _;
    // }

    /**
     * @notice Vault hook for adding liquidity to a pool (including the first time, "initializing" the pool).
     * @dev This fn can only be called from the Vault, from `joinPool`.
     */
    fn on_join_pool(
        poolId: u8,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    ) -> (Vec<u64>, Vec<u64>) {
        // _scaling_factors function exist in BaseWeightedPool contract
        let x = abi(WeightedPool, WeightedPool_contract_id);
        let scalingFactors = x._scaling_factors();

        // if (totalSupply() == 0) {
        // _onInitializePool exist BaseWeightedPool cotract 
        if TOTAL_SUPPLY == 0 {
            let(bptAmountOut, amountsIn) = _onInitializePool(
                poolId,
                sender,
                recipient,
                scalingFactors,
                userData
            );

            // On initialization, we lock _get_minimum_bpt() by minting it for the zero address. This BPT acts as a
            // minimum as it will never be burned, which reduces potential issues with rounding, and also prevents the
            // Pool from ever being fully drained.
            require(bptAmountOut >= _get_minimum_bpt(), MINIMUM_BPT);
            _mintPoolTokens(address(0), _get_minimum_bpt());
            _mintPoolTokens(recipient, bptAmountOut - _get_minimum_bpt());

            // amountsIn are amounts entering the Pool, so we round up.
            let amountsIn = _downscale_up_array(amountsIn, scalingFactors);

            return (amountsIn, ~Vec::with_capacity(balances.len()));
        } else {
            let balances = _upscale_array(balances, scalingFactors);
            // _on_join_pool function exist in BaseWightedPool
            let(bptAmount, amountsIn) = _on_join_pool(
                // poolId,
                sender,
                // recipient,
                balances,
                // lastChangeBlock,
                protocolSwapFeePercentage,
                scalingFactors,
                userData
            );

            // Note we no longer use `balances` after calling `_on_join_pool`, which may mutate it.

            _mintPoolTokens(recipient, bptAmountOut);

            // amountsIn are amounts entering the Pool, so we round up.
            let amountsIn = _downscale_up_array(amountsIn, scalingFactors);

            // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
            return (amountsIn, ~Vec::with_capacity(balances.len()));
        }
    }

    /**
     * @notice Vault hook for removing liquidity from a pool.
     * @dev This fn can only be called from the Vault, from `exitPool`.
     */
    fn on_exit_pool(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    ) -> (Vec<u64>, Vec<u64>) {
        let scalingFactors = _scaling_factors();
        let balances = _upscale_array(balances, scalingFactors);

        let(bptAmountIn, amountsOut) = _on_exit_pool(
            // poolId,
            sender,
            // recipient,
            balances,
            // lastChangeBlock,
            protocolSwapFeePercentage,
            scalingFactors,
            userData
        );

        // Note we no longer use `balances` after calling `_on_exit_pool`, which may mutate it.

        _burnPoolTokens(sender, bptAmountIn);

        // amountsOut are amounts exiting the Pool, so we round down.
        let amountsOut = _downscale_down_array(amountsOut, scalingFactors);

        // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
        (amountsOut, ~Vec::with_capacity(balances.len()))
    }

    // Query fns

    /**
     * @notice "Dry run" `on_join_pool`.
     * @dev Returns the amount of BPT that would be granted to `recipient` if the `on_join_pool` hook were called by the
     * Vault with the same arguments, along with the number of tokens `sender` would have to supply.
     *
     * This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
     * data, such as the protocol swap fee percentage and Pool balances.
     *
     * Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
     * explicitly use eth_call instead of eth_sendTransaction.
     */
    fn query_join(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    ) {
        ensure_input_length_match(balances.len(), _get_total_tokens());

        _queryAction(
            poolId,
            sender,
            recipient,
            balances,
            lastChangeBlock,
            protocolSwapFeePercentage,
            userData,
            _on_join_pool,
            _downscale_up
        );

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        // (bptOut, amountsIn)
    }

    /**
     * @notice "Dry run" `on_exit_pool`.
     * @dev Returns the amount of BPT that would be burned from `sender` if the `on_exit_pool` hook were called by the
     * Vault with the same arguments, along with the number of tokens `recipient` would receive.
     *
     * This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
     * data, such as the protocol swap fee percentage and Pool balances.
     *
     * Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
     * explicitly use eth_call instead of eth_sendTransaction.
     */
    fn query_exit(
        poolId: u8,
        sender: Address,
        arecipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: Vec<b256>
    ) {
        ensure_input_length_match(balances.len(), _get_total_tokens());

        _queryAction(
            poolId,
            sender,
            recipient,
            balances,
            lastChangeBlock,
            protocolSwapFeePercentage,
            userData,
            _on_exit_pool,
            _downscale_down_array
        );

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        // (bptIn, amountsOut)
    }

    // Internal hooks to be overridden by derived contracts - all token amounts (except BPT) in these interfaces are
    // upscaled.

    /**
     * @dev Called when the Pool is joined for the first time; that is, when the BPT total supply is zero.
     *
     * Returns the amount of BPT to mint, and the token amounts the Pool will receive in return.
     *
     * Minted BPT will be sent to `recipient`, except for _get_minimum_bpt(), which will be deducted from this amount and
     * sent to the zero address instead. This will cause that BPT to remain forever locked there, preventing total BTP
     * from ever dropping below that value, and ensuring `_onInitializePool` can only be called once in the entire
     * Pool's lifetime.
     *
     * The tokens granted to the Pool will be transferred from `sender`. These amounts are considered upscaled and will
     * be downscaled (rounding up) before being returned to the Vault.
     */
    // fn _onInitializePool(
    //     poolId: u8,
    //     sender: Address,
    //     arecipient: Address,
    //     scalingFactors: Vec<u64>,
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);

    /**
     * @dev Called whenever the Pool is joined after the first initialization join (see `_onInitializePool`).
     *
     * Returns the amount of BPT to mint, the token amounts that the Pool will receive in return, and the number of
     * tokens to pay in protocol swap fees.
     *
     * Implementations of this fn might choose to mutate the `balances` array to save gas (e.g. when
     * performing intermediate calculations, such as subtraction of due protocol fees). This can be done safely.
     *
     * Minted BPT will be sent to `recipient`.
     *
     * The tokens granted to the Pool will be transferred from `sender`. These amounts are considered upscaled and will
     * be downscaled (rounding up) before being returned to the Vault.
     *
     * Due protocol swap fees will be taken from the Pool's balance in the Vault (see `IBasePool.on_join_pool`). These
     * amounts are considered upscaled and will be downscaled (rounding down) before being returned to the Vault.
     */
    // fn _on_join_pool(
    //     poolId: u8,
    //     sender: Address,
    //     arecipient: Address,
    //     balances: Vec<u64>,
    //     lastChangeBlock: u64,
    //     protocolSwapFeePercentage: u64,
    //     scalingFactors: Vec<u64>
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);

    /**
     * @dev Called whenever the Pool is exited.
     *
     * Returns the amount of BPT to burn, the token amounts for each Pool token that the Pool will grant in return, and
     * the number of tokens to pay in protocol swap fees.
     *
     * Implementations of this fn might choose to mutate the `balances` array to save gas (e.g. when
     * performing intermediate calculations, such as subtraction of due protocol fees). This can be done safely.
     *
     * BPT will be burnt from `sender`.
     *
     * The Pool will grant tokens to `recipient`. These amounts are considered upscaled and will be downscaled
     * (rounding down) before being returned to the Vault.
     *
     * Due protocol swap fees will be taken from the Pool's balance in the Vault (see `IBasePool.on_exit_pool`). These
     * amounts are considered upscaled and will be downscaled (rounding down) before being returned to the Vault.
     */
    // fn _on_exit_pool(
    //     poolId: u8,
    //     sender: Address,
    //     arecipient: Address,
    //     balances: Vec<u64>,
    //     lastChangeBlock: u64,
    //     protocolSwapFeePercentage: u64,
    //     scalingFactors: Vec<u64>
    //     userData: Vec<b256>
    // ) -> (u64, Vec<u64>);

    // Internal fns

    /**
     * @dev Pays protocol fees by minting `bptAmount` to the Protocol Fee Collector.
     */
    fn _pay_protocol_fees(bptAmount: u64) {
        _mintPoolTokens(address(get_protocol_fees_collector()), bptAmount);
    }

    /**
     * @dev Adds swap fee amount to `amount`, returning a higher value.
     */
    fn _add_swap_fee_amount(amount: u64) -> u64 {
        // This returns amount + fee amount, so we round up (favoring a higher fee amount).
        div_up(amount, sub(ONE, get_swap_fee_percentage()))
    }

    /**
     * @dev Subtracts swap fee amount from `amount`, returning a lower value.
     */
    fn _subtract_swap_fee_amount(amount: u64) -> u64 {
        // This returns amount - fee amount, so we round up (favoring a higher fee amount).
        let feeAmount = mul_up(amount, get_swap_fee_percentage());
        sub(amount, feeAmount)
    }

    // Scaling

    /**
     * @dev Returns a scaling factor that, when multiplied to a token amount for `token`, normalizes its balance as if
     * it had 18 decimals.
     */
    fn _compute_scaling_factor(token: ContractId) -> u64 {
        // if (address(token) == address(this)) {
        //     ONE
        // }

        // Tokens that don't implement the `decimals` method are not supported.
        // uint256 tokenDecimals = ERC20(address(token)).decimals();

        // Tokens with more than 18 decimals are not supported.

        // -> -> -> -> -> -> -> -> fuel doesn't have 18 dacimals in token so I made it to 0, may can change ion future
        let tokenDecimals = 0;
        let decimalsDifference = sub(1, tokenDecimals);
        // return ONE * 10**decimalsDifference;\
        return ONE * 10;
    }

    /**
     * @dev Returns the scaling factor for one of the Pool's tokens. Reverts if `token` is not a token registered by the
     * Pool.
     *
     * All scaling factors are fixed-point values with 18 decimals, to allow for this fn to be overridden by
     * derived contracts that need to apply further scaling, making these factors potentially non-integer.
     *
     * The largest 'base' scaling factor (i.e. in tokens with less than 18 decimals) is 10**18, which in fixed-point is
     * 10**36. This value can be multiplied with a 112 bit Vault balance with no overflow by a factor of ~1e7, making
     * even relatively 'large' factors safe to use.
     *
     * The 1e7 figure is the result of 2**256 / (1e18 * 1e18 * 2**112).
     */
    // fn _scalingFactor(IERC20 token) internal view virtual returns (uint256);

    /**
     * @dev Same as `_scalingFactor()`, except for all registered tokens (in the same order as registered). The Vault
     * will always pass balances in this order when calling any of the Pool hooks.
     */
    // fn _scaling_factors() internal view virtual returns (uint256[] memory);

    fn get_scaling_factors() -> Vec<u64> {
        _scaling_factors()
    }

    /**
     * @dev Applies `scalingFactor` to `amount`, resulting in a larger or equal value depending on whether it needed
     * scaling or not.
     */
    fn _upscale(amount: u64, scalingFactor: u64) -> u64 {
        // Upscale rounding wouldn't necessarily always go in the same direction: in a swap for example the balance of
        // token in should be rounded up, and that of token out rounded down. This is the only place where we round in
        // the same direction for all amounts, as the impact of this rounding is expected to be minimal (and there's no
        // rounding error unless `_scalingFactor()` is overriden).
        mul_down(amount, scalingFactor)
    }

    /**
     * @dev Reverses the `scalingFactor` applied to `amount`, resulting in a smaller or equal value depending on
     * whether it needed scaling or not. The result is rounded down.
     */
    fn _downscale_down(amount: u64, scalingFactor: u64) -> u64 {
        div_down(amount, scalingFactor)
    }

    /**
     * @dev Reverses the `scalingFactor` applied to `amount`, resulting in a smaller or equal value depending on
     * whether it needed scaling or not. The result is rounded up.
     */
    fn _downscale_up(amount: u64, scalingFactor: u64) -> u64 {
        div_up(amount, scalingFactor)
    }

    // fn _getAuthorizer() internal view override returns (IAuthorizer) {
    //     // Access control management is delegated to the Vault's Authorizer. This lets Balancer Governance manage which
    //     // accounts can call permissioned fns: for example, to perform emergency pauses.
    //     // If the owner is delegated, then *all* permissioned fns, including `set_swap_fee_percentage`, will be under
    //     // Governance control.
    //     return getVault().get_authorizer();
    // }

    // fn _queryAction(
    //     poolId: u8,
    //     sender: Address,
    //     arecipient: Address,
    //     balances: Vec<u64>,
    //     lastChangeBlock: u64,
    //     protocolSwapFeePercentage: u64,
    //     userData: Vec<b256>,
    //     fn(bytes32, address, address, uint256[] memory, uint256, uint256, uint256[] memory, bytes memory)
    //         internal
    //         returns (uint256, uint256[] memory) _action,
    //     fn(uint256[] memory, uint256[] memory) internal view _downscaleArray
    // ) private {
    //     // This uses the same technique used by the Vault in queryBatchSwap. Refer to that fn for a detailed
    //     // explanation.

    //     if (msg.sender != address(this)) {
    //         // We perform an external call to ourselves, forwarding the same calldata. In this call, the else clause of
    //         // the preceding if statement will be executed instead.

    //         // solhint-disable-next-line avoid-low-level-calls
    //         (bool success, ) = address(this).call(msg.data);

    //         // solhint-disable-next-line no-inline-assembly
    //         assembly {
    //             // This call should always revert to decode the bpt and token amounts from the revert reason
    //             switch success
    //                 case 0 {
    //                     // Note we are manually writing the memory slot 0. We can safely overwrite whatever is
    //                     // stored there as we take full control of the execution and then immediately return.

    //                     // We copy the first 4 bytes to check if it matches with the expected signature, otherwise
    //                     // there was another revert reason and we should forward it.
    //                     returndatacopy(0, 0, 0x04)
    //                     let error := and(mload(0), 0xffffffff00000000000000000000000000000000000000000000000000000000)

    //                     // If the first 4 bytes don't match with the expected signature, we forward the revert reason.
    //                     if eq(eq(error, 0x43adbafb00000000000000000000000000000000000000000000000000000000), 0) {
    //                         returndatacopy(0, 0, returndatasize())
    //                         revert(0, returndatasize())
    //                     }

    //                     // The returndata contains the signature, followed by the raw memory representation of the
    //                     // `bptAmount` and `tokenAmounts` (array: length + data). We need to return an ABI-encoded
    //                     // representation of these.
    //                     // An ABI-encoded response will include one additional field to indicate the starting offset of
    //                     // the `tokenAmounts` array. The `bptAmount` will be laid out in the first word of the
    //                     // returndata.
    //                     //
    //                     // In returndata:
    //                     // [ signature ][ bptAmount ][ tokenAmounts length ][ tokenAmounts values ]
    //                     // [  4 bytes  ][  32 bytes ][       32 bytes      ][ (32 * length) bytes ]
    //                     //
    //                     // We now need to return (ABI-encoded values):
    //                     // [ bptAmount ][ tokeAmounts offset ][ tokenAmounts length ][ tokenAmounts values ]
    //                     // [  32 bytes ][       32 bytes     ][       32 bytes      ][ (32 * length) bytes ]

    //                     // We copy 32 bytes for the `bptAmount` from returndata into memory.
    //                     // Note that we skip the first 4 bytes for the error signature
    //                     returndatacopy(0, 0x04, 32)

    //                     // The offsets are 32-bytes long, so the array of `tokenAmounts` will start after
    //                     // the initial 64 bytes.
    //                     mstore(0x20, 64)

    //                     // We now copy the raw memory array for the `tokenAmounts` from returndata into memory.
    //                     // Since bpt amount and offset take up 64 bytes, we start copying at address 0x40. We also
    //                     // skip the first 36 bytes from returndata, which correspond to the signature plus bpt amount.
    //                     returndatacopy(0x40, 0x24, sub(returndatasize(), 36))

    //                     // We finally return the ABI-encoded uint256 and the array, which has a total length equal to
    //                     // the size of returndata, plus the 32 bytes of the offset but without the 4 bytes of the
    //                     // error signature.
    //                     return(0, add(returndatasize(), 28))
    //                 }
    //                 default {
    //                     // This call should always revert, but we fail nonetheless if that didn't happen
    //                     invalid()
    //                 }
    //         }
    //     } else {
    //         scalingFactors: Vec<u64> = _scaling_factors();
    //         let balances = _upscale_array(balances, scalingFactors);

    //         (bptAmount: u64, uint256[] memory tokenAmounts) = _action(
    //             poolId,
    //             sender,
    //             recipient,
    //             balances,
    //             lastChangeBlock,
    //             protocolSwapFeePercentage,
    //             scalingFactors,
    //             userData
    //         );

    //         _downscaleArray(tokenAmounts, scalingFactors);

    //         // solhint-disable-next-line no-inline-assembly
    //         assembly {
    //             // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
    //             // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
    //             // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
    //             let size := mul(mload(tokenAmounts), 32)

    //             // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
    //             // will be at least one available slot due to how the memory scratch space works.
    //             // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
    //             let start := sub(tokenAmounts, 0x20)
    //             mstore(start, bptAmount)

    //             // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
    //             // We use the previous slot to `bptAmount`.
    //             mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
    //             start := sub(start, 0x04)

    //             // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
    //             // the `bptAmount`, the array 's length, and the error signature.
    //             revert(start, add(size, 68))
    //         }
    //     }
    // }
}
