use fuels::prelude::*;
use fuels::test_helpers::WalletsConfig;
use fuels::tx::{AssetId, Input, Output, TxPointer};
use fuels::types::resource::Resource;

abigen!(Predicate(
    name = "SwapPredicate",
    abi = "project/swap-predicate/out/debug/swap-predicate-abi.json"
));

// The fee-paying base asset
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
// Offered asset is the asset that will be locked behind the predicate
pub const OFFERED_ASSET: AssetId = AssetId::new([2u8; 32]);

// Get the balance of a given token of an address
async fn get_balance(provider: &Provider, address: &Bech32Address, asset: AssetId) -> u64 {
    provider.get_asset_balance(address, asset).await.unwrap()
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
            }).to_vec(),
    )
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
        ..Config::local_node()
    };

    let wallets = &launch_custom_provider_and_get_wallets(
        configure_wallets(asked_asset),
        Some(provider_config),
        None,
    )
    .await;

    let receiver_wallet = &wallets[0];
    let taker_wallet = &wallets[1];

    let provider = receiver_wallet.get_provider().unwrap();

    let predicate =
        SwapPredicate::load_from("../swap-predicate/out/debug/swap-predicate.bin").unwrap();

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    predicate
        .receive(receiver_wallet, offered_amount, OFFERED_ASSET, None)
        .await
        .unwrap();

    let initial_taker_offered_token_balance =
        get_balance(provider, taker_wallet.address(), OFFERED_ASSET).await;
    let initial_taker_asked_token_balance =
        get_balance(provider, taker_wallet.address(), asked_asset).await;
    let initial_receiver_balance = get_balance(provider, &receiver_address, asked_asset).await;

    // The predicate root has received the coin
    assert_eq!(
        get_balance(provider, predicate.address(), OFFERED_ASSET).await,
        offered_amount
    );

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_resources(predicate.address(), OFFERED_ASSET, 1)
        .await
        .unwrap()[0];
    let predicate_coin_utxo_id = match predicate_coin {
        Resource::Coin(coin) => coin.utxo_id,
        _ => panic!(),
    };

    // Get other coin to spend
    let swap_coin = &provider
        .get_spendable_resources(taker_wallet.address(), asked_asset, 1)
        .await
        .unwrap()[0];
    let (swap_coin_utxo_id, swap_coin_amount) = match swap_coin {
        Resource::Coin(coin) => (coin.utxo_id, coin.amount),
        _ => panic!(),
    };

    // Configure inputs and outputs to send coins from the predicate root to another address
    // The predicate allows to spend its tokens if `ask_amount` is sent to the receiver.

    // Offered asset coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        tx_pointer: TxPointer::default(),
        owner: predicate.address().into(),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
        maturity: 0,
        predicate: predicate.code(),
        predicate_data: vec![],
    };

    // Asked asset coin belonging to the wallet taking the order
    let input_from_taker = Input::CoinSigned {
        utxo_id: swap_coin_utxo_id,
        tx_pointer: TxPointer::default(),
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

    // Change output for unspent asked asset
    let output_asked_change = Output::Change {
        to: Address::from(taker_wallet.address()),
        amount: 0,
        asset_id: asked_asset,
    };

    let mut tx = Wallet::build_transfer_tx(
        &[input_predicate, input_from_taker],
        &[output_to_receiver, output_to_taker, output_asked_change],
        TxParameters::new(None, Some(10_000_000), None),
    );

    taker_wallet.sign_transaction(&mut tx).await.unwrap();
    let _receipts = provider.send_transaction(&tx).await.unwrap();

    let predicate_balance = get_balance(provider, predicate.address(), OFFERED_ASSET).await;
    let taker_asked_token_balance =
        get_balance(provider, taker_wallet.address(), asked_asset).await;
    let taker_offered_token_balance =
        get_balance(provider, taker_wallet.address(), OFFERED_ASSET).await;
    let receiver_balance = get_balance(provider, &receiver_address, asked_asset).await;

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
// `correct_owner` is a boolean flag to set in order to test passing and failing conditions
pub async fn recover_predicate_as_owner(correct_owner: bool) {
    let provider_config = Config {
        utxo_validation: true,
        ..Config::local_node()
    };

    let wallets = &launch_custom_provider_and_get_wallets(
        configure_wallets(BASE_ASSET),
        Some(provider_config),
        None,
    )
    .await;

    let wallet = match correct_owner {
        true => &wallets[0],
        false => &wallets[1],
    };

    // Get provider
    let provider = wallet.get_provider().unwrap();

    let initial_wallet_balance = get_balance(provider, wallet.address(), OFFERED_ASSET).await;

    let predicate =
        SwapPredicate::load_from("../swap-predicate/out/debug/swap-predicate.bin").unwrap();

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    predicate
        .receive(wallet, offered_amount, OFFERED_ASSET, None)
        .await
        .unwrap();

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_resources(predicate.address(), OFFERED_ASSET, 1)
        .await
        .unwrap()[0];
    let predicate_coin_utxo_id = match predicate_coin {
        Resource::Coin(coin) => coin.utxo_id,
        _ => panic!(),
    };

    // Get fee coin
    let base_coin = &provider
        .get_spendable_resources(wallet.address(), BASE_ASSET, 1)
        .await
        .unwrap()[0];
    let (base_coin_utxo_id, swap_coin_amount) = match base_coin {
        Resource::Coin(coin) => (coin.utxo_id, coin.amount),
        _ => panic!(),
    };

    // Offered asset coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        tx_pointer: TxPointer::default(),
        owner: predicate.address().into(),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
        maturity: 0,
        predicate: predicate.code(),
        predicate_data: vec![],
    };

    // Fee coin belonging to the wallet
    let input_coin = Input::CoinSigned {
        utxo_id: base_coin_utxo_id,
        tx_pointer: TxPointer::default(),
        owner: Address::from(wallet.address()),
        amount: swap_coin_amount,
        asset_id: BASE_ASSET,
        witness_index: 0,
        maturity: 0,
    };

    // Use a change output to send the unlocked coins back to the wallet
    let output_offered_change = Output::Change {
        to: Address::from(wallet.address()),
        amount: 0,
        asset_id: OFFERED_ASSET,
    };

    let mut tx = Wallet::build_transfer_tx(
        &[input_predicate, input_coin],
        &[output_offered_change],
        TxParameters::new(None, Some(10_000_000), None),
    );

    wallet.sign_transaction(&mut tx).await.unwrap();
    let _receipts = provider.send_transaction(&tx).await.unwrap();

    // The predicate root's coin has been spent
    let predicate_balance = get_balance(provider, predicate.address(), OFFERED_ASSET).await;
    assert_eq!(predicate_balance, 0);

    // Wallet balance is the same as before it sent the coins to the predicate
    let wallet_balance = get_balance(provider, wallet.address(), OFFERED_ASSET).await;
    assert_eq!(wallet_balance, initial_wallet_balance);
}
