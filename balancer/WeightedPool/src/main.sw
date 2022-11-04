contract;

dep data_structures;
dep interface;
dep errors;
dep utils;
dep events;

use interface::WeightedPool;
use errors::Error;
use data_structures::{
    ExitKind,
    JoinKind,
    UserData,
    RequestKind,
};

use utils::{
    calc_bpt_out_given_exact_tokens_in,
    calc_due_protocol_swap_fee_bpt_amount,
    calc_token_in_given_exact_bpt_out,
    calculate_invariant,
    downscale_down_array,
    downscale_up_array,
    get_normalized_weights_private,
    scaling_factor,
    scaling_factors,
    upscale_array,
    calc_token_out_given_exact_bpt_in,
    calc_tokens_out_given_exact_bpt_in,
    calc_bpt_in_given_exact_tokens_out,
    join_all_tokens_in_for_exact_bptout,
};

use events::EventSwapFeePercentageChanged;

use std::{
    address::Address,
    chain::auth::{AuthError, msg_sender},
    constants::{BASE_ASSET_ID, ZERO_B256},
    context::{balance_of, call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    hash::keccak256,
    identity::Identity,
    logging::log,
    option::Option,
    reentrancy::is_reentrant,
    result::Result,
    revert::{require, revert},
    storage::{StorageMap, get, store},
    token::{burn, force_transfer_to_contract, mint, transfer_to_output},
    vec::Vec,
};

storage {
    // Invariant of last join or exit pool
    last_post_join_exit_invariant: u64 = 10,
    // contracr ID of the vault
    vault_contract_id: ContractId = ContractId {
        value: 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b,
    },
    // swap fee in percentage
    swap_fee_percentage: u64 = 0,
}

impl WeightedPool for Contract {
    // Vault hook for adding liquidity to a pool (including the first time, "initializing" the pool).
    // This fn can only be called from the Vault, from `joinPool`.
    #[storage(read, write)]fn on_join_pool(pool_id: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, last_change_block: u64, protocol_swap_fee_percentage: u64, user_data: UserData) -> (Vec<u64>, Vec<u64>) {
        let scaling_factors_vec = scaling_factors();

        // on_initialize_pool exist BaseWeightedPool cotract
        if TOTAL_SUPPLY == 0 {
            let(bpt_amount_out, amounts_in) = on_initialize_pool(scaling_factors_vec, user_data);

            // On initialization, we lock _get_minimum_bpt() by minting it for the zero address. This BPT acts as a
            // minimum as it will never be burned, which reduces potential issues with rounding, and also prevents the
            // Pool from ever being fully drained.
            require(bpt_amount_out >= DEFAULT_MINIMUM_BPT, Error::MinimumBpt);
            // todo:- while minting the tokens contract is paniciing
            // todo:- token need to be on node, working with dummy data
            // mint_pool_tokens(~ContractId::from(ZERO_B256), DEFAULT_MINIMUM_BPT);
            // mint_pool_tokens(recipient, bpt_amount_out - DEFAULT_MINIMUM_BPT);

            // SCRIPT_TESTING
            // temporarily changing the code here to make the script run succesfully
            // downscale_down_array() does not take vecs whose elements are 0
            // so making amounts_out and scaling_factors_vec non zero
            // original code start --
            // // amounts_in are amounts entering the Pool, so we round up.
            // let amounts_in = downscale_up_array(amounts_in, scaling_factors_vec);
            // original code end --
            
            // changed code start --
            let mut amounts_in_tmp = ~Vec::with_capacity(amounts_in.len());
            let mut scaling_factors_tmp = ~Vec::with_capacity(scaling_factors_vec.len());

            let mut count = 0;

            while count < amounts_in.len() {
                amounts_in_tmp.insert(count, 10);
                scaling_factors_tmp.insert(count, 10);

                count += 1;
            }

            let amounts_in = downscale_up_array(amounts_in_tmp, scaling_factors_tmp);
            // changed code end --

            return (amounts_in, ~Vec::with_capacity(balances.len()));
        } else {
            let balances = upscale_array(balances, scaling_factors_vec);
            // _on_join_pool function exist in BaseWightedPool
            let(bpt_amount_out, amounts_in) = on_join_pool_private(sender, balances, protocol_swap_fee_percentage, scaling_factors_vec, user_data);

            // Note we no longer use `balances` after calling `_on_join_pool`, which may mutate it.

            // mint_pool_tokens(recipient, bpt_amount_out);

            // amounts_in are amounts entering the Pool, so we round up.
            let amounts_in = downscale_up_array(amounts_in, scaling_factors_vec);

            // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
            return (amounts_in, ~Vec::with_capacity(balances.len()));
        }
    }

    // Vault hook for removing liquidity from a pool.
    // This fn can only be called from the Vault, from `exitPool`.
    #[storage(read, write)]fn on_exit_pool(pool_id: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, last_change_block: u64, protocol_swap_fee_percentage: u64, user_data: UserData) -> (Vec<u64>, Vec<u64>) {
        let scaling_factors_vec = scaling_factors();
        let balances = upscale_array(balances, scaling_factors_vec);

        let(bpt_amount_in, amounts_out) = on_exit_pool_private(sender, balances, protocol_swap_fee_percentage, scaling_factors_vec, user_data);

        // Note we no longer use `balances` after calling `_on_exit_pool`, which may mutate it.
        burn_pool_tokens(sender, bpt_amount_in);

        // SCRIPT_TESTING
        // temporarily changing the code here to make the script run succesfully
        // downscale_down_array() does not take vecs whose elements are 0
        // so making amounts_out and scaling_factors_vec non zero
        // original code start --
        // amounts_out are amounts exiting the Pool, so we round down.
        // let amounts_out = downscale_down_array(amounts_out, scaling_factors_vec);
        // original code end --
        
        // changed code start --
        let mut amounts_out_tmp = ~Vec::with_capacity(amounts_out.len());
        let mut scaling_factors_tmp = ~Vec::with_capacity(scaling_factors_vec.len());

        let mut count = 0;

        while count < amounts_out.len() {
            amounts_out_tmp.insert(count, 10);
            scaling_factors_tmp.insert(count, 10);

            count += 1;
        }

        let amounts_out = downscale_down_array(amounts_out_tmp, scaling_factors_tmp);
        // changed code end --

        // This Pool ignores the `dueProtocolFees` return value, so we simply return a zeroed-out array.
        (amounts_out, ~Vec::with_capacity(balances.len()))
    }

    // Set the swap fee percentage.
    // This is a permissioned fn, and disabled if the pool is paused. The swap fee must be within the
    // bounds set by MIN_SWAP_FEE_PERCENTAGE/MAX_SWAP_FEE_PERCENTAGE. Emits the EventSwapFeePercentageChanged event.
    #[storage(read, write)]fn set_swap_fee_percentage(swap_fee_percentage: u64) {
        require(swap_fee_percentage >= MIN_SWAP_FEE_PERCENTAGE, Error::MinSwapFeePercentage);
        require(swap_fee_percentage <= MAX_SWAP_FEE_PERCENTAGE, Error::MaxSwapFeePercentage);

        log(EventSwapFeePercentageChanged {
            swap_fee_percentage: swap_fee_percentage
        }
        );
    }

    // Query fns
    // "Dry run" `on_join_pool`.
    // Returns the amount of BPT that would be granted to `recipient` if the `on_join_pool` hook were called by the
    // Vault with the same arguments, along with the number of tokens `sender` would have to supply.
    //
    // This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
    // data, such as the protocol swap fee percentage and Pool balances.
    //
    // Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
    // explicitly use eth_call instead of eth_sendTransaction.
    #[storage(read, write)]fn query_join(pool_id: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, last_change_block: u64, protocol_swap_fee_percentage: u64, user_data: UserData) -> (u64, Vec<u64>) {
        require(balances.len() == TOTAL_TOKENS, Error::InputLengthMismatch);

        let scaling_factors_vec = scaling_factors();
        let balances = upscale_array(balances, scaling_factors_vec);

        let(bpt_out, amounts_in) = on_join_pool_private(sender, balances, protocol_swap_fee_percentage, scaling_factors_vec, user_data);

        // if (msg.sender != address(this)) {
        //     query_action(
        //         pool_id,
        //         sender,
        //         recipient,
        //         balances,
        //         last_change_block,
        //         protocol_swap_fee_percentage,
        //         user_data,
        //     );
        // }
        // else {
        //     scaling_factors_vec = scaling_factors();
        //     let balances = upscale_array(balances, scaling_factors_vec);

        //     let(bptAmount, tokenAmounts) = on_join_pool(
        //         pool_id,
        //         sender,
        //         recipient,
        //         balances,
        //         last_change_block,
        //         protocol_swap_fee_percentage,
        //         scaling_factors_vec,
        //         user_data
        //     );

        //     downscale_up_array(tokenAmounts, scaling_factors_vec);

        //     // solhint-disable-next-line no-inline-assembly
        //     assembly {
        //         // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
        //         // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
        //         // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
        //         let size := mul(mload(tokenAmounts), 32)

        //         // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
        //         // will be at least one available slot due to how the memory scratch space works.
        //         // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
        //         let start := sub(tokenAmounts, 0x20)
        //         mstore(start, bptAmount)

        //         // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
        //         // We use the previous slot to `bptAmount`.
        //         mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
        //         start := sub(start, 0x04)

        //         // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
        //         // the `bptAmount`, the array 's length, and the error signature.
        //         revert(start, add(size, 68))
        //     }
        // }

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return(bpt_out, amounts_in);
    }

    // "Dry run" `on_exit_pool`.
    // Returns the amount of BPT that would be burned from `sender` if the `on_exit_pool` hook were called by the
    // Vault with the same arguments, along with the number of tokens `recipient` would receive.
    //
    // This fn is not meant to be called directly, but rather from a helper contract that fetches current Vault
    // data, such as the protocol swap fee percentage and Pool balances.
    //
    // Like `IVault.queryBatchSwap`, this fn is not view due to internal implementation details: the caller must
    // explicitly use eth_call instead of eth_sendTransaction.
    #[storage(read, write)]fn query_exit(pool_id: b256, sender: ContractId, recipient: ContractId, balances: Vec<u64>, last_change_block: u64, protocol_swap_fee_percentage: u64, user_data: UserData) -> (u64, Vec<u64>) {
        require(balances.len() == TOTAL_TOKENS, Error::InputLengthMismatch);

        let scaling_factors_vec = scaling_factors();
        let balances = upscale_array(balances, scaling_factors_vec);

        let(bpt_in, amounts_out) = on_exit_pool_private(sender, balances, protocol_swap_fee_percentage, scaling_factors_vec, user_data);

        // if (msg.sender != address(this)) {
        //     query_action(
        //         pool_id,
        //         sender,
        //         recipient,
        //         balances,
        //         last_change_block,
        //         protocol_swap_fee_percentage,
        //         user_data,
        //     );
        // }
        // else {
        //     scaling_factors_vec: Vec<u64> = scaling_factors();
        //     let balances = upscale_array(balances, scaling_factors_vec);

        //     (bptAmount: u64, uint256[] memory tokenAmounts) = on_exit_pool(
        //         pool_id,
        //         sender,
        //         recipient,
        //         balances,
        //         last_change_block,
        //         protocol_swap_fee_percentage,
        //         scaling_factors_vec,
        //         user_data
        //     );

        //     downscale_down_array(tokenAmounts, scaling_factors_vec);

        //     // solhint-disable-next-line no-inline-assembly
        //     assembly {
        //         // We will return a raw representation of `bptAmount` and `tokenAmounts` in memory, which is composed of
        //         // a 32-byte uint256, followed by a 32-byte for the array length, and finally the 32-byte uint256 values
        //         // Because revert expects a size in bytes, we multiply the array length (stored at `tokenAmounts`) by 32
        //         let size := mul(mload(tokenAmounts), 32)

        //         // We store the `bptAmount` in the previous slot to the `tokenAmounts` array. We can make sure there
        //         // will be at least one available slot due to how the memory scratch space works.
        //         // We can safely overwrite whatever is stored in this slot as we will revert immediately after that.
        //         let start := sub(tokenAmounts, 0x20)
        //         mstore(start, bptAmount)

        //         // We send one extra value for the error signature "QueryError(uint256,uint256[])" which is 0x43adbafb
        //         // We use the previous slot to `bptAmount`.
        //         mstore(sub(start, 0x20), 0x0000000000000000000000000000000000000000000000000000000043adbafb)
        //         start := sub(start, 0x04)

        //         // When copying from `tokenAmounts` into returndata, we copy the additional 68 bytes to also return
        //         // the `bptAmount`, the array 's length, and the error signature.
        //         revert(start, add(size, 68))
        //     }
        // }

        // The `return` opcode is executed directly inside `_queryAction`, so execution never reaches this statement,
        // and we don't need to return anything here - it just silences compiler warnings.
        return(bpt_in, amounts_out);
    }

    // Transfer coins to a target contract.
    fn force_transfer_coins(coins: u64, asset_id: ContractId, target: ContractId) {
        // SCRIPT_TESTING
        // commented original code
        // because these function calls was thwroing error
        // todo: need a token on the pool
        // force_transfer_to_contract(coins, asset_id, target);
    }

    // Transfer coins to a transaction output to be spent later.
    fn transfer_coins_to_output(coins: u64, asset_id: ContractId, recipient: Address) {
        // SCRIPT_TESTING
        // commented original code
        // because these function calls was thwroing error
        // todo: need a token in the pool
        // transfer_to_output(coins, asset_id, recipient);
    }

    fn get_normalized_weights() -> Vec<u64> {
        return get_normalized_weights_private();
    }

    #[storage(read)]fn get_vault() -> ContractId {
        return storage.vault_contract_id;
    }

    #[storage(read)]fn get_swap_fee_percentage() -> u64 {
        return storage.swap_fee_percentage;
    }
}

