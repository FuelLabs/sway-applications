contract;

dep data_structure;
dep interface;
use utils;
dep errors;
use data_structure::{JoinPoolRequest,TargetsSet};
use errors::{LinearPoolErrors};
use interface::{
    LinearPool,
    _TOTAL_TOKENS,
    _INITIAL_BPT_SUPPLY,
    TOTAL_SUPPLY,
    _LOWER_TARGET_OFFSET,
    _UPPER_TARGET_OFFSET,
    _MAX_UPPER_TARGET
};

use std::{
    address::Address,
    contract_id::ContractId,
    revert::{revert, require},
    vec::Vec,
    identity::Identity,
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    option::Option,
};
use LinearMath::{
    Params,
    _calc_bpt_in_per_main_out,
    _calc_bpt_out_per_wrapped_in,
    _calc_bpt_out_per_main_in,
    _calc_bpt_in_per_wrapped_out,
    _calc_invariant,
    _to_nominal
};

use WordCodec::{decode_uint, encode_uint};

use FixedPoint::{
    mul_down,
    ONE,
    sub,
    div_Up
};
use Authentication::{_authenticate_caller};
use BaseMinimalSwapInfoPool::{swapRequest};
use ERC20Helper::{_sort_tokens, _get_sorted_token_indexes};
use LegacyBasePool::{
    LegacyBasePool,
    _compute_scaling_factor,
    PoolSpecialization,
    get_pool_id,
    only_vault,
    _upscale,
    _downscale_up,
    _downscale_down,
    _get_misc_data,
    get_swap_fee_percentage,
};

use PoolTokens::{PoolTokens};
use TemporarilyPausable::{TemporarilyPausable};
use BalancerPoolToken::{BalancerPoolToken};



// Storage Variables
storage {
    // The indices of each token when registered, which can then be used to access the balances array.
    _mainIndex: u64,
    _bptIndex: u64,
    _wrappedIndex: u64,
    _mainToken: ContractId,
    _wrappedToken: ContractId,
    _scalingFactorMainToken: u64,
    _scalingFactorWrappedToken: u64,
    BalancerPoolTokenContractId: ContractId = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    TemporarilyPausableContractId: ContractId = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    PoolTokensContractId: ContractId = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    LegacyBasePoolContractId: ContractId = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    AuthenticationContractId: ContractId = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
}

#[storage(read)]
pub fn get_main_token()->ContractId {
    return storage._mainToken;
}

pub fn total_supply()-> u64{
    return TOTAL_SUPPLY;
}

#[storage(read)]
pub fn get_wrapped_token()->ContractId {
    return storage._wrappedToken;
}

#[storage(read)]
pub fn get_bpt_index()->u64 {
    return storage._bptIndex;
}

#[storage(read)]
pub fn get_main_index()->u64 {
    return storage._mainIndex;
}

#[storage(read)]
pub fn get_wrapped_index() ->u64 {
    return storage._wrappedIndex;
}

#[storage(read)]
fn _on_swap_given_in(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
)->u64 {
    if (request.tokenIn == contract_id()) {
        return _swap_given_bpt_in(request, balances, params);
    } else if (request.tokenIn == _mainToken) {
        return _swap_given_main_in(request, balances, params);
    } else if (request.tokenIn == _wrappedToken) {
        return _swap_given_wrapped_in(request, balances, params);
    } else {
        revert(INVALID_TOKEN);
    }
}


#[storage(read)]
fn _swap_given_bpt_in(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
) ->u64 {
    require(request.tokenOut == _mainToken || request.tokenOut == _wrappedToken, LinearPoolErrors::INVALID_TOKEN);
    return
        (request.tokenOut == _mainToken ? _calc_main_out_per_bpt_in : _calc_wrapped_out_per_bpt_in)(
            request.amount,
            balances.get(_mainIndex).unwrap(),
            balances.get(_wrappedIndex).unwrap(),
            _get_approximate_virtual_supply(balances.get(_bptIndex).unwrap()),
            params
        );
}

