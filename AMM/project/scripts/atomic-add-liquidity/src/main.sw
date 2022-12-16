script;

use libraries::Exchange;

enum InputError {
    DesiredLiquidityZero: (),
}

fn main(
    exchange_contract_id: ContractId,
    assets: (ContractId, ContractId),
    deposit_amounts: (u64, u64),
    desired_liquidity: u64,
    deadline: u64,
) -> u64 {
    require(desired_liquidity > 0, InputError::DesiredLiquidityZero);

    let exchange_contract = abi(Exchange, exchange_contract_id.into());

    // deposit first asset
    exchange_contract.deposit {
        gas: 70_000,
        coins: deposit_amounts.0,
        asset_id: assets.0.into(),
    }();

    // deposit second asset
    exchange_contract.deposit {
        gas: 70_000,
        coins: deposit_amounts.1,
        asset_id: assets.1.into(),
    }();

    // add liquidity
    exchange_contract.add_liquidity {
        gas: 15_000_000,
    }(desired_liquidity, deadline)
}
