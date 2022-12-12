script;

use libraries::Exchange;
use std::block::height;

enum InputError {
    DesiredLiquidityZero: (),
}

fn main(
    exchange_contract_id: ContractId,
    assets: (ContractId, ContractId),
    deposit_amounts: (u64, u64),
    desired_liquidity: u64,
) -> u64 {
    require(desired_liquidity > 0, InputError::DesiredLiquidityZero);

    let exchange_contract = abi(Exchange, exchange_contract_id.into());

    // deposit first asset
    if deposit_amounts.0 != 0 {
        exchange_contract.deposit {
            gas: 100_000,
            coins: deposit_amounts.0,
            asset_id: assets.0.into(),
        }();
    }

    // deposit second asset
    if deposit_amounts.1 != 0 {
        exchange_contract.deposit {
            gas: 100_000,
            coins: deposit_amounts.1,
            asset_id: assets.1.into(),
        }();
    }

    let deadline = height() + 5;

    // add liquidity
    exchange_contract.add_liquidity {
        gas: 15_000_000,
    }(desired_liquidity, deadline)
}
