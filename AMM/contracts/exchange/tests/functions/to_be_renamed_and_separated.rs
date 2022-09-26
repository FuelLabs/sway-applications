use crate::utils::{
    abi_calls::{
        deposit, get_add_liquidity, get_balance, get_pool_info, get_swap_with_maximum,
        get_swap_with_minimum, remove_liquidity, swap_with_maximum, swap_with_minimum, withdraw,
    },
    test_helpers::{deposit_and_add_liquidity, setup},
    Identity,
};
use fuels::prelude::*;

#[tokio::test]
async fn exchange_contract() {
    let (
        exchange_instance,
        token_instance,
        wallet,
        native_contract_id,
        token_asset_id,
        lp_asset_id,
    ) = setup().await;

    ////////////////////////////////////////////////////////
    // Mint some tokens to the wallet
    ////////////////////////////////////////////////////////

    // Get the contract ID and a handle to it
    let wallet_token_amount = 20000;

    // Initialize token contract
    token_instance
        .initialize(
            Identity::Address(Address::from(wallet.address())),
            wallet_token_amount,
        )
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

    let call_params = CallParameters::new(Some(native_amount), None, None);
    deposit(&exchange_instance, call_params).await;

    // Check contract balance
    let balance = get_balance(&exchange_instance, native_contract_id.clone()).await;
    assert_eq!(balance, native_amount);

    withdraw(
        &exchange_instance,
        native_amount,
        native_contract_id.clone(),
    )
    .await;

    // Check contract balance
    let balance = get_balance(&exchange_instance, native_contract_id).await;
    assert_eq!(balance, 0);

    ////////////////////////////////////////////////////////
    // Deposit tokens and create pool
    ////////////////////////////////////////////////////////

    let native_amount_deposit = native_amount;
    let token_amount_deposit = 200;
    // Check user position
    let lp_amount_received = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        token_amount_deposit,
        token_asset_id,
    )
    .await;
    assert_eq!(lp_amount_received, native_amount);

    ////////////////////////////////////////////////////////
    // Remove liquidity and receive assets back
    ////////////////////////////////////////////////////////

    // Remove LP tokens from liquidity it should keep proportion 1:2
    // It should return the exact amount added on the add liquidity
    let call_params = CallParameters::new(
        Some(lp_amount_received),
        Some(lp_asset_id.clone()),
        Some(100_000_000),
    );
    let tx_params = TxParameters {
        gas_price: 0,
        gas_limit: 100_000_000,
        maturity: 0,
    };
    let result = remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    assert_eq!(result.eth_amount, native_amount_deposit);
    assert_eq!(result.token_amount, token_amount_deposit);

    ////////////////////////////////////////////////////////
    // Setup the pool
    ////////////////////////////////////////////////////////

    // Check user position
    let _t = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        token_amount_deposit,
        token_asset_id,
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
    let amount_expected =
        get_swap_with_minimum(&exchange_instance, CallParameters::default(), amount).await;
    assert!(amount_expected.has_liquidity);
    // Swap using expected amount ETH -> TOKEN
    let call_params = CallParameters::new(Some(amount), None, None);
    let response = swap_with_minimum(
        &exchange_instance,
        call_params,
        1000,
        amount_expected.amount,
    )
    .await;
    assert_eq!(response.value, amount_expected.amount);

    ////////////////////////////////////////////////////////
    // SWAP WITH MINIMUM (TOKEN -> ETH)
    ////////////////////////////////////////////////////////

    // Get expected swap amount TOKEN -> ETH
    let call_params = CallParameters::new(Some(0), Some(token_asset_id.clone()), None);
    let amount_expected = get_swap_with_minimum(&exchange_instance, call_params, amount).await;
    assert!(amount_expected.has_liquidity);
    // Swap using expected amount TOKEN -> ETH
    let call_params = CallParameters::new(Some(amount), Some(token_asset_id.clone()), None);
    let response = swap_with_minimum(
        &exchange_instance,
        call_params,
        1000,
        amount_expected.amount,
    )
    .await;
    assert_eq!(response.value, amount_expected.amount);

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
    let amount_expected =
        get_swap_with_maximum(&exchange_instance, CallParameters::default(), amount).await;
    assert!(amount_expected.has_liquidity);
    // Swap using expected amount ETH -> TOKEN
    let call_params = CallParameters::new(Some(amount_expected.amount), None, None);
    let response = swap_with_maximum(&exchange_instance, call_params, amount, 1000).await;
    assert_eq!(response.value, amount_expected.amount);

    ////////////////////////////////////////////////////////
    // SWAP WITH MAXIMUM (TOKEN -> ETH)
    ////////////////////////////////////////////////////////

    // Get expected swap amount TOKEN -> ETH
    let call_params = CallParameters::new(None, Some(token_asset_id.clone()), None);
    let amount_expected = get_swap_with_maximum(&exchange_instance, call_params, amount).await;
    assert!(amount_expected.has_liquidity);
    // Swap using expected amount TOKEN -> ETH
    let call_params = CallParameters::new(
        Some(amount_expected.amount),
        Some(token_asset_id.clone()),
        None,
    );
    let response = swap_with_maximum(&exchange_instance, call_params, amount, 1000).await;
    assert_eq!(response.value, amount_expected.amount);

    ////////////////////////////////////////////////////////
    // Add more liquidity to the contract
    ////////////////////////////////////////////////////////

    let call_params = CallParameters::new(
        Some(amount_expected.amount),
        Some(token_asset_id.clone()),
        Some(100_000_000),
    );
    let tx_params = TxParameters {
        gas_price: 0,
        gas_limit: 100_000_000,
        maturity: 0,
    };
    let add_liquidity_preview = get_add_liquidity(
        &exchange_instance,
        call_params,
        tx_params,
        eth_to_add_liquidity_amount,
        Bits256(*BASE_ASSET_ID),
    )
    .await;
    assert_eq!(add_liquidity_preview.lp_token_received, 99);

    let lp_amount_received = deposit_and_add_liquidity(
        &exchange_instance,
        native_amount_deposit,
        add_liquidity_preview.token_amount,
        token_asset_id,
    )
    .await
        + lp_amount_received;
    // The amount of tokens returned should be smaller
    // as swaps already happen
    assert_eq!(lp_amount_received, expected_final_lp_amount);

    ////////////////////////////////////////////////////////
    // Remove liquidity and receive assets back
    ////////////////////////////////////////////////////////

    let call_params = CallParameters::new(
        Some(lp_amount_received),
        Some(lp_asset_id.clone()),
        Some(100_000_000),
    );
    let tx_params = TxParameters {
        gas_price: 0,
        gas_limit: 100_000_000,
        maturity: 0,
    };
    let result = remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    assert_eq!(result.eth_amount, remove_liquidity_eth_amount);
    assert_eq!(result.token_amount, remove_liquidity_token_amount);

    ////////////////////////////////////////////////////////
    // Check contract pool is zero
    ////////////////////////////////////////////////////////

    let pool_info = get_pool_info(&exchange_instance).await;
    assert_eq!(pool_info.eth_reserve, 0);
    assert_eq!(pool_info.token_reserve, 0);
    assert_eq!(pool_info.lp_token_supply, 0);
}
