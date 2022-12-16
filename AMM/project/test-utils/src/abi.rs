use fuels::prelude::*;

abigen!(
    AMM,
    "./project/contracts/AMM-contract/out/debug/AMM-contract-abi.json"
);

abigen!(
    Exchange,
    "./project/contracts/exchange-contract/out/debug/exchange-contract-abi.json"
);

script_abigen!(
    AtomicAddLiquidityScript,
    "./project/scripts/atomic-add-liquidity/out/debug/atomic-add-liquidity-abi.json"
);

script_abigen!(
    SwapExactInputScript,
    "./project/scripts/swap-exact-input/out/debug/swap-exact-input-abi.json"
);

script_abigen!(
    SwapExactOutputScript,
    "./project/scripts/swap-exact-output/out/debug/swap-exact-output-abi.json"
);

pub mod amm {
    use super::*;

    pub async fn initialize(contract: &AMM, exchange_bytecode_root: ContractId) {
        contract
            .methods()
            .initialize(exchange_bytecode_root)
            .call()
            .await
            .unwrap();
    }

    pub async fn add_pool(contract: &AMM, asset_pair: (AssetId, AssetId), pool: ContractId) {
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
            .unwrap();
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

    pub mod gas_limit_tolerances {
        pub const ADD_LIQUIDITY: f64 = 9.0;
        pub const PREVIEW_ADD_LIQUDITIY: f64 = 10.0;
        pub const PREVIEW_SWAP_EXACT_INPUT: f64 = 4.0;
        pub const PREVIEW_SWAP_EXACT_OUTPUT: f64 = 4.0;
        pub const REMOVE_LIQUIDITY: f64 = 10.0;
        pub const SWAP_EXACT_INPUT: f64 = 5.0;
        pub const SWAP_EXACT_OUTPUT: f64 = 5.0;
    }

    pub async fn add_liquidity(
        contract: &Exchange,
        desired_liquidity: u64,
        deadline: u64,
        gas_tolerance: Option<f64>,
    ) -> u64 {
        let mut call_handler = contract
            .methods()
            .add_liquidity(desired_liquidity, deadline)
            // `add_liquidity` adds liquidity by using up at least one of the assets
            // one variable output is for the minted liquidity pool asset
            // the other variable output is for the asset that is not used up
            .append_variable_outputs(2);

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(None)
                .await
                .unwrap()
                .gas_used;

            dbg!(&estimated_gas);

            call_handler = call_handler.tx_params(TxParameters::new(None, Some(10_000_000), None));
        }

        let result = call_handler.call().await.unwrap();

        if gas_tolerance.is_some() {
            dbg!(&result.gas_used);
        }

        result.value
    }

    pub async fn constructor(contract: &Exchange, asset_pair: (AssetId, AssetId)) {
        contract
            .methods()
            .constructor((
                ContractId::new(*asset_pair.0),
                ContractId::new(*asset_pair.1),
            ))
            .call()
            .await
            .unwrap();
    }

    pub async fn deposit(contract: &Exchange, amount: u64, asset: AssetId) {
        contract
            .methods()
            .deposit()
            .call_params(CallParameters::new(Some(amount), Some(asset), None))
            .call()
            .await
            .unwrap();
    }

    pub async fn remove_liquidity(
        contract: &Exchange,
        exchange_id: ContractId,
        amount: u64,
        min_asset_a: u64,
        min_asset_b: u64,
        deadline: u64,
        gas_tolerance: Option<f64>,
    ) -> RemoveLiquidityInfo {
        let mut call_handler = contract
            .methods()
            .remove_liquidity(min_asset_a, min_asset_b, deadline)
            .call_params(CallParameters::new(
                Some(amount),
                Some(AssetId::new(*exchange_id)),
                None,
            ))
            .append_variable_outputs(2);

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn swap_exact_input(
        contract: &Exchange,
        input_asset: AssetId,
        input_amount: u64,
        min_output: Option<u64>,
        deadline: u64,
        gas_tolerance: Option<f64>,
    ) -> u64 {
        let mut call_handler = contract
            .methods()
            .swap_exact_input(min_output, deadline)
            .call_params(CallParameters::new(
                Some(input_amount),
                Some(input_asset),
                None,
            ))
            .append_variable_outputs(1);

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn swap_exact_output(
        contract: &Exchange,
        input_asset: AssetId,
        input_amount: u64,
        output: u64,
        deadline: u64,
        gas_tolerance: Option<f64>,
    ) -> u64 {
        let mut call_handler = contract
            .methods()
            .swap_exact_output(output, deadline)
            .call_params(CallParameters::new(
                Some(input_amount),
                Some(input_asset),
                None,
            ))
            .append_variable_outputs(2);

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }

    pub async fn withdraw(contract: &Exchange, amount: u64, asset: AssetId) {
        contract
            .methods()
            .withdraw(amount, ContractId::new(*asset))
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
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
        gas_tolerance: Option<f64>,
    ) -> PreviewAddLiquidityInfo {
        let mut call_handler = contract
            .methods()
            .preview_add_liquidity(amount, ContractId::new(*asset));

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
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
        gas_tolerance: Option<f64>,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract
            .methods()
            .preview_swap_exact_input(exact_input, ContractId::new(*input_asset));

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
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
        gas_tolerance: Option<f64>,
    ) -> PreviewSwapInfo {
        let mut call_handler = contract
            .methods()
            .preview_swap_exact_output(exact_output, ContractId::new(*output_asset));

        if gas_tolerance.is_some() {
            let estimated_gas = call_handler
                .estimate_transaction_cost(gas_tolerance)
                .await
                .unwrap()
                .gas_used;

            call_handler =
                call_handler.tx_params(TxParameters::new(None, Some(estimated_gas), None));
        }

        call_handler.call().await.unwrap().value
    }
}
