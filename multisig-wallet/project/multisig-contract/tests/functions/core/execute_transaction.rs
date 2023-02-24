use crate::utils::{
    interface::{
        core::{constructor, execute_transaction},
        info::{compute_transaction_hash, nonce},
    },
    setup::{
        base_asset_contract_id, compute_signatures, default_users, setup_env, transfer_parameters,
        DEFAULT_TRANSFER_AMOUNT, VALID_SIGNER_PK,
    },
};
use fuels::{
    prelude::{TxParameters, BASE_ASSET_ID},
    signers::fuel_crypto::Message,
};

mod success {

    use super::*;
    use crate::utils::{interface::info::balance, setup::TransferEvent};

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

    #[ignore]
    #[tokio::test]
    async fn executes_call_without_value() {}

    #[ignore]
    #[tokio::test]
    async fn executes_call_with_value() {}
}

mod revert {

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {}

    mod transfer {

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAnAssetId")]
        async fn transfer_requires_an_asset_id() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAValue")]
        async fn transfer_requires_a_value() {}

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
