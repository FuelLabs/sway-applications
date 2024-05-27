use fuels::{
    prelude::{abigen, AssetId, CallParameters, ContractId, TxPolicies, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
};

abigen!(
    Contract(
        name = "AMM",
        abi = "./AMM-contract/out/debug/AMM-contract-abi.json"
    ),
    Contract(
        name = "Exchange",
        abi = "./exchange-contract/out/debug/exchange-contract-abi.json"
    ),
    Script(
        name = "AtomicAddLiquidityScript",
        abi = "./atomic-add-liquidity/out/debug/atomic-add-liquidity-abi.json"
    ),
    Script(
        name = "SwapExactInputScript",
        abi = "./swap-exact-input/out/debug/swap-exact-input-abi.json"
    ),
    Script(
        name = "SwapExactOutputScript",
        abi = "./swap-exact-output/out/debug/swap-exact-output-abi.json"
    )
);

const GAS_TOLERANCE: f64 = 20.0; // TODO: this should be closer to 0.0. gas estimation issue is under investigation

pub mod amm {

    use super::*;

    pub async fn initialize(
        contract: &AMM<WalletUnlocked>,
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
        contract: &AMM<WalletUnlocked>,
        asset_pair: (AssetId, AssetId),
        pool: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .add_pool((asset_pair.0, asset_pair.1), pool)
            .with_contract_ids(&[pool.into()])
            .call()
            .await
            .unwrap()
    }

    pub async fn pool(
        contract: &AMM<WalletUnlocked>,
        asset_pair: (AssetId, AssetId),
    ) -> Option<ContractId> {
        contract
            .methods()
            .pool((asset_pair.0, asset_pair.1))
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod exchange {

    use super::*;

    pub async fn add_liquidity(
        contract: &Exchange<WalletUnlocked>,
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
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn constructor(
        contract: &Exchange<WalletUnlocked>,
        asset_pair: (AssetId, AssetId),
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor(asset_pair.0, asset_pair.1)
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(
        contract: &Exchange<WalletUnlocked>,
        amount: u64,
        asset: AssetId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .deposit()
            .call_params(CallParameters::new(amount, asset, 1_000_000))
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub async fn remove_liquidity(
        contract: &Exchange<WalletUnlocked>,
        asset_id: AssetId,
        amount: u64,
        min_asset_a: u64,
        min_asset_b: u64,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<RemoveLiquidityInfo> {
        let mut call_handler = contract
            .methods()
            .remove_liquidity(min_asset_a, min_asset_b, deadline)
            .call_params(CallParameters::new(amount, asset_id, 1_000_000))
            .unwrap()
            .append_variable_outputs(2);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn swap_exact_input(
        contract: &Exchange<WalletUnlocked>,
        input_asset: AssetId,
        input_amount: u64,
        min_output: Option<u64>,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<u64> {
        let mut call_handler = contract
            .methods()
            .swap_exact_input(min_output, deadline)
            .call_params(CallParameters::new(input_amount, input_asset, 1_000_000))
            .unwrap()
            .append_variable_outputs(1);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn swap_exact_output(
        contract: &Exchange<WalletUnlocked>,
        input_asset: AssetId,
        input_amount: u64,
        output: u64,
        deadline: u64,
        override_gas_limit: bool,
    ) -> FuelCallResponse<u64> {
        let mut call_handler = contract
            .methods()
            .swap_exact_output(output, deadline)
            .call_params(CallParameters::new(input_amount, input_asset, 1_000_000))
            .unwrap()
            .append_variable_outputs(2);

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap()
    }

    pub async fn withdraw(
        contract: &Exchange<WalletUnlocked>,
        amount: u64,
        asset: AssetId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(Asset { id: asset, amount })
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn balance(contract: &Exchange<WalletUnlocked>, asset: AssetId) -> u64 {
        contract
            .methods()
            .balance(asset)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn pool_info(contract: &Exchange<WalletUnlocked>) -> PoolInfo {
        contract.methods().pool_info().call().await.unwrap().value
    }

    pub async fn preview_add_liquidity(
        contract: &Exchange<WalletUnlocked>,
        amount: u64,
        asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewAddLiquidityInfo {
        let mut call_handler = contract
            .methods()
            .preview_add_liquidity(Asset { id: asset, amount });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn preview_swap_exact_input(
        contract: &Exchange<WalletUnlocked>,
        exact_input: u64,
        input_asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract.methods().preview_swap_exact_input(Asset {
            id: input_asset,
            amount: exact_input,
        });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn preview_swap_exact_output(
        contract: &Exchange<WalletUnlocked>,
        exact_output: u64,
        output_asset: AssetId,
        override_gas_limit: bool,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract.methods().preview_swap_exact_output(Asset {
            id: output_asset,
            amount: exact_output,
        });

        if override_gas_limit {
            let estimated_gas = call_handler
                .estimate_transaction_cost(Some(GAS_TOLERANCE), None)
                .await
                .unwrap()
                .gas_used;

            call_handler = call_handler
                .with_tx_policies(TxPolicies::default().with_script_gas_limit(estimated_gas));
        }

        call_handler.call().await.unwrap().value
    }
}
