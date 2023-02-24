use fuels::{
    prelude::{abigen, AssetId, CallParameters, ContractId, TxParameters},
    programs::call_response::FuelCallResponse,
};

abigen!(
    Contract(
        name = "AMM",
        abi = "./contracts/AMM-contract/out/debug/AMM-contract-abi.json"
    ),
    Contract(
        name = "Exchange",
        abi = "./contracts/exchange-contract/out/debug/exchange-contract-abi.json"
    ),
    Script(
        name = "AtomicAddLiquidityScript",
        abi = "./scripts/atomic-add-liquidity/out/debug/atomic-add-liquidity-abi.json"
    ),
    Script(
        name = "SwapExactInputScript",
        abi = "./scripts/swap-exact-input/out/debug/swap-exact-input-abi.json"
    ),
    Script(
        name = "SwapExactOutputScript",
        abi = "./scripts/swap-exact-output/out/debug/swap-exact-output-abi.json"
    )
);

pub const SCRIPT_GAS_LIMIT: u64 = 100_000_000; // TODO: hardcoded until scripts have gas estimation
const GAS_TOLERANCE: f64 = 20.0; // TODO: this should be closer to 0.0. gas estimation issue is under investigation

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
            .set_contract_ids(&[pool.into()])
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
        desired_liquidity: u64,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<u64> {
        let mut call_handler = contract
            .methods()
            .add_liquidity(desired_liquidity, deadline)
            // `add_liquidity` adds liquidity by using up at least one of the assets
            // one variable output is for the minted liquidity pool asset
            // the other variable output is for the asset that is not used up
            .append_variable_outputs(2);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn constructor(
        contract: &Exchange,
        asset_pair: (AssetId, AssetId),
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor(
                ContractId::new(*asset_pair.0),
                ContractId::new(*asset_pair.1),
            )
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(contract: &Exchange, amount: u64, asset: AssetId) -> FuelCallResponse<()> {
        contract
            .methods()
            .deposit()
            .call_params(CallParameters::new(Some(amount), Some(asset), None))
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub async fn remove_liquidity(
        contract: &Exchange,
        exchange_id: ContractId,
        amount: u64,
        min_asset_a: u64,
        min_asset_b: u64,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<RemoveLiquidityInfo> {
        let mut call_handler = contract
            .methods()
            .remove_liquidity(min_asset_a, min_asset_b, deadline)
            .call_params(CallParameters::new(
                Some(amount),
                Some(AssetId::new(*exchange_id)),
                None,
            ))
            .unwrap()
            .append_variable_outputs(2);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn swap_exact_input(
        contract: &Exchange,
        input_asset: AssetId,
        input_amount: u64,
        min_output: Option<u64>,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<u64> {
        let mut call_handler = contract
            .methods()
            .swap_exact_input(min_output, deadline)
            .call_params(CallParameters::new(
                Some(input_amount),
                Some(input_asset),
                None,
            ))
            .unwrap()
            .append_variable_outputs(1);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn swap_exact_output(
        contract: &Exchange,
        input_asset: AssetId,
        input_amount: u64,
        output: u64,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<u64> {
        let mut call_handler = contract
            .methods()
            .swap_exact_output(output, deadline)
            .call_params(CallParameters::new(
                Some(input_amount),
                Some(input_asset),
                None,
            ))
            .unwrap()
            .append_variable_outputs(2);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn withdraw(
        contract: &Exchange,
        amount: u64,
        asset: AssetId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(Asset {
                id: ContractId::new(*asset),
                amount,
            })
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn balance(contract: &Exchange, asset: AssetId) -> u64 {
        contract
            .methods()
            .balance(ContractId::new(*asset))
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn pool_info(contract: &Exchange) -> PoolInfo {
        contract.methods().pool_info().call().await.unwrap().value
    }

    pub async fn preview_add_liquidity(
        contract: &Exchange,
        amount: u64,
        asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewAddLiquidityInfo {
        let mut call_handler = contract.methods().preview_add_liquidity(Asset {
            id: ContractId::new(*asset),
            amount,
        });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn preview_swap_exact_input(
        contract: &Exchange,
        exact_input: u64,
        input_asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract.methods().preview_swap_exact_input(Asset {
            id: ContractId::new(*input_asset),
            amount: exact_input,
        });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn preview_swap_exact_output(
        contract: &Exchange,
        exact_output: u64,
        output_asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract.methods().preview_swap_exact_output(Asset {
            id: ContractId::new(*output_asset),
            amount: exact_output,
        });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE))
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }
}