#[storage(read)]
fn _swap_given_main_in(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
) ->u64 {
    require(request.tokenOut == _wrappedToken || request.tokenOut == this, LinearPoolErrors::INVALID_TOKEN);
    return
        request.tokenOut == contract_id()
            ? LinearMath._calc_bpt_out_per_main_in(
                request.amount,
                balances.get(_mainIndex).unwrap(),
                balances.get(_wrappedIndex).unwrap(),
                _get_approximate_virtual_supply(balances.get(_bptIndex).unwrap()),
                params
            )
            : _calc_wrapped_out_per_main_in(request.amount, balances.get(_mainIndex).unwrap(), params);
}

#[storage(read)]
fn _swap_given_wrapped_in(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
) ->u64 {
    require(request.tokenOut == _mainToken || request.tokenOut == this, LinearPoolErrors::INVALID_TOKEN);
    return
        request.tokenOut == this
            ? LinearMath._calc_bpt_out_per_wrapped_in(
                request.amount,
                balances.get(_mainIndex).unwrap(),
                balances.get(_wrappedIndex).unwrap(),
                _get_approximate_virtual_supply(balances.get(_bptIndex).unwrap()),
                params
            )
            : _calcMainOutPerWrappedIn(request.amount, balances.get(_mainIndex).unwrap(), params);
}

#[storage(read)]
fn _on_swap_given_out(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
)->u64 {
    if (request.tokenOut == this) {
        return _swap_given_bpt_out(request, balances, params);
    } else if (request.tokenOut == _mainToken) {
        return _swap_given_main_out(request, balances, params);
    } else if (request.tokenOut == _wrappedToken) {
        return _swap_given_wrapped_out(request, balances, params);
    } else {
        revert(INVALID_TOKEN);
    }
}

#[storage(read)]
fn _swap_given_bpt_out(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
)->u64 {
    require(request.tokenIn == _mainToken || request.tokenIn == _wrappedToken, LinearPoolErrors::INVALID_TOKEN);
    return
        (request.tokenIn == _mainToken ? LinearMath._calcMainInPerBptOut : LinearMath._calcWrappedInPerBptOut)(
            request.amount,
            balances.get(_mainIndex).unwrap(),
            balances.get(_wrappedIndex).unwrap(),
            _get_approximate_virtual_supply(balances.get(_bptIndex).unwrap()),
            params
        );
}

#[storage(read)]
fn _swap_given_main_out(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
)->u64 {
    _require(request.tokenIn == _wrappedToken || request.tokenIn == this, LinearPoolErrors::INVALID_TOKEN);
    return
        request.tokenIn == this
            ? _calc_bpt_in_per_main_out(
                request.amount,
                balances.get(_mainIndex).unwrap(),
                balances.get(_wrappedIndex).unwrap(),
                _get_approximate_virtual_supply(balances.get(_bptIndex).unwrap()),
                params
            )
            :_calc_wrapped_in_per_main_out(request.amount, balances.get(_mainIndex).unwrap(), params);
}

#[storage(read)]
fn _swap_given_wrapped_out(
    request: SwapRequest,
    balances: Vec<u64>,
    params: Params
) ->u64 {
    require(request.tokenIn == _mainToken || request.tokenIn == this, LinearPoolErrors::INVALID_TOKEN);
    return
        request.tokenIn == this
            ? _calc_bpt_in_per_wrapped_out(
                request.amount,
                balances[_mainIndex],
                balances[_wrappedIndex],
                _get_approximate_virtual_supply(balances[_bptIndex]),
                params
            )
            : _calc_main_in_per_wrapped_out(request.amount, balances.get(_mainIndex).unwrap(), params);
}

fn _get_max_tokens() ->u64 {
    return _TOTAL_TOKENS;
}

fn _get_minimum_bpt() ->u64 {
    // Linear Pools don't lock any BPT, as the total supply will already be forever non-zero due to the preminting
    // mechanism, ensuring initialization only occurs once.
    return 0;
}

fn _get_total_tokens() ->u64 {
    return _TOTAL_TOKENS;
}

