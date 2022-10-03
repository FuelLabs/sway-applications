use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
};
use std::str::FromStr;

abigen!(Exchange, "out/debug/exchange-abi.json");

pub mod abi_calls {
    use super::*;

    pub async fn add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        deadline: u64,
        liquidity: u64,
    ) -> CallResponse<u64> {
        contract
            .add_liquidity(deadline, liquidity)
            .call_params(call_params)
            .append_variable_outputs(2)
            .tx_params(tx_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn balance(contract: &Exchange, asset: ContractId) -> u64 {
        contract.balance(asset).simulate().await.unwrap().value
    }

    pub async fn constructor(contract: &Exchange, asset: ContractId) -> CallResponse<()> {
        contract.constructor(asset).call().await.unwrap()
    }

    pub async fn deposit(contract: &Exchange, call_params: CallParameters) -> CallResponse<()> {
        contract
            .deposit()
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn pool_info(contract: &Exchange) -> PoolInfo {
        contract.pool_info().simulate().await.unwrap().value
    }

    pub async fn preview_add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        amount: u64,
        asset: AssetId,
    ) -> PreviewAddLiquidityInfo {
        contract
            .preview_add_liquidity(amount, ContractId::new(*asset))
            .call_params(call_params)
            .tx_params(tx_params)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn preview_swap_with_maximum(
        contract: &Exchange,
        call_params: CallParameters,
        amount: u64,
    ) -> PreviewInfo {
        contract
            .preview_swap_with_maximum(amount)
            .call_params(call_params)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn preview_swap_with_minimum(
        contract: &Exchange,
        call_params: CallParameters,
        amount: u64,
    ) -> PreviewInfo {
        contract
            .preview_swap_with_minimum(amount)
            .call_params(call_params)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn remove_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        deadline: u64,
        base: u64,
        other: u64,
    ) -> RemoveLiquidityInfo {
        contract
            .remove_liquidity(deadline, base, other)
            .call_params(call_params)
            .tx_params(tx_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn swap_with_maximum(
        contract: &Exchange,
        call_params: CallParameters,
        amount: u64,
        deadline: u64,
    ) -> CallResponse<u64> {
        contract
            .swap_with_maximum(amount, deadline)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn swap_with_minimum(
        contract: &Exchange,
        call_params: CallParameters,
        deadline: u64,
        amount: u64,
    ) -> CallResponse<u64> {
        contract
            .swap_with_minimum(deadline, amount)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(contract: &Exchange, amount: u64, asset: ContractId) -> CallResponse<()> {
        contract
            .withdraw(amount, asset)
            .append_variable_outputs(1)
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
        base_asset_amount: u64,
        other_asset_amount: u64,
        other_asset_id: AssetId,
    ) -> u64 {
        // Deposit some base asset
        let call_params = CallParameters::new(Some(base_asset_amount), None, None);
        deposit(exchange_instance, call_params).await;

        // Deposit some other asset
        let call_params =
            CallParameters::new(Some(other_asset_amount), Some(other_asset_id.clone()), None);
        deposit(exchange_instance, call_params).await;

        // Add liquidity for the second time. Keeping the proportion 1:2
        // It should return the same amount of LP as the amount of base asset deposited
        let call_params =
            CallParameters::new(Some(0), Some(other_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = add_liquidity(exchange_instance, call_params, tx_params, 1000, 1).await;

        result.value
    }

    pub async fn setup() -> (Exchange, WalletUnlocked, AssetId, AssetId, AssetId, AssetId) {
        let mut wallet = WalletUnlocked::new_random(None);

        let base_asset_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let other_asset_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();
        let invalid_asset_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap();

        let asset_base = AssetConfig {
            id: base_asset_id,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_other = AssetConfig {
            id: other_asset_id,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_invalid = AssetConfig {
            id: invalid_asset_id,
            num_coins: 1,
            coin_amount: 10,
        };

        let assets = vec![asset_base, asset_other, asset_invalid];
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

        let exchange_instance =
            ExchangeBuilder::new(exchange_contract_id.to_string(), wallet.clone()).build();

        let liquidity_pool_asset_id = AssetId::from(*exchange_contract_id.hash());

        (
            exchange_instance,
            wallet,
            liquidity_pool_asset_id,
            base_asset_id,
            other_asset_id,
            invalid_asset_id,
        )
    }

    pub async fn setup_and_initialize(
    ) -> (Exchange, WalletUnlocked, AssetId, AssetId, AssetId, AssetId) {
        let (
            exchange_instance,
            wallet,
            liquidity_pool_asset_id,
            base_asset_id,
            other_asset_id,
            invalid_asset_id,
        ) = setup().await;
        constructor(&exchange_instance, ContractId::new(*other_asset_id)).await;

        (
            exchange_instance,
            wallet,
            liquidity_pool_asset_id,
            base_asset_id,
            other_asset_id,
            invalid_asset_id,
        )
    }
}
