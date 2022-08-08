use fuel_gql_client::fuel_vm::{consts::REG_ONE, prelude::Opcode};
use fuels::contract::script::Script;
use fuels::prelude::*;
use fuels::test_helpers::WalletsConfig;
use fuels::tx::{AssetId, Contract, Input, Output, Transaction};

// The fee-paying base asset
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
// Offered asset is the asset that will be locked behind the predicate
pub const OFFERED_ASSET: AssetId = AssetId::new([2u8; 32]);

// Get the balance of a given token of an address
async fn get_balance(provider: &Provider, address: &Bech32Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(address, asset).await.unwrap();
    balance
}

// Create wallet config for two wallets with base, offered, and ask assets
pub fn configure_wallets(asked_asset: AssetId) -> WalletsConfig {
    let assets = [BASE_ASSET, OFFERED_ASSET, asked_asset];

    WalletsConfig::new_multiple_assets(
        2,
        assets
            .map(|asset| AssetConfig {
                id: asset,
                num_coins: 1,
                coin_amount: 1_000_000_000,
            })
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
    )
}

// Get the bytecode and bytecode root of a predicate, given a path to its binary
pub fn predicate_bytecode_and_root_from_bin(path_to_bin: &str) -> (Vec<u8>, Bech32Address) {
    // Get predicate bytecode and root
    let predicate_bytecode = std::fs::read(path_to_bin).unwrap();
    let predicate_root: [u8; 32] = (*Contract::root_from_code(&predicate_bytecode)).into();
    let predicate_root = Address::from(predicate_root);
    let predicate_root = Bech32Address::from(predicate_root);
    (predicate_bytecode, predicate_root)
}

/// Tests that the predicate can be spent. Parameterized by test cases
pub async fn test_predicate_spend_with_parameters(
    ask_amount: u64,
    asked_asset: AssetId,
    receiver: &str,
) {
    let receiver_address: Bech32Address = receiver.parse().unwrap();

    let provider_config = Config {
        utxo_validation: true,
        predicates: true,
        ..Config::local_node()
    };

    let wallets = &launch_custom_provider_and_get_wallets(
        configure_wallets(asked_asset),
        Some(provider_config),
    )
    .await;

    let receiver_wallet = &wallets[0];
    let taker_wallet = &wallets[1];

    // Get provider
    let provider = receiver_wallet.get_provider().unwrap();

    let (predicate_bytecode, predicate_root) = predicate_bytecode_and_root_from_bin(
        "../otc-swap-predicate/out/debug/otc-swap-predicate.bin",
    );

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    let _receipt = receiver_wallet
        .transfer(
            &predicate_root,
            offered_amount,
            OFFERED_ASSET,
            TxParameters::default(),
        )
        .await
        .unwrap();

    let initial_taker_offered_token_balance =
        get_balance(&provider, taker_wallet.address(), OFFERED_ASSET).await;
    let initial_taker_asked_token_balance =
        get_balance(&provider, taker_wallet.address(), asked_asset).await;
    let initial_receiver_balance = get_balance(&provider, &receiver_address, asked_asset).await;

    // The predicate root has received the coin
    assert_eq!(
        get_balance(&provider, &predicate_root, OFFERED_ASSET).await,
        offered_amount
    );

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_coins(&predicate_root, OFFERED_ASSET, 1)
        .await
        .unwrap()[0];
    let predicate_coin_utxo_id = predicate_coin.utxo_id.clone().into();

    // Get other coin to spend
    let swap_coin = &provider
        .get_spendable_coins(taker_wallet.address(), asked_asset, 1)
        .await
        .unwrap()[0];
    let swap_coin_utxo_id = swap_coin.utxo_id.clone().into();
    let swap_coin_amount: u64 = swap_coin.amount.clone().into();

    // Get base asset coin for gas
    let gas_coin = &provider
        .get_spendable_coins(taker_wallet.address(), BASE_ASSET, 1)
        .await
        .unwrap()[0];
    let gas_coin_utxo_id = gas_coin.utxo_id.clone().into();
    let gas_coin_amount: u64 = gas_coin.amount.clone().into();

    // Configure inputs and outputs to send coins from the predicate root to another address
    // The predicate allows to spend its tokens if `ask_amount` is sent to the receiver.

    // Base asset input for gas
    let input_gas = Input::CoinSigned {
        utxo_id: gas_coin_utxo_id,
        owner: Address::from(taker_wallet.address()),
        amount: gas_coin_amount,
        asset_id: BASE_ASSET,
        witness_index: 0,
        maturity: 0,
    };

    // Offered asset coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        owner: Address::from(&predicate_root),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![0u8], // Predicate data is the index of the output that pays the receiver
    };

    // Asked asset coin belonging to the wallet taking the order
    let input_from_taker = Input::CoinSigned {
        utxo_id: swap_coin_utxo_id,
        owner: Address::from(taker_wallet.address()),
        amount: swap_coin_amount,
        asset_id: asked_asset,
        witness_index: 0,
        maturity: 0,
    };

    // Output for the asked coin transferred from the taker to the receiver
    let output_to_receiver = Output::Coin {
        to: Address::from(receiver_address.clone()),
        amount: ask_amount,
        asset_id: asked_asset,
    };

    // Output for the offered coin transferred from the predicate to the order taker
    let output_to_taker = Output::Coin {
        to: Address::from(taker_wallet.address()),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
    };

    // Change output for unspent fees (base asset)
    let output_base_change = Output::Change {
        to: Address::from(taker_wallet.address()),
        amount: 0,
        asset_id: BASE_ASSET,
    };

    // Change output for unspent asked asset
    let output_asked_change = Output::Change {
        to: Address::from(taker_wallet.address()),
        amount: 0,
        asset_id: asked_asset,
    };

    let mut tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 10_000_000,
        maturity: 0,
        byte_price: 0,
        receipts_root: Default::default(),
        script: Opcode::RET(REG_ONE).to_bytes().to_vec(),
        script_data: vec![],
        inputs: vec![input_gas, input_predicate, input_from_taker],
        outputs: vec![
            output_to_receiver, // Position in Vec must match with index provided in predicate data
            output_to_taker,
            output_base_change,
            output_asked_change,
        ],
        witnesses: vec![],
        metadata: None,
    };

    // Sign and execute the transaction
    taker_wallet.sign_transaction(&mut tx).await.unwrap();
    let script = Script::new(tx);
    let _receipts = script.call(provider).await.unwrap();

    let predicate_balance = get_balance(&provider, &predicate_root, OFFERED_ASSET).await;
    let taker_asked_token_balance =
        get_balance(&provider, taker_wallet.address(), asked_asset).await;
    let taker_offered_token_balance =
        get_balance(&provider, taker_wallet.address(), OFFERED_ASSET).await;
    let receiver_balance = get_balance(&provider, &receiver_address, asked_asset).await;

    // The predicate root's coin has been spent
    assert_eq!(predicate_balance, 0);

    // Receiver has been paid `ask_amount`
    assert_eq!(receiver_balance, initial_receiver_balance + ask_amount);

    // Taker has sent `ask_amount` of the asked token and received `offered_amount` of the offered token in return
    assert_eq!(
        taker_asked_token_balance,
        initial_taker_asked_token_balance - ask_amount
    );
    assert_eq!(
        taker_offered_token_balance,
        initial_taker_offered_token_balance + offered_amount
    );
}

