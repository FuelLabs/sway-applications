script;

use libraries::Exchange;
use std::block::height;

fn main(
    exchange_contract_id: ContractId,
    asset_a_id: ContractId,
    asset_b_id: ContractId,
    asset_a_amount: u64,
    asset_b_amount: u64,
) -> u64 {
    let exchange_contract = abi(Exchange, exchange_contract_id.into());
    exchange_contract.deposit {
        gas: 100_000,
        coins: asset_a_amount,
        asset_id: asset_a_id.into(),
    }();
    exchange_contract.deposit {
        gas: 100_000,
        coins: asset_b_amount,
        asset_id: asset_b_id.into(),
    }();
    let min_liquidity = if asset_a_amount < asset_b_amount {
        asset_a_amount
    } else {
        asset_b_amount
    };
    let deadline = height() + 10;
    exchange_contract.add_liquidity {
        gas: 1_000_000,
        coins: 0,
    }(min_liquidity, deadline)
}
