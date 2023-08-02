use crate::utils::{
    interface::{
        callable_contract::{check_counter_map, check_deposit_map},
        core::{constructor, execute_transaction},
        info::{compute_hash, nonce},
    },
    setup::{
        base_asset_contract_id, call_parameters, default_users, deploy_callable_contract,
        setup_env, transfer_parameters, transfer_signatures, TypeToHash,
        DEFAULT_CALLDATA_VALUE_PARAM, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK,
    },
};
use fuels::{
    accounts::{fuel_crypto::Message, Account},
    prelude::{TxParameters, BASE_ASSET_ID},
};

mod success {
    use super::*;
    use crate::utils::{interface::info::balance, setup::ExecuteTransactionEvent};

    mod transfer {

        use super::*;

        #[tokio::test]
        async fn executes_transfer() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (receiver_wallet, _receiver, transaction) =
                transfer_parameters(&deployer, initial_nonce);

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
                .provider()
                .unwrap()
                .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
                .await
                .unwrap();

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await
            .unwrap();

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.get(0).unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                    transfer_params: transaction.transfer_params,
                }
            );

            // check balances post-transfer
            let final_contract_balance = balance(&deployer.contract, base_asset_contract_id())
                .await
                .value;
            let final_receiver_balance = deployer
                .wallet
                .provider()
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
    }
    mod call {

        use super::*;

        #[tokio::test]
        async fn executes_call_without_value() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let callable_contract = deploy_callable_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &callable_contract, false);

            // Check counter_map pre-call
            let initial_counter =
                check_counter_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await
            .unwrap();

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.get(0).unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                    transfer_params: transaction.transfer_params,
                }
            );

            // Check counter_map post-call
            let final_counter =
                check_counter_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            assert_eq!(initial_counter, 0);
            assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE_PARAM);
            assert!(final_counter > initial_counter);
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

            let callable_contract = deploy_callable_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &callable_contract, true);

            // Check counter_map pre-call
            let initial_counter =
                check_counter_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            // Check deposit_map pre-call
            let initial_deposit =
                check_deposit_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            // Check balances pre-call
            let initial_multisig_balance = balance(&deployer.contract, base_asset_contract_id())
                .await
                .value;
            let initial_callable_contract_balance = deployer
                .wallet
                .provider()
                .unwrap()
                .get_contract_asset_balance(callable_contract.contract_id(), BASE_ASSET_ID)
                .await
                .unwrap();

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await
            .unwrap();

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.get(0).unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                    transfer_params: transaction.transfer_params,
                }
            );

            // Check counter_map post-call
            let final_counter =
                check_counter_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            // Check deposit_map post-call
            let final_deposit =
                check_deposit_map(&callable_contract, deployer.wallet.address().into())
                    .await
                    .value;

            // Check balances post-call
            let final_multisig_balance = balance(&deployer.contract, base_asset_contract_id())
                .await
                .value;
            let final_callable_contract_balance = deployer
                .wallet
                .provider()
                .unwrap()
                .get_contract_asset_balance(callable_contract.contract_id(), BASE_ASSET_ID)
                .await
                .unwrap();

            assert_eq!(initial_counter, 0);
            assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE_PARAM);
            assert!(final_counter > initial_counter);

            assert_eq!(initial_deposit, 0);
            assert_eq!(final_deposit, DEFAULT_TRANSFER_AMOUNT);
            assert!(final_deposit > initial_deposit);

            assert_eq!(initial_multisig_balance, DEFAULT_TRANSFER_AMOUNT);
            assert_eq!(initial_callable_contract_balance, 0);

            assert_eq!(final_multisig_balance, 0);
            assert_eq!(final_callable_contract_balance, DEFAULT_TRANSFER_AMOUNT);

            assert!(final_multisig_balance < initial_multisig_balance);
            assert!(final_callable_contract_balance > initial_callable_contract_balance);
        }
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;

        let (_receiver_wallet, _receiver, transaction) =
            transfer_parameters(&deployer, initial_nonce);

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Transaction(transaction.clone()),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let signatures = transfer_signatures(private_key, tx_hash).await;

        let _response = execute_transaction(
            &deployer.contract,
            transaction.contract_call_params.clone(),
            signatures,
            transaction.target.clone(),
            transaction.transfer_params.clone(),
        )
        .await;
    }

    mod transfer {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAValue")]
        async fn transfer_requires_a_value() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, _receiver, mut transaction) =
                transfer_parameters(&deployer, initial_nonce);

            transaction.transfer_params.value = None;

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let _response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, _receiver, transaction) =
                transfer_parameters(&deployer, initial_nonce);

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let _response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "IncorrectSignerOrdering")]
        async fn incorrect_signer_ordering() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, _receiver, transaction) =
                transfer_parameters(&deployer, initial_nonce);

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

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            let incorrectly_ordered_signatures = vec![signatures[1].clone(), signatures[0].clone()];

            let _response = execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                incorrectly_ordered_signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let (_receiver_wallet, _receiver, transaction) =
                transfer_parameters(&deployer, initial_nonce);

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

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let mut signatures = transfer_signatures(private_key, tx_hash).await;

            signatures.remove(0);

            execute_transaction(
                &deployer.contract,
                transaction.contract_call_params.clone(),
                signatures,
                transaction.target.clone(),
                transaction.transfer_params.clone(),
            )
            .await
            .unwrap();
        }
    }
    mod call {}
}
