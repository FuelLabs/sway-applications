contract;

dep errors;
dep utils;

use errors::{InitError, InputError, TransactionError};
use libraries::{
    data_structures::{
        PoolInfo,
        PreviewAddLiquidityInfo,
        PreviewSwapInfo,
        RemoveLiquidityInfo,
    },
    Exchange,
};
use std::{
    block::height,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    context::{
        call_frames::{
            contract_id,
            msg_asset_id,
        },
        msg_amount,
    },
    logging::log,
    math::*,
    prelude::*,
    storage::StorageMap,
    token::{
        burn,
        mint,
        transfer,
    },
};
use utils::{
    div_multiply,
    get_maximum_input_for_exact_output,
    get_minimum_output_given_exact_input,
    multiply_div,
};

storage {
    /// Deposit amounts per (depositer, asset) that can be used to add liquidity or be withdrawn.
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Total amount of the liquidity pool asset that has a unique identifier different from the identifiers of assets on either side of the pool.
    liquidity_pool_supply: u64 = 0,
    /// The unique identifiers that make up the pool that can be set only once using the `constructor`.
    pair: Option<(ContractId, ContractId)> = Option::None(),
    /// Reserve amounts per asset A and asset B
    reserves: StorageMap<ContractId, u64> = StorageMap {},
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);
        require(deadline > height(), InputError::DeadlinePassed);
        require(msg_amount() == 0, InputError::AmountMustBeZero);
        require(MINIMUM_LIQUIDITY <= desired_liquidity, InputError::AmountTooLow(desired_liquidity));
        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.liquidity_pool_supply;
        let asset_a_in_deposit = storage.deposits.get((sender, asset_a_id));
        let asset_b_in_deposit = storage.deposits.get((sender, asset_b_id));
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut liquidity_to_add = 0;
        let mut minted = 0;
        require(asset_a_in_deposit != 0, TransactionError::DepositCannotBeZero);
        require(asset_b_in_deposit != 0, TransactionError::DepositCannotBeZero);
        if asset_a_in_reserve == 0 && asset_b_in_reserve == 0 {
            liquidity_to_add = (asset_a_in_deposit * asset_b_in_deposit).sqrt();
            require(desired_liquidity <= liquidity_to_add, TransactionError::DesiredAmountTooHigh(desired_liquidity));
            storage.reserves.insert(asset_a_id, asset_a_in_deposit);
            storage.reserves.insert(asset_b_id, asset_b_in_deposit);
            mint(liquidity_to_add);
            storage.liquidity_pool_supply = liquidity_to_add;
            transfer(liquidity_to_add, contract_id(), sender);
        } else {
            let mut added_a = 0;
            let mut added_b = 0;
            let b_to_attempt = multiply_div(asset_a_in_deposit, asset_b_in_reserve, asset_a_in_reserve);
            if b_to_attempt <= asset_b_in_deposit {
                liquidity_to_add = multiply_div(b_to_attempt, total_liquidity, asset_b_in_reserve);
                require(desired_liquidity <= liquidity_to_add, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_a = asset_a_in_deposit;
                added_b = b_to_attempt;
            } else {
                let a_to_attempt = multiply_div(asset_b_in_deposit, asset_a_in_reserve, asset_b_in_reserve);
                liquidity_to_add = multiply_div(a_to_attempt, total_liquidity, asset_a_in_reserve);
                require(desired_liquidity <= liquidity_to_add, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_a = a_to_attempt;
                added_b = asset_b_in_deposit;
            }
            storage.reserves.insert(asset_a_id, asset_a_in_reserve + added_a);
            storage.reserves.insert(asset_b_id, asset_b_in_reserve + added_b);
            mint(liquidity_to_add);
            storage.liquidity_pool_supply = total_liquidity + liquidity_to_add;
            transfer(liquidity_to_add, contract_id(), sender);

            let refund_a = asset_a_in_deposit - added_a;
            if refund_a > 0 {
                transfer(refund_a, asset_a_id, sender);
            }
            let refund_b = asset_b_in_deposit - added_b;
            if refund_b > 0 {
                transfer(refund_b, asset_b_id, sender);
            }
        }
        storage.deposits.insert((sender, asset_a_id), 0);
        storage.deposits.insert((sender, asset_b_id), 0);
        liquidity_to_add
    }

    #[storage(read, write)]
    fn constructor(pair: (ContractId, ContractId)) {
        require(storage.pair.is_none(), InitError::CannotReinitialize);
        require(pair.0 != pair.1, InitError::IdenticalAssets);

        storage.pair = Option::Some(pair);
    }

    #[storage(read, write)]
    fn deposit() {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let deposit_asset = msg_asset_id();

        require(deposit_asset == storage.pair.unwrap().0 || deposit_asset == storage.pair.unwrap().1, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let new_deposit_amount = storage.deposits.get((sender, deposit_asset, )) + msg_amount();
        storage.deposits.insert((sender, deposit_asset, ), new_deposit_amount);
    }

    #[storage(read, write)]
    fn preview_swap_with_exact_input(exact_input: u64, input_asset: ContractId) -> PreviewSwapInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(input_asset == asset_a_id || input_asset == asset_b_id, InputError::InvalidAsset);

        let output_asset = if input_asset == asset_a_id {
            asset_b_id
        } else {
            asset_a_id
        };
        let input_asset_in_reserve = storage.reserves.get(input_asset);
        let output_asset_in_reserve = storage.reserves.get(output_asset);
        let min_output = get_minimum_output_given_exact_input(exact_input, input_asset_in_reserve, output_asset_in_reserve, LIQUIDITY_MINER_FEE);
        let output_reserve_sufficient = min_output <= output_asset_in_reserve;
        PreviewSwapInfo {
            amount: min_output,
            output_reserve_sufficient: output_reserve_sufficient,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_exact_output(exact_output: u64, output_asset: ContractId) -> PreviewSwapInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(output_asset == asset_a_id || output_asset == asset_b_id, InputError::InvalidAsset);

        let input_asset = if output_asset == asset_a_id {
            asset_b_id
        } else {
            asset_a_id
        };
        let input_asset_in_reserve = storage.reserves.get(input_asset);
        let output_asset_in_reserve = storage.reserves.get(output_asset);
        require(exact_output <= output_asset_in_reserve, TransactionError::DesiredAmountTooHigh(exact_output));
        let max_input = get_maximum_input_for_exact_output(exact_output, input_asset_in_reserve, output_asset_in_reserve, LIQUIDITY_MINER_FEE);
        let output_reserve_sufficient = exact_output <= output_asset_in_reserve;
        PreviewSwapInfo {
            amount: max_input,
            output_reserve_sufficient: output_reserve_sufficient,
        }
    }

    #[storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let total_liquidity = storage.liquidity_pool_supply;
        require(total_liquidity > 0, TransactionError::LiquidityCannotBeZero);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(msg_asset_id() == contract_id(), InputError::InvalidAsset);
        require(min_asset_a > 0 && min_asset_b > 0, InputError::AmountCannotBeZero);
        require(deadline > height(), InputError::DeadlinePassed);

        let amount = msg_amount();

        require(amount > 0, InputError::AmountCannotBeZero);

        let sender = msg_sender().unwrap();
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let asset_a_amount_to_remove = multiply_div(amount, asset_a_in_reserve, total_liquidity);
        let asset_b_amount_to_remove = multiply_div(amount, asset_b_in_reserve, total_liquidity);

        require(asset_a_amount_to_remove >= min_asset_a, TransactionError::DesiredAmountTooHigh(min_asset_a));
        require(asset_b_amount_to_remove >= min_asset_b, TransactionError::DesiredAmountTooHigh(min_asset_b));

        burn(amount);
        storage.liquidity_pool_supply = total_liquidity - amount;
        storage.reserves.insert(asset_b_id, asset_b_in_reserve - asset_b_amount_to_remove);
        storage.reserves.insert(asset_a_id, asset_a_in_reserve - asset_a_amount_to_remove);
        transfer(asset_a_amount_to_remove, asset_a_id, sender);
        transfer(asset_b_amount_to_remove, asset_b_id, sender);
        RemoveLiquidityInfo {
            asset_a_amount: asset_a_amount_to_remove,
            asset_b_amount: asset_b_amount_to_remove,
            liquidity: amount,
        }
    }

    #[storage(read, write)]
    fn swap_with_exact_input(min_output: Option<u64>, deadline: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();
        let input_asset = msg_asset_id();

        require(input_asset == asset_a_id || input_asset == asset_b_id, InputError::InvalidAsset);
        require(deadline >= height(), InputError::DeadlinePassed);

        let exact_input = msg_amount();
        require(exact_input > 0, InputError::AmountCannotBeZero);

        let sender = msg_sender().unwrap();
        let output_asset = if input_asset == asset_a_id {
            asset_b_id
        } else {
            asset_a_id
        };
        let input_asset_in_reserve = storage.reserves.get(input_asset);
        let output_asset_in_reserve = storage.reserves.get(output_asset);
        let bought = get_minimum_output_given_exact_input(exact_input, input_asset_in_reserve, output_asset_in_reserve, LIQUIDITY_MINER_FEE);
        require(bought <= output_asset_in_reserve, TransactionError::InsufficientLiquidity);
        if min_output.is_some() {
            require(bought >= min_output.unwrap(), TransactionError::DesiredAmountTooHigh(min_output.unwrap()));
        }
        transfer(bought, output_asset, sender);
        storage.reserves.insert(input_asset, input_asset_in_reserve + exact_input);
        storage.reserves.insert(output_asset, output_asset_in_reserve - bought);
        bought
    }

    #[storage(read, write)]
    fn swap_with_exact_output(output: u64, deadline: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();
        let input_asset = msg_asset_id();

        require(input_asset == asset_a_id || input_asset == asset_b_id, InputError::InvalidAsset);
        require(deadline > height(), InputError::DeadlinePassed);
        require(output > 0, InputError::AmountCannotBeZero);

        let input_amount = msg_amount();
        require(input_amount > 0, InputError::AmountCannotBeZero);

        let sender = msg_sender().unwrap();
        let output_asset = if input_asset == asset_a_id {
            asset_b_id
        } else {
            asset_a_id
        };
        let input_asset_in_reserve = storage.reserves.get(input_asset);
        let output_asset_in_reserve = storage.reserves.get(output_asset);

        require(output <= output_asset_in_reserve, TransactionError::InsufficientLiquidity);

        let sold = get_maximum_input_for_exact_output(output, input_asset_in_reserve, output_asset_in_reserve, LIQUIDITY_MINER_FEE);

        require(input_amount >= sold, TransactionError::ProvidedAmountTooLow(input_amount));

        let refund = input_amount - sold;
        if refund > 0 {
            transfer(refund, input_asset, sender);
        };
        transfer(output, output_asset, sender);
        storage.reserves.insert(input_asset, input_asset_in_reserve + sold);
        storage.reserves.insert(output_asset, output_asset_in_reserve - output);
        sold
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId) {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(asset == asset_a_id || asset == asset_b_id, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let deposited_amount = storage.deposits.get((sender, asset));

        require(deposited_amount >= amount, TransactionError::InsufficientDeposit);

        let new_amount = deposited_amount - amount;
        storage.deposits.insert((sender, asset), new_amount);
        transfer(amount, asset, sender)
    }

    #[storage(read)]
    fn balance(asset: ContractId) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);
        require(asset == storage.pair.unwrap().0 || asset == storage.pair.unwrap().1, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        storage.deposits.get((sender, asset))
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();
        PoolInfo {
            asset_a_id: asset_a_id,
            asset_b_id: asset_b_id,
            asset_a_reserve: storage.reserves.get(asset_a_id),
            asset_b_reserve: storage.reserves.get(asset_b_id),
            liquidity: storage.liquidity_pool_supply,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        let total_liquidity = storage.liquidity_pool_supply;
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);

        let asset_a_in_deposit = if asset == asset_a_id || asset_b_in_reserve == 0 {
            amount
        } else {
            multiply_div(amount, asset_a_in_reserve, asset_b_in_reserve)
        };
        let asset_b_in_deposit = if asset == asset_b_id || asset_a_in_reserve == 0 {
            amount
        } else {
            multiply_div(amount, asset_b_in_reserve, asset_a_in_reserve)
        };

        let mut liquidity_to_add = 0;
        let mut added_a = asset_a_in_deposit;
        let mut added_b = asset_b_in_deposit;

        if asset_a_in_reserve == 0 && asset_b_in_reserve == 0 {
            liquidity_to_add = (asset_a_in_deposit * asset_b_in_deposit).sqrt();
        } else {
            let added_b = multiply_div(asset_a_in_deposit, asset_b_in_reserve, asset_a_in_reserve);
            liquidity_to_add = multiply_div(added_b, total_liquidity, asset_b_in_reserve);
        }
        PreviewAddLiquidityInfo {
            other_asset_amount_to_add: if asset == asset_a_id {
                added_b
            } else {
                added_a
            },
            liquidity_asset_amount_to_receive: liquidity_to_add,
        }
    }
}