fn _scaling_factor(token: ContractId) ->u64 {
    if (token == _mainToken) {
        return _scalingFactorMainToken;
    } else if (token == _wrappedToken) {
        // The wrapped token's scaling factor is not constant, but increases over time as the wrapped token
        // increases in value.
        return mul_down(_scalingFactorWrappedToken, _get_wrapped_token_rate());
    } else if (token == contract_id()) {
        return ONE;
    } else {
        revert(INVALID_TOKEN);
    }
}



fn _scaling_factors() ->Vec<u64> {
    let mut scalingFactors: Vec<u64> = ~Vec::with_capacity(_TOTAL_TOKENS);

    // The wrapped token's scaling factor is not constant, but increases over time as the wrapped token increases in
    // value.
    scalingFactors.insert(_mainIndex, _scalingFactorMainToken);
    scalingFactors.insert(_wrappedIndex, mul_down(_scalingFactorWrappedToken, _get_wrapped_token_rate()));
    scalingFactors.insert(_bptIndex,ONE );

    return scalingFactors;
}

fn get_scaling_factors() ->Vec<u64> {
    return _scaling_factors();
}

// Price rates


/// * For a Linear Pool, the rate represents the appreciation of BPT with respect to the underlying tokens. This
/// * rate increases slowly as the wrapped token appreciates in value.
///
fn get_rate() ->u64 {
    let poolId = get_pool_id();
    let pool_tokens_abi = abi(PoolTokens, PoolTokensContractId);
    //let (_, balances, ) = get_vault().get_pool_tokens(poolId);
    let (_, balances,_ ) = pool_tokens_abi.get_pool_tokens(poolId);
    _upscale_array(balances, _scaling_factors());

    let (lowerTarget, upperTarget) = get_targets();
    let params = Params{
        fee: get_swap_fee_percentage(),
        lowerTarget: lowerTarget,
        upperTarget: upperTarget
    };

    let totalBalance = _calc_invariant(
        _to_nominal(balances.get(_mainIndex).unwrap(), params),
        balances.get(_wrappedIndex).unwrap()
    );

    // Note that we're dividing by the virtual supply, which may be zero (causing this call to revert). However, the
    // only way for that to happen would be for all LPs to exit the Pool, and nothing prevents new LPs from
    // joining it later on.
    return totalBalance.div_Up(_get_approximate_virtual_supply(balances[_bptIndex]));
}

fn getWrappedTokenRate() ->u64 {
    return _get_wrapped_token_rate();
}

/*
    * Should be 1e18 for the subsequent calculation of the wrapper token scaling factor.
    */
//! need to do some work around when token standard is added    
fn _get_wrapped_token_rate() ->{
    return 1
};

fn get_targets()->(u64, u64) {
    let miscData = _get_misc_data();
    let lowerTarget = decode_uint(miscData, _LOWER_TARGET_OFFSET, 96);
    let upperTarget = decode_uint(miscData, _UPPER_TARGET_OFFSET, 96);
}

fn _is_main_balance_within_targets(lowerTarget: u64, upperTarget: u64)->bool {
    let poolId = get_pool_id();
    let pool_tokens_abi = abi(PoolTokens, PoolTokensContractId);
    let (_, balances, _) = pool_tokens_abi.get_pool_tokens(poolId);
    let mainTokenBalance = _upscale(balances.get(_mainIndex).unwrap(), _scaling_factor(_mainToken));

    return mainTokenBalance >= lowerTarget && mainTokenBalance <= upperTarget;
}

fn _isOwnerOnlyAction(bytes32 actionId) internal view virtual override returns (bool) {
    return actionId == getActionId(this.set_targets.selector) || super._isOwnerOnlyAction(actionId);
}


/// * Returns the number of tokens in circulation.
/// *
/// * In other pools, this would be the same as `total_supply`, but since this pool pre-mints all BPT, `total_supply`
/// * remains constant, whereas `virtualSupply` increases as users join the pool and decreases as they exit it.
///
fn get_virtual_supply() ->u64 {
    let pool_tokens_abi = abi(PoolTokens, PoolTokensContractId);
    let (_, balances, _) = pool_tokens_abi.get_pool_tokens(get_pool_id());
    /// We technically don't need to upscale the BPT balance as its scaling factor is equal to one (since BPT has
    /// 18 decimals), but we do it for completeness.
    let bptBalance = _upscale(balances.get(_bptIndex).unwrap(), _scaling_factor(contract_id()));

    return _get_virtual_supply(bptBalance);
}

