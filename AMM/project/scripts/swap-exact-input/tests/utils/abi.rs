use crate::utils::{Exchange, PreviewSwapInfo, AMM};
use fuels::{contract::call_response::FuelCallResponse, prelude::*};

pub mod amm {
    use super::*;

    pub async fn initialize(
        contract: &AMM,
        exchange_bytecode_root: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .initialize(exchange_bytecode_root)
            .call()
            .await
            .unwrap()
    }

    pub async fn add_pool(
        contract: &AMM,
        asset_pair: (AssetId, AssetId),
        pool: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .add_pool(
                (
                    ContractId::new(*asset_pair.0),
                    ContractId::new(*asset_pair.1),
                ),
                pool,
            )
            .set_contracts(&[pool.into()])
            .call()
            .await
            .unwrap()
    }
}

pub mod exchange {
    use super::*;

    pub async fn add_liquidity(
        contract: &Exchange,
        desired_liquidity: u64,
        deadline: u64,
    ) -> FuelCallResponse<u64> {
        contract
            .methods()
            .add_liquidity(desired_liquidity, deadline)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn constructor(
        contract: &Exchange,
        asset_pair: (AssetId, AssetId),
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor((
                ContractId::new(*asset_pair.0),
                ContractId::new(*asset_pair.1),
            ))
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(contract: &Exchange, call_params: CallParameters) -> FuelCallResponse<()> {
        contract
            .methods()
            .deposit()
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn preview_swap_exact_input(
        contract: &Exchange,
        exact_input: u64,
        input_asset: AssetId,
    ) -> FuelCallResponse<PreviewSwapInfo> {
        contract
            .methods()
            .preview_swap_exact_input(exact_input, ContractId::new(*input_asset))
            .tx_params(TxParameters {
                gas_price: 0,
                gas_limit: 10_000_000,
                maturity: 0,
            })
            .call()
            .await
            .unwrap()
    }
}
