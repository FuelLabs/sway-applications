use fuel_crypto::{
    Signature,
    PublicKey,
};

use fuel_types::Bytes32;

use fuels::{
    contract::{
        predicate::Predicate,
        script::Script,
    },
    prelude::*,
    signers::fuel_crypto::SecretKey,
    tx::{
        AssetId, Receipt, Transaction
    },
};


pub async fn test_predicate_spend_with_parameters(private_key: &str) {
    //Setup wallets
    let secret_key1: SecretKey =
            private_key
                .parse()
                .unwrap();
    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    let receiver = WalletUnlocked::new_random(None);
    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    //Setup provider
    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            predicates: true,
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //Setup predicate
    let predicate = Predicate::load_from(
        "../eth-account-abstraction/out/debug/eth-account-abstraction.bin",
    ).unwrap();
    let predicate_code = predicate.code();
    let predicate_address = predicate.address();
    let amount_to_predicate = 1000;
    let asset_id = AssetId::default();

    //Fund predicate
    wallet
        .transfer(
            predicate_address,
            amount_to_predicate,
            asset_id,
            TxParameters::default(),
        )
        .await.unwrap();

    let predicate_balance = provider
        .get_asset_balance(predicate.address(), asset_id)
        .await.unwrap();
    assert_eq!(predicate_balance, amount_to_predicate);

    //Create signature
    let data_to_sign = [0; 32];
    let signature1 = wallet.sign_message(&data_to_sign).await.unwrap().to_vec();

    
    // let debug_sig = wallet.sign_message(&data_to_sign).await.unwrap();
    // println!("----------------------------");
    // println!("----------------------------");
    // println!("signature: {:?}", debug_sig);
    // println!("----------------------------");
    // println!("----------------------------");


    let signatures = vec![signature1];

    //Spend predicate
    let predicate_data = signatures.into_iter().flatten().collect();
    let _ = wallet
        .spend_predicate(
            predicate_address,
            predicate_code,
            amount_to_predicate,
            asset_id,
            receiver.address(),
            Some(predicate_data),
            TxParameters::default(),
        )
        .await.unwrap();

    //Check that spend was succesful
    let receiver_balance_after = provider
        .get_asset_balance(receiver.address(), asset_id)
        .await.unwrap();
    assert_eq!(amount_to_predicate, receiver_balance_after);

    let predicate_balance = provider
        .get_asset_balance(predicate.address(), asset_id)
        .await.unwrap();
    assert_eq!(predicate_balance, 0);
}





//Runs ecr_script.sw (as main.sw) for debugging
pub async fn test_ecr_script(private_key: &str) {
    //Setup wallets
    let secret_key1: SecretKey =
            private_key
                .parse()
                .unwrap();
    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    //Setup provider
    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            predicates: true,
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //Create signature
    let data_to_sign = [0; 32];
    let signature1 = wallet.sign_message(&data_to_sign).await.unwrap().to_vec();

    //prepare script and tx
    let script_data: Vec<u8> = [
        signature1,
    ]
    .into_iter()
    .flatten()
    .collect();

    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    //run script
    let receipts = run_compiled_script(path_to_bin, None, script_data).await.unwrap();

    //gather return data
    let return_data = receipts[0].data().unwrap();

    let returned_signature = unsafe { Signature::from_slice_unchecked(&return_data[..64]) };
    let recovered_public_key = unsafe { PublicKey::from_slice_unchecked(&return_data[64..128]) };
    let recovered_fuel_address = unsafe { Bytes32::from_slice_unchecked(&return_data[128..160]) };
    let recovered_evm_address = unsafe { Bytes32::from_slice_unchecked(&return_data[160..]) };

    //Display for comparison
    println!("----------------------------");
    println!("SDK signature:          {:?}", wallet.sign_message(&data_to_sign).await.unwrap());
    println!("returned signature:     {:?}", returned_signature);
    println!("recovered public key:   {:?}", recovered_public_key);
    println!("recovered fuel address: {:?}", recovered_fuel_address);
    println!("recovered EVM address:  {:?}", recovered_evm_address);
    println!("----------------------------");
}

//custom run_compiled_script; with input data
pub async fn run_compiled_script(
    binary_filepath: &str,
    provider: Option<Provider>,
    script_data: Vec<u8>,
) -> Result<Vec<Receipt>, Error> {
    let script_binary = std::fs::read(binary_filepath)?;
    let server = FuelService::new_node(Config::local_node()).await.unwrap();
    let provider = provider.unwrap_or(Provider::connect(server.bound_address.to_string()).await?);

    let script = build_script(script_binary, script_data);

    script.call(&provider).await
}

fn build_script(script_binary: Vec<u8>, script_data: Vec<u8>) -> Script {
    let tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 100_000_000,
        maturity: 0,
        receipts_root: Default::default(),
        script: script_binary, // Pass the compiled script into the tx
        script_data: script_data,
        inputs: vec![],
        outputs: vec![],
        witnesses: vec![vec![].into()],
        metadata: None,
    };

    Script::new(tx)
}





//
pub async fn test_addresses(private_key: &str) {
    //Setup wallets
    let secret_key1: SecretKey =
            private_key
                .parse()
                .unwrap();
    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    //Setup provider
    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            predicates: true,
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //prep script
    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    //run script
    let receipts = run_compiled_script(path_to_bin, None, vec![]).await.unwrap();

    //gather return data
    let return_data = receipts[0].data().unwrap();

    let recovered_public_key = unsafe { PublicKey::from_slice_unchecked(&return_data[..64]) };
    let recovered_fuel_address = unsafe { Bytes32::from_slice_unchecked(&return_data[64..96]) };
    let recovered_evm_address = unsafe { Bytes32::from_slice_unchecked(&return_data[96..]) };

    //Display for comparison
    println!("----------------------------");
    println!("recovered public key:   {:?}", recovered_public_key);
    println!("recovered fuel address: {:?}", recovered_fuel_address);
    println!("recovered EVM address:  {:?}", recovered_evm_address);
    println!("----------------------------");
}

pub async fn test_ecr(private_key: &str) {
    //Setup wallets
    let secret_key1: SecretKey =
            private_key
                .parse()
                .unwrap();
    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    //Setup provider
    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            predicates: true,
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //prep script
    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    //run script
    let receipts = run_compiled_script(path_to_bin, None, vec![]).await.unwrap();

    //gather return data
    let return_data = receipts[0].data().unwrap();

    let returned_signature = unsafe { Signature::from_slice_unchecked(&return_data[..64]) };
    let recovered_public_key = unsafe { PublicKey::from_slice_unchecked(&return_data[64..128]) };

    //Display for comparison
    println!("----------------------------");
    println!("returned signature:     {:?}", returned_signature);
    println!("recovered public key:   {:?}", recovered_public_key);
    println!("----------------------------");
}