fn _get_virtual_supply(bptBalance: u64) ->u64 {
    return sub(total_supply(), bptBalance);
}


/// * Computes an approximation of virtual supply, which costs less gas than `_get_virtual_supply` and returns the
/// * same value in all cases except when the emergency pause has been enabled and BPT burned as part of the emergency
/// * exit process.

fn _get_approximate_virtual_supply(bptBalance: u64)-> u64 {
    /// No need for checked arithmetic as _INITIAL_BPT_SUPPLY is always greater than any valid Vault BPT balance.
    return _INITIAL_BPT_SUPPLY - bptBalance;
}

impl LinearPool for Contract {
    //Todo need workaround when Token standard is added
    #[storage(read, write)]
    fn init_linear_pool(
        vault: Address,
        mainToken: ContractId, 
        wrappedToken: ContractId,
        swapFeePercentage: u64,
        pauseWindowDuration: u64,
        bufferPeriodDuration: u64,
        owner: Identity
        ) 
    {
        let legacy_base_pool_abi = abi(LegacyBasePool, LegacyBasePoolContractId);
        legacy_base_pool_abi.init_legacy_base_pool(
            PoolSpecialization.GENERAL,
            _sort_tokens(mainToken, wrappedToken, contract_id()),
            ~Vec::with_capacity(_TOTAL_TOKENS),
            swapFeePercentage
            owner
        )
         // Set tokens
        storage._mainToken = mainToken;
        storage._wrappedToken = wrappedToken;
        let (mainIndex, wrappedIndex, bptIndex) = _get_sorted_token_indexes(mainToken, wrappedToken, contract_id());
        storage._bptIndex = bptIndex;
        storage._mainIndex = mainIndex;
        storage._wrappedIndex = wrappedIndex;
        //Set scaling factors
        storage._scalingFactorMainToken = _compute_scaling_factor(mainToken);
        storage._scalingFactorWrappedToken = _compute_scaling_factor(wrappedToken);
        // Set initial targets. Lower target must be set to zero because initially there are no fees accumulated.
        // Otherwise the pool will owe fees at start which results in a manipulable rate.
        let lowerTarget = 0;
        _set_targets(mainToken, lowerTarget, upperTarget);
    }

    fn initialize() {
        let poolId = get_pool_id();
        let pool_tokens_abi = abi(PoolTokens, PoolTokensContractId);
        let (tokens, _, _ ) = pool_tokens_abi.get_pool_tokens(poolId);

        // Joins typically involve the Pool receiving tokens in exchange for newly-minted BPT. In this case however, the
        // Pool will mint the entire BPT supply to itself, and join itself with it.
        let mut maxAmountsIn: Vec<u64> = ~Vec::with_capacity(_TOTAL_TOKENS);
        maxAmountsIn.push(_bptIndex) = _INITIAL_BPT_SUPPLY;

        // The first time this executes, it will call `_on_initialize_pool` (as the BPT supply will be zero). Future calls
        // will be routed to `_on_join_pool`, which always reverts, meaning `initialize` will only execute once.
        let request = JoinPoolRequest({
            assets: _asIAsset(tokens),
            maxAmountsIn: maxAmountsIn,
            userData: "",
            fromInternalBalance: false
        });

        join_pool(poolId, contract_id(), contract_id(), request);
    }

