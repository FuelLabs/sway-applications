contract;

dep data_structures;
dep interface;
dep utils;

use data_structures::{
    PoolInfo,
    PositionInfo,
    PreviewAddLiquidityInfo,
    PreviewInfo,
    RemoveLiquidityInfo,
};
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

storage {
    lp_token_supply: u64 = 0,
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
}

impl Exchange for Contract {
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
            lp_token_supply: total_liquidity,
            eth_reserve: eth_reserve,
            token_reserve: token_reserve,
            eth_amount: eth_amount,
            token_amount: token_amount,
        }
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
            token_amount: token_amount,
            lp_token_received: lp_token_received,
        }
    }

    #[storage(read, write)]
    fn deposit() {
        assert(msg_asset_id().into() == ETH_ID || msg_asset_id().into() == get::<b256>(TOKEN_ID_KEY));

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

    #[storage(read, write)]
    fn withdraw(amount: u64, asset_id: ContractId) {
        assert(asset_id.into() == ETH_ID || asset_id.into() == get::<b256>(TOKEN_ID_KEY));

        let sender = msg_sender().unwrap();

        let deposited_amount = storage.deposits.get((sender, asset_id));
        assert(deposited_amount >= amount);

        let new_amount = deposited_amount - amount;
        storage.deposits.insert((sender, asset_id), new_amount);

        transfer(amount, asset_id, sender)
    }

    #[storage(read, write)]
    fn add_liquidity(min_liquidity: u64, deadline: u64) -> u64 {
        assert(msg_amount() == 0);
        assert(deadline > height());
        assert(msg_asset_id().into() == ETH_ID || msg_asset_id().into() == get::<b256>(TOKEN_ID_KEY));

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

        assert(current_eth_amount > 0);

        let mut minted: u64 = 0;
        if total_liquidity > 0 {
            assert(min_liquidity > 0);

            let eth_reserve = get_current_reserve(ETH_ID);
            let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
            let token_amount = mutiply_div(current_eth_amount, token_reserve, eth_reserve);
            let liquidity_minted = mutiply_div(current_eth_amount, total_liquidity, eth_reserve);

            assert(liquidity_minted >= min_liquidity);

            // if token ratio is correct, proceed with liquidity operation
            // otherwise, return current user balances in contract
            if (current_token_amount >= token_amount) {
                // Add fund to the reserves
                add_reserve(get::<b256>(TOKEN_ID_KEY), token_amount);
                add_reserve(ETH_ID, current_eth_amount);
                // Mint LP token
                mint(liquidity_minted);
                storage.lp_token_supply = total_liquidity + liquidity_minted;

                transfer(liquidity_minted, contract_id(), sender);

                // If user sent more than the correct ratio, we deposit back the extra tokens
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
            assert(current_eth_amount > MINIMUM_LIQUIDITY);

            let initial_liquidity = current_eth_amount;

            // Add fund to the reserves
            add_reserve(get::<b256>(TOKEN_ID_KEY), current_token_amount);
            add_reserve(ETH_ID, current_eth_amount);

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
    fn remove_liquidity(min_eth: u64, min_tokens: u64, deadline: u64) -> RemoveLiquidityInfo {
        assert(msg_amount() > 0);
        assert(msg_asset_id().into() == (contract_id()).into());
        assert(deadline > height());
        assert(min_eth > 0 && min_tokens > 0);

        let sender = msg_sender().unwrap();

        let total_liquidity = storage.lp_token_supply;
        assert(total_liquidity > 0);

        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let eth_amount = mutiply_div(msg_amount(), eth_reserve, total_liquidity);
        let token_amount = mutiply_div(msg_amount(), token_reserve, total_liquidity);

        assert((eth_amount >= min_eth) && (token_amount >= min_tokens));

        burn(msg_amount());
        storage.lp_token_supply = total_liquidity - msg_amount();

        // Add fund to the reserves
        remove_reserve(get::<b256>(TOKEN_ID_KEY), token_amount);
        remove_reserve(ETH_ID, eth_amount);

        // Send tokens back
        transfer(eth_amount, ~ContractId::from(ETH_ID), sender);
        transfer(token_amount, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);

        RemoveLiquidityInfo {
            eth_amount: eth_amount,
            token_amount: token_amount,
        }
    }

    #[storage(read, write)]
    fn swap_with_minimum(min: u64, deadline: u64) -> u64 {
        let asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        assert(deadline >= height());
        assert(forwarded_amount > 0 && min > 0);
        assert(asset_id == ETH_ID || asset_id == get::<b256>(TOKEN_ID_KEY));

        let sender = msg_sender().unwrap();

        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));

        let mut bought = 0;
        if (asset_id == ETH_ID) {
            let tokens_bought = get_input_price(forwarded_amount, eth_reserve, token_reserve);
            assert(tokens_bought >= min);
            transfer(tokens_bought, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            bought = tokens_bought;
            // Update reserve
            add_reserve(ETH_ID, forwarded_amount);
            remove_reserve(get::<b256>(TOKEN_ID_KEY), tokens_bought);
        } else {
            let eth_bought = get_input_price(forwarded_amount, token_reserve, eth_reserve);
            assert(eth_bought >= min);
            transfer(eth_bought, ~ContractId::from(ETH_ID), sender);
            bought = eth_bought;
            // Update reserve
            remove_reserve(ETH_ID, eth_bought);
            add_reserve(get::<b256>(TOKEN_ID_KEY), bought);
        };
        bought
    }

    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64 {
        let asset_id = msg_asset_id().into();
        let forwarded_amount = msg_amount();

        assert(deadline >= height());
        assert(amount > 0 && forwarded_amount > 0);
        assert(asset_id == ETH_ID || asset_id == get::<b256>(TOKEN_ID_KEY));

        let sender = msg_sender().unwrap();
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));

        let mut sold = 0;
        if (asset_id == ETH_ID) {
            let eth_sold = get_output_price(amount, eth_reserve, token_reserve);
            assert(forwarded_amount >= eth_sold);
            let refund = forwarded_amount - eth_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(ETH_ID), sender);
            };
            transfer(amount, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            sold = eth_sold;
            // Update reserve
            add_reserve(ETH_ID, eth_sold);
            remove_reserve(get::<b256>(TOKEN_ID_KEY), amount);
        } else {
            let tokens_sold = get_output_price(amount, token_reserve, eth_reserve);
            assert(forwarded_amount >= tokens_sold);
            let refund = forwarded_amount - tokens_sold;
            if refund > 0 {
                transfer(refund, ~ContractId::from(get::<b256>(TOKEN_ID_KEY)), sender);
            };
            transfer(amount, ~ContractId::from(ETH_ID), sender);
            sold = tokens_sold;
            // Update reserve
            remove_reserve(ETH_ID, amount);
            add_reserve(get::<b256>(TOKEN_ID_KEY), tokens_sold);
        };
        sold
    }

    #[storage(read, write)]
    fn get_swap_with_minimum(amount: u64) -> PreviewInfo {
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let mut sold = 0;
        let mut has_liquidity = true;
        if (msg_asset_id().into() == ETH_ID) {
            sold = get_input_price(amount, eth_reserve, token_reserve);
            has_liquidity = sold < token_reserve;
        } else {
            sold = get_input_price(amount, token_reserve, eth_reserve);
            has_liquidity = sold < eth_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }

    #[storage(read, write)]
    fn get_swap_with_maximum(amount: u64) -> PreviewInfo {
        let eth_reserve = get_current_reserve(ETH_ID);
        let token_reserve = get_current_reserve(get::<b256>(TOKEN_ID_KEY));
        let mut sold = 0;
        let mut has_liquidity = true;
        if (msg_asset_id().into() == ETH_ID) {
            assert(amount < token_reserve);
            sold = get_output_price(amount, eth_reserve, token_reserve);
            has_liquidity = sold < eth_reserve;
        } else {
            assert(amount < eth_reserve);
            sold = get_output_price(amount, token_reserve, eth_reserve);
            has_liquidity = sold < token_reserve;
        }
        PreviewInfo {
            amount: sold,
            has_liquidity: has_liquidity,
        }
    }
}
