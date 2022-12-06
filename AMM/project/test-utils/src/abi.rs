use fuels::{contract::call_response::FuelCallResponse, prelude::*};

abigen!(
    AMM,
    "./project/contracts/AMM-contract/out/debug/AMM-contract-abi.json"
);

abigen!(
    Exchange,
    "./project/contracts/exchange-contract/out/debug/exchange-contract-abi.json"
);

script_abigen!(
    SwapScript,
    "./project/scripts/swap-exact-input/out/debug/swap-exact-input-abi.json"
);

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

    pub async fn pool(contract: &AMM, asset_pair: (AssetId, AssetId)) -> Option<ContractId> {
        contract
            .methods()
            .pool((
                ContractId::new(*asset_pair.0),
                ContractId::new(*asset_pair.1),
            ))
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod exchange {
    use super::*;

    pub async fn add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        desired_liquidity: u64,
        deadline: u64,
    ) -> FuelCallResponse<u64> {
        contract
            .methods()
            .add_liquidity(desired_liquidity, deadline)
            .call_params(call_params)
            // `add_liquidity` adds liquidity by using up at least one of the assets
            // one variable output is for the minted liquidity pool asset
            // the other variable output is for the asset that is not used up
            .append_variable_outputs(2)
            .tx_params(tx_params)
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

    pub async fn remove_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        min_asset_a: u64,
        min_asset_b: u64,
        deadline: u64,
    ) -> FuelCallResponse<RemoveLiquidityInfo> {
        contract
            .methods()
            .remove_liquidity(min_asset_a, min_asset_b, deadline)
            .call_params(call_params)
            .tx_params(TxParameters::new(None, Some(10_000_000), None))
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn swap_exact_input(
        contract: &Exchange,
        call_params: CallParameters,
        min_output: Option<u64>,
        deadline: u64,
    ) -> FuelCallResponse<u64> {
        contract
            .methods()
            .swap_exact_input(min_output, deadline)
            .call_params(call_params)
            .tx_params(TxParameters::new(None, Some(10_000_000), None))
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn swap_exact_output(
        contract: &Exchange,
        call_params: CallParameters,
        output: u64,
        deadline: u64,
    ) -> FuelCallResponse<u64> {
        contract
            .methods()
            .swap_exact_output(output, deadline)
            .call_params(call_params)
            .tx_params(TxParameters::new(None, Some(10_000_000), None))
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(
        contract: &Exchange,
        amount: u64,
        asset: AssetId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(amount, ContractId::new(*asset))
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn balance(contract: &Exchange, asset: AssetId) -> FuelCallResponse<u64> {
        contract
            .methods()
            .balance(ContractId::new(*asset))
            .call()
            .await
            .unwrap()
    }

    pub async fn pool_info(contract: &Exchange) -> FuelCallResponse<PoolInfo> {
        contract.methods().pool_info().call().await.unwrap()
    }

    pub async fn preview_add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        amount: u64,
        asset: AssetId,
    ) -> FuelCallResponse<PreviewAddLiquidityInfo> {
        contract
            .methods()
            .preview_add_liquidity(amount, ContractId::new(*asset))
            .call_params(call_params)
            .tx_params(tx_params)
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
            .tx_params(TxParameters::new(None, Some(10_000_000), None))
            .call()
            .await
            .unwrap()
    }

    pub async fn preview_swap_exact_output(
        contract: &Exchange,
        exact_output: u64,
        output_asset: AssetId,
    ) -> FuelCallResponse<PreviewSwapInfo> {
        contract
            .methods()
            .preview_swap_exact_output(exact_output, ContractId::new(*output_asset))
            .tx_params(TxParameters::new(None, Some(10_000_000), None))
            .call()
            .await
            .unwrap()
    }
}
