use std::{vec, str::FromStr};
use fuel_tx::{AssetId, ContractId, Bytes32};
use fuels::prelude::*;
use fuels_abigen_macro::abigen;
use fuels::tx::StorageSlot;

///////////////////////////////
// Load the Exchange Contract abi
///////////////////////////////
abigen!(TestExchange, "out/debug/exchange_contract-abi.json");

///////////////////////////////
// Load the Token Contract abi
///////////////////////////////
abigen!(
    TestToken,
    "../token_contract/out/debug/token_contract-abi.json"
);

async fn deposit_and_add_liquidity(
    exchange_instance: &TestExchange,
    native_amount: u64,
    token_asset_id: AssetId,
    token_amount_deposit: u64,
) -> u64 {
    // Deposit some Native Asset
    let _t = exchange_instance
        .deposit()
        .call_params(CallParameters::new(Some(native_amount), None, None))
        .call()
        .await
        .unwrap();

    // Deposit some Token Asset
    let _t = exchange_instance
        .deposit()
        .call_params(CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        ))
        .call()
        .await
        .unwrap();

    // Add liquidity for the second time. Keeping the proportion 1:2
    // It should return the same amount of LP as the amount of ETH deposited
    let result = exchange_instance
        .add_liquidity(1, 1000)
        .call_params(CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000)))
        .append_variable_outputs(2)
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .call()
        .await
        .unwrap();

    result.value
}

