use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    tx::{AssetId, ContractId},
};
use paths::SCRIPT_BINARY_PATH;
use test_helpers::{expected_swap_output, setup};

script_abigen!(SwapScript, "out/debug/swap_route-abi.json");
abigen!(AMM, "../../contracts/AMM/out/debug/amm-abi.json");
abigen!(
    Exchange,
    "../../contracts/exchange/out/debug/exchange-abi.json"
);

pub mod amm_abi_calls {
    use super::*;

    pub async fn initialize(contract: &AMM, exchange_id: ContractId) -> CallResponse<()> {
        contract
            .methods()
            .initialize(exchange_id)
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
    use amm_abi_calls::{add_pool, initialize, pool};
    use exchange_abi_calls::{
        add_liquidity, constructor, deposit, pool_info, swap_with_exact_input,
    };
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
        asset_a: AssetId,
        asset_a_amount: u64,
        asset_b: AssetId,
        asset_b_amount: u64,
    ) -> u64 {
        let call_params = CallParameters::new(Some(asset_a_amount), Some(asset_a.clone()), None);
        deposit(exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(asset_b_amount), Some(asset_b.clone()), None);
        deposit(exchange_instance, call_params).await;

        let call_params = CallParameters::new(None, None, Some(10_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = add_liquidity(
            exchange_instance,
            call_params,
            tx_params,
            asset_a_amount,
            1000,
        )
        .await;

        result.value
    }

    pub async fn setup_wallet_and_provider() -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let mut wallet = WalletUnlocked::new_random(None);
        let num_assets = 5;
        let coins_per_asset = 10;
        let amount_per_coin = 100000;

        let (coins, asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            num_assets,
            coins_per_asset,
            amount_per_coin,
        );
        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None).await;
        wallet.set_provider(provider.clone());
        (wallet, asset_ids, provider)
    }

    pub async fn setup_amm_contract(wallet: WalletUnlocked) -> (AMM, ContractId) {
        let amm_contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();
        let amm_instance = AMM::new(amm_contract_id.clone(), wallet.clone());
        initialize_amm_contract(&wallet, &amm_instance).await;
        (amm_instance, amm_contract_id.into())
    }

    pub async fn setup_exchange_contract(
        wallet: WalletUnlocked,
        asset_a: AssetId,
        asset_b: AssetId,
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

        let exchange_instance = Exchange::new(exchange_contract_id.clone(), wallet.clone());

        constructor(
            &exchange_instance,
            (ContractId::new(*asset_a), ContractId::new(*asset_b)),
        )
        .await;
        let asset_a_amount = 100;
        let asset_b_amount = 100 * reverse_ratio;

        deposit_and_add_liquidity(
            &exchange_instance,
            asset_a,
            asset_a_amount,
            asset_b,
            asset_b_amount,
        )
        .await;

        let contract_id = ContractId::from(exchange_contract_id);
        println!("{:#?}", contract_id);
        println!("{:#?}", pool_info(&exchange_instance).await);

        (exchange_instance, contract_id)
    }

    pub async fn setup_exchange_contracts(
        wallet: WalletUnlocked,
        amm_instance: &AMM,
        asset_ids: Vec<AssetId>,
    ) -> (Vec<Exchange>, Vec<ContractId>) {
        let mut exchange_instances: Vec<Exchange> = Vec::new();
        let mut exchange_ids: Vec<ContractId> = Vec::new();
        for (i, asset) in asset_ids.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let asset_a = asset_ids.get(i - 1).unwrap();
            let asset_b = asset;
            let asset_pair = (
                ContractId::from(*asset_a.clone()),
                ContractId::from(*asset_b.clone()),
            );
            let (exchange_instance, exchange_id) = setup_exchange_contract(
                wallet.clone(),
                *asset_a,
                *asset_b,
                i as u64 * 6,
                [i as u8 + 1; 32],
            )
            .await;
            add_pool(&amm_instance, asset_pair, exchange_id).await;
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
        let (amm_instance, amm_contract_id) = setup_amm_contract(wallet.clone()).await;
        let (exchange_instances, exchange_ids) =
            setup_exchange_contracts(wallet.clone(), &amm_instance, asset_ids.clone()).await;
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

    pub async fn expected_swap_output(
        amm_instance: AMM,
        exchange_instances: Vec<Exchange>,
        exchange_ids: Vec<ContractId>,
        swap_amount: u64,
        path: Vec<AssetId>,
    ) -> u64 {
        let mut i = 0;
        let mut output_amount = swap_amount;
        while i < path.len() - 1 {
            dbg!(output_amount);
            let asset_a = path.get(i).unwrap();
            let asset_b = path.get(i + 1).unwrap();
            let asset_pair = (
                ContractId::new(*asset_a.clone()),
                ContractId::new(*asset_b.clone()),
            );
            let exchange_contract_id = pool(&amm_instance, asset_pair).await.unwrap();
            let index = exchange_ids
                .iter()
                .position(|&e| e == exchange_contract_id)
                .unwrap();
            let exchange_contract = exchange_instances.get(index).unwrap();
            output_amount = swap_with_exact_input(
                &exchange_contract,
                CallParameters::new(Some(output_amount), Some(*asset_a), Some(10_000_000)),
                Option::None,
                1000,
            )
            .await
            .value;
            i += 1;
        }
        output_amount
    }
}

#[tokio::test]
async fn can_swap_with_exact_input_along_path() {
    let (
        wallet,
        _provider,
        amm_instance,
        amm_contract_id,
        exchange_instances,
        exchange_ids,
        asset_ids,
    ) = setup().await;
    let swap_amount: u64 = 150;
    let path = vec![asset_ids[0], asset_ids[1], asset_ids[2]];
    let script_instance = SwapScript::new(wallet.clone(), SCRIPT_BINARY_PATH);
    // waiting for a function similar to set_contracts()
    let result = script_instance
        .main(
            amm_contract_id,
            ContractId::new(*path[0]),
            ContractId::new(*path[1]),
            ContractId::new(*path[2]),
            swap_amount,
        )
        .await
        .unwrap();
    let expected_result = expected_swap_output(
        amm_instance,
        exchange_instances,
        exchange_ids,
        swap_amount,
        path,
    )
    .await;
    assert_eq!(expected_result, result);
}
