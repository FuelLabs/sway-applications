script;

use core::num::*;
use libraries::{AMM, Exchange};
use std::{
    constants::ZERO_B256,
    contract_id::ContractId,
    logging::log,
    prelude::*,
    revert::require,
    u128::U128,
};

fn main(
    amm_contract_address: b256,
    exchange_contract_1_address: b256,
    exchange_contract_2_address: b256,
    base_asset_id: b256,
    asset_1_id: b256,
    asset_2_id: b256,
    swap_amount: u64,
) -> (u64, u64) {
    let exchange_contract_1_caller = abi(Exchange, exchange_contract_1_address);
    let exchange_contract_2_caller = abi(Exchange, exchange_contract_2_address);
    let preview_1_to_base = exchange_contract_1_caller.preview_swap_with_maximum {
        gas: 1_000_000,
        coins: 0,
        asset_id: asset_1_id,
    }(swap_amount);
    let preview_1_to_base_amount = preview_1_to_base.amount;
    let preview_base_to_2 = exchange_contract_2_caller.preview_swap_with_maximum {
        gas: 1_000_000,
        coins: 0,
        asset_id: base_asset_id,
    }(preview_1_to_base_amount);
    let preview_base_to_2_amount = preview_base_to_2.amount;
    let swap_1_to_base = exchange_contract_1_caller.swap_with_maximum {
        gas: 10_000_000,
        coins: preview_1_to_base_amount,
        asset_id: asset_1_id,
    }(1000, swap_amount);
    log(swap_1_to_base);
    let swap_base_to_2 = exchange_contract_2_caller.swap_with_maximum {
        gas: 10_000_000,
        coins: preview_base_to_2_amount,
        asset_id: base_asset_id,
    }(1000, swap_1_to_base);
    log(swap_base_to_2);
    (swap_1_to_base, swap_base_to_2, )
}
