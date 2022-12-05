pub mod abi;
pub mod setup;
pub mod transaction;

use abi::exchange::preview_swap_exact_input;
use fuels::prelude::*;
use setup::AMMContract;

script_abigen!(
    SwapScript,
    "./project/scripts/swap-exact-input/out/debug/swap-exact-input-abi.json"
);

abigen!(
    AMM,
    "./project/contracts/AMM-contract/out/debug/AMM-contract-abi.json"
);

abigen!(
    Exchange,
    "./project/contracts/exchange-contract/out/debug/exchange-contract-abi.json"
);

pub mod paths {
    pub const AMM_CONTRACT_BINARY_PATH: &str =
        "../../contracts/AMM-contract/out/debug/AMM-contract.bin";
    pub const AMM_CONTRACT_STORAGE_PATH: &str =
        "../../contracts/AMM-contract/out/debug/AMM-contract-storage_slots.json";
    pub const EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../../contracts/exchange-contract/out/debug/exchange-contract.bin";
    pub const EXCHANGE_CONTRACT_STORAGE_PATH: &str =
        "../../contracts/exchange-contract/out/debug/exchange-contract-storage_slots.json";
    pub const SCRIPT_BINARY_PATH: &str = "out/debug/swap-exact-input.bin";
}

pub mod amounts {
    pub const COINS_PER_ASSET: u64 = 100;
    pub const AMOUNT_PER_COIN: u64 = 100_000;
    pub const MAXIMUM_INPUT_AMOUNT: u64 = 1_000_000;
}

pub async fn expected_swap_output(amm: &AMMContract, input_amount: u64, route: Vec<AssetId>) -> u64 {
    let mut expected_output = input_amount;
    let mut i = 0;
    while i < route.len() - 1 {
        let pair = (*route.get(i).unwrap(), *route.get(i + 1).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        expected_output = preview_swap_exact_input(&exchange, expected_output, pair.0)
            .await
            .value
            .amount;

        i += 1;
    }
    expected_output
}
