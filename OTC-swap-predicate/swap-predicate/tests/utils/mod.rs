use fuels::{
    accounts::{predicate::Predicate, Account},
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, AssetConfig, AssetId,
        Bech32Address, Provider, ResourceFilter, TxPolicies,
    },
    test_helpers::WalletsConfig,
    types::{
        coin_type::CoinType,
        input::Input,
        output::Output,
        transaction_builders::{
            BuildableTransaction, ScriptTransactionBuilder, TransactionBuilder,
        },
    },
};

abigen!(Predicate(
    name = "SwapPredicate",
    abi = "./swap-predicate/out/debug/swap-predicate-abi.json"
));

// The fee-paying base asset
const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
// Offered asset is the asset that will be locked behind the predicate
const OFFERED_ASSET: AssetId = AssetId::new([2u8; 32]);
const PREDICATE_BINARY: &str = "../swap-predicate/out/debug/swap-predicate.bin";

// Get the balance of a given asset of an address
async fn get_balance(provider: &Provider, address: &Bech32Address, asset: AssetId) -> u64 {
    provider.get_asset_balance(address, asset).await.unwrap()
}

// Create wallet config for two wallets with base, offered, and ask assets
fn configure_wallets(asked_asset: AssetId) -> WalletsConfig {
    let assets = [BASE_ASSET, OFFERED_ASSET, asked_asset];

    WalletsConfig::new_multiple_assets(
        2,
        assets
            .map(|asset| AssetConfig {
                id: asset,
                num_coins: 1,
                coin_amount: 1_000_000_000,
            })
            .to_vec(),
    )
}

