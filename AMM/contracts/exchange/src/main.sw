contract;

dep data_structures;
dep errors;
dep interface;
dep utils;

use data_structures::{
    PoolInfo,
    PositionInfo,
    PreviewAddLiquidityInfo,
    PreviewInfo,
    RemoveLiquidityInfo,
};
use errors::{InputError, TransactionError};
use interface::Exchange;
use std::{
    address::Address,
    assert::assert,
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
    storage::*,
    token::{
        burn,
        mint,
        transfer,
    },
};
use utils::*;

/// Token ID of Ether
pub const ETH_ID = 0x0000000000000000000000000000000000000000000000000000000000000000;
// Liquidity miner fee apply to all swaps
pub const LIQUIDITY_MINER_FEE = 333;
/// Minimum ETH liquidity to open a pool
pub const MINIMUM_LIQUIDITY = 1; // A more realistic value would be 1000000000
/// The token ID key from storage, set of deploy time
/// Contract ID of the token on the other side of the pool
pub const TOKEN_ID_KEY = 0x0000000000000000000000000000000000000000000000000000000000000001;

storage {
    /// Map that stores deposit amounts per (depositer, token identifier) 
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Token supply in liquidity pool
    lp_token_supply: u64 = 0,
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64 {
        require(msg_amount() == 0, InputError::MessageAmountShouldBeZero);
        require(deadline > height(), TransactionError::DeadlineHasPassed);
        require(msg_asset_id().into() == ETH_ID || msg_asset_id().into() == get::<b256>(TOKEN_ID_KEY), InputError::MessageAssetIdDoesNotMatch);

        let sender = msg_sender().unwrap();

        let total_liquidity = storage.lp_token_supply;

        let current_eth_amount = storage.deposits.get((
            sender,
            ~ContractId::from(ETH_ID),
        ));
        let current_token_amount = storage.deposits.get((
            sender,
            ~ContractId::from(get::<b256>(TOKEN_ID_KEY)),
        ));

        require(current_eth_amount > 0, TransactionError::InsufficientReserve);

        let mut minted: u64 = 0;
        if total_liquidity > 0 {
            require(min_liquidity > 0, InputError::PassedAmountCannotBeZero);

            let eth_reserve = get_current_reserve(ETH_ID);
            let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
            let token_amount = mutiply_div(current_eth_amount, token_reserve, eth_reserve);
            let liquidity_minted = mutiply_div(current_eth_amount, total_liquidity, eth_reserve);

            require(liquidity_minted >= min_liquidity, TransactionError::InsufficientReserve);

            // If token ratio is correct, proceed with liquidity operation
            // Otherwise, return current user balances in contract
            if (current_token_amount >= token_amount) {
                // Add fund to the reserves
                add_reserve(token_amount, get::<b256>(TOKEN_ID_KEY));
                add_reserve(current_eth_amount, ETH_ID);
                // Mint LP token
                mint(liquidity_minted);
                storage.lp_token_supply = total_liquidity + liquidity_minted;

                transfer(liquidity_minted, contract_id(), sender);

                // If user sent more than the correct ratio, deposit the extra tokens back
                let token_extra = current_token_amount - token_amount;
                if (token_extra > 0) {
                    transfer(token_extra, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
                }

                minted = liquidity_minted;
            } else {
                transfer(current_token_amount, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
                transfer(current_eth_amount, ~ContractId::from(ETH_ID), sender);
                minted = 0;
            }
        } else {
            require(current_eth_amount > MINIMUM_LIQUIDITY, TransactionError::InsufficientLiquidity);

            let initial_liquidity = current_eth_amount;

            // Add fund to the reserves
            add_reserve(current_token_amount, get::<b256>(TOKEN_ID_KEY));
            add_reserve(current_eth_amount, ETH_ID);

            // Mint LP token
            mint(initial_liquidity);
            storage.lp_token_supply = initial_liquidity;

            transfer(initial_liquidity, contract_id(), sender);

            minted = initial_liquidity;
        };

        // Clear user contract balances after finishing add/create liquidity
        storage.deposits.insert((
            sender,
            ~ContractId::from(get::<b256>(TOKEN_ID_KEY)),
        ), 0);
        storage.deposits.insert((
            sender,
            ~ContractId::from(ETH_ID),
        ), 0);

        minted
    }

    #[storage(read, write)]
    fn deposit() {
        require(msg_asset_id().into() == ETH_ID || msg_asset_id().into() == get::<b256>(TOKEN_ID_KEY), InputError::MessageAssetIdDoesNotMatch);

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

    #[storage(read)]
    fn get_add_liquidity(amount: u64, asset_id: b256) -> PreviewAddLiquidityInfo {
        let token_id = get::<b256>(TOKEN_ID_KEY);
        let total_liquidity = storage.lp_token_supply;
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(token_id);
        let mut current_eth_amount = amount;
        let mut lp_token_received = 0;
        let mut token_amount = 0;

        if (asset_id == token_id) {
            current_eth_amount = mutiply_div(amount, eth_reserve, token_reserve);
        }

        if total_liquidity > 0 {
            token_amount = mutiply_div(current_eth_amount, token_reserve, eth_reserve);
            lp_token_received = mutiply_div(current_eth_amount, total_liquidity, eth_reserve);
        } else {
            lp_token_received = current_eth_amount;
        };

        if (asset_id == token_id) {
            token_amount = current_eth_amount;
        }

        PreviewAddLiquidityInfo {
            lp_token_received: lp_token_received,
            token_amount: token_amount,
        }
    }

    #[storage(read)]
    fn get_balance(asset_id: ContractId) -> u64 {
        let sender = msg_sender().unwrap();
        storage.deposits.get((sender, asset_id))
    }

    #[storage(read)]
    fn get_pool_info() -> PoolInfo {
        PoolInfo {
            eth_reserve: get_current_reserve(ETH_ID),
            token_reserve: get_current_reserve(get::<b256>(TOKEN_ID_KEY)),
            lp_token_supply: storage.lp_token_supply,
        }
    }

    #[storage(read)]
    fn get_position(amount: u64) -> PositionInfo {
        let total_liquidity = storage.lp_token_supply;
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let eth_amount = mutiply_div(amount, eth_reserve, total_liquidity);
        let token_amount = mutiply_div(amount, token_reserve, total_liquidity);

        PositionInfo {
            eth_amount: eth_amount,
            eth_reserve: eth_reserve,
            lp_token_supply: total_liquidity,
            token_amount: token_amount,
            token_reserve: token_reserve,
        }
    }

    #[storage(read, write)]
    fn get_swap_with_maximum(amount: u64) -> PreviewInfo {
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let mut sold = 0;
        let mut has_liquidity = true;
        if (msg_asset_id().into() == ETH_ID) {
            require(amount < token_reserve, TransactionError::InsufficientReserve);
            sold = get_output_price(eth_reserve, LIQUIDITY_MINER_FEE, amount, token_reserve);
            has_liquidity = sold < eth_reserve;
        } else {
            require(amount < eth_reserve, TransactionError::InsufficientReserve);
            sold = get_output_price(token_reserve, LIQUIDITY_MINER_FEE, amount, eth_reserve);
            has_liquidity = sold < token_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }

    #[storage(read, write)]
    fn get_swap_with_minimum(amount: u64) -> PreviewInfo {
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let mut sold = 0;
        let mut has_liquidity = true;
        if (msg_asset_id().into() == ETH_ID) {
            sold = get_input_price(amount, eth_reserve, LIQUIDITY_MINER_FEE, token_reserve);
            has_liquidity = sold < token_reserve;
        } else {
            sold = get_input_price(amount, token_reserve, LIQUIDITY_MINER_FEE, eth_reserve);
            has_liquidity = sold < eth_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }

    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_eth: u64, min_tokens: u64) -> RemoveLiquidityInfo {
        require(msg_amount() > 0, InputError::MessageAmountCannotBeZero);
        require(msg_asset_id().into() == (contract_id()).into(), InputError::PassedAssetIdDoesNotMatch);
        require(deadline > height(), TransactionError::DeadlineHasPassed);
        require(min_eth > 0 && min_tokens > 0, InputError::PassedAmountCannotBeZero);

        let sender = msg_sender().unwrap();

        let total_liquidity = storage.lp_token_supply;
        require(total_liquidity > 0, TransactionError::InsufficientLiquidity);

        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let eth_amount = mutiply_div(msg_amount(), eth_reserve, total_liquidity);
        let token_amount = mutiply_div(msg_amount(), token_reserve, total_liquidity);

        require((eth_amount >= min_eth) && (token_amount >= min_tokens), TransactionError::InsufficientReserve);

        burn(msg_amount());
        storage.lp_token_supply = total_liquidity - msg_amount();

        // Add fund to the reserves
        remove_reserve(token_amount, get::<b256>(TOKEN_ID_KEY));
        remove_reserve(eth_amount, ETH_ID);

        // Send tokens back
        transfer(eth_amount, ~ContractId::from(ETH_ID), sender);
        transfer(token_amount, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);

        RemoveLiquidityInfo {
            eth_amount: eth_amount,
            token_amount: token_amount,
        }
    }

    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64 {
        let asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        require(deadline > height(), TransactionError::DeadlineHasPassed);
        require(forwarded_amount > 0, InputError::PassedAmountCannotBeZero);
        require(amount > 0, InputError::MessageAmountCannotBeZero);
        require(asset_id == ETH_ID || asset_id == get::<b256>(TOKEN_ID_KEY), InputError::MessageAssetIdDoesNotMatch);

        let sender = msg_sender().unwrap();
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));

        let mut sold = 0;
        if (asset_id == ETH_ID) {
            let eth_sold = get_output_price(eth_reserve, LIQUIDITY_MINER_FEE, amount, token_reserve);
            require(forwarded_amount >= eth_sold, TransactionError::InsufficientInput);
            let refund = forwarded_amount - eth_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(ETH_ID), sender);
            };
            transfer(amount, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            sold = eth_sold;
            // Update reserve
            add_reserve(eth_sold, ETH_ID);
            remove_reserve(amount, get::<b256>(TOKEN_ID_KEY));
        } else {
            let tokens_sold = get_output_price(token_reserve, LIQUIDITY_MINER_FEE, amount, eth_reserve);
            require(forwarded_amount >= tokens_sold, TransactionError::InsufficientInput);
            let refund = forwarded_amount - tokens_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            };
            transfer(amount, ~ContractId::from(ETH_ID), sender);
            sold = tokens_sold;
            // Update reserve
            remove_reserve(amount, ETH_ID);
            add_reserve(tokens_sold, get::<b256>(TOKEN_ID_KEY));
        };
        sold
    }

    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64 {
        let asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        require(deadline >= height(), TransactionError::DeadlineHasPassed);
        require(forwarded_amount > 0, InputError::MessageAmountCannotBeZero);
        require(min > 0, InputError::PassedAmountCannotBeZero);
        require(asset_id == ETH_ID || asset_id == get::<b256>(TOKEN_ID_KEY), InputError::MessageAssetIdDoesNotMatch);

        let sender = msg_sender().unwrap();

        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));

        let mut bought = 0;
        if (asset_id == ETH_ID) {
            let tokens_bought = get_input_price(forwarded_amount, eth_reserve, LIQUIDITY_MINER_FEE, token_reserve);
            require(tokens_bought >= min, TransactionError::InsufficientInput);
            transfer(tokens_bought, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            bought = tokens_bought;
            // Update reserve
            add_reserve(forwarded_amount, ETH_ID);
            remove_reserve(tokens_bought, get::<b256>(TOKEN_ID_KEY));
        } else {
            let eth_bought = get_input_price(forwarded_amount, token_reserve, LIQUIDITY_MINER_FEE, eth_reserve);
            require(eth_bought >= min, TransactionError::InsufficientInput);
            transfer(eth_bought, ~ContractId::from(ETH_ID), sender);
            bought = eth_bought;
            // Update reserve
            remove_reserve(eth_bought, ETH_ID);
            add_reserve(bought, get::<b256>(TOKEN_ID_KEY));
        };
        bought
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset_id: ContractId) {
        require(asset_id.into() == ETH_ID || asset_id.into() == get::<b256>(TOKEN_ID_KEY), InputError::MessageAssetIdDoesNotMatch);

        let sender = msg_sender().unwrap();

        let deposited_amount = storage.deposits.get((sender, asset_id));
        require(deposited_amount >= amount, TransactionError::SenderDoesNotHaveEnoughBalance);

        let new_amount = deposited_amount - amount;
        storage.deposits.insert((sender, asset_id), new_amount);

        transfer(amount, asset_id, sender)
    }
}
