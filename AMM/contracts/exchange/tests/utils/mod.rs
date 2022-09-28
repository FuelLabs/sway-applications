use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
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

    pub async fn initialize(
        contract: &Exchange,
        asset_id: AssetId,
        contract_id: ContractId,
    ) -> CallResponse<()> {
        contract
            .initialize(ContractId::new(*asset_id), contract_id)
            .call()
            .await
            .unwrap()
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

    pub async fn token_initialize(
        contract: &MyToken,
        identity: Identity,
        amount: u64,
    ) -> CallResponse<()> {
        contract.initialize(identity, amount).call().await.unwrap()
    }

    pub async fn token_mint(contract: &MyToken) -> CallResponse<()> {
        contract
            .mint()
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
    use abi_calls::{add_liquidity, deposit, initialize, token_initialize, token_mint};

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

    pub async fn setup() -> (Exchange, ContractId, AssetId, AssetId) {
        let mut wallet = WalletUnlocked::new_random(None);

        let asset_base = AssetConfig {
            id: BASE_ASSET_ID,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_token = AssetConfig {
            id: AssetId::from_str(
                "0x0000000000000000000000000000000000000000000000000000000000000001",
            )
            .unwrap(),
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_invalid = AssetConfig {
            id: AssetId::from_str(
                "0x0000000000000000000000000000000000000000000000000000000000000002",
            )
            .unwrap(),
            num_coins: 1,
            coin_amount: 10,
        };

        let assets = vec![asset_base, asset_token, asset_invalid];

        let coins = setup_custom_assets_coins(wallet.address(), &assets);
        let (provider, _socket_addr) = setup_test_provider(coins, vec![], None).await;
        wallet.set_provider(provider);

        let token_contract_id = Contract::deploy(
            "../token/out/debug/token.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        // Deploy contract and get ID
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
        let token_instance =
            MyTokenBuilder::new(token_contract_id.to_string(), wallet.clone()).build();

        let asset_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();
        let contract_id = ContractId::new(*token_contract_id.hash());

        initialize(&exchange_instance, asset_id, contract_id).await;

        let native_contract_id = ContractId::new(*BASE_ASSET_ID);
        let token_asset_id = AssetId::from(*token_contract_id.hash());
        let lp_asset_id = AssetId::from(*exchange_contract_id.hash());

        // Mint some tokens to the wallet
        token_initialize(
            &token_instance,
            Identity::Address(Address::from(wallet.address())),
            20000,
        )
        .await;
        token_mint(&token_instance).await;

        (
            exchange_instance,
            native_contract_id,
            token_asset_id,
            lp_asset_id,
        )
    }
}