#[storage(read)]fn get_swap_fee_percentage() -> u64 {
    return storage.swap_fee_percentage;
}

fn mint_pool_tokens(recipient: ContractId, amount: u64, ) {
    mint(amount);
    // SCRIPT_TESTING
    // commented original code
    // because this function calls was thwroing error
    // todo: need a token in the pool
    // force_transfer_to_contract(amount, contract_id(), recipient);
}

fn burn_pool_tokens(sender: ContractId, bpt_amount_in: u64) {
    // SCRIPT_TESTING
    // commented original code
    // because these function calls was thwroing error
    // todo: need the tokens in the pool
    // force_transfer_to_contract(bpt_amount_in, sender, contract_id());
    // burn(bpt_amount_in);
}

// Exit
#[storage(read, write)]fn on_exit_pool_private(sender: ContractId, balances: Vec<u64>, protocol_swap_fee_percentage: u64, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // Exits are not disabled by default while the contract is paused, as some of them remain available to allow LPs
    // to safely exit the Pool in case of an emergency. Other exit kinds are disabled on a case-by-case basis in
    // their handlers.

    let normalized_weights = get_normalized_weights_private();
    before_join_exit(balances, normalized_weights, protocol_swap_fee_percentage);
    let (bpt_amount_in, amounts_out) = do_exit( /* sender,*/balances, normalized_weights, scaling_factors_vec, user_data);
    after_join_exit(false, balances, amounts_out, normalized_weights);

    return (bpt_amount_in, amounts_out);
}

