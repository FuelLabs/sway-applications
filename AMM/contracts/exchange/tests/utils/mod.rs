use fuels::{contract::contract::CallResponse, prelude::*};
use std::str::FromStr;

abigen!(Exchange, "out/debug/exchange-abi.json");

pub struct MetaExchange {
    pub asset_a_id: ContractId,
    pub asset_b_id: ContractId,
    pub contract: Exchange,
    pub liquidity_pool_id: ContractId,
}

pub mod abi_calls {
    use super::*;

    pub async fn add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        desired_liquidity: u64,
        deadline: u64,
    ) -> CallResponse<u64> {
        contract
            .methods()
            .add_liquidity(desired_liquidity, deadline)
            .call_params(call_params)
            .append_variable_outputs(2)
            .tx_params(tx_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn constructor(
        contract: &Exchange,
        pair: (ContractId, ContractId),
    ) -> CallResponse<()> {
        contract.methods().constructor(pair).call().await.unwrap()
    }

    pub async fn deposit(contract: &Exchange, call_params: CallParameters) -> CallResponse<()> {
        contract
            .methods()
            .deposit()
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn preview_swap_with_exact_input(
        contract: &Exchange,
        exact_input: u64,
        input_asset: ContractId,
    ) -> CallResponse<PreviewSwapInfo> {
        contract
            .methods()
            .preview_swap_with_exact_input(exact_input, input_asset)
            .call()
            .await
            .unwrap()
    }

    pub async fn preview_swap_with_exact_output(
        contract: &Exchange,
        exact_output: u64,
        output_asset: ContractId,
    ) -> CallResponse<PreviewSwapInfo> {
        contract
            .methods()
            .preview_swap_with_exact_output(exact_output, output_asset)
            .call()
            .await
            .unwrap()
    }

    pub async fn remove_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        min_asset_a: u64,
        min_asset_b: u64,
        deadline: u64,
    ) -> CallResponse<RemoveLiquidityInfo> {
        contract
            .methods()
            .remove_liquidity(min_asset_a, min_asset_b, deadline)
            .call_params(call_params)
            .tx_params(tx_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn swap_with_exact_input(
        contract: &Exchange,
        call_params: CallParameters,
        min_output: Option<u64>,
        deadline: u64,
    ) -> CallResponse<u64> {
        contract
            .methods()
            .swap_with_exact_input(min_output, deadline)
            .call_params(call_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn swap_with_exact_output(
        contract: &Exchange,
        call_params: CallParameters,
        output: u64,
        deadline: u64,
    ) -> CallResponse<u64> {
        contract
            .methods()
            .swap_with_exact_output(output, deadline)
            .call_params(call_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(contract: &Exchange, amount: u64, asset: ContractId) -> CallResponse<()> {
        contract
            .methods()
            .withdraw(amount, asset)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn balance(contract: &Exchange, asset: ContractId) -> CallResponse<u64> {
        contract.methods().balance(asset).call().await.unwrap()
    }

    pub async fn pool_info(contract: &Exchange) -> CallResponse<PoolInfo> {
        contract.methods().pool_info().call().await.unwrap()
    }

    pub async fn preview_add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        amount: u64,
        asset: AssetId,
    ) -> CallResponse<PreviewAddLiquidityInfo> {
        contract
            .methods()
            .preview_add_liquidity(amount, ContractId::new(*asset))
            .call_params(call_params)
            .tx_params(tx_params)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {
    use super::*;
    use abi_calls::{add_liquidity, constructor, deposit};

    pub async fn deposit_and_add_liquidity(
        exchange_instance: &Exchange,
        asset_a_id: AssetId,
        asset_a_amount: u64,
        asset_b_id: AssetId,
        asset_b_amount: u64,
        desired_liquidity: u64,
        deadline: u64,
    ) -> u64 {
        deposit_but_do_not_add_liquidity(
            &exchange_instance,
            asset_a_id,
            asset_a_amount,
            asset_b_id,
            asset_b_amount,
        )
        .await;

        let added = add_liquidity(
            exchange_instance,
            CallParameters::new(Some(0), Some(asset_b_id), Some(10_000_000)),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            desired_liquidity,
            deadline,
        )
        .await;

        added.value
    }

    pub async fn deposit_but_do_not_add_liquidity(
        exchange_instance: &Exchange,
        asset_a_id: AssetId,
        asset_a_amount: u64,
        asset_b_id: AssetId,
        asset_b_amount: u64,
    ) {
        deposit(
            exchange_instance,
            CallParameters::new(Some(asset_a_amount), Some(asset_a_id), None),
        )
        .await;

        deposit(
            exchange_instance,
            CallParameters::new(Some(asset_b_amount), Some(asset_b_id), None),
        )
        .await;
    }

    pub async fn setup() -> (
        Exchange,
        WalletUnlocked,
        ContractId,
        ContractId,
        ContractId,
        ContractId,
    ) {
        let mut wallet = WalletUnlocked::new_random(None);

        let asset_a_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let asset_b_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();
        let asset_c_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap();

        let asset_a = AssetConfig {
            id: asset_a_id,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_b = AssetConfig {
            id: asset_b_id,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_c = AssetConfig {
            id: asset_c_id,
            num_coins: 1,
            coin_amount: 10,
        };

        let assets = vec![asset_a, asset_b, asset_c];
        let coins = setup_custom_assets_coins(wallet.address(), &assets);
        let (provider, _socket_addr) = setup_test_provider(coins, vec![], None).await;
        wallet.set_provider(provider);

        let exchange_contract_id = Contract::deploy(
            "out/debug/exchange.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let exchange_instance = Exchange::new(exchange_contract_id.to_string(), wallet.clone());

        let liquidity_pool_asset_id = AssetId::from(*exchange_contract_id.hash());

        (
            exchange_instance,
            wallet,
            ContractId::new(*liquidity_pool_asset_id),
            ContractId::new(*asset_a_id),
            ContractId::new(*asset_b_id),
            ContractId::new(*asset_c_id),
        )
    }

    pub async fn setup_and_initialize() -> (MetaExchange, WalletUnlocked, ContractId) {
        let (
            exchange_instance,
            wallet,
            liquidity_pool_asset_id,
            asset_a_id,
            asset_b_id,
            asset_c_id,
        ) = setup().await;
        constructor(&exchange_instance, (asset_a_id, asset_b_id)).await;

        let exchange = MetaExchange {
            asset_a_id: asset_a_id,
            asset_b_id: asset_b_id,
            contract: exchange_instance,
            liquidity_pool_id: liquidity_pool_asset_id,
        };

        (exchange, wallet, asset_c_id)
    }
}