#[tokio::test]
async fn exchange_contract() {
    // default initial amount 1000000000
    let wallet = launch_provider_and_get_wallet().await;
    // Wallet address
    let address = wallet.address();

    //////////////////////////////////////////
    // Setup contracts
    //////////////////////////////////////////

    let token_contract_id = Contract::deploy(
        "../token_contract/out/debug/token_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default()
    )
    .await
    .unwrap();


    let key = Bytes32::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    let value = token_contract_id.hash();
    let storage_slot = StorageSlot::new(key, value);
    let storage_vec = vec![storage_slot.clone()];

    // Deploy contract and get ID
    let exchange_contract_id = Contract::deploy(
        "out/debug/exchange_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_manual_storage(Some(storage_vec)),
    )
    .await
    .unwrap();
    let exchange_instance = TestExchangeBuilder::new(exchange_contract_id.to_string(), wallet.clone()).build();
    let token_instance = TestTokenBuilder::new(token_contract_id.to_string(), wallet.clone()).build();

    // Native contract id
    let native_contract_id = ContractId::new(*BASE_ASSET_ID);
    // Token contract id
    let token_contract_id = token_contract_id;
    // Token asset id
    let token_asset_id = AssetId::from(*token_contract_id.hash());
    // LP Token asset id
    let lp_asset_id = AssetId::from(*exchange_contract_id.hash());

    ////////////////////////////////////////////////////////
    // Mint some tokens to the wallet
    ////////////////////////////////////////////////////////

    // Get the contract ID and a handle to it
    let wallet_token_amount = 20000;

    // Initialize token contract
    token_instance
        .initialize(wallet_token_amount, Address::from(address))
        .call()
        .await
        .unwrap();

    // Mint some alt tokens
    token_instance
        .mint()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    ////////////////////////////////////////////////////////
    // Test deposit & withdraw NativeToken from ExchangeContract
    ////////////////////////////////////////////////////////

    // Total amount of native amounts
    // send to the wallet
    let native_amount = 100;

    // Deposit some native assets
    exchange_instance
        .deposit()
        .call_params(CallParameters::new(Some(native_amount), None, None))
        .call()
        .await
        .unwrap();

    // Check contract balance
    let response = exchange_instance
        .get_balance(native_contract_id.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, native_amount);

    exchange_instance
        .withdraw(native_amount, native_contract_id.clone())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    // Check contract balance
    let response = exchange_instance
        .get_balance(native_contract_id)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, 0);

    ////////////////////////////////////////////////////////
    // Deposit tokens and create pool
    ////////////////////////////////////////////////////////

    let native_amount_deposit = native_amount;
    let token_amount_deposit = 200;
    // Check user position
    let lp_amount_received = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        token_asset_id,
        token_amount_deposit,
    )
    .await;
    assert_eq!(lp_amount_received, native_amount);

    ////////////////////////////////////////////////////////
    // Remove liquidity and receive assets back
    ////////////////////////////////////////////////////////

    // Remove LP tokens from liquidity it should keep proportion 1:2
    // It should return the exact amount added on the add liquidity
    let result = exchange_instance
        .remove_liquidity(1, 1, 1000)
        .call_params(CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000)
        ))
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();
    assert_eq!(result.value.eth_amount, native_amount_deposit);
    assert_eq!(result.value.token_amount, token_amount_deposit);

    ////////////////////////////////////////////////////////
    // Setup the pool
    ////////////////////////////////////////////////////////

    // Check user position
    let _t = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        token_asset_id,
        token_amount_deposit,
    )
    .await;

    ////////////////////////////////////////////////////////
    // Amounts
    ////////////////////////////////////////////////////////

    // Swap amount
    let amount: u64 = 10;
    // Amount used on a second add_liquidity
    let eth_to_add_liquidity_amount: u64 = 100;
    // Final balance of LP tokens
    let expected_final_lp_amount: u64 = 199;
    // Final eth amount removed from the Pool
    let remove_liquidity_eth_amount: u64 = 201;
    // Final token amount removed from the Pool
    let remove_liquidity_token_amount: u64 = 388;

    ////////////////////////////////////////////////////////
    // SWAP WITH MINIMUM (ETH -> TOKEN)
    ////////////////////////////////////////////////////////

    // Get expected swap amount ETH -> TOKEN
    let amount_expected = exchange_instance
        .get_swap_with_minimum(amount)
        .call()
        .await
        .unwrap();
    assert!(amount_expected.value.has_liquidity);
    // Swap using expected amount ETH -> TOKEN
    let response = exchange_instance
        .swap_with_minimum(amount_expected.value.amount, 1000)
        .call_params(CallParameters::new(Some(amount), None, None))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, amount_expected.value.amount);

    ////////////////////////////////////////////////////////
    // SWAP WITH MINIMUM (TOKEN -> ETH)
    ////////////////////////////////////////////////////////

    // Get expected swap amount TOKEN -> ETH
    let amount_expected = exchange_instance
        .get_swap_with_minimum(amount)
        .call_params(CallParameters::new(Some(0), Some(token_asset_id.clone()), None))
        .call()
        .await
        .unwrap();
    assert!(amount_expected.value.has_liquidity);
    // Swap using expected amount TOKEN -> ETH
    let response = exchange_instance
        .swap_with_minimum(amount_expected.value.amount, 1000)
        .call_params(CallParameters::new(
            Some(amount),
            Some(token_asset_id.clone()),
            None
        ))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, amount_expected.value.amount);

    ////////////////////////////////////////////////////////
    // SWAP WITH MAXIMUM EXPECT ERRORS (ETH -> TOKEN)
    ////////////////////////////////////////////////////////

    // Should throw error
    // If the output is bigger them the reserve
    let is_err = exchange_instance
        .get_swap_with_maximum(1000)
        .call()
        .await
        .is_err();
    assert!(is_err);

    ////////////////////////////////////////////////////////
    // SWAP WITH MAXIMUM EXPECT ERRORS (TOKEN -> ETH)
    ////////////////////////////////////////////////////////

    // Should return u64::MAX
    // If the output is equal to the reserve
    let is_err = exchange_instance
        .get_swap_with_maximum(token_amount_deposit + 1)
        .call()
        .await
        .is_err();
    assert!(is_err);

    ////////////////////////////////////////////////////////
    // SWAP WITH MAXIMUM (ETH -> TOKEN)
    ////////////////////////////////////////////////////////

    // Get expected swap amount ETH -> TOKEN
    let amount_expected = exchange_instance
        .get_swap_with_maximum(amount)
        .call()
        .await
        .unwrap();
    assert!(amount_expected.value.has_liquidity);
    // Swap using expected amount ETH -> TOKEN
    let response = exchange_instance
        .swap_with_maximum(amount, 1000)
        .call_params(CallParameters::new(
            Some(amount_expected.value.amount),
            None,
            None
        ))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, amount_expected.value.amount);

    ////////////////////////////////////////////////////////
    // SWAP WITH MAXIMUM (TOKEN -> ETH)
    ////////////////////////////////////////////////////////

    // Get expected swap amount TOKEN -> ETH
    let amount_expected = exchange_instance
        .get_swap_with_maximum(amount)
        .call_params(CallParameters::new(None, Some(token_asset_id.clone()), None))
        .call()
        .await
        .unwrap();
    assert!(amount_expected.value.has_liquidity);
    // Swap using expected amount TOKEN -> ETH
    let response = exchange_instance
        .swap_with_maximum(amount, 1000)
        .call_params(CallParameters::new(
            Some(amount_expected.value.amount),
            Some(token_asset_id.clone()),
            None
        ))
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value, amount_expected.value.amount);

    ////////////////////////////////////////////////////////
    // Add more liquidity to the contract
    ////////////////////////////////////////////////////////

    let add_liquidity_preview = exchange_instance
        .get_add_liquidity(eth_to_add_liquidity_amount, *BASE_ASSET_ID)
        .call_params(CallParameters::new(
            Some(amount_expected.value.amount),
            Some(token_asset_id.clone()),
            Some(100_000_000),
        ))
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .simulate()
        .await
        .unwrap();
    assert_eq!(add_liquidity_preview.value.lp_token_received, 99);

    let lp_amount_received = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        token_asset_id,
        add_liquidity_preview.value.token_amount
    )
    .await
        + lp_amount_received;
    // The amount of tokens returned should be smaller
    // as swaps already happen
    assert_eq!(lp_amount_received, expected_final_lp_amount);

    ////////////////////////////////////////////////////////
    // Remove liquidity and receive assets back
    ////////////////////////////////////////////////////////

    let response = exchange_instance
        .remove_liquidity(1, 1, 1000)
        .call_params(CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000)
        ))
        .tx_params(TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        })
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();
    assert_eq!(response.value.eth_amount, remove_liquidity_eth_amount);
    assert_eq!(response.value.token_amount, remove_liquidity_token_amount);

    ////////////////////////////////////////////////////////
    // Check contract pool is zero
    ////////////////////////////////////////////////////////

    let pool_info = exchange_instance.get_pool_info().call().await.unwrap();
    assert_eq!(pool_info.value.eth_reserve, 0);
    assert_eq!(pool_info.value.token_reserve, 0);
    assert_eq!(pool_info.value.lp_token_supply, 0);
}