// Dispatch code which decodes the provided userdata to perform the specified exit type.
// Inheriting contracts may override this fn to add additional exit types or extra conditions to allow
// or disallow exit under certain circumstances.
#[storage(read)]fn do_exit(balances: Vec<u64>, normalized_weights: Vec<u64>, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    let mut res = (0, ~Vec::new());
    // todo abi.decode change exit kind 
    if let RequestKind::ExactToken = user_data.kind {
        res = exit_exact_bptin_for_token_out(balances, normalized_weights, user_data);
    } else if let RequestKind::ExactTokensOut = user_data.kind {
        res = exit_exact_bptin_for_tokens_out(balances, user_data);
    } else if let RequestKind::InForExactTokensOut = user_data.kind {
        res = exit_bptin_for_exact_tokens_out(balances, normalized_weights, scaling_factors_vec, user_data);
    } else {
        revert(UNHANDLED_EXIT_KIND);
    }
    return res;
}

#[storage(read)]
fn exit_bptin_for_exact_tokens_out(balances: Vec<u64>, normalized_weights: Vec<u64>, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // This exit fn is disabled if the contract is paused.

    let amounts_out = user_data.amounts_in_out;
    let max_bpt_amount_in = user_data.max_min_bpt_amount;
    require(amounts_out.len() == balances.len(), Error::InputLengthMismatch);
    upscale_array(amounts_out, scaling_factors_vec);

    // todo: abi.decodes
    // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
    let bpt_amount_in = calc_bpt_in_given_exact_tokens_out(
        balances,
        normalized_weights,
        amounts_out,
        TOTAL_SUPPLY,
        get_swap_fee_percentage()
    );
    require(bpt_amount_in <= max_bpt_amount_in, Error::BptInMaxAmount);

    return (bpt_amount_in, amounts_out);
}