    ///
    /// * Implementation of on_swap, from IGeneralPool.
    ///
    fn on_swap(
        request: SwapRequest,
        balances: Vec<u64>,
        indexIn: u64,
        indexOut: u64
    )  ->u64 {
        only_vault(request.poolId);
        let temporarily_pausable_abi = abi(TemporarilyPausable, TemporarilyPausableContractId);
        temporarily_pausable_abi._is_not_paused();
  
        // Sanity check: this is not entirely necessary as the Vault's interface enforces the indices to be valid, but
        // the check is cheap to perform.
        require(indexIn < _TOTAL_TOKENS && indexOut < _TOTAL_TOKENS, LinearPoolErrors::OUT_OF_BOUNDS);

        // Note that we already know the indices of the main token, wrapped token and BPT, so there is no need to pass
        // these indices to the inner functions.

        // Upscale balances by the scaling factors (taking into account the wrapped token rate)
        let scalingFactors: Vec<u64> = _scaling_factors();
        _upscale_array(balances, scalingFactors);

        let (lowerTarget, upperTarget) = get_targets();
        let params: Params = Params({
            fee: get_swap_fee_percentage(),
            lowerTarget: lowerTarget,
            upperTarget: upperTarget
        });

        if (request.kind == IVault.SwapKind.GIVEN_IN) {
            // The amount given is for token in, the amount calculated is for token out
            request.amount = _upscale(request.amount, scalingFactors[indexIn]);
            let amountOut = _on_swap_given_in(request, balances, params);

            // amountOut tokens are exiting the Pool, so we round down.
            return _downscale_down(amountOut, scalingFactors.get(indexOut).unwrap());
        } else {
            // The amount given is for token out, the amount calculated is for token in
            request.amount = _upscale(request.amount, scalingFactors[indexOut]);
            let amountIn = _on_swap_given_out(request, balances, params);

            // amountIn tokens are entering the Pool, so we round up.
            return _downscale_up(amountIn, scalingFactors[indexIn]);
        }
    }

    

    //! Needed workaround when bytes operations are added
    fn _on_initialize_pool(
        poolId: b256,
        address sender: Address,
        address recipient: Address,
        scalingFactors: Vec<u64>,
        //userData: bytes
    ) ->(u64, Vec<u64>){
        let temporarily_pausable_abi = abi(TemporarilyPausable, TemporarilyPausableContractId);
        temporarily_pausable_abi._is_not_paused()
        // Linear Pools can only be initialized by the Pool performing the initial join via the `initialize` function.
        require(sender == contract_id(), LinearPoolErrors::INVALID_INITIALIZATION);
        require(recipient == contract_id(), LinearPoolErrors::INVALID_INITIALIZATION);

        // The full BPT supply will be minted and deposited in the Pool. Note that there is no need to approve the Vault
        // as it already has infinite BPT allowance.
        let bptAmountOut = _INITIAL_BPT_SUPPLY;

        let mut amountsIn: Vec<u64> = ~Vec::with_capacity(_TOTAL_TOKENS);
        amountsIn.insert(_bptIndex, _INITIAL_BPT_SUPPLY);

        return (bptAmountOut, amountsIn);
    }

