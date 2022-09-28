contract;

dep data_structures;
dep errors;
dep interface;
dep utils;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewInfo, RemoveLiquidityInfo};
use errors::{InputError, TransactionError};
use interface::Exchange;
use std::{
    block::height,
    chain::auth::{
        msg_sender,
    },
    context::{
        call_frames::*,
        msg_amount,
    },
    prelude::*,
    result::Result,
    storage::StorageMap,
    token::{
        burn,
        mint,
        transfer,
    },
};
use utils::{calculate_amount_with_fee, div_mutiply, get_input_price, get_output_price, mutiply_div};

storage {
    /// Nonnative asset's ID to its contract ID
    asset: StorageMap<ContractId, ContractId> = StorageMap {},
    /// Deposit amounts per (depositer, asset identifier)
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Nonnative asset supply in liquidity pool
    lp_asset_supply: u64 = 0,
    /// Mapping of asset ID to reserve amount
    reserves: StorageMap<ContractId, u64> = StorageMap {},
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64 {
        require(deadline > height(), TransactionError::DeadlinePassed);
        require(msg_amount() == 0, InputError::SentInvalidAmount);

        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));

        require(msg_asset_id().into() == eth_id || msg_asset_id() == asset_contract_id, InputError::SentInvalidAsset);

        let sender = msg_sender();
        let sender = msg_sender().unwrap();
        let total_liquidity = storage.lp_asset_supply;
        let eth_amount_in_deposit = storage.deposits.get((
            sender,
            ~ContractId::from(eth_id),
        ));
        let asset_amount_in_deposit = storage.deposits.get((
            sender,
            asset_contract_id,
        ));

        require(eth_amount_in_deposit > 0, TransactionError::InsufficientDeposit);

        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(asset_contract_id);

        let mut minted = 0;

        if total_liquidity > 0 {
            require(min_liquidity > 0, InputError::SentInvalidAmount);

            let asset_amount_to_add = mutiply_div(eth_amount_in_deposit, asset_reserve, eth_reserve);
            let liquidity_to_mint = mutiply_div(eth_amount_in_deposit, total_liquidity, eth_reserve);

            require(liquidity_to_mint >= min_liquidity, TransactionError::CannotSatisfyConstraint);

            // If ratio is correct, proceed with liquidity operation
            // Otherwise, return current user balances in contract
            if (asset_amount_in_deposit >= asset_amount_to_add) {
                // Add fund to the reserves
                storage.reserves.insert(asset_contract_id, asset_reserve + asset_amount_to_add);
                storage.reserves.insert(~ContractId::from(eth_id), eth_reserve + eth_amount_in_deposit);

                            // Mint LP asset
                mint(liquidity_to_mint);
                storage.lp_asset_supply = total_liquidity + liquidity_to_mint;

                transfer(liquidity_to_mint, contract_id(), sender);

                // If user sent more than the correct ratio, deposit the extra assets back
                let asset_amount_extra = asset_amount_in_deposit - asset_amount_to_add;
                if (asset_amount_extra > 0) {
                    transfer(asset_amount_extra, storage.asset.get(~ContractId::from(asset_id)), sender);
                }

                minted = liquidity_to_mint;
            } else {
                transfer(asset_amount_in_deposit, storage.asset.get(~ContractId::from(asset_id)), sender);
                transfer(eth_amount_in_deposit, ~ContractId::from(eth_id), sender);
                minted = 0;
            }
        } else {
            require(eth_amount_in_deposit > minimum_liquidity, TransactionError::CannotSatisfyConstraint);

            let initial_liquidity = eth_amount_in_deposit;

            // Add fund to the reserves
            storage.reserves.insert(asset_contract_id, asset_reserve + asset_amount_in_deposit);
            storage.reserves.insert(~ContractId::from(eth_id), eth_reserve + eth_amount_in_deposit);

            // Mint LP asset
            mint(initial_liquidity);
            storage.lp_asset_supply = initial_liquidity;

            transfer(initial_liquidity, contract_id(), sender);

            minted = initial_liquidity;
        };

        // Clear user contract balances after finishing add/create liquidity
        storage.deposits.insert((
            sender,
            asset_contract_id,
        ), 0);
        storage.deposits.insert((
            sender,
            ~ContractId::from(eth_id),
        ), 0);

        minted
    }

    #[storage(read)]
    fn balance(id: ContractId) -> u64 {
        let sender = msg_sender().unwrap();
        storage.deposits.get((sender, id))
    }

    #[storage(read, write)]
    fn deposit() {
        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));

        require(msg_asset_id().into() == eth_id || msg_asset_id() == asset_contract_id, InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();

        let total_amount = storage.deposits.get((
            sender,
            msg_asset_id(),
        )) + msg_amount();
        storage.deposits.insert((
            sender,
            msg_asset_id(),
        ), total_amount);
    }

    #[storage(write)]
    fn initialize(asset_id: ContractId, asset_contract_id: ContractId) {
        storage.asset.insert(asset_id, asset_contract_id);
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        PoolInfo {
            eth_reserve: storage.reserves.get(~ContractId::from(eth_id)),
            token_reserve: storage.reserves.get(storage.asset.get(~ContractId::from(asset_id))),
            lp_token_supply: storage.lp_asset_supply,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(amount: u64, id: ContractId) -> PreviewAddLiquidityInfo {
        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));
        let total_liquidity = storage.lp_asset_supply;
        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(asset_contract_id);
        let mut current_eth_amount = amount;
        let mut lp_asset_received = 0;
        let mut asset_amount = 0;

        if (id == asset_contract_id) {
            current_eth_amount = mutiply_div(amount, eth_reserve, asset_reserve);
        }

        if total_liquidity > 0 {
            asset_amount = mutiply_div(current_eth_amount, asset_reserve, eth_reserve);
            lp_asset_received = mutiply_div(current_eth_amount, total_liquidity, eth_reserve);
        } else {
            lp_asset_received = current_eth_amount;
        };

        if (id == asset_contract_id) {
            asset_amount = current_eth_amount;
        }

        PreviewAddLiquidityInfo {
            lp_token_received: lp_asset_received,
            token_amount: asset_amount,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_maximum(amount: u64) -> PreviewInfo {
        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(storage.asset.get(~ContractId::from(asset_id)));
        let mut sold = 0;
        let mut has_liquidity = true;

        if (msg_asset_id().into() == eth_id) {
            require(amount < asset_reserve, TransactionError::InsufficientReserve);
            sold = get_output_price(eth_reserve, liquidity_miner_fee, amount, asset_reserve);
            has_liquidity = sold < eth_reserve;
        } else {
            require(amount < eth_reserve, TransactionError::InsufficientReserve);
            sold = get_output_price(asset_reserve, liquidity_miner_fee, amount, eth_reserve);
            has_liquidity = sold < asset_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }

    #[storage(read, write)]
    fn preview_swap_with_minimum(amount: u64) -> PreviewInfo {
        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(storage.asset.get(~ContractId::from(asset_id)));
        let mut sold = 0;
        let mut has_liquidity = true;

        if (msg_asset_id().into() == eth_id) {
            sold = get_input_price(amount, eth_reserve, liquidity_miner_fee, asset_reserve);
            has_liquidity = sold < asset_reserve;
        } else {
            sold = get_input_price(amount, asset_reserve, liquidity_miner_fee, eth_reserve);
            has_liquidity = sold < eth_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }

    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_eth: u64, min_tokens: u64) -> RemoveLiquidityInfo {
        require(min_eth > 0 && min_tokens > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline > height(), TransactionError::DeadlinePassed);
        require(msg_amount() > 0, InputError::SentInvalidAmount);
        require(msg_asset_id().into() == (contract_id()).into(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.lp_asset_supply;
        require(total_liquidity > 0, TransactionError::InsufficientLiquidity);

        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));
        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(asset_contract_id);
        let eth_amount = mutiply_div(msg_amount(), eth_reserve, total_liquidity);
        let asset_amount = mutiply_div(msg_amount(), asset_reserve, total_liquidity);

        require((eth_amount >= min_eth) && (asset_amount >= min_tokens), TransactionError::CannotSatisfyConstraint);

        burn(msg_amount());
        storage.lp_asset_supply = total_liquidity - msg_amount();

        // Remove fund from reserves
        storage.reserves.insert(asset_contract_id, asset_reserve - asset_amount);
        storage.reserves.insert(~ContractId::from(eth_id), eth_reserve - eth_amount);

        // Send tokens back
        transfer(eth_amount, ~ContractId::from(eth_id), sender);
        transfer(asset_amount, asset_contract_id, sender);

        RemoveLiquidityInfo {
            eth_amount: eth_amount,
            token_amount: asset_amount,
        }
    }

    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64 {
        require(amount > 0, InputError::SentInvalidAmount);
        require(deadline > height(), TransactionError::DeadlinePassed);

        let swap_asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        require(forwarded_amount > 0, InputError::SentInvalidAmount);

        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));

        require(swap_asset_id == eth_id || swap_asset_id == asset_contract_id.into(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();
        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(asset_contract_id);

        let mut sold = 0;

        if (swap_asset_id == eth_id) {
            let eth_sold = get_output_price(eth_reserve, liquidity_miner_fee, amount, asset_reserve);
            require(forwarded_amount >= eth_sold, TransactionError::InsufficientReserve);
            let refund = forwarded_amount - eth_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(eth_id), sender);
            };
            transfer(amount, asset_contract_id, sender);
            sold = eth_sold;
            // Update reserves
            storage.reserves.insert(~ContractId::from(eth_id), eth_reserve + eth_sold);
            storage.reserves.insert(asset_contract_id, asset_reserve - amount);
        } else {
            let sold_asset_amount = get_output_price(asset_reserve, liquidity_miner_fee, amount, eth_reserve);
            require(forwarded_amount >= sold_asset_amount, TransactionError::InsufficientReserve);
            let refund = forwarded_amount - sold_asset_amount;
            if refund > 0 {
                transfer(refund, asset_contract_id, sender);
            };
            transfer(amount, ~ContractId::from(eth_id), sender);
            sold = sold_asset_amount;
            // Update reserves
            storage.reserves.insert(~ContractId::from(eth_id), eth_reserve - amount);
            storage.reserves.insert(asset_contract_id, asset_reserve + sold_asset_amount);
        };
        sold
    }

    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64 {
        require(min > 0, TransactionError::CannotSatisfyConstraint);
        require(deadline >= height(), TransactionError::DeadlinePassed);

        let swap_asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        require(forwarded_amount > 0, InputError::SentInvalidAmount);

        let asset_contract_id = storage.asset.get(~ContractId::from(asset_id));

        require(swap_asset_id == eth_id || swap_asset_id == asset_contract_id.into(), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();

        let eth_reserve = storage.reserves.get(~ContractId::from(eth_id));
        let asset_reserve = storage.reserves.get(asset_contract_id);

        let mut bought = 0;
        if (swap_asset_id == eth_id) {
            let bought_asset_amount = get_input_price(forwarded_amount, eth_reserve, liquidity_miner_fee, asset_reserve);
            require(bought_asset_amount >= min, TransactionError::CannotSatisfyConstraint);
            transfer(bought_asset_amount, asset_contract_id, sender);
            bought = bought_asset_amount;
            // Update reserves
            storage.reserves.insert(~ContractId::from(eth_id), eth_reserve + forwarded_amount);
            storage.reserves.insert(asset_contract_id, asset_reserve - bought_asset_amount);
        } else {
            let eth_bought = get_input_price(forwarded_amount, asset_reserve, liquidity_miner_fee, eth_reserve);
            require(eth_bought >= min, TransactionError::CannotSatisfyConstraint);
            transfer(eth_bought, ~ContractId::from(eth_id), sender);
            bought = eth_bought;
            // Update reserves
            storage.reserves.insert(~ContractId::from(eth_id), eth_reserve - eth_bought);
            storage.reserves.insert(asset_contract_id, asset_reserve + bought);
        };
        bought
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, id: ContractId) {
        require(id.into() == eth_id || id == storage.asset.get(~ContractId::from(asset_id)), InputError::SentInvalidAsset);

        let sender = msg_sender().unwrap();

        let deposited_amount = storage.deposits.get((sender, id));
        require(deposited_amount >= amount, TransactionError::InsufficientDeposit);

        let new_amount = deposited_amount - amount;
        storage.deposits.insert((sender, id), new_amount);

        transfer(amount, id, sender)
    }
}