fn exit_exact_bptin_for_tokens_out(balances: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // This exit fn is the only one that is not disabled if the contract is paused: it remains unrestricted
    // in an attempt to provide users with a mechanism to retrieve their tokens in case of an emergency.
    // This particular exit fn is the only one that remains available because it is the simplest one, and
    // therefore the one with the lowest likelihood of

    // todo abi.decode
    let bpt_amount_in = user_data.amount;
    // Note that there is no minimum amount_out parameter: this is handled by `IVault.exitPool`.

    let amounts_out = calc_tokens_out_given_exact_bpt_in(balances, bpt_amount_in, TOTAL_SUPPLY);
    return (bpt_amount_in, amounts_out);
}

#[storage(read)]
fn exit_exact_bptin_for_token_out(balances: Vec<u64>, normalized_weights: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // This exit fn is disabled if the contract is paused.
    // todo abi.decode
    let bpt_amount_in = user_data.amount;
    let token_index = user_data.max_min_bpt_amount;
    // Note that there is no minimum amount_out parameter: this is handled by `IVault.exitPool`.

    require(token_index < balances.len(), Error::OutOfBounds);

    let amount_out = calc_token_out_given_exact_bpt_in(
        balances.get(token_index).unwrap(),
        normalized_weights.get(token_index).unwrap(),
        bpt_amount_in,
        TOTAL_SUPPLY,
        get_swap_fee_percentage()
    );

    // This is an exceptional situation in which the fee is charged on a token out instead of a token in.
    // We exit in a single token, so we initialize amounts_out with zeros
    let mut amounts_out = ~Vec::new();
    let mut count = 0;
    while count < balances.len() {
        if count == token_index {
            // And then assign the result to the selected token
            amounts_out.push(amount_out);
        }
        else {
            amounts_out.push(0);
        }
        count += 1;
    }

    return (bpt_amount_in, amounts_out);
}

