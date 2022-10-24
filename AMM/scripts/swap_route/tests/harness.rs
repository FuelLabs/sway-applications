use fuels::{
    contract::{contract::CallResponse, script::Script},
    prelude::*,
    signers::fuel_crypto::SecretKey,
    tx::{AssetId, Bytes32, ContractId, Input, Output, Transaction, TxPointer, UtxoId},
};
use paths::SCRIPT_BINARY_PATH;
use std::str::FromStr;
use test_helpers::setup;

abigen!(AMM, "../../contracts/AMM/out/debug/amm-abi.json");
abigen!(
    Exchange,
    "../../contracts/exchange/out/debug/exchange-abi.json"
);

pub mod amm_abi_calls {
    use super::*;

    pub async fn initialize(contract: &AMM, exchange_contract_id: ContractId) -> CallResponse<()> {
        contract
            .methods()
            .initialize(exchange_contract_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn add_pool(
        contract: &AMM,
        asset_pair: (ContractId, ContractId),
        pool: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .add_pool(asset_pair, pool)
            .set_contracts(&[pool.into()])
            .call()
            .await
            .unwrap()
    }

    pub async fn pool(contract: &AMM, asset_pair: (ContractId, ContractId)) -> Option<ContractId> {
        contract
            .methods()
            .pool(asset_pair)
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod exchange_abi_calls {
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
        pool: (ContractId, ContractId),
    ) -> CallResponse<()> {
        contract.methods().constructor(pool).call().await.unwrap()
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

    pub async fn pool_info(contract: &Exchange) -> PoolInfo {
        contract
            .methods()
            .pool_info()
            .simulate()
            .await
            .unwrap()
            .value
    }
}

pub mod paths {
    pub const AMM_CONTRACT_BINARY_PATH: &str = "../../contracts/AMM/out/debug/amm.bin";
    pub const EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../../contracts/exchange/out/debug/exchange.bin";
    pub const SCRIPT_BINARY_PATH: &str = "out/debug/swap_route.bin";
}

pub mod test_helpers {
    use super::*;
    use amm_abi_calls::{add_pool, initialize};
    use exchange_abi_calls::{add_liquidity, constructor, deposit, pool_info};
    use paths::{AMM_CONTRACT_BINARY_PATH, EXCHANGE_CONTRACT_BINARY_PATH};

    pub async fn initialize_amm_contract(wallet: &WalletUnlocked, amm_instance: &AMM) {
        let exchange_contract_id = Contract::deploy(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        initialize(amm_instance, exchange_contract_id.into()).await;
    }

    pub async fn deposit_and_add_liquidity(
        exchange_instance: &Exchange,
        base_asset_amount: u64,
        other_asset_amount: u64,
        other_asset_id: AssetId,
    ) -> u64 {
        let call_params = CallParameters::new(Some(base_asset_amount), None, None);
        deposit(exchange_instance, call_params).await;

        let call_params =
            CallParameters::new(Some(other_asset_amount), Some(other_asset_id.clone()), None);
        deposit(exchange_instance, call_params).await;

        let call_params =
            CallParameters::new(Some(0), Some(other_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = add_liquidity(
            exchange_instance,
            call_params,
            tx_params,
            base_asset_amount,
            1000,
        )
        .await;

        result.value
    }

    pub async fn setup_wallet_and_provider() -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let secret =
            SecretKey::from_str("5f70feeff1f229e4a95e1056e8b4d80d0b24b565674860cc213bdb07127ce1b1")
                .unwrap();
        let mut wallet = WalletUnlocked::new_from_private_key(secret, None);
        let asset_0_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let asset_1_id =
            AssetId::from_str("0xdbfa2d9d067fc87327b3c1d87d54653af13a895cb9a8d23ff21f4a977b12adb9")
                .unwrap();
        let asset_2_id =
            AssetId::from_str("0xd6415bf62db18c15e39f1c7348ba97ba621cd245eab4b94ee4fb482ed0f4f11f")
                .unwrap();
        let asset_3_id =
            AssetId::from_str("0xb94ff6d58c0a4ae348249edad9530adacec58ae38baa0a759c96958bdd295580")
                .unwrap();
        let asset_4_id =
            AssetId::from_str("0x11cdb42733e624752bf36a5de332b99ae0ee1797734ba614f1d140245a356535")
                .unwrap();
        let asset_0 = AssetConfig {
            id: asset_0_id,
            num_coins: 10,
            coin_amount: 1000000,
        };
        let asset_1 = AssetConfig {
            id: asset_1_id,
            num_coins: 10,
            coin_amount: 100000,
        };
        let asset_2 = AssetConfig {
            id: asset_2_id,
            num_coins: 10,
            coin_amount: 100000,
        };
        let asset_3 = AssetConfig {
            id: asset_3_id,
            num_coins: 10,
            coin_amount: 100000,
        };
        let asset_4 = AssetConfig {
            id: asset_4_id,
            num_coins: 10,
            coin_amount: 100000,
        };

        let asset_ids = vec![asset_0_id, asset_1_id, asset_2_id, asset_3_id, asset_4_id];
        let assets = vec![asset_0, asset_1, asset_2, asset_3, asset_4];
        let coins = setup_custom_assets_coins(wallet.address(), &assets);
        let (provider, _socket_addr) = setup_test_provider(coins, vec![], None).await;
        wallet.set_provider(provider.clone());
        (wallet, asset_ids, provider)
    }

    pub async fn setup_amm_contract(
        wallet: WalletUnlocked,
        asset_ids: Vec<AssetId>,
        exchange_ids: Vec<ContractId>,
    ) -> (AMM, ContractId) {
        let amm_contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let amm_instance = AMM::new(amm_contract_id.to_string(), wallet.clone());
        initialize_amm_contract(&wallet, &amm_instance).await;
        let mut asset_ids = asset_ids.clone();
        let mut exchange_ids = exchange_ids.clone();
        while let Some(asset) = asset_ids.pop() {
            if asset == BASE_ASSET_ID {
                continue;
            }
            let exchange = exchange_ids.pop().unwrap();
            add_pool(
                &amm_instance,
                (ContractId::from(*BASE_ASSET_ID), ContractId::from(*asset)),
                exchange,
            )
            .await;
        }

        (amm_instance, amm_contract_id.into())
    }

    pub async fn setup_exchange_contract(
        wallet: WalletUnlocked,
        asset: AssetId,
        reverse_ratio: u64,
        salt: [u8; 32],
    ) -> (Exchange, ContractId) {
        let exchange_contract_id = Contract::deploy_with_parameters(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
            Salt::from(salt),
        )
        .await
        .unwrap();

        let exchange_instance = Exchange::new(exchange_contract_id.to_string(), wallet.clone());

        constructor(
            &exchange_instance,
            (ContractId::new(*BASE_ASSET_ID), ContractId::new(*asset)),
        )
        .await;
        let base_asset_amount = 100;
        let other_asset_amount = 100 * reverse_ratio;

        deposit_and_add_liquidity(
            &exchange_instance,
            base_asset_amount,
            other_asset_amount,
            asset,
        )
        .await;

        let contract_id = ContractId::from(exchange_contract_id);
        println!("{:#?}", contract_id);
        println!("{:#?}", pool_info(&exchange_instance).await);

        (exchange_instance, contract_id)
    }

    pub async fn setup_exchange_contracts(
        wallet: WalletUnlocked,
        asset_ids: Vec<AssetId>,
    ) -> (Vec<Exchange>, Vec<ContractId>) {
        let mut exchange_instances: Vec<Exchange> = Vec::new();
        let mut exchange_ids: Vec<ContractId> = Vec::new();
        for (i, asset) in asset_ids.iter().enumerate() {
            if *asset == BASE_ASSET_ID {
                continue;
            }
            let (exchange_instance, exchange_id) =
                setup_exchange_contract(wallet.clone(), *asset, i as u64 * 6, [i as u8 + 1; 32])
                    .await;
            exchange_instances.push(exchange_instance);
            exchange_ids.push(exchange_id);
        }
        (exchange_instances, exchange_ids)
    }

    pub async fn setup() -> (
        WalletUnlocked,
        Provider,
        AMM,
        ContractId,
        Vec<Exchange>,
        Vec<ContractId>,
        Vec<AssetId>,
    ) {
        let (wallet, asset_ids, provider) = setup_wallet_and_provider().await;
        let (exchange_instances, exchange_ids) =
            setup_exchange_contracts(wallet.clone(), asset_ids.clone()).await;
        let (amm_instance, amm_contract_id) =
            setup_amm_contract(wallet.clone(), asset_ids.clone(), exchange_ids.clone()).await;
        (
            wallet,
            provider,
            amm_instance,
            amm_contract_id,
            exchange_instances,
            exchange_ids,
            asset_ids,
        )
    }
}

#[tokio::test]
async fn can_swap_maximum_base_with_other() {
    let (
        wallet,
        provider,
        _amm_instance,
        amm_contract_id,
        _exchange_instances,
        exchange_ids,
        asset_ids,
    ) = setup().await;

    let swap_amount: u64 = 150;
    let path = vec![asset_ids[1], asset_ids[0], asset_ids[2]];
    let zeroes = Bytes32::zeroed();

    let mut inputs = vec![Input::contract(
        UtxoId::new(zeroes, 0),
        zeroes,
        zeroes,
        TxPointer::default(),
        amm_contract_id,
    )];
    let mut outputs = vec![Output::contract(0, zeroes, zeroes)];

    dbg!(exchange_ids.len());
    let mut i = 0;
    while i < exchange_ids.len() {
        inputs.push(Input::contract(
            UtxoId::new(zeroes, 0),
            zeroes,
            zeroes,
            TxPointer::default(),
            exchange_ids[i],
        ));
        outputs.push(Output::contract((i + 1) as u8, zeroes, zeroes));
        i += 1;
    }
    i = 0;
    dbg!(path.len());

    while i < path.len() {
        inputs.extend(
            wallet
                .get_asset_inputs_for_amount(path[i], 1000, 0)
                .await
                .unwrap(),
        );
        outputs.push(Output::change(wallet.address().into(), 0, path[i]));
        outputs.push(Output::Variable {
            amount: 0,
            to: Address::zeroed(),
            asset_id: AssetId::default(),
        });
        i += 1;
    }

    // // this doesn't work until script vec args are supported
    // let script_data: Vec<u8> = [
    //     amm_contract_id.to_vec(),
    //     asset_ids.to_vec(),
    //     swap_amount.to_be_bytes().to_vec(),
    // ]
    // .into_iter()
    // .flatten()
    // .collect();

    let script_data: Vec<u8> = [
        amm_contract_id.to_vec(),
        asset_ids[1].to_vec(),
        asset_ids[0].to_vec(),
        asset_ids[2].to_vec(),
        swap_amount.to_be_bytes().to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect();

    let script_binary = std::fs::read(SCRIPT_BINARY_PATH).unwrap();

    let tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 100_000_000,
        maturity: 0,
        receipts_root: Default::default(),
        script: script_binary,
        script_data: script_data,
        inputs: inputs,
        outputs: outputs,
        witnesses: vec![vec![].into()],
        metadata: None,
    };

    let receipts = Script::new(tx).call(&provider).await;

    dbg!(receipts);
}
