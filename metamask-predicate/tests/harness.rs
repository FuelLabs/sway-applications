use fuel_vm::fuel_asm::{op, RegId};
use fuels::{
    test_helpers::{Config, launch_custom_provider_and_get_wallets, WalletsConfig},
    tx::{Address, AssetId, Contract, Output},
    types::{
        input::Input,
        resource::Resource,
        transaction_builders::{ScriptTransactionBuilder, TransactionBuilder},
        unresolved_bytes::UnresolvedBytes,
        Token,
        Bits256,
        B512,
    }, prelude::{TxParameters, Wallet, WalletUnlocked, abigen, Signer, Account, ViewOnlyAccount, ResourceFilter},
};
use std::str::FromStr;

async fn setup() -> (Vec<u8>, Address, WalletUnlocked, u64, AssetId) {
    let predicate_code =
        std::fs::read("./out/debug/metamask-predicate.bin")
            .unwrap();
    let predicate_address = (*Contract::root_from_code(&predicate_code)).into();

    let wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::default(),
        Some(Config {
            utxo_validation: true,
            ..Config::local_node()
        }),
        None,
    )
    .await;

    (
        predicate_code,
        predicate_address,
        wallets[0].clone(),
        1000,
        AssetId::default(),
    )
}

async fn create_predicate(
    predicate_address: Address,
    wallet: &WalletUnlocked,
    amount_to_predicate: u64,
    asset_id: AssetId,
) {
    let wallet_coins = wallet
        .get_asset_inputs_for_amount(
            asset_id,
            wallet.get_asset_balance(&asset_id).await.unwrap(),
            None,
        )
        .await
        .unwrap();

    let output_coin = Output::coin(predicate_address, amount_to_predicate, asset_id);
    let output_change = Output::change(wallet.address().into(), 0, asset_id.into());

    let mut tx = ScriptTransactionBuilder::prepare_transfer(
        wallet_coins,
        vec![output_coin, output_change],
        TxParameters::default()
            .set_gas_price(1)
            .set_gas_limit(1_000_000),
    )
    .set_script(op::ret(RegId::ONE).to_bytes().to_vec())
    .build()
    .unwrap();

    wallet.sign_transaction(&mut tx).unwrap();
    wallet
        .provider()
        .unwrap()
        .send_transaction(&tx)
        .await
        .unwrap();
}

async fn submit_to_predicate(
    predicate_code: Vec<u8>,
    predicate_address: Address,
    wallet: &Wallet,
    amount_to_predicate: u64,
    asset_id: AssetId,
    receiver_address: Address,
    predicate_data: UnresolvedBytes,
) {
    let filter = ResourceFilter {
        from: predicate_address.into(),
        asset_id,
        amount: amount_to_predicate,
        ..Default::default()
    };

    let utxo_predicate_hash = wallet
        .provider()
        .unwrap()
        .get_spendable_resources(filter)
        .await
        .unwrap();

    let mut inputs = vec![];
    let mut total_amount_in_predicate = 0;

    for resource in utxo_predicate_hash {
        match &resource {
            Resource::Coin(coin) => {
                inputs.push(Input::resource_predicate(
                    resource.clone(),
                    predicate_code.to_vec(),
                    predicate_data.clone(),
                ));
                total_amount_in_predicate += coin.amount;
            }
            Resource::Message(_) => {}
        }
    }

    let output_coin = Output::coin(receiver_address, total_amount_in_predicate, asset_id);
    let output_change = Output::change(predicate_address, 0, asset_id);

    let params = wallet
        .provider()
        .unwrap()
        .consensus_parameters()
        .await
        .unwrap();
    let new_tx = ScriptTransactionBuilder::prepare_transfer(
        inputs,
        vec![output_coin, output_change],
        TxParameters::default().set_gas_limit(1_000_000),
    )
    .set_consensus_parameters(params)
    .build()
    .unwrap();

    let _call_result = wallet.provider().unwrap().send_transaction(&new_tx).await;
}

async fn get_balance(wallet: &Wallet, address: Address, asset_id: AssetId) -> u64 {
    wallet
        .provider()
        .unwrap()
        .get_asset_balance(&address.into(), asset_id)
        .await
        .unwrap()
}

#[tokio::test]
async fn valid_predicate_data_simple() {
    abigen!(Predicate(
        name = "MyPredicate",
        abi = "./out/debug/metamask-predicate-abi.json"
    ));

    let msg_hash: Bits256 = Bits256::from_hex_str("0xee45573606c96c98ba970ff7cf9511f1b8b25e6bcd52ced30b89df1e4a9c4323").unwrap();

    let expected = Bits256::from_hex_str("0x7aae2d980be4c3275c72ce5b527fa23ffb97b766966559dd062e2b78fd9d3766").unwrap();

    let hi_bits = Bits256::from_hex_str(
        "0xbd0c9b8792876713afa8bff383eebf31c43437823ed761cc3600d0016de5110c",
    ).unwrap();
    let lo_bits = Bits256::from_hex_str(
        "0x44ac566bd156b4fc71a4a4cb2655d3dd360c695edb17dc3b64d611e122fea23d",
    ).unwrap();
    let signature = B512::from((hi_bits, lo_bits));

    let predicate_data = MyPredicate::encode_data(msg_hash, signature);

    let receiver_address =
        Address::from_str("0xde97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c")
            .unwrap();
    let (predicate_code, predicate_address, wallet, amount_to_predicate, asset_id) = setup().await;

    create_predicate(predicate_address, &wallet, amount_to_predicate, asset_id).await;

    let receiver_balance_before = get_balance(&wallet, receiver_address, asset_id).await;
    assert_eq!(receiver_balance_before, 0);

    submit_to_predicate(
        predicate_code,
        predicate_address,
        &wallet,
        amount_to_predicate,
        asset_id,
        receiver_address,
        predicate_data,
    )
    .await;

    let receiver_balance_after = get_balance(&wallet, receiver_address, asset_id).await;
    assert_eq!(
        receiver_balance_before + amount_to_predicate,
        receiver_balance_after
    );

    let predicate_balance = get_balance(&wallet, predicate_address, asset_id).await;
    assert_eq!(predicate_balance, 0);
}
