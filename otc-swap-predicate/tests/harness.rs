use fuel_core::service::Config;
use fuel_gql_client::fuel_tx::{AssetId, Contract, Input, Output, Transaction};
use fuel_gql_client::fuel_vm::{consts::REG_ONE, prelude::Opcode};
use fuels::prelude::*;
use fuels_contract::script::Script;

async fn get_balance(provider: &Provider, address: Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(&address, asset).await.unwrap();
    balance
}

#[tokio::test]
async fn otc_swap_with_predicate() {
    // Set up a wallet and send some base asset to the predicate root
    let base_asset: AssetId = Default::default();

    let mut provider_config = Config::local_node();
    provider_config.predicates = true; // predicates are currently disabled by default
    let wallet = launch_custom_provider_and_get_single_wallet(Some(provider_config)).await;

    // Get provider and client
    let provider = wallet.get_provider().unwrap();
    let client = &provider.client;

    // Get predicate bytecode and root
    let predicate_bytecode =
        std::fs::read("../otc-swap-predicate/out/debug/otc-swap-predicate.bin").unwrap();
    let predicate_root: [u8; 32] = (*Contract::root_from_code(&predicate_bytecode)).into();
    let predicate_root = Address::from(predicate_root);

    // Transfer some coins to the predicate root
    let offered_amount = 1000;

    let _receipt = wallet
        .transfer(
            &predicate_root,
            offered_amount,
            base_asset,
            TxParameters::default(),
        )
        .await
        .unwrap();

    let initial_predicate_balance = get_balance(&provider, predicate_root, base_asset).await;
    let initial_wallet_balance = get_balance(&provider, wallet.address(), base_asset).await;
    let receiver_address = Address::new([3u8; 32]);
    let initial_receiver_balance = get_balance(&provider, receiver_address, base_asset).await;

    // The predicate root has received the coin
    assert_eq!(initial_predicate_balance, offered_amount);

    // Get predicate coin to unlock
    let predicate_coin = &provider.get_coins(&predicate_root).await.unwrap()[0];
    let predicate_coin_utxo_id = predicate_coin.utxo_id.clone().into();

    // Get other coin to spend
    let swap_coin = &provider.get_coins(&wallet.address()).await.unwrap()[0];
    let swap_coin_utxo_id = swap_coin.utxo_id.clone().into();
    let swap_coin_amount: u64 = swap_coin.amount.clone().into();

    // Configure inputs and outputs to send coins from the predicate root to another address

    // The predicate allows to spend its tokens if `ask_amount` is sent to the offer maker.
    // This must match the amount in the predicate
    let ask_amount = 42;

    // This is the coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        owner: predicate_root,
        amount: offered_amount,
        asset_id: base_asset,
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![1u8, 0u8], // Predicate data is the index of the input and output that pay the receiver
    };

    // This is the coin belonging to the wallet taking the order
    let input_from_taker = Input::CoinSigned {
        utxo_id: swap_coin_utxo_id,
        owner: wallet.address(),
        amount: swap_coin_amount,
        asset_id: base_asset,
        witness_index: 0,
        maturity: 0,
    };

    // A coin output for the transfer to the receiver
    let output_to_receiver = Output::Coin {
        to: receiver_address,
        amount: ask_amount,
        asset_id: base_asset,
    };

    // A coin output for the transfer to the order taker
    let output_to_taker = Output::Coin {
        to: wallet.address(),
        amount: offered_amount,
        asset_id: base_asset,
    };

    // A Change output
    let output_change = Output::Change {
        to: wallet.address(),
        amount: 0,
        asset_id: Default::default(),
    };

    let mut tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 10_000_000,
        maturity: 0,
        byte_price: 0,
        receipts_root: Default::default(),
        script: Opcode::RET(REG_ONE).to_bytes().to_vec(),
        script_data: vec![],
        inputs: vec![input_predicate, input_from_taker],
        outputs: vec![output_to_receiver, output_to_taker, output_change],
        witnesses: vec![],
        metadata: None,
    };

    // Sign and execute the transaction
    wallet.sign_transaction(&mut tx).await.unwrap();
    let script = Script::new(tx);
    let _receipts = script.call(&client).await.unwrap();

    let predicate_balance = get_balance(&provider, predicate_root, base_asset).await;
    let wallet_balance = get_balance(&provider, wallet.address(), base_asset).await;
    let receiver_balance = get_balance(&provider, receiver_address, base_asset).await;

    // The predicate root's coin has been spent
    assert_eq!(predicate_balance, 0);

    // Receiver has been paid the ask amount
    assert_eq!(receiver_balance, initial_receiver_balance + ask_amount);

    // Taker has send `ask_amount` tokens and got `offered_amount` in return
    assert_eq!(
        wallet_balance,
        initial_wallet_balance - ask_amount + offered_amount
    );
}