/// Tests that the predicate can be spent. Parameterized by test cases
pub async fn test_predicate_spend_with_parameters(
    ask_amount: u64,
    asked_asset: AssetId,
    receiver: &str,
) {
    let receiver_address = receiver.parse().unwrap();

    let wallets =
        &launch_custom_provider_and_get_wallets(configure_wallets(asked_asset), None, None)
            .await
            .unwrap();

    let receiver_wallet = &wallets[0];
    let taker_wallet = &wallets[1];

    let provider = receiver_wallet.provider().unwrap();

    let initial_taker_offered_asset_balance =
        get_balance(provider, taker_wallet.address(), OFFERED_ASSET).await;
    let initial_taker_asked_asset_balance =
        get_balance(provider, taker_wallet.address(), asked_asset).await;
    let initial_receiver_balance = get_balance(provider, &receiver_address, asked_asset).await;

    let predicate = Predicate::load_from(PREDICATE_BINARY).unwrap();

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    receiver_wallet
        .transfer(
            predicate.address(),
            offered_amount,
            OFFERED_ASSET,
            TxPolicies::default(),
        )
        .await
        .unwrap();

    // The predicate root has received the coin
    assert_eq!(
        get_balance(provider, predicate.address(), OFFERED_ASSET).await,
        offered_amount
    );

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_resources(ResourceFilter {
            from: predicate.address().clone(),
            asset_id: Some(OFFERED_ASSET),
            amount: 1,
            ..Default::default()
        })
        .await
        .unwrap()[0];

    // Get other coin to spend
    let swap_coin = &provider
        .get_spendable_resources(ResourceFilter {
            from: taker_wallet.address().clone(),
            asset_id: Some(asked_asset),
            amount: 1,
            ..Default::default()
        })
        .await
        .unwrap()[0];

    // Configure inputs and outputs to send coins from the predicate root to another address
    // The predicate allows to spend its assets if `ask_amount` is sent to the receiver.

    // Offered asset coin belonging to the predicate root
    let input_predicate = match predicate_coin {
        CoinType::Coin(_) => Input::resource_predicate(
            predicate_coin.clone(),
            predicate.code().to_vec(),
            Vec::default(),
        ),
        _ => panic!("Predicate coin resource type does not match"),
    };

    // Asked asset coin belonging to the wallet taking the order
    let input_from_taker = match swap_coin {
        CoinType::Coin(_) => Input::resource_signed(swap_coin.clone()),
        _ => panic!("Swap coin resource type does not match"),
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

    let mut tb = ScriptTransactionBuilder::prepare_transfer(
        vec![input_predicate, input_from_taker],
        vec![output_to_receiver, output_to_taker, output_asked_change],
        TxPolicies::default(),
    );
    tb.add_signer(taker_wallet.clone()).unwrap();
    let tx = tb.build(provider).await.unwrap();

    let _tx_status = provider
        .send_transaction_and_await_commit(tx)
        .await
        .unwrap();

    let predicate_balance = get_balance(provider, predicate.address(), OFFERED_ASSET).await;
    let taker_asked_asset_balance =
        get_balance(provider, taker_wallet.address(), asked_asset).await;
    let taker_offered_asset_balance =
        get_balance(provider, taker_wallet.address(), OFFERED_ASSET).await;
    let receiver_balance = get_balance(provider, &receiver_address, asked_asset).await;

    // The predicate root's coin has been spent
    assert_eq!(predicate_balance, 0);

    // Receiver has been paid `ask_amount`
    assert_eq!(receiver_balance, initial_receiver_balance + ask_amount);

    // Taker has sent `ask_amount` of the asked asset and received `offered_amount` of the offered asset in return
    assert_eq!(
        taker_asked_asset_balance,
        initial_taker_asked_asset_balance - ask_amount
    );
    assert_eq!(
        taker_offered_asset_balance,
        initial_taker_offered_asset_balance + offered_amount
    );
}

// Tests that the predicate can be recovered by the owner
// `correct_owner` is a boolean flag to set in order to test passing and failing conditions
pub async fn recover_predicate_as_owner(correct_owner: bool) {
    let wallets =
        &launch_custom_provider_and_get_wallets(configure_wallets(BASE_ASSET), None, None)
            .await
            .unwrap();

    let wallet = match correct_owner {
        true => &wallets[0],
        false => &wallets[1],
    };

    let provider = wallet.provider().unwrap();

    let initial_wallet_balance = get_balance(provider, wallet.address(), OFFERED_ASSET).await;

    let predicate = Predicate::load_from(PREDICATE_BINARY)
        .unwrap()
        .with_configurables(
            SwapPredicateConfigurables::default()
                .with_RECEIVER(wallets[0].address().into())
                .unwrap(),
        );

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    wallet
        .transfer(
            &predicate.address().clone(),
            offered_amount,
            OFFERED_ASSET,
            TxPolicies::default(),
        )
        .await
        .unwrap();

    // Get predicate coin to unlock
    let predicate_coin = &provider
        .get_spendable_resources(ResourceFilter {
            from: predicate.address().clone(),
            asset_id: Some(OFFERED_ASSET),
            amount: 1,
            ..Default::default()
        })
        .await
        .unwrap()[0];
    let input_predicate = match predicate_coin {
        CoinType::Coin(_) => Input::resource_predicate(
            predicate_coin.clone(),
            predicate.code().to_vec(),
            Vec::default(),
        ),
        _ => panic!("Predicate coin resource type does not match"),
    };

    // Use a change output to send the unlocked coins back to the wallet
    let output_offered_change = Output::Change {
        to: Address::from(wallet.address()),
        amount: 0,
        asset_id: OFFERED_ASSET,
    };

    let tx = ScriptTransactionBuilder::prepare_transfer(
        vec![input_predicate],
        vec![output_offered_change],
        TxPolicies::default(),
    )
    .build(provider)
    .await
    .unwrap();

    let _tx_status = provider
        .send_transaction_and_await_commit(tx)
        .await
        .unwrap();

    // The predicate root's coin has been spent
    let predicate_balance = get_balance(provider, predicate.address(), OFFERED_ASSET).await;
    assert_eq!(predicate_balance, 0);

    // Wallet balance is the same as before it sent the coins to the predicate
    let wallet_balance = get_balance(provider, wallet.address(), OFFERED_ASSET).await;
    assert_eq!(wallet_balance, initial_wallet_balance);
}