use crate::utils::{
    interface::{
        core::{constructor, execute_transaction},
        info::{compute_transaction_hash, nonce},
        test_contract::{check_counter_map, check_deposit_map},
    },
    setup::{
        base_asset_contract_id, call_parameters, compute_signatures, default_users,
        deploy_test_contract, setup_env, transfer_parameters, CallParams,
        DEFAULT_CALLDATA_VALUE_PARAM, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK,
    },
};
use fuels::{
    prelude::{TxParameters, BASE_ASSET_ID},
    signers::fuel_crypto::Message,
};

mod success {

    use fuels::types::Identity;

    use super::*;
    use crate::utils::{
        interface::info::balance,
        setup::{CallEvent, TransferEvent},
    };

    #[tokio::test]
    async fn executes_transfer() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        let (receiver_wallet, tx) = transfer_parameters(&deployer, initial_nonce).await;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        // Check balances pre-transfer
        let initial_contract_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let initial_receiver_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        let tx_hash = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target.clone(),
            tx.function_selector.clone(),
            tx.calldata.clone(),
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = compute_signatures(private_key, tx_hash).await;

        let response = execute_transaction(
            &deployer.contract,
            tx.asset_id,
            tx.calldata,
            tx.forwarded_gas,
            tx.function_selector,
            signatures,
            tx.single_value_type_arg,
            tx.target.clone(),
            tx.value,
        )
        .await;