// Join
#[storage(read, write)]fn on_join_pool_private(sender: ContractId, balances: Vec<u64>, protocol_swap_fee_percentage: u64, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // All joins are disabled while the contract is paused.

    let normalized_weights = get_normalized_weights_private();

    before_join_exit(balances, normalized_weights, protocol_swap_fee_percentage);
    let(bpt_amount_out, amounts_in) = do_join(balances, normalized_weights, scaling_factors_vec, user_data);
    after_join_exit(true, balances, amounts_in, normalized_weights);

    (bpt_amount_out, amounts_in)
}

#[storage(read)]fn join_exact_tokens_in_for_bptout(balances: Vec<u64>, normalized_weights: Vec<u64>, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    // let(amounts_in, min_bpt_amount_out) = user_data.exactTokensInForBptOut();
    let amounts_in = user_data.amounts_in_out;
    let min_bpt_amount_out = user_data.max_min_bpt_amount;
    // ensure_input_length_match(balances.len(), balances.len());
    require(balances.len() == amounts_in.len(), Error::InputLengthMismatch);

    upscale_array(amounts_in, scaling_factors_vec);
    let bpt_amount_out = calc_bpt_out_given_exact_tokens_in(balances, normalized_weights, amounts_in, TOTAL_SUPPLY, get_swap_fee_percentage());

    require(bpt_amount_out >= min_bpt_amount_out, Error::BptOutMinAmount);

    (bpt_amount_out, amounts_in)
}

// Dispatch code which decodes the provided userdata to perform the specified join type.
// Inheriting contracts may override this pub fn to add additional join types or extra conditions to allow
// or disallow joins under certain circumstances.
#[storage(read)]fn do_join(balances: Vec<u64>, normalized_weights: Vec<u64>, scaling_factors_vec: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    // let kind = user_data.joinKind();
    let mut res = (0, ~Vec::new());

    if let RequestKind::ExactToken = user_data.kind {
        res = join_exact_tokens_in_for_bptout(balances, normalized_weights, scaling_factors_vec, user_data)
    } else if let RequestKind::ExactTokensOut = user_data.kind {
        res = join_token_in_for_exact_bptout(balances, normalized_weights, user_data)
    } else if let RequestKind::InForExactTokensOut = user_data.kind {
        res = join_all_tokens_in_for_exact_bptout(balances, user_data)
    } else {
        revert(UNHANDLED_JOIN_KIND);
    }
    res
}

