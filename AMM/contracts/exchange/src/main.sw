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
    /// Deposit amounts per (depositer, asset). Can be used to add liquidity or be withdrawn.
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Supply of the asset that makes up the liquidity pool.
    /// This asset has a unique identifier different from the identifiers of assets on either side of the pool.
    liquidity_pool_supply: u64 = 0,
    /// The unique identifiers that make up the pool that can be set only once using the `constructor`.
    pair: Option<(ContractId, ContractId)> = Option::None(),
    /// Reserve amounts per asset identifiers that make up the pool.
    reserves: StorageMap<ContractId, u64> = StorageMap {},
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);
        require(deadline > height(), TransactionError::DeadlinePassed);
        require(msg_amount() == 0, InputError::SentInvalidAmount);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(msg_asset_id() == asset_a_id || msg_asset_id() == asset_b_id, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.liquidity_pool_supply;
        let asset_a_in_deposit = storage.deposits.get((sender, asset_a_id, ));

        require(asset_a_in_deposit > 0, TransactionError::InsufficientDeposit);

        let asset_b_in_deposit = storage.deposits.get((sender, asset_b_id, ));
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut minted = 0;
        if total_liquidity > 0 {
            require(min_liquidity > 0, InputError::SentInvalidAmount);

            let asset_b_amount_to_add = multiply_div(asset_a_in_deposit, asset_b_in_reserve, asset_a_in_reserve);
            let liquidity_to_mint = multiply_div(asset_a_in_deposit, total_liquidity, asset_a_in_reserve);

            require(liquidity_to_mint >= min_liquidity, TransactionError::CannotSatisfyConstraint);

            if (asset_b_in_deposit >= asset_b_amount_to_add) {
                storage.reserves.insert(asset_b_id, asset_b_in_reserve + asset_b_amount_to_add);
                storage.reserves.insert(asset_a_id, asset_a_in_reserve + asset_a_in_deposit);
                mint(liquidity_to_mint);
                storage.liquidity_pool_supply = total_liquidity + liquidity_to_mint;
                transfer(liquidity_to_mint, contract_id(), sender);
                let asset_b_amount_extra = asset_b_in_deposit - asset_b_amount_to_add;
                if (asset_b_amount_extra > 0) {
                    transfer(asset_b_amount_extra, asset_b_id, sender);
                }
                minted = liquidity_to_mint;
            } else {
                transfer(asset_b_in_deposit, asset_b_id, sender);
                transfer(asset_a_in_deposit, asset_a_id, sender);
                minted = 0;
            }
        } else {
            require(asset_a_in_deposit > minimum_liquidity, TransactionError::CannotSatisfyConstraint);

            let initial_liquidity = asset_a_in_deposit;
            storage.reserves.insert(asset_b_id, asset_b_in_reserve + asset_b_in_deposit);
            storage.reserves.insert(asset_a_id, asset_a_in_reserve + asset_a_in_deposit);
            mint(initial_liquidity);
            storage.liquidity_pool_supply = initial_liquidity;
            transfer(initial_liquidity, contract_id(), sender);
            minted = initial_liquidity;
        };
        storage.deposits.insert((sender, asset_b_id, ), 0);
        storage.deposits.insert((sender, asset_a_id, ), 0);
        minted
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
    fn preview_swap_with_maximum(exact_output_amount: u64) -> PreviewSwapInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let asset_to_sell = msg_asset_id();
        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(asset_to_sell == asset_a_id || asset_to_sell == asset_b_id, InputError::SentInvalidAsset);

        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut maximum_amount_to_input = 0;
        let mut reserve_remains = false;
        if (asset_to_sell == asset_a_id) {
            require(exact_output_amount < asset_b_in_reserve, TransactionError::InsufficientReserve);

            maximum_amount_to_input = get_maximum_input_for_exact_output(asset_a_in_reserve, liquidity_miner_fee, exact_output_amount, asset_b_in_reserve);
            reserve_remains = maximum_amount_to_input < asset_a_in_reserve;
        } else {
            require(exact_output_amount < asset_a_in_reserve, TransactionError::InsufficientReserve);

            maximum_amount_to_input = get_maximum_input_for_exact_output(asset_b_in_reserve, liquidity_miner_fee, exact_output_amount, asset_a_in_reserve);
            reserve_remains = maximum_amount_to_input < asset_b_in_reserve;
        }
        PreviewSwapInfo {
            amount: maximum_amount_to_input,
            reserve_depleted: !reserve_remains,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_minimum(exact_input_amount: u64) -> PreviewSwapInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let asset_to_sell = msg_asset_id();
        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(asset_to_sell == asset_a_id || asset_to_sell == asset_b_id, InputError::SentInvalidAsset);

        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut minimum_amount_of_output = 0;
        let mut reserve_remains = false;
        if (asset_to_sell == asset_a_id) {
            minimum_amount_of_output = get_minimum_output_given_exact_input(exact_input_amount, asset_a_in_reserve, liquidity_miner_fee, asset_b_in_reserve);
            reserve_remains = minimum_amount_of_output < asset_b_in_reserve;
        } else {
            minimum_amount_of_output = get_minimum_output_given_exact_input(exact_input_amount, asset_b_in_reserve, liquidity_miner_fee, asset_a_in_reserve);
            reserve_remains = minimum_amount_of_output < asset_a_in_reserve;
        }
        PreviewSwapInfo {
            amount: minimum_amount_of_output,
            reserve_depleted: !reserve_remains,
        }
    }

    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_asset_a: u64, min_asset_b: u64, ) -> RemoveLiquidityInfo {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let total_liquidity = storage.liquidity_pool_supply;
        require(total_liquidity > 0, TransactionError::InsufficientLiquidity);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();

        require(msg_asset_id() == contract_id(), InputError::SentInvalidAsset);
        require(min_asset_a > 0 && min_asset_b > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline > height(), TransactionError::DeadlinePassed);

        let amount = msg_amount();

        require(amount > 0, InputError::SentInvalidAmount);

        let sender = msg_sender().unwrap();
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let asset_a_amount_to_remove = multiply_div(amount, asset_a_in_reserve, total_liquidity);
        let asset_b_amount_to_remove = multiply_div(amount, asset_b_in_reserve, total_liquidity);

        require((asset_a_amount_to_remove >= min_asset_a) && (asset_b_amount_to_remove >= min_asset_b), TransactionError::CannotSatisfyConstraint);

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
    fn swap_with_maximum(deadline: u64, exact_output_amount: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();
        let asset_to_sell = msg_asset_id();

        require(asset_to_sell == asset_a_id || asset_to_sell == asset_b_id, InputError::SentInvalidAsset);
        require(exact_output_amount > 0, InputError::SentInvalidAmount);
        require(deadline > height(), TransactionError::DeadlinePassed);

        let input_amount = msg_amount();
        require(input_amount > 0, InputError::SentInvalidAmount);

        let sender = msg_sender().unwrap();
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut sold = 0;
        if (asset_to_sell == asset_a_id) {
            let asset_a_sold = get_maximum_input_for_exact_output(asset_a_in_reserve, liquidity_miner_fee, exact_output_amount, asset_b_in_reserve);

            require(input_amount >= asset_a_sold, TransactionError::InsufficientReserve);

            let refund = input_amount - asset_a_sold;
            if refund > 0 {
                transfer(refund, asset_a_id, sender);
            };
            transfer(exact_output_amount, asset_b_id, sender);
            sold = asset_a_sold;
            storage.reserves.insert(asset_a_id, asset_a_in_reserve + asset_a_sold);
            storage.reserves.insert(asset_b_id, asset_b_in_reserve - exact_output_amount);
        } else {
            let asset_b_sold = get_maximum_input_for_exact_output(asset_b_in_reserve, liquidity_miner_fee, exact_output_amount, asset_a_in_reserve);

            require(input_amount >= asset_b_sold, TransactionError::InsufficientReserve);

            let refund = input_amount - asset_b_sold;
            if refund > 0 {
                transfer(refund, asset_b_id, sender);
            };
            transfer(exact_output_amount, asset_a_id, sender);
            sold = asset_b_sold;
            storage.reserves.insert(asset_a_id, asset_a_in_reserve - exact_output_amount);
            storage.reserves.insert(asset_b_id, asset_b_in_reserve + asset_b_sold);
        };
        sold
    }

    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, exact_input_amount: u64) -> u64 {
        require(storage.pair.is_some(), InitError::NotInitialized);

        let (asset_a_id, asset_b_id) = storage.pair.unwrap();
        let asset_to_sell = msg_asset_id();

        require(asset_to_sell == asset_a_id || asset_to_sell == asset_b_id, InputError::SentInvalidAsset);
        require(exact_input_amount > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline >= height(), TransactionError::DeadlinePassed);

        let output_amount = msg_amount();

        require(output_amount > 0, InputError::SentInvalidAmount);

        let sender = msg_sender().unwrap();
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let mut bought = 0;
        if (asset_to_sell == asset_a_id) {
            let asset_b_bought = get_minimum_output_given_exact_input(output_amount, asset_a_in_reserve, liquidity_miner_fee, asset_b_in_reserve);

            require(asset_b_bought >= exact_input_amount, TransactionError::CannotSatisfyConstraint);

            transfer(asset_b_bought, asset_b_id, sender);
            bought = asset_b_bought;
            storage.reserves.insert(asset_a_id, asset_a_in_reserve + output_amount);
            storage.reserves.insert(asset_b_id, asset_b_in_reserve - asset_b_bought);
        } else {
            let asset_a_bought = get_minimum_output_given_exact_input(output_amount, asset_b_in_reserve, liquidity_miner_fee, asset_a_in_reserve);

            require(asset_a_bought >= exact_input_amount, TransactionError::CannotSatisfyConstraint);

            transfer(asset_a_bought, asset_a_id, sender);
            bought = asset_a_bought;
            storage.reserves.insert(asset_a_id, asset_a_in_reserve - asset_a_bought);
            storage.reserves.insert(asset_b_id, asset_b_in_reserve + bought);
        };
        bought
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

        require(asset == asset_a_id || asset == asset_b_id, InputError::SentInvalidAsset);

        let total_liquidity = storage.liquidity_pool_supply;
        let asset_a_in_reserve = storage.reserves.get(asset_a_id);
        let asset_b_in_reserve = storage.reserves.get(asset_b_id);
        let current_asset_a_amount = if asset == asset_a_id {
            amount
        } else {
            require(asset_b_in_reserve != 0, TransactionError::InsufficientReserve);

            multiply_div(amount, asset_a_in_reserve, asset_b_in_reserve)
        };
        let mut liquidity_to_mint = 0;
        let mut other_asset_amount_to_add = 0;

        if total_liquidity > 0 {
            other_asset_amount_to_add = if asset == asset_a_id {
                multiply_div(current_asset_a_amount, asset_b_in_reserve, asset_a_in_reserve)
            } else {
                current_asset_a_amount
            };
            liquidity_to_mint = multiply_div(current_asset_a_amount, total_liquidity, asset_a_in_reserve);
        } else {
            liquidity_to_mint = current_asset_a_amount;
        }
        PreviewAddLiquidityInfo {
            other_asset_amount_to_add: other_asset_amount_to_add,
            liquidity_asset_amount_to_receive: liquidity_to_mint,
        }
    }
}