        let log = response.get_logs_with_type::<TransferEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            TransferEvent {
                asset: tx.asset_id.unwrap(),
                nonce: tx.nonce,
                target: tx.target,
                value: tx.value.unwrap(),
            }
        );

        // check balances post-transfer
        let final_contract_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let final_receiver_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        assert_eq!(initial_contract_balance, DEFAULT_TRANSFER_AMOUNT);
        assert_eq!(initial_receiver_balance, 0);

        assert_eq!(final_contract_balance, 0);
        assert_eq!(final_receiver_balance, DEFAULT_TRANSFER_AMOUNT);

        assert!(final_contract_balance < initial_contract_balance);
        assert!(final_receiver_balance > initial_receiver_balance);
    }

    #[tokio::test]
    async fn executes_call_without_value() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        let test_contract = deploy_test_contract(deployer.wallet.clone()).await.unwrap();

        let tx = call_parameters(&deployer, initial_nonce, &test_contract, false).await;

        // Check counter_map pre-call
        let initial_counter = check_counter_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        let tx_hash = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target.clone(),
            tx.function_selector.clone(),
            tx.calldata.clone(),
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = compute_signatures(private_key, tx_hash).await;

        let response = execute_transaction(
            &deployer.contract,
            tx.asset_id,
            tx.calldata.clone(),
            tx.forwarded_gas,
            tx.function_selector.clone(),
            signatures,
            tx.single_value_type_arg,
            tx.target.clone(),
            tx.value,
        )
        .await;

        let log = response.get_logs_with_type::<CallEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            CallEvent {
                call_params: CallParams {
                    coins: tx.value.unwrap_or(0),
                    asset_id: tx.asset_id.unwrap_or(base_asset_contract_id()),
                    gas: tx.forwarded_gas.unwrap_or(0),
                },
                nonce: tx.nonce,
                target_contract_id: match tx.target {
                    Identity::ContractId(contract_identifier) => contract_identifier,
                    _ => base_asset_contract_id(),
                },
                // function_selector: tx.function_selector.unwrap(), // Required SDK support for decoding Vectors within Structs
                // calldata: tx.calldata.unwrap(),
            }
        );

        // Check counter_map post-call
        let final_counter = check_counter_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        assert_eq!(initial_counter, 0);
        assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE_PARAM);
        assert_ne!(initial_counter, final_counter);
    }

    #[tokio::test]
    async fn executes_call_with_value() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let test_contract = deploy_test_contract(deployer.wallet.clone()).await.unwrap();

        let tx = call_parameters(&deployer, initial_nonce, &test_contract, true).await;

        // Check counter_map pre-call
        let initial_counter = check_counter_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        // Check deposit_map pre-call
        let initial_deposit = check_deposit_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        // Check balances pre-call
        let initial_multisig_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let initial_test_contract_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_contract_asset_balance(test_contract.contract_id(), BASE_ASSET_ID)
            .await
            .unwrap();

        let tx_hash = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target.clone(),
            tx.function_selector.clone(),
            tx.calldata.clone(),
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = compute_signatures(private_key, tx_hash).await;

        let response = execute_transaction(
            &deployer.contract,
            tx.asset_id,
            tx.calldata.clone(),
            tx.forwarded_gas,
            tx.function_selector.clone(),
            signatures,
            tx.single_value_type_arg,
            tx.target.clone(),
            tx.value,
        )
        .await;

        let log = response.get_logs_with_type::<CallEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            CallEvent {
                call_params: CallParams {
                    coins: tx.value.unwrap_or(0),
                    asset_id: tx.asset_id.unwrap_or(base_asset_contract_id()),
                    gas: tx.forwarded_gas.unwrap_or(0),
                },
                nonce: tx.nonce,
                target_contract_id: match tx.target {
                    Identity::ContractId(contract_identifier) => contract_identifier,
                    _ => base_asset_contract_id(),
                },
                // function_selector: tx.function_selector.unwrap(), // Required SDK support for decoding Vectors within Structs
                // calldata: tx.calldata.unwrap(),
            }
        );

        // Check counter_map post-call
        let final_counter = check_counter_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        // Check deposit_map post-call
        let final_deposit = check_deposit_map(&test_contract, deployer.wallet.address().into())
            .await
            .value;

        // Check balances post-call
        let final_multisig_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let final_test_contract_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_contract_asset_balance(test_contract.contract_id(), BASE_ASSET_ID)
            .await
            .unwrap();

        assert_eq!(initial_counter, 0);
        assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE_PARAM);
        assert_ne!(initial_counter, final_counter);

        assert_eq!(initial_deposit, 0);
        assert_eq!(final_deposit, DEFAULT_TRANSFER_AMOUNT);
        assert_ne!(initial_deposit, final_deposit);

        assert_eq!(initial_multisig_balance, DEFAULT_TRANSFER_AMOUNT);
        assert_eq!(initial_test_contract_balance, 0);

        assert_eq!(final_multisig_balance, 0);
        assert_eq!(final_test_contract_balance, DEFAULT_TRANSFER_AMOUNT);

        assert!(final_multisig_balance < initial_multisig_balance);
        assert!(final_test_contract_balance > initial_test_contract_balance);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;

        let (_receiver_wallet, tx) = transfer_parameters(&deployer, initial_nonce).await;

        let tx_hash = compute_transaction_hash(
            &deployer.contract,
            tx.contract_identifier,
            tx.nonce,
            tx.value,
            tx.asset_id,
            tx.target.clone(),
            tx.function_selector.clone(),
            tx.calldata.clone(),
            tx.single_value_type_arg,
            tx.forwarded_gas,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = compute_signatures(private_key, tx_hash).await;

        execute_transaction(
            &deployer.contract,
            tx.asset_id,
            tx.calldata,
            tx.forwarded_gas,
            tx.function_selector,
            signatures,
            tx.single_value_type_arg,
            tx.target.clone(),
            tx.value,
        )
        .await;
    }

    mod transfer {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAnAssetId")]
        async fn transfer_requires_an_asset_id() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, mut tx) = transfer_parameters(&deployer, initial_nonce).await;
            tx.asset_id = None;

            let tx_hash = compute_transaction_hash(
                &deployer.contract,
                tx.contract_identifier,
                tx.nonce,
                tx.value,
                tx.asset_id,
                tx.target.clone(),
                tx.function_selector.clone(),
                tx.calldata.clone(),
                tx.single_value_type_arg,
                tx.forwarded_gas,
            )
            .await
            .value
            .0;
            let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

            let signatures = compute_signatures(private_key, tx_hash).await;

            execute_transaction(
                &deployer.contract,
                tx.asset_id,
                tx.calldata,
                tx.forwarded_gas,
                tx.function_selector,
                signatures,
                tx.single_value_type_arg,
                tx.target.clone(),
                tx.value,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAValue")]
        async fn transfer_requires_a_value() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, mut tx) = transfer_parameters(&deployer, initial_nonce).await;
            tx.value = None;

            let tx_hash = compute_transaction_hash(
                &deployer.contract,
                tx.contract_identifier,
                tx.nonce,
                tx.value,
                tx.asset_id,
                tx.target.clone(),
                tx.function_selector.clone(),
                tx.calldata.clone(),
                tx.single_value_type_arg,
                tx.forwarded_gas,
            )
            .await
            .value
            .0;
            let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

            let signatures = compute_signatures(private_key, tx_hash).await;

            execute_transaction(
                &deployer.contract,
                tx.asset_id,
                tx.calldata,
                tx.forwarded_gas,
                tx.function_selector,
                signatures,
                tx.single_value_type_arg,
                tx.target.clone(),
                tx.value,
            )
            .await;
        }

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {}
    }

    mod call {

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CannotCallFunctionsOnAddresses")]
        async fn cannot_call_functions_on_addresses() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CallingFunctionsRequiresCalldata")]
        async fn calling_functions_requires_calldata() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CallingFunctionsRequiresSingleValueTypeArg")]
        async fn calling_functions_requires_single_value_type_arg() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAnAssetId")]
        async fn transfer_requires_an_asset_id() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {}
    }
}
