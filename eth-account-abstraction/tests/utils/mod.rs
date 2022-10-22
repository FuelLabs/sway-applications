use fuels::contract::predicate::Predicate;
use fuels::prelude::*;
use fuels::signers::fuel_crypto::SecretKey;

pub async fn test_predicate_spend_with_parameters(private_key: &str) {
    //Setup wallets
    let secret_key1: SecretKey =
            // "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301"
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