contract;



dep data_structures;
dep errors;

use data_structures::{PoolSpecialization, SwapFeePercentageChanged};
use errors::{
    LegacyBasePoolErrors,
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

use Authentication::{
    _authenticate_caller,
};

use InputHelpers::{
    ensure_array_is_sorted,
};

use PoolRegistry::{
    register_pool,
};

use PoolTokens::{
    register_tokens,
    get_pool_token_info
};

use BalancerPoolToken::{
    get_vault,
    _mint_pool_tokens
};
use WordCodec::{
    decode_uint,
    insert_uint,
    insert_bits192
};

use TemporarilyPausable::{
    _is_not_paused,
    _set_paused
};

use FixedPoint::{
    ONE,
    div_up,
    div_down,
    sub
};

storage {
    _miscData: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
    _poolId: b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
}

const _MIN_TOKENS = 2;
const _DEFAULT_MINIMUM_BPT = 1000000;

// 1e18 corresponds to 1.0, or a 100% fee
const _MIN_SWAP_FEE_PERCENTAGE = 1000000000000; // 0.0001%
const _MAX_SWAP_FEE_PERCENTAGE = 100000000000000000; // 10% - this fits in 64 bits
const _SWAP_FEE_PERCENTAGE_OFFSET = 192;
const _TOTAL_TOKENS: u64 = 3; // Main token, wrapped token, BPT


/// Returns poolId
#[storage(read)]
pub fn get_pool_id() ->b256 {
    return _poolId;
}

/*
    * Returns the minimum BPT supply. This amount is minted to the zero address during initialization, effectively
    * locking it.
    *
    * This is useful to make sure Pool initialization happens only once, but derived Pools can change this value (even
    * to zero) by overriding this fn.
*/
pub fn _get_minimum_bpt()-> u64 {
    return _DEFAULT_MINIMUM_BPT;
}


pub fn _is_owner_only_action(actionId: b256) ->bool {
    let selector:b256 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
    return(actionId == get_action_id(selector));
}

/*
    * Return the current value of the swap fee percentage.
    * This is stored in the MSB 64 bits of the `_miscData`.
*/
#[storage(read)]
pub fn get_swap_fee_percentage()-> u64 {
    return decode_uint(storage._miscData,_SWAP_FEE_PERCENTAGE_OFFSET, 64);
}

#[storage(read)]
pub fn _get_misc_data() -> b256{
    return storage._miscData;
}

impl LegacyBasePool for Contract{
    //Todo need workaround when Token standard is added
     #[storage(read, write)]
    fn init_legacy_base_pool(
        specialization: PoolSpecialization,
        tokens: Vec<ContractId>,
        assetManagers: Vec<ContractId>,
        swapFeePercentage: u64,
        // pauseWindowDuration: u64,
        // bufferPeriodDuration: u64,
        owner: Identity    
        )
    {
        require(tokens.len() >= _MIN_TOKENS, LegacyBasePoolErrors::MIN_TOKENS);
        require(tokens.len() <= _getMaxTokens(), LegacyBasePoolErrors::MAX_TOKENS);

        // The Vault only requires the token list to be ordered for the Two Token Pools specialization. However,
        // to make the developer experience consistent, we are requiring this condition for all the native pools.
        // Also, since these Pools will register tokens only once, we can ensure the Pool tokens will follow the same
        // order. We rely on this property to make Pools simpler to write, as it lets us assume that the
        // order of token-specific parameters (such as token weights) will not change.
        ensure_array_is_sorted(tokens);

        _set_swap_fee_percentage(swapFeePercentage);

        let poolId = register_pool(specialization);

        register_tokens(poolId, tokens, assetManagers);

        // Set immutable state variables - these cannot be read from during construction
        storage._poolId = poolId;
    }

    #[storage(read, write)]
    fn set_swap_fee_percentage(swapFeePercentage: u64) {
        _authenticate_caller();
        _is_not_paused();
        _set_swap_fee_percentage(swapFeePercentage);
    }
    #[storage(read, write)]
    fn _set_swap_fee_percentage(swapFeePercentage: u64) {
        require(swapFeePercentage >= _MIN_SWAP_FEE_PERCENTAGE, LegacyBasePoolErrors::MIN_SWAP_FEE_PERCENTAGE);
        require(swapFeePercentage <= _MAX_SWAP_FEE_PERCENTAGE, LegacyBasePoolErrors::MAX_SWAP_FEE_PERCENTAGE);

        storage._miscData = insert_uint(storage._miscData, swapFeePercentage, _SWAP_FEE_PERCENTAGE_OFFSET, 64);
        //emit SwapFeePercentageChanged(swapFeePercentage);
    }
    //Todo when Bytes operations are added
    fn set_asset_manager_pool_config(
        token: ContractId,
        //poolConfig: bytes
        )
    {
        _is_not_paused();
        _authenticate_caller();
        _set_asset_manager_pool_config(token, poolConfig);
    }
    //Todo when Bytes operations are added
    fn _set_asset_manager_pool_config( 
        token: ContractId, 
        //poolConfig: bytes
        )
    {
        let poolId = get_pool_id();
        let (_, _, _,  assetManager) = get_pool_token_info(poolId, token);

        // IAssetManager(assetManager).setConfig(poolId, poolConfig);
    }

    #[storage(read, write)]
    fn pause() {
        _authenticate_caller();
        _set_paused(true);
    }

    #[storage(read, write)]
    fn unpause() {
        _authenticate_caller();
        _set_paused(false);
    }



    /*
     * Inserts data into the least-significant 192 bits of the misc data storage slot.
     * Note that the remaining 64 bits are used for the swap fee percentage and cannot be overloaded.
     */
    #[storage(read, write)] 
    fn _set_misc_data(newData: b256) {
        storage._miscData = insert_bits192(storage._miscData, newData, 0);
    }

    // Join / Exit Hooks

    fn only_vault(poolId: b256) {
        let sender: Result<Address, AuthError> = msg_sender();
        let Address::Address(addr) = sender.unwrap(); 
        require(addr == ~Address::from(get_vault()), LegacyBasePoolErrors::CALLER_NOT_VAULT);
        require(poolId == get_pool_id(), LegacyBasePoolErrors::INVALID_POOL_ID);
    }

    // Query functions

    /*
     * "Dry run" `onJoinPool`.
     * Returns the amount of BPT that would be granted to `recipient` if the `onJoinPool` hook were called by the
     * Vault with the same arguments, along with the number of tokens `sender` would have to supply.
     *
     * This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
     * data, such as the protocol swap fee percentage and Pool balances.
     *
     * Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
     * explicitly use eth_call instead of eth_sendTransaction.
     */
    fn query_join(
        poolId: b256,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        //userData: bytes
    ) ->(u64, Vec<u64>){
        ensure_input_length_match(balances.len(), _get_total_tokens());

        _queryAction(
            poolId,
            sender,
            recipient,
            balances,
            lastChangeBlock,
            protocolSwapFeePercentage,
            userData,
            _onJoinPool,
            _downscaleUpArray
        );

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return (bptOut, amountsIn);
    }

    /*
     * "Dry run" `onExitPool`.
     * Returns the amount of BPT that would be burned from `sender` if the `onExitPool` hook were called by the
     * Vault with the same arguments, along with the number of tokens `recipient` would receive.
     *
     * This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
     * data, such as the protocol swap fee percentage and Pool balances.
     *
     * Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
     * explicitly use eth_call instead of eth_sendTransaction.
     */
    fn query_exit(
        poolId: b256,
        sender: Address,
        recipient: Address,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        //userData: bytes
    )->(u64, Vec<u64>) {
        ensure_input_length_match(balances.len(), _get_total_tokens());

        _queryAction(
            poolId,
            sender,
            recipient,
            balances,
            lastChangeBlock,
            protocolSwapFeePercentage,
            userData,
            _onExitPool,
            _downscale_down_array
        );

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return (bptIn, amountsOut);
    }

    // Internal hooks to be overridden by derived contracts - all token amounts (except BPT) in these interfaces are
    // upscaled.

    /*
     * Called when the Pool is joined for the first time; that is, when the BPT total supply is zero.
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
    fn _onInitializePool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        scalingFactors: Vec<u64>,
        //userData: bytes
    )->(Vec<u64>, Vec<u64>){

    }


    /*
     * Adds swap fee amount to `amount`, returning a higher value.
     */
    fn _add_swap_fee_amount(amount: u64)-> u64 {
        // This returns amount + fee amount, so we round up (favoring a higher fee amount).
        return div_up(amount, (sub(ONE, get_swap_fee_percentage())));
    }

    /*
     * Subtracts swap fee amount from `amount`, returning a lower value.
     */
    fn _subtract_swap_fee_amount(amount: u64) ->u64 {
        // This returns amount - fee amount, so we round up (favoring a higher fee amount).
        let feeAmount = mul_up(amount, get_swap_fee_percentage());
        return sub(amount, feeAmount);
    }

    // Scaling

    /*
     * Returns a scaling factor that, when multiplied to a token amount for `token`, normalizes its balance as if
     * it had 18 decimals.
     */
    fn _compute_scaling_factor(token: ContractId) ->u64 {
        if (token == contract_id()) {
            return FixedPoint.ONE;
        }
        //!will  be changed when token standard is added for time being using dummy value of 8 decimals
        // Tokens that don't implement the `decimals` method are not supported.
        //let  tokenDecimals = ERC20(address(token)).decimals();
        let tokenDecimals: u64 = 8;

        // Tokens with more than 18 decimals are not supported.
        let  decimalsDifference = sub(18, tokenDecimals);
        return (ONE * 10.pow(decimalsDifference));
    }

    // /*
    //  * Returns the scaling factor for one of the Pool's tokens. Reverts if `token` is not a token registered by the
    //  * Pool.
    //  *
    //  * All scaling factors are fixed-point values with 18 decimals, to allow for this fn to be overridden by
    //  * derived contracts that need to apply further scaling, making these factors potentially non-integer.
    //  *
    //  * The largest 'base' scaling factor (i.e. in tokens with less than 18 decimals) is 10**18, which in fixed-point is
    //  * 10**36. This value can be multiplied with a 112 bit Vault balance with no overflow by a factor of ~1e7, making
    //  * even relatively 'large' factors safe to use.
    //  *
    //  * The 1e7 figure is the result of 2**256 / (1e18 * 1e18 * 2**112).
    //  */
    // fn _scaling_factor(IERC20 token) internal view virtual returns (uint256);

    // /*
    //  * Same as `_scalingFactor()`, except for all registered tokens (in the same order as registered). The Vault
    //  * will always pass balances in this order when calling any of the Pool hooks.
    //  */
    // fn _scaling_factors() internal view virtual returns (uint256[] memory);

    // /*
    //  * Return the set of scaling factors for the pool tokens.
    //  * Scaling factors are used to convert token balances to and from 18-decimal floating point values.
    //  * The Vault expects all values to be 18-decimal, yet all I/O is performed in native decimals. So we scale "up"
    //  * when sending user-supplied balances to the Vault, and scale "down" to return results.
    //  * For instance, an 18-decimal token has a scaling factor of 1, while a 6-decimal token has a scaling factor of
    //  * 10^12.
    //  */
    // fn getScalingFactors() external view returns (uint256[] memory) {
    //     return _scaling_factors();
    // }

    /*
     * Applies `scalingFactor` to `amount`, resulting in a larger or equal value depending on whether it needed
     * scaling or not.
     */
    fn _upscale(amount: u64, scalingFactor: u64) ->u64 {
        // Upscale rounding wouldn't necessarily always go in the same direction: in a swap for example the balance of
        // token in should be rounded up, and that of token out rounded down. This is the only place where we round in
        // the same direction for all amounts, as the impact of this rounding is expected to be minimal (and there's no
        // rounding error unless `_scalingFactor()` is overriden).
        return mul_down(amount, scalingFactor);
    }
    fn _get_total_tokens() ->u64 {
        return _TOTAL_TOKENS;
    }

    /*
     * Same as `_upscale`, but for an entire array. This fn does not return anything, but instead *mutates*
     * the `amounts` array.
     */
    fn _upscale_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) {
        // let mut amounts = amounts;
        let mut i = 0;
        while i <  _get_total_tokens() {
            let amount = amount.get(i).unwrap();
            amounts.remove(i);
            amounts.insert(i, mul_down(amount, scalingFactors.get(i).unwrap()));
            i = i + 1;
        }
    }

    /*
     * Reverses the `scalingFactor` applied to `amount`, resulting in a smaller or equal value depending on
     * whether it needed scaling or not. The result is rounded down.
     */
    fn _downscale_down(amount: u64, scalingFactor: u64) ->u64 {
        return div_down(amount, scalingFactor);
    }

    /*
     * Same as `_downscale_down`, but for an entire array. This fn does not return anything, but instead
     * *mutates* the `amounts` array.
     */
    fn _downscale_down_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) {
        let mut i = 0;
        while i <  _get_total_tokens() {
            let amount = amount.get(i).unwrap();
            amounts.remove(i);
            amounts.insert(i, div_down(amount, scalingFactors.get(i).unwrap()));
            i = i + 1;
        }
    }

    /*
     * Reverses the `scalingFactor` applied to `amount`, resulting in a smaller or equal value depending on
     * whether it needed scaling or not. The result is rounded up.
     */
    fn _downscale_up(amount: u64, scalingFactorL: u64)->u64 {
        return div_up(amount, scalingFactor);
    }

    /*
     * Same as `_downscaleUp`, but for an entire array. This fn does not return anything, but instead
     * *mutates* the `amounts` array.
     */
    fn _downscale_up_array(amounts: Vec<u64>, scalingFactors: Vec<u64>) {
        let mut i = 0;
        while i <  _get_total_tokens() {
            let amount = amount.get(i).unwrap();
            amounts.remove(i);
            amounts.insert(i, div_up(amount, scalingFactors.get(i).unwrap()));
            i = i + 1;
        }
    }

    fn _queryAction(
        bytes32 poolId,
        address sender,
        address recipient,
        uint256[] memory balances,
        uint256 lastChangeBlock,
        uint256 protocolSwapFeePercentage,
        bytes memory userData,
        fn(bytes32, address, address, uint256[] memory, uint256, uint256, uint256[] memory, bytes memory)
            internal
            returns (uint256, uint256[] memory, uint256[] memory) _action,
        fn(uint256[] memory, uint256[] memory) internal view _downscaleArray
    ) private {
        // This uses the same technique used by the Vault in queryBatchSwap. Refer to that fn for a detailed
        // explanation.

        if (msg.sender != address(this)) {
            // We perform an external call to ourselves, forwarding the same calldata. In this call, the else clause of
            // the preceding if statement will be executed instead.

            // solhint-disable-next-line avoid-low-level-calls
            (bool success, ) = address(this).call(msg.data);

            // solhint-disable-next-line no-inline-assembly
            assembly {
                // This call should always revert to decode the bpt and token amounts from the revert reason
                switch success
                    case 0 {
                        // Note we are manually writing the memory slot 0. We can safely overwrite whatever is
                        // stored there as we take full control of the execution and then immediately return.

                        // We copy the first 4 bytes to check if it matches with the expected signature, otherwise
                        // there was another revert reason and we should forward it.
                        returndatacopy(0, 0, 0x04)
                        let error := and(mload(0), 0xffffffff00000000000000000000000000000000000000000000000000000000)

                        // If the first 4 bytes don't match with the expected signature, we forward the revert reason.
                        if eq(eq(error, 0x43adbafb00000000000000000000000000000000000000000000000000000000), 0) {
                            returndatacopy(0, 0, returndatasize())
                            revert(0, returndatasize())
                        }

                        // The returndata contains the signature, followed by the raw memory representation of the
                        // `bptAmount` and `tokenAmounts` (array: length + data). We need to return an ABI-encoded
                        // representation of these.
                        // An ABI-encoded response will include one additional field to indicate the starting offset of
                        // the `tokenAmounts` array. The `bptAmount` will be laid out in the first word of the
                        // returndata.
                        //
                        // In returndata:
                        // [ signature ][ bptAmount ][ tokenAmounts length ][ tokenAmounts values ]
                        // [  4 bytes  ][  32 bytes ][       32 bytes      ][ (32 * length) bytes ]
                        //
                        // We now need to return (ABI-encoded values):
                        // [ bptAmount ][ tokeAmounts offset ][ tokenAmounts length ][ tokenAmounts values ]
                        // [  32 bytes ][       32 bytes     ][       32 bytes      ][ (32 * length) bytes ]

                        // We copy 32 bytes for the `bptAmount` from returndata into memory.
                        // Note that we skip the first 4 bytes for the error signature
                        returndatacopy(0, 0x04, 32)

                        // The offsets are 32-bytes long, so the array of `tokenAmounts` will start after
                        // the initial 64 bytes.
                        mstore(0x20, 64)

                        // We now copy the raw memory array for the `tokenAmounts` from returndata into memory.
                        // Since bpt amount and offset take up 64 bytes, we start copying at address 0x40. We also
                        // skip the first 36 bytes from returndata, which correspond to the signature plus bpt amount.
                        returndatacopy(0x40, 0x24, sub(returndatasize(), 36))

                        // We finally return the ABI-encoded uint256 and the array, which has a total length equal to
                        // the size of returndata, plus the 32 bytes of the offset but without the 4 bytes of the
                        // error signature.
                        return(0, add(returndatasize(), 28))
                    }
                    default {
                        // This call should always revert, but we fail nonetheless if that didn't happen
                        invalid()
                    }
            }
        } else {
            uint256[] memory scalingFactors = _scaling_factors();
            _upscaleArray(balances, scalingFactors);

            (uint256 bptAmount, uint256[] memory tokenAmounts, ) = _action(
                poolId,
                sender,
                recipient,
                balances,
                lastChangeBlock,
                protocolSwapFeePercentage,
                scalingFactors,
                userData
            );

            _downscaleArray(tokenAmounts, scalingFactors);

            // solhint-disable-next-line no-inline-assembly
            assembly {
                // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
                // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
                // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
                let size := mul(mload(tokenAmounts), 32)

                // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
                // will be at least one available slot due to how the memory scratch space works.
                // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
                let start := sub(tokenAmounts, 0x20)
                mstore(start, bptAmount)

                // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
                // We use the previous slot to `bptAmount`.
                mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
                start := sub(start, 0x04)

                // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
                // the `bptAmount`, the array 's length, and the error signature.
                revert(start, add(size, 68))
            }
        }
    }
}
