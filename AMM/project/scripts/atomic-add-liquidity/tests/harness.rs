use fuel_gql_client::{
    client::schema::resource::Resource,
    prelude::{Address, Bytes32, Output, UtxoId},
};
use fuels::{
    contract::call_response::FuelCallResponse,
    prelude::*,
    tx::{AssetId, ContractId, Input, TxPointer},
};
use paths::SCRIPT_BINARY_PATH;
use test_helpers::{
    setup, transaction_input_coin, transaction_input_contract, transaction_output_contract,
    transaction_output_variable,
};

script_abigen!(
    AtomicAddLiquidityScript,
    "./project/scripts/atomic-add-liquidity/out/debug/atomic-add-liquidity-abi.json"
);

abigen!(
    Exchange,
    "./project/contracts/exchange-contract/out/debug/exchange-contract-abi.json"
);

pub struct MetaAmounts {
    asset_a_deposit: u64,
    asset_b_deposit: u64,
    liquidity: u64,
}

pub struct MetaExchange {
    id: ContractId,
    pair: (AssetId, AssetId),
}

pub mod exchange_abi_calls {
    use super::*;

    pub async fn constructor(
        contract: &Exchange,
        pair: (AssetId, AssetId),
    ) -> FuelCallResponse<()> {
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
        "../../contracts/exchange-contract/out/debug/exchange-contract.bin";
    pub const SCRIPT_BINARY_PATH: &str = "out/debug/atomic-add-liquidity.bin";
}

pub mod test_helpers {
    use super::*;
    use exchange_abi_calls::constructor;
    use paths::EXCHANGE_CONTRACT_BINARY_PATH;

    pub async fn transaction_input_coin(
        provider: &Provider,
        from: &Bech32Address,
        asset_id: AssetId,
        amount: u64,
    ) -> Input {
        let coin = &provider
            .get_spendable_resources(from, asset_id, amount)
            .await
            .unwrap()[0];

        let (coin_utxo_id, coin_amount) = match coin {
            Resource::Coin(coin) => (coin.utxo_id.clone(), coin.amount.clone()),
            _ => panic!(),
        };

        Input::CoinSigned {
            utxo_id: coin_utxo_id.into(),
            owner: Address::from(from),
            amount: coin_amount.into(),
            asset_id: asset_id,
            tx_pointer: TxPointer::default(),
            witness_index: 0,
            maturity: 0,
        }
    }

    pub fn transaction_input_contract(contract_id: ContractId) -> Input {
        Input::Contract {
            utxo_id: UtxoId::new(Bytes32::zeroed(), 0),
            balance_root: Bytes32::zeroed(),
            state_root: Bytes32::zeroed(),
            tx_pointer: TxPointer::default(),
            contract_id,
        }
    }

    pub fn transaction_output_contract(input_index: u8) -> Output {
        Output::Contract {
            input_index,
            balance_root: Bytes32::zeroed(),
            state_root: Bytes32::zeroed(),
        }
    }

    pub fn transaction_output_variable() -> Output {
        Output::Variable {
            amount: 0,
            to: Address::zeroed(),
            asset_id: AssetId::default(),
        }
    }

    pub async fn setup_wallet_and_provider() -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let mut wallet = WalletUnlocked::new_random(None);
        let num_assets = 3;
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
async fn adds_when_neither_asset_is_base_asset() {
    let (wallet, provider, exchange, amounts) = setup().await;
    let script_instance = AtomicAddLiquidityScript::new(wallet.clone(), SCRIPT_BINARY_PATH);

    let input_a = transaction_input_coin(
        &provider,
        wallet.address(),
        exchange.pair.0,
        amounts.asset_a_deposit,
    )
    .await;
    let input_b = transaction_input_coin(
        &provider,
        wallet.address(),
        exchange.pair.1,
        amounts.asset_b_deposit,
    )
    .await;
    let input_contract = transaction_input_contract(exchange.id);
    let output_contract = transaction_output_contract(0);
    let output_variable = transaction_output_variable();

    let added_liquidity = script_instance
        .main(
            exchange.id,
            ContractId::new(*exchange.pair.0),
            ContractId::new(*exchange.pair.1),
            amounts.asset_a_deposit,
            amounts.asset_b_deposit,
        )
        .with_inputs(vec![input_contract, input_a, input_b])
        .with_outputs(vec![
            output_contract,
            output_variable.clone(),
            output_variable,
        ])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(added_liquidity, amounts.liquidity);
}

// TODO (supiket): when one of the assets being added is the base asset, the built transaction is not valid because of duplicate asset.
#[ignore]
#[tokio::test]
async fn adds_when_one_of_the_assets_is_base_asset() {}