// Tests that the predicate can be recovered by the owner
pub async fn recover_predicate_as_owner(correct_owner: bool) {
    let provider_config = Config {
        utxo_validation: true,
        predicates: true,
        ..Config::local_node()
    };

    let wallets = &launch_custom_provider_and_get_wallets(
        configure_wallets(BASE_ASSET),
        Some(provider_config),
    )
    .await;

    let wallet = match correct_owner {
        true => &wallets[0],
        false => &wallets[1],
    };

    // Get provider
    let provider = wallet.get_provider().unwrap();

    let initial_wallet_balance = get_balance(&provider, wallet.address(), OFFERED_ASSET).await;

    let (predicate_bytecode, predicate_root) = predicate_bytecode_and_root_from_bin(
        "../otc-swap-predicate/out/debug/otc-swap-predicate.bin",
    );

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    let _receipt = wallet
        .transfer(
            &predicate_root,
            offered_amount,
            OFFERED_ASSET,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_coins(&predicate_root, OFFERED_ASSET, 1)
        .await
        .unwrap()[0];
    let predicate_coin_utxo_id = predicate_coin.utxo_id.clone().into();

    // Get base asset coin for gas
    let gas_coin = &provider
        .get_spendable_coins(wallet.address(), BASE_ASSET, 1)
        .await
        .unwrap()[0];
    let gas_coin_utxo_id = gas_coin.utxo_id.clone().into();
    let gas_coin_amount: u64 = gas_coin.amount.clone().into();

    // Base asset input for gas
    let input_gas = Input::CoinSigned {
        utxo_id: gas_coin_utxo_id,
        owner: Address::from(wallet.address()),
        amount: gas_coin_amount,
        asset_id: BASE_ASSET,
        witness_index: 0,
        maturity: 0,
    };

    // Offered asset coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        owner: Address::from(&predicate_root),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![0u8], // Predicate data is the index of the output that pays the receiver
    };

    // Change outputs for unspent fees (base asset)
    let output_base_change = Output::Change {
        to: Address::from(wallet.address()),
        amount: 0,
        asset_id: BASE_ASSET,
    };

    // Use a change output to send the unlocked coins back to the wallet
    let output_offered_change = Output::Change {
        to: Address::from(wallet.address()),
        amount: 0,
        asset_id: OFFERED_ASSET,
    };

    let mut tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 10_000_000,
        maturity: 0,
        byte_price: 0,
        receipts_root: Default::default(),
        script: Opcode::RET(REG_ONE).to_bytes().to_vec(),
        script_data: vec![],
        inputs: vec![input_gas, input_predicate],
        outputs: vec![output_base_change, output_offered_change],
        witnesses: vec![],
        metadata: None,
    };

    // Sign and execute the transaction
    wallet.sign_transaction(&mut tx).await.unwrap();
    let script = Script::new(tx);
    let _receipts = script.call(provider).await.unwrap();

    // The predicate root's coin has been spent
    let predicate_balance = get_balance(&provider, &predicate_root, OFFERED_ASSET).await;
    assert_eq!(predicate_balance, 0);

    // Wallet balance is the same as before it sent the coins to the predicate
    let wallet_balance = get_balance(&provider, wallet.address(), OFFERED_ASSET).await;
    assert_eq!(wallet_balance, initial_wallet_balance);
}