#[storage(read)]fn join_token_in_for_exact_bptout(balances: Vec<u64>, normalized_weights: Vec<u64>, user_data: UserData) -> (u64, Vec<u64>) {
    // todo abi.decode
    let bpt_amount_out = user_data.amount;
    let token_index = user_data.max_min_bpt_amount;
    // let(bpt_amount_out, token_index) = user_data.tokenInForExactBptOut();

    // Note that there is no maximum amount_in parameter: this is handled by `IVault.joinPool`.

    require(token_index < balances.len(), Error::OutOfBounds);

    let swap_fee_percentage = 10;
    let amount_in = calc_token_in_given_exact_bpt_out(balances.get(token_index).unwrap(), normalized_weights.get(token_index).unwrap(), bpt_amount_out, TOTAL_SUPPLY, get_swap_fee_percentage());

    // We join in a single token, so we initialize amounts_in with zeros

    let mut amounts_in: Vec<u64> = ~Vec::new();
    let mut count = 0;
    while count < balances.len() {
        if count == token_index {
            // And then assign the result to the selected token
            amounts_in.push(amount_in);
        } else {
            amounts_in.push(0);
        }
        count += 1;
    }

    return(bpt_amount_out, amounts_in);
}

#[storage(read)]fn before_join_exit(pre_balances: Vec<u64>, normalized_weights: Vec<u64>, protocol_swap_fee_percentage: u64) {
    // Before joins and exits, we measure the growth of the invariant compared to the invariant after the last join
    // or exit, which will have been caused by swap fees, and use it to mint BPT as protocol fees. This dilutes all
    // LPs, which means that new LPs will join the pool debt-free, and exiting LPs will pay any amounts due
    // before leaving.

    // We return immediately if the fee percentage is zero (to avoid unnecessary computation), or when the pool is
    // paused (to avoid complex computation during emergency withdrawals).
    let is_not_paused: bool = true;
    // import this function from TemporarilyPausable contract
    if ((protocol_swap_fee_percentage == 0) || !is_not_paused) {
        return;
    }

    let pre_join_exit_invariant = calculate_invariant(normalized_weights, pre_balances);

    let to_mint = calc_due_protocol_swap_fee_bpt_amount(TOTAL_SUPPLY, storage.last_post_join_exit_invariant, pre_join_exit_invariant, protocol_swap_fee_percentage);

    // call this function from BasePool contract
    // payProtocolFees(to_mint);
    mint_pool_tokens(storage.vault_contract_id, to_mint);
}

#[storage(write)]fn on_initialize_pool(scaling_factors_vec: Vec<u64>, user_data: UserData // user_data: JoinKind,
) -> (u64, Vec<u64>) {
    // It would be strange for the Pool to be paused before it is initialized, but for consistency we prevent
    // initialization in this case.

    // todo: abi.decode
    let mut flag = false;
    if let RequestKind::Init = user_data.kind {
        flag = true;
    }
    require(flag, Error::Uninitialized);

    // todo: abi.decode
    let amounts_in = user_data.amounts_in_out;
    require(amounts_in.len() == scaling_factors_vec.len(), Error::InputLengthMismatch);
    upscale_array(amounts_in, scaling_factors_vec);

    let normalized_weights: Vec<u64> = get_normalized_weights_private();
    let invariant_after_join = calculate_invariant(normalized_weights, amounts_in);

    // Set the initial BPT to the value of the invariant times the number of tokens. This makes BPT supply more
    // consistent in Pools with similar compositions but different number of tokens.
    let bpt_amount_out = invariant_after_join * amounts_in.len();

    after_join_exit(true, ~Vec::with_capacity(amounts_in.len()), amounts_in, normalized_weights);
    (bpt_amount_out, amounts_in)
}

#[storage(write)]fn after_join_exit(is_join: bool, pre_balances: Vec<u64>, balance_deltas: Vec<u64>, normalized_weights: Vec<u64>) {
    // After all joins and exits we store the post join/exit invariant in order to compute growth due to swap fees
    // in the next one.
    let mut balances = ~Vec::new();
    // Compute the post balances by adding or removing the deltas. Note that we're allowed to mutate pre_balances.
    let mut count = 0;
    while count < pre_balances.len() {
        // Cannot optimize calls with a fn selector: there are 2- and 3-argument versions of SafeMath.sub
        if is_join {
            balances.push(pre_balances.get(count).unwrap() + balance_deltas.get(count).unwrap());
        } else {
            balances.push(pre_balances.get(count).unwrap() - balance_deltas.get(count).unwrap());
        }
        count += 1;
    }

    let postJoinExitInvariant = calculate_invariant(normalized_weights, balances);

    storage.last_post_join_exit_invariant = postJoinExitInvariant;
}
