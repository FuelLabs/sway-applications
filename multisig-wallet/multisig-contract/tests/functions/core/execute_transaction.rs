use crate::utils::{
    interface::{
        core::{constructor, execute_transaction},
        info::{compute_hash, nonce},
    },
    setup::{
        call_parameters, default_users, deploy_target_contract, setup_env, transfer_parameters,
        transfer_signatures, TypeToHash, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK,
    },
};
use fuels::{
    accounts::{fuel_crypto::Message, Account},
    prelude::{TxPolicies, BASE_ASSET_ID},
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
                    TxPolicies::default(),
                )
                .await
                .unwrap();

            // Check balances pre-transfer
            let initial_contract_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;
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
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                }
            );

            // check balances post-transfer
            let final_contract_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;
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
        use crate::utils::{
            interface::target_contract::{count, deposit},
            setup::DEFAULT_CALLDATA_VALUE,
        };

        #[tokio::test]
        async fn executes_call_without_value() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &target_contract, false);

            // Check counter_map pre-call
            let initial_counter = count(&target_contract, deployer.wallet.address().into())
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
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                }
            );

            // Check counter_map post-call
            let final_counter = count(&target_contract, deployer.wallet.address().into())
                .await
                .value;

            assert_eq!(initial_counter, 0);
            assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE);
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
                    TxPolicies::default(),
                )
                .await
                .unwrap();

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &target_contract, true);

            // Check counter_map pre-call
            let initial_counter = count(&target_contract, deployer.wallet.address().into())
                .await
                .value;

            // Check deposit_map pre-call
            let initial_deposit = deposit(&target_contract, deployer.wallet.address().into())
                .await
                .value;

            // Check balances pre-call
            let initial_multisig_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;
            let initial_target_contract_balance = deployer
                .wallet
                .provider()
                .unwrap()
                .get_contract_asset_balance(target_contract.contract_id(), BASE_ASSET_ID)
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
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;

            let log = response
                .decode_logs_with_type::<ExecuteTransactionEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                ExecuteTransactionEvent {
                    nonce: transaction.nonce,
                    target: transaction.target,
                }
            );

            // Check counter_map post-call
            let final_counter = count(&target_contract, deployer.wallet.address().into())
                .await
                .value;

            // Check deposit_map post-call
            let final_deposit = deposit(&target_contract, deployer.wallet.address().into())
                .await
                .value;

            // Check balances post-call
            // Uncomment when https://github.com/FuelLabs/fuel-core/issues/1535 is resolved
            // let final_multisig_balance = balance(&deployer.contract, BASE_ASSET_ID).await.value;
            let final_multisig_balance = deployer
                .wallet
                .provider()
                .unwrap()
                .get_contract_asset_balance(deployer.contract.contract_id(), BASE_ASSET_ID)
                .await
                .unwrap();
            let final_target_contract_balance = deployer
                .wallet
                .provider()
                .unwrap()
                .get_contract_asset_balance(target_contract.contract_id(), BASE_ASSET_ID)
                .await
                .unwrap();

            assert_eq!(initial_counter, 0);
            assert_eq!(final_counter, DEFAULT_CALLDATA_VALUE);
            assert!(final_counter > initial_counter);

            assert_eq!(initial_deposit, 0);
            assert_eq!(final_deposit, DEFAULT_TRANSFER_AMOUNT);
            assert!(final_deposit > initial_deposit);

            assert_eq!(initial_multisig_balance, DEFAULT_TRANSFER_AMOUNT);
            assert_eq!(initial_target_contract_balance, 0);

            assert_eq!(final_multisig_balance, 0);
            assert_eq!(final_target_contract_balance, DEFAULT_TRANSFER_AMOUNT);

            assert!(final_multisig_balance < initial_multisig_balance);
            assert!(final_target_contract_balance > initial_target_contract_balance);
        }
    }
}

mod revert {
    use super::*;
    use crate::utils::setup::{TransactionParameters, TransferParams};

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

        execute_transaction(
            &deployer.contract,
            signatures,
            transaction.target.clone(),
            transaction.transaction_parameters.clone(),
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

            transaction.transaction_parameters = TransactionParameters::Transfer(TransferParams {
                asset_id: BASE_ASSET_ID,
                value: None,
            });

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            execute_transaction(
                &deployer.contract,
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
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

            execute_transaction(
                &deployer.contract,
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
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
                    TxPolicies::default(),
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

            execute_transaction(
                &deployer.contract,
                incorrectly_ordered_signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
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
                    TxPolicies::default(),
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
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;
        }
    }
    mod call {

        use super::*;
        use fuels::{
            programs::call_utils::TxDependencyExtension,
            types::{Address, Identity},
        };

        #[tokio::test]
        #[should_panic(expected = "CanOnlyCallContracts")]
        async fn can_only_call_contracts() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let mut transaction =
                call_parameters(&deployer, initial_nonce, &target_contract, false);

            let target_as_contract_id = target_contract.contract_id();
            transaction.target =
                Identity::Address(Address::from(*target_contract.contract_id().hash));

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            deployer
                .contract
                .methods()
                .execute_transaction(
                    signatures,
                    transaction.target,
                    transaction.transaction_parameters,
                )
                .append_variable_outputs(1)
                .with_contract_ids(&[target_as_contract_id.clone()])
                .call()
                .await
                .unwrap();
        }

        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &target_contract, true);

            let tx_hash = compute_hash(
                &deployer.contract,
                TypeToHash::Transaction(transaction.clone()),
            )
            .await
            .value
            .0;
            let tx_hash = Message::from_bytes(tx_hash);
            let signatures = transfer_signatures(private_key, tx_hash).await;

            execute_transaction(
                &deployer.contract,
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "IncorrectSignerOrdering")]
        async fn incorrect_signer_ordering() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &target_contract, false);

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

            execute_transaction(
                &deployer.contract,
                incorrectly_ordered_signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {
            let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

            constructor(&deployer.contract, default_users()).await;

            let initial_nonce = nonce(&deployer.contract).await.value;

            let target_contract = deploy_target_contract(deployer.wallet.clone())
                .await
                .unwrap();

            let transaction = call_parameters(&deployer, initial_nonce, &target_contract, false);

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
                signatures,
                transaction.target.clone(),
                transaction.transaction_parameters.clone(),
            )
            .await;
        }
    }
}
