use fuel_core::service::Config;
use fuel_gql_client::fuel_vm::{consts::REG_ONE, prelude::Opcode};
use fuels::contract::script::Script;
use fuels::prelude::*;
use fuels::test_helpers::WalletsConfig;
use fuels::tx::{AssetId, Contract, Input, Output, Transaction};


pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
pub const OFFERED_ASSET: AssetId = AssetId::new([1u8; 32]);

async fn get_balance(provider: &Provider, address: &Bech32Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(address, asset).await.unwrap();
    balance
}

/// Test function to be parameterized by test cases
pub async fn test_predicate_spend_with_parameters(
    ask_amount: u64,
    asked_asset: AssetId,
    receiver_address: Bech32Address,
) {

    let mut provider_config = Config::local_node();
    provider_config.predicates = true; // predicates are currently disabled by default

    // Base asset
    let base_asset_config = AssetConfig {
        id: BASE_ASSET,
        num_coins: 1,
        coin_amount: 1_000_000_000,
    };
    // Offered asset
    let offered_asset_config = AssetConfig {
        id: OFFERED_ASSET,
        num_coins: 1,
        coin_amount: 1_000_000_000,
    };

    // Asked asset
    let asked_asset_config = AssetConfig {
        id: asked_asset,
        num_coins: 1,
        coin_amount: 1_000_000_000,
    };

    let wallet_config = WalletsConfig::new_multiple_assets(
        1,
        vec![
            base_asset_config,
            offered_asset_config,
            asked_asset_config,
        ],
    );

    let wallet = &launch_custom_provider_and_get_wallets(
        wallet_config,
        Some(provider_config),
    )
    .await[0];

    // Get provider and client
    let provider = wallet.get_provider().unwrap();
    let client = &provider.client;

    // Get predicate bytecode and root
    let predicate_bytecode =
        std::fs::read("../otc-swap-predicate/out/debug/otc-swap-predicate.bin").unwrap();
    let predicate_root: [u8; 32] = (*Contract::root_from_code(&predicate_bytecode)).into();
    let predicate_root = Address::from(predicate_root);
    let predicate_root_bech32 = Bech32Address::from(predicate_root);

    // Transfer some coins to the predicate root
    let offered_amount = 1000;
    let _receipt = wallet
        .transfer(
            &predicate_root_bech32,
            offered_amount,
            OFFERED_ASSET,
            TxParameters::default(),
        )
        .await
        .unwrap();

    let initial_predicate_balance = get_balance(&provider, &predicate_root_bech32, OFFERED_ASSET).await;
    let initial_wallet_offered_token_balance = get_balance(&provider, wallet.address(), OFFERED_ASSET).await;
    let initial_wallet_asked_token_balance = get_balance(&provider, wallet.address(), asked_asset).await;
    let initial_receiver_balance = get_balance(&provider, &receiver_address, asked_asset).await;

    // The predicate root has received the coin
    assert_eq!(initial_predicate_balance, offered_amount);
        
    // TO DO : Despite succcesful transfer (assert above), no coins are found ? 
    
    // Get predicate coin to unlock
    let predicate_coin = &provider.get_coins(&predicate_root_bech32).await.unwrap()[0];
    let predicate_coin_utxo_id = predicate_coin.utxo_id.clone().into();

    // Get other coin to spend
    let swap_coin = &provider.get_coins(wallet.address()).await.unwrap()[0];
    let swap_coin_utxo_id = swap_coin.utxo_id.clone().into();
    let swap_coin_amount: u64 = swap_coin.amount.clone().into();



    // Configure inputs and outputs to send coins from the predicate root to another address

    // The predicate allows to spend its tokens if `ask_amount` is sent to the offer maker.

    // Coin belonging to the predicate root
    let input_predicate = Input::CoinPredicate {
        utxo_id: predicate_coin_utxo_id,
        owner: predicate_root,
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![0u8], // Predicate data is the index of the output that pays the receiver
    };

    // Coin belonging to the wallet taking the order
    let input_from_taker = Input::CoinSigned {
        utxo_id: swap_coin_utxo_id,
        owner: Address::from(wallet.address()),
        amount: swap_coin_amount,
        asset_id: asked_asset,
        witness_index: 0,
        maturity: 0,
    };

    // Output for the coin transferred to the receiver
    let output_to_receiver = Output::Coin {
        to: Address::from(receiver_address.clone()),
        amount: ask_amount,
        asset_id: asked_asset,
    };

    // Output for the coin transferred to the order taker
    let output_to_taker = Output::Coin {
        to: Address::from(wallet.address()),
        amount: offered_amount,
        asset_id: OFFERED_ASSET,
    };

    // Change output for unspent fees
    let output_change = Output::Change {
        to: Address::from(wallet.address()),
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

    let predicate_balance = get_balance(&provider, &predicate_root_bech32, OFFERED_ASSET).await;
    let wallet_asked_token_balance = get_balance(&provider, wallet.address(), asked_asset).await;
    let wallet_offered_token_balance = get_balance(&provider, wallet.address(), OFFERED_ASSET).await;
    let receiver_balance = get_balance(&provider, &receiver_address, asked_asset).await;

    // The predicate root's coin has been spent
    assert_eq!(predicate_balance, 0);

    // Receiver has been paid `ask_amount`
    assert_eq!(receiver_balance, initial_receiver_balance + ask_amount);

    // Taker has sent `ask_amount` tokens and received `offered_amount` in return
    assert_eq!(wallet_asked_token_balance, initial_wallet_asked_token_balance - ask_amount);
    assert_eq!(wallet_offered_token_balance, initial_wallet_offered_token_balance + offered_amount);

}
