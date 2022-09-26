use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, Bytes32, ContractId, StorageSlot},
};
use std::str::FromStr;

abigen!(Exchange, "out/debug/exchange-abi.json");
abigen!(MyToken, "../token/out/debug/token-abi.json");

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

    pub async fn deposit(contract: &Exchange, call_params: CallParameters) -> CallResponse<()> {
        contract
            .deposit()
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_add_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        amount: u64,
        asset: Bits256,
    ) -> PreviewAddLiquidityInfo {
        contract
            .get_add_liquidity(amount, asset)
            .call_params(call_params)
            .tx_params(tx_params)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn get_balance(contract: &Exchange, asset: ContractId) -> u64 {
        contract.get_balance(asset).call().await.unwrap().value
    }

    pub async fn get_pool_info(contract: &Exchange) -> PoolInfo {
        contract.get_pool_info().call().await.unwrap().value
    }

    pub async fn get_swap_with_maximum(
        contract: &Exchange,
        call_params: CallParameters,
        amount: u64,
    ) -> PreviewInfo {
        contract
            .get_swap_with_maximum(amount)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn get_swap_with_minimum(
        contract: &Exchange,
        call_params: CallParameters,
        amount: u64,
    ) -> PreviewInfo {
        contract
            .get_swap_with_minimum(amount)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn remove_liquidity(
        contract: &Exchange,
        call_params: CallParameters,
        tx_params: TxParameters,
        deadline: u64,
        eth: u64,
        tokens: u64,
    ) -> RemoveLiquidityInfo {
        contract
            .remove_liquidity(deadline, eth, tokens)
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
    use abi_calls::{add_liquidity, deposit};

    pub async fn deposit_and_add_liquidity(
        exchange_instance: &Exchange,
        native_amount: u64,
        token_amount_deposit: u64,
        token_asset_id: AssetId,
    ) -> u64 {
        // Deposit some Native Asset
        let call_params = CallParameters::new(Some(native_amount), None, None);
        let _t = deposit(exchange_instance, call_params).await;

        // Deposit some Token Asset
        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(exchange_instance, call_params).await;

        // Add liquidity for the second time. Keeping the proportion 1:2
        // It should return the same amount of LP as the amount of ETH deposited
        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = add_liquidity(exchange_instance, call_params, tx_params, 1000, 1).await;

        result.value
    }

    pub async fn setup() -> (
        Exchange,
        MyToken,
        WalletUnlocked,
        ContractId,
        AssetId,
        AssetId,
    ) {
        // default initial amount 1000000000
        let wallet = launch_provider_and_get_wallet().await;

        let token_contract_id = Contract::deploy(
            "../token/out/debug/token.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let key =
            Bytes32::from_str("0x0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();
        let value = token_contract_id.hash();
        let storage_slot = StorageSlot::new(key, value);
        let storage_vec = vec![storage_slot.clone()];

        // Deploy contract and get ID
        let exchange_contract_id = Contract::deploy(
            "out/debug/exchange.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_manual_storage(Some(storage_vec)),
        )
        .await
        .unwrap();
        let exchange_instance =
            ExchangeBuilder::new(exchange_contract_id.to_string(), wallet.clone()).build();
        let token_instance =
            MyTokenBuilder::new(token_contract_id.to_string(), wallet.clone()).build();

        // Native contract id
        let native_contract_id = ContractId::new(*BASE_ASSET_ID);
        // Token contract id
        let token_contract_id = token_contract_id;
        // Token asset id
        let token_asset_id = AssetId::from(*token_contract_id.hash());
        // LP Token asset id
        let lp_asset_id = AssetId::from(*exchange_contract_id.hash());

        (
            exchange_instance,
            token_instance,
            wallet,
            native_contract_id,
            token_asset_id,
            lp_asset_id,
        )
    }
}