    //Todo when Bytes operations are added
    fn on_join_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        //userData: bytes
    )->(Vec<u64>, Vec<u64>){
        let mut scalingFactors = _scaling_factors();
        let temporarily_pausable_abi = abi(TemporarilyPausable, TemporarilyPausableContractId);
        // Joins are unsupported when paused
        // It would be strange for the Pool to be paused before it is initialized, but for consistency we prevent
        // initialization in this case.
        temporarily_pausable_abi._ensure_not_paused();

        if (total_supply() == 0) {
            (bptAmountOut, amountsIn) = _on_initialize_pool(
                poolId,
                sender,
                recipient,
                scalingFactors,
               // userData
            );

            // On initialization, we lock _get_minimum_bpt() by minting it for the zero address. This BPT acts as a
            // minimum as it will never be burned, which reduces potential issues with rounding, and also prevents the
            // Pool from ever being fully drained.
            require(bptAmountOut >= _get_minimum_bpt(), LinearPoolErrors::MINIMUM_BPT);
            mint_pool_tokens(address(0), _get_minimum_bpt());
            mint_pool_tokens(recipient, bptAmountOut - _get_minimum_bpt());

            // amountsIn are amounts entering the Pool, so we round up.
            _downscale_up_array(amountsIn, scalingFactors);

            return (amountsIn, ~Vec::with_capacity(_get_total_tokens()));
        }
        } else {
            _upscale_array(balances, scalingFactors);
            let (bptAmountOut, amountsIn, dueProtocolFeeAmounts) = _on_join_pool(
                poolId,
                sender,
                recipient,
                balances,
                lastChangeBlock,
                inRecoveryMode() ? 0 : protocolSwapFeePercentage, // Protocol fees are disabled while in recovery mode
                scalingFactors,
                //userData
            );

            // Note we no longer use `balances` after calling `_on_join_pool`, which may mutate it.
            let balancer_pool_token_abi = abi(BalancerPoolToken, BalancerPoolTokenContractId);
            balancer_pool_token_abi._mint_pool_tokens(recipient, bptAmountOut);

            // amountsIn are amounts entering the Pool, so we round up.
            _downscale_up_array(amountsIn, scalingFactors);
            // dueProtocolFeeAmounts are amounts exiting the Pool, so we round down.
            _downscale_down_array(dueProtocolFeeAmounts, scalingFactors);

            return (amountsIn, dueProtocolFeeAmounts);
        }
    }

    //Todo when Bytes operations are added
    fn _on_join_pool(
        poolId :b256,
        sender :address,
        recipient: address,
        balances: Vec<u64>uint256[] memory,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        scalingFactors: Vec<u64>,
        //userData: bytes
    ) -> (u64, Vec<u64>, Vec<u64>)
    {
        _revert(UNHANDLED_BY_LINEAR_POOL);
    }

    //!Todo when Bytes operations are added
    fn _on_exit_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        scalingFactors: u64,
        //userData: bytes
    )-> (u64, Vec<u64>, Vec<u64>)
    {
        // Exits typically revert, except for the proportional exit when the emergency pause mechanism has been
        // triggered. This allows for a simple and safe way to exit the Pool.

        // Note that the rate cache will not be automatically updated in such a scenario (though this can be still done
        // manually). This however should not lead to any issues as the rate is not important during the emergency exit.
        // On the contrary, decoupling the rate provider from the emergency exit might be useful under these
        // circumstances.
        //Todo- Need some workaround
        // LinearPoolUserData.ExitKind kind = userData.exitKind();
        // if (kind != LinearPoolUserData.ExitKind.EMERGENCY_EXACT_BPT_IN_FOR_TOKENS_OUT) {
        //     _revert(UNHANDLED_BY_LINEAR_POOL);
        // } else{} 
        let temporarily_pausable_abi = abi(TemporarilyPausable, TemporarilyPausableContractId);
        temporarily_pausable_abi._ensure_paused();
        // Note that this will cause the user's BPT to be burned, which is not something that happens during
        // regular operation of this Pool, and may lead to accounting  Because of this, it is highly
        // advisable to stop using a Pool after it is paused and the pause window expires.

        (bptAmountIn, amountsOut) = _emergency_proportional_exit(balances, userData);

        // Due protocol fees are set to zero as this Pool accrues no fees and pays no protocol fees.
        let dueProtocolFeeAmounts = ~Vec::with_capacity(_get_total_tokens());

        (bptAmountIn, amountsOut, dueProtocolFeeAmounts)
    }

    //!need some workaround when bytes are implemented
    fn on_exit_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        //userData: bytes
    )->(Vec<u64>, Vec<u64>) {
        only_vault(poolId);
        let mut dueProtocolFeeAmounts: Vec<u64> = ~Vec::new();
        let mut amountsOut: Vec<u64> = ~Vec::new();
        let mut bptAmountIn: u64 = 0;


        if (userData.isRecoveryModeExitKind()) {
            // This exit kind is only available in Recovery Mode.
            _ensureInRecoveryMode();

            // Protocol fees are skipped when processing recovery mode exits, since these are pool-agnostic and it
            // is therefore impossible to know how many fees are due. For consistency, all regular joins and exits are
            // processed as if the protocol swap fee percentage was zero.
            dueProtocolFeeAmounts = new uint256[](balances.length);

            (bptAmountIn, amountsOut) = _doRecoveryModeExit(balances, total_supply(), userData);
        } else {
            // Exits are unsupported when paused
            let temporarily_pausable_abi = abi(TemporarilyPausable, TemporarilyPausableContractId);
            temporarily_pausable_abi._ensure_not_paused();

            uint256[] memory scalingFactors = _scaling_factors();
            _upscaleArray(balances, scalingFactors);

            // Note we no longer use `balances` after calling `_onExitPool`, which may mutate it.
            (bptAmountIn, amountsOut, dueProtocolFeeAmounts) = _on_exit_pool(
                poolId,
                sender,
                recipient,
                balances,
                lastChangeBlock,
                inRecoveryMode() ? 0 : protocolSwapFeePercentage, // Protocol fees are disabled while in recovery mode
                scalingFactors,
                //userData
            );

            // Both amountsOut and dueProtocolFeeAmounts are amounts exiting the Pool, so we round down.
            _downscale_down_array(amountsOut, scalingFactors);
            _downscale_down_array(dueProtocolFeeAmounts, scalingFactors);
        }
        let balancer_pool_token_abi = abi(BalancerPoolToken, BalancerPoolTokenContractId);

        balancer_pool_token_abi._burn_pool_tokens(sender, bptAmountIn);

        return (amountsOut, dueProtocolFeeAmounts);
    }

    //!need some workaround when bytes are implemented
    fn _emergency_proportional_exit(
        balances: Vec<u64>,
        ///userData: bytes
        )-> (u64, Vec<u64>)
    {
      

        //let bptAmountIn = userData.exactBptInForTokensOut();
        //!using dummy data because of bytes operations not yet added
        let bptAmountIn = 10;
        // Note that there is no minimum amountOut parameter: this is handled by `IVault.exitPool`.

        // This process burns BPT, rendering `_get_approximate_virtual_supply` inaccurate, so we use the real method here
        let amountsOut = _calc_tokens_out_given_exact_bpt_in(
            balances,
            bptAmountIn,
            _get_virtual_supply(balances.get(_bptIndex).unwrap()),
            _bptIndex
        );

        return (bptAmountIn, amountsOut);
    }

    fn _set_targets(
        mainToken: ContractId,
        lowerTarget: u64,
        upperTargetL: u64
    ) {
        require(lowerTarget <= upperTarget, LinearPoolErrors::LOWER_GREATER_THAN_UPPER_TARGET);
        require(upperTarget <= _MAX_UPPER_TARGET, LinearPoolErrors::UPPER_TARGET_TOO_HIGH);

        let legacy_base_pool_abi = abi(LegacyBasePool, LegacyBasePoolContractId);
        legacy_base_pool_abi._set_misc_data(
            encode_uint(lowerTarget, _LOWER_TARGET_OFFSET, 96) |
                encode_uint(upperTarget, _UPPER_TARGET_OFFSET, 96)
        );
        log({
            token: mainToken, lowerTarget: lowerTarget, upperTarget: upperTarget
        })
    }

    fn set_targets(newLowerTarget: u64, newUpperTarget: u64) {
        let authentication_abi = abi(Authentication, AuthenticationContractId);
        authentication_abi._authenticate_caller();       
        let (currentLowerTarget: u64, currentUpperTarget: u64) = get_targets();
        require(_is_main_balance_within_targets(currentLowerTarget, currentUpperTarget), LinearPoolErrors::OUT_OF_TARGET_RANGE);
        require(_is_main_balance_within_targets(newLowerTarget, newUpperTarget), LinearPoolErrors::OUT_OF_NEW_TARGET_RANGE);

        _set_targets(_mainToken, newLowerTarget, newUpperTarget);
    }

    fn set_swap_fee_percentage_(swapFeePercentage: u64){
    
        let (lowerTarget, upperTarget) = get_targets();
        require(_is_main_balance_within_targets(lowerTarget, upperTarget), LinearPoolErrors::OUT_OF_TARGET_RANGE);

        set_swap_fee_percentage(swapFeePercentage);
    }
}
