use crate::utils::{
    interface::{
        core::{constructor, execute_transaction},
        info::{compute_hash, nonce},
    },
    setup::{
        base_asset_contract_id, default_users, setup_env, transfer_parameters, transfer_signatures,
        TypeToHash, DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK,
    },
};
use fuels::{
    accounts::{fuel_crypto::Message, Account},
    prelude::{TxParameters, BASE_ASSET_ID},
};

mod success {
    use super::*;

    mod transfer {

        use super::*;
        use crate::utils::{interface::info::balance, setup::ExecuteTransactionEvent};

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
            .await;

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
    mod call {}
}

mod revert {
    use super::*;

    mod transfer {

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
            .await;
        }
    }
    mod call {}
}
