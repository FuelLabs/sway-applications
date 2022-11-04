use fuels::{
    contract::{
        script::Script,
    },
    prelude::*,
    signers::fuel_crypto::{
        Message,
        SecretKey,
    },
    tx::{
        AssetId, Receipt, Transaction
    },
};

pub async fn test_recover_and_match_address_with_parameters(private_key: &str) {
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
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //Create signature
    let data_to_sign = "Data to sign";

    let signature1 = wallet.sign_message(data_to_sign).await.unwrap().to_vec();

    let signed_message = Message::new(data_to_sign).to_vec();

    //prepare script and tx
    let script_data: Vec<u8> = [
        signature1,
        signed_message,
    ]
    .into_iter()
    .flatten()
    .collect();

    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    //run script
    let receipts = run_compiled_script(path_to_bin, None, script_data).await.unwrap();

    let return_value = receipts[0].val().unwrap();

    //Script returns bool
    //1 == true
    //0 == false
    assert_eq!(1, return_value);
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
    let tx_params = TxParameters::default();
    
    Script::new(Transaction::script(
        tx_params.gas_price,
        tx_params.gas_limit,
        tx_params.maturity,
        script_binary,
        script_data,
        vec![],
        vec![],
        vec![],
    ))
}
