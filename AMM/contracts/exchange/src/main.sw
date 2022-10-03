contract;

dep data_structures;
dep errors;
dep interface;
dep utils;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewInfo, RemoveLiquidityInfo};
use errors::{InitError, InputError, TransactionError};
use interface::Exchange;
use std::{
    block::height,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    context::{
        call_frames::*,
        msg_amount,
    },
    contract_id::ContractId,
    identity::Identity,
    option::Option,
    result::Result,
    revert::{
        require,
        revert,
    },
    storage::StorageMap,
    token::{
        burn,
        mint,
        transfer,
    },
};
use utils::{calculate_amount_with_fee, div_mutiply, get_input_price, get_output_price, mutiply_div};

storage {
    /// Asset that is on the other side of the pool
    other_asset: Option<ContractId> = Option::None(),
    /// Deposit amounts per (depositer, asset)
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Total asset supply
    total_liquidity: u64 = 0,
    /// Reserve amount per asset
    reserves: StorageMap<ContractId, u64> = StorageMap {},
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64 {
        require(storage.other_asset.is_some(), InitError::NotInitialized);
        require(deadline > height(), TransactionError::DeadlinePassed);
        require(msg_amount() == 0, InputError::SentInvalidAmount);

        let other_asset = storage.other_asset.unwrap();

        require(msg_asset_id().into() == base_asset || msg_asset_id() == other_asset, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.total_liquidity;
        let base_asset_in_deposit = storage.deposits.get((
            sender,
            ~ContractId::from(base_asset),
        ));

        require(base_asset_in_deposit > 0, TransactionError::InsufficientDeposit);

        let other_asset_in_deposit = storage.deposits.get((
            sender,
            other_asset,
        ));
        let base_asset_in_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_in_reserve = storage.reserves.get(other_asset);
        let mut minted = 0;
        if total_liquidity > 0 {
            require(min_liquidity > 0, InputError::SentInvalidAmount);

            let other_asset_amount_to_add = mutiply_div(base_asset_in_deposit, other_asset_in_reserve, base_asset_in_reserve);
            let liquidity_to_mint = mutiply_div(base_asset_in_deposit, total_liquidity, base_asset_in_reserve);

            require(liquidity_to_mint >= min_liquidity, TransactionError::CannotSatisfyConstraint);

            // If ratio is correct, proceed with liquidity operation
            // Otherwise, return current user balances in contract
            if (other_asset_in_deposit >= other_asset_amount_to_add) {
                // Add fund to the reserves
                storage.reserves.insert(other_asset, other_asset_in_reserve + other_asset_amount_to_add);
                storage.reserves.insert(~ContractId::from(base_asset), base_asset_in_reserve + base_asset_in_deposit);
                // Mint LP asset
                mint(liquidity_to_mint);
                storage.total_liquidity = total_liquidity + liquidity_to_mint;
                transfer(liquidity_to_mint, contract_id(), sender);
                // If user sent more than the correct ratio, deposit the extra assets back
                let asset_amount_extra = other_asset_in_deposit - other_asset_amount_to_add;
                if (asset_amount_extra > 0) {
                    transfer(asset_amount_extra, other_asset, sender);
                }
                minted = liquidity_to_mint;
            } else {
                transfer(other_asset_in_deposit, other_asset, sender);
                transfer(base_asset_in_deposit, ~ContractId::from(base_asset), sender);
                minted = 0;
            }
        } else {
            require(base_asset_in_deposit > min_liquidity, TransactionError::CannotSatisfyConstraint);

            let initial_liquidity = base_asset_in_deposit;
            // Add fund to the reserves
            storage.reserves.insert(other_asset, other_asset_in_reserve + other_asset_in_deposit);
            storage.reserves.insert(~ContractId::from(base_asset), base_asset_in_reserve + base_asset_in_deposit);
            // Mint LP asset
            mint(initial_liquidity);
            storage.total_liquidity = initial_liquidity;
            transfer(initial_liquidity, contract_id(), sender);
            minted = initial_liquidity;
        };
        // Clear user contract balances after finishing add/create liquidity
        storage.deposits.insert((
            sender,
            other_asset,
        ), 0);
        storage.deposits.insert((
            sender,
            ~ContractId::from(base_asset),
        ), 0);
        minted
    }

    #[storage(read)]
    fn balance(asset: ContractId) -> u64 {
        require(storage.other_asset.is_some(), InitError::NotInitialized);
        require(asset.into() == base_asset || asset == storage.other_asset.unwrap(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        storage.deposits.get((sender, asset))
    }

    #[storage(read, write)]
    fn constructor(asset: ContractId) {
        require(storage.other_asset.is_none(), InitError::NotInitialized);

        storage.other_asset = Option::Some(asset);
    }

    #[storage(read, write)]
    fn deposit() {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let deposit_asset = msg_asset_id();

        require(deposit_asset.into() == base_asset || deposit_asset == storage.other_asset.unwrap(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let total_amount = storage.deposits.get((
            sender,
            deposit_asset,
        )) + msg_amount();
        storage.deposits.insert((
            sender,
            deposit_asset,
        ), total_amount);
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        require(storage.other_asset.is_some(), InitError::NotInitialized);
        PoolInfo {
            base_asset_reserve: storage.reserves.get(~ContractId::from(base_asset)),
            other_asset_reserve: storage.reserves.get(storage.other_asset.unwrap()),
            total_liquidity: storage.total_liquidity,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let other_asset = storage.other_asset.unwrap();

        require(asset.into() == base_asset || asset == other_asset, InputError::SentInvalidAsset);

        let total_liquidity = storage.total_liquidity;
        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let mut current_base_asset_amount = amount;
        let mut liquidity_to_mint = 0;
        let mut other_asset_amount_to_add = 0;
        if (asset == other_asset) {
            current_base_asset_amount = mutiply_div(amount, base_asset_reserve, other_asset_reserve);
        }
        if total_liquidity > 0 {
            other_asset_amount_to_add = mutiply_div(current_base_asset_amount, other_asset_reserve, base_asset_reserve);
            liquidity_to_mint = mutiply_div(current_base_asset_amount, total_liquidity, base_asset_reserve);
        } else {
            liquidity_to_mint = current_base_asset_amount;
        };
        if (asset == other_asset) {
            other_asset_amount_to_add = current_base_asset_amount;
        }
        PreviewAddLiquidityInfo {
            other_asset_amount: other_asset_amount_to_add,
            received_liquidity: liquidity_to_mint,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_maximum(amount: u64) -> PreviewInfo {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let asset_to_preview = msg_asset_id();
        let other_asset = storage.other_asset.unwrap();

        require(asset_to_preview.into() == base_asset || asset_to_preview == other_asset, InputError::SentInvalidAsset);

        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let mut amount_to_sell = 0;
        let mut liquidity_remains = false;
        if (asset_to_preview.into() == base_asset) {
            require(amount < other_asset_reserve, TransactionError::InsufficientReserve);
            amount_to_sell = get_output_price(base_asset_reserve, liquidity_miner_fee, amount, other_asset_reserve);
            liquidity_remains = amount_to_sell < base_asset_reserve;
        } else {
            require(amount < base_asset_reserve, TransactionError::InsufficientReserve);
            amount_to_sell = get_output_price(other_asset_reserve, liquidity_miner_fee, amount, base_asset_reserve);
            liquidity_remains = amount_to_sell < other_asset_reserve;
        }
        PreviewInfo {
            amount: amount_to_sell,
            has_liquidity: liquidity_remains,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_minimum(amount: u64) -> PreviewInfo {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let asset_to_preview = msg_asset_id();
        let other_asset = storage.other_asset.unwrap();

        require(asset_to_preview.into() == base_asset || asset_to_preview == other_asset, InputError::SentInvalidAsset);

        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let mut amount_to_sell = 0;
        let mut liquidity_remains = false;
        if (asset_to_preview.into() == base_asset) {
            amount_to_sell = get_input_price(amount, base_asset_reserve, liquidity_miner_fee, other_asset_reserve);
            liquidity_remains = amount_to_sell < other_asset_reserve;
        } else {
            amount_to_sell = get_input_price(amount, other_asset_reserve, liquidity_miner_fee, base_asset_reserve);
            liquidity_remains = amount_to_sell < base_asset_reserve;
        }
        PreviewInfo {
            amount: amount_to_sell,
            has_liquidity: liquidity_remains,
        }
    }

    #[storage(read, write)]
    fn remove_liquidity(
        deadline: u64,
        min_base_asset: u64,
        min_other_asset: u64,
    ) -> RemoveLiquidityInfo {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let other_asset = storage.other_asset.unwrap();

        require(msg_asset_id() == contract_id(), InputError::SentInvalidAsset);
        require(min_base_asset > 0 && min_other_asset > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline > height(), TransactionError::DeadlinePassed);

        let amount = msg_amount();

        require(amount > 0, InputError::SentInvalidAmount);

        let total_liquidity = storage.total_liquidity;
        require(total_liquidity > 0, TransactionError::InsufficientLiquidity);

        let sender = msg_sender().unwrap();
        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let base_asset_amount_to_remove = mutiply_div(amount, base_asset_reserve, total_liquidity);
        let other_asset_amount_to_remove = mutiply_div(amount, other_asset_reserve, total_liquidity);

        require((base_asset_amount_to_remove >= min_base_asset) && (other_asset_amount_to_remove >= min_other_asset), TransactionError::CannotSatisfyConstraint);

        burn(amount);
        storage.total_liquidity = total_liquidity - amount;
        // Remove fund from reserves
        storage.reserves.insert(other_asset, other_asset_reserve - other_asset_amount_to_remove);
        storage.reserves.insert(~ContractId::from(base_asset), base_asset_reserve - base_asset_amount_to_remove);
        // Send assets back
        transfer(base_asset_amount_to_remove, ~ContractId::from(base_asset), sender);
        transfer(other_asset_amount_to_remove, other_asset, sender);
        RemoveLiquidityInfo {
            base_asset_amount: base_asset_amount_to_remove,
            other_asset_amount: other_asset_amount_to_remove,
        }
    }

    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64 {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let other_asset = storage.other_asset.unwrap();
        let swap_asset_id = msg_asset_id();

        require(swap_asset_id.into() == base_asset || swap_asset_id == other_asset, InputError::SentInvalidAsset);
        require(amount > 0, InputError::SentInvalidAmount);
        require(deadline > height(), TransactionError::DeadlinePassed);

        let forwarded_amount = msg_amount();

        require(forwarded_amount > 0, InputError::SentInvalidAmount);

        let sender = msg_sender().unwrap();
        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let mut sold = 0;
        if (swap_asset_id.into() == base_asset) {
            let base_asset_sold = get_output_price(base_asset_reserve, liquidity_miner_fee, amount, other_asset_reserve);

            require(forwarded_amount >= base_asset_sold, TransactionError::InsufficientReserve);

            let refund = forwarded_amount - base_asset_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(base_asset), sender);
            };
            transfer(amount, other_asset, sender);
            sold = base_asset_sold;
            // Update reserves
            storage.reserves.insert(~ContractId::from(base_asset), base_asset_reserve + base_asset_sold);
            storage.reserves.insert(other_asset, other_asset_reserve - amount);
        } else {
            let other_asset_sold = get_output_price(other_asset_reserve, liquidity_miner_fee, amount, base_asset_reserve);

            require(forwarded_amount >= other_asset_sold, TransactionError::InsufficientReserve);

            let refund = forwarded_amount - other_asset_sold;
            if refund > 0 {
                transfer(refund, other_asset, sender);
            };
            transfer(amount, ~ContractId::from(base_asset), sender);
            sold = other_asset_sold;
            // Update reserves
            storage.reserves.insert(~ContractId::from(base_asset), base_asset_reserve - amount);
            storage.reserves.insert(other_asset, other_asset_reserve + other_asset_sold);
        };
        sold
    }

    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64 {
        require(storage.other_asset.is_some(), InitError::NotInitialized);

        let other_asset = storage.other_asset.unwrap();
        let swap_asset_id = msg_asset_id();

        require(swap_asset_id.into() == base_asset || swap_asset_id == other_asset, InputError::SentInvalidAsset);
        require(min > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline >= height(), TransactionError::DeadlinePassed);

        let forwarded_amount = msg_amount();

        require(forwarded_amount > 0, InputError::SentInvalidAmount);

        let sender = msg_sender().unwrap();
        let base_asset_reserve = storage.reserves.get(~ContractId::from(base_asset));
        let other_asset_reserve = storage.reserves.get(other_asset);
        let mut bought = 0;
        if (swap_asset_id.into() == base_asset) {
            let other_asset_bought = get_input_price(forwarded_amount, base_asset_reserve, liquidity_miner_fee, other_asset_reserve);

            require(other_asset_bought >= min, TransactionError::CannotSatisfyConstraint);

            transfer(other_asset_bought, other_asset, sender);
            bought = other_asset_bought;
            // Update reserves
            storage.reserves.insert(~ContractId::from(base_asset), base_asset_reserve + forwarded_amount);
            storage.reserves.insert(other_asset, other_asset_reserve - other_asset_bought);
        } else {
            let base_asset_bought = get_input_price(forwarded_amount, other_asset_reserve, liquidity_miner_fee, base_asset_reserve);

            require(base_asset_bought >= min, TransactionError::CannotSatisfyConstraint);

            transfer(base_asset_bought, ~ContractId::from(base_asset), sender);
            bought = base_asset_bought;
            // Update reserves
            storage.reserves.insert(~ContractId::from(base_asset), base_asset_reserve - base_asset_bought);
            storage.reserves.insert(other_asset, other_asset_reserve + bought);
        };
        bought
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId) {
        require(storage.other_asset.is_some(), InitError::NotInitialized);
        require(asset.into() == base_asset || asset == storage.other_asset.unwrap(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let deposited_amount = storage.deposits.get((sender, asset));

        require(deposited_amount >= amount, TransactionError::InsufficientDeposit);

        let new_amount = deposited_amount - amount;
        storage.deposits.insert((sender, asset), new_amount);
        transfer(amount, asset, sender)
    }
}
