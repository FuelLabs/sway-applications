use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
};
use paths::SCRIPT_BINARY_PATH;
use test_helpers::setup;

script_abigen!(
    DepositAndAddLiquidityScript,
    "out/debug/deposit_and_add_liquidity-abi.json"
);
abigen!(
    Exchange,
    "../../contracts/exchange/out/debug/exchange-abi.json"
);

pub struct MetaAmounts {
    asset_a_deposit: u64,
    asset_b_deposit: u64,
    liquidity: u64,
}

pub struct MetaExchange {
    instance: Exchange,
    id: ContractId,
    pair: (AssetId, AssetId),
}

pub mod exchange_abi_calls {
    use super::*;

    pub async fn constructor(contract: &Exchange, pair: (AssetId, AssetId)) -> CallResponse<()> {
        contract
            .methods()
            .constructor((ContractId::new(*pair.0), ContractId::new(*pair.1)))
            .call()
            .await
            .unwrap()
    }
}

pub mod paths {
    pub const EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../../contracts/exchange/out/debug/exchange.bin";
    pub const SCRIPT_BINARY_PATH: &str = "out/debug/deposit_and_add_liquidity.bin";
}

pub mod test_helpers {
    use super::*;
    use exchange_abi_calls::constructor;
    use paths::EXCHANGE_CONTRACT_BINARY_PATH;

    pub async fn setup_wallet_and_provider() -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let mut wallet = WalletUnlocked::new_random(None);
        let num_assets = 2;
        let coins_per_asset = 10;
        let amount_per_coin = 100000;

        let (coins, asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            num_assets,
            coins_per_asset,
            amount_per_coin,
        );
        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;
        wallet.set_provider(provider.clone());
        (wallet, asset_ids, provider)
    }

    pub async fn setup_exchange_contract(
        wallet: WalletUnlocked,
        asset_pair: &(AssetId, AssetId),
    ) -> MetaExchange {
        let exchange_contract_id = Contract::deploy(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let exchange_instance = Exchange::new(exchange_contract_id.clone(), wallet.clone());
        constructor(&exchange_instance, *asset_pair).await;

        let contract_id = ContractId::from(exchange_contract_id);

        MetaExchange {
            instance: exchange_instance,
            id: contract_id,
            pair: *asset_pair,
        }
    }

    pub async fn setup() -> (WalletUnlocked, Provider, MetaExchange, MetaAmounts) {
        let (wallet, asset_ids, provider) = setup_wallet_and_provider().await;
        let asset_pair = (*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap());
        let exchange = setup_exchange_contract(wallet.clone(), &asset_pair).await;
        let amounts = MetaAmounts {
            asset_a_deposit: 100,
            asset_b_deposit: 400,
            liquidity: 200,
        };
        (wallet, provider, exchange, amounts)
    }
}

#[tokio::test]
async fn can_deposit_and_add_liquidity_atomically() {
    let (wallet, _provider, exchange, amounts) = setup().await;
    let script_instance = DepositAndAddLiquidityScript::new(wallet, SCRIPT_BINARY_PATH);

    let added_liquidity = script_instance
        .main(
            exchange.id,
            ContractId::new(*exchange.pair.0),
            ContractId::new(*exchange.pair.1),
            amounts.asset_a_deposit,
            amounts.asset_b_deposit,
        )
        .await
        .unwrap();

    assert_eq!(added_liquidity, amounts.liquidity);
}
