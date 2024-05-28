use crate::utils::{
    interface::{
        core::{constructor, set_threshold},
        info::{compute_hash, nonce, threshold},
    },
    setup::{
        default_users, setup_env, transfer_signatures, Threshold, TypeToHash, DEFAULT_THRESHOLD,
        VALID_SIGNER_PK,
    },
};
use fuels::accounts::fuel_crypto::Message;

mod success {

    use super::*;
    use crate::utils::setup::SetThresholdEvent;

    #[tokio::test]
    async fn sets_the_threshold() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let previous_threshold = threshold(&deployer.contract).await.value;

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().into(),
                nonce: initial_nonce,
                threshold: previous_threshold - 1,
            }),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let signatures = transfer_signatures(private_key, tx_hash).await;

        let response = set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD - 1).await;

        let final_nonce = nonce(&deployer.contract).await.value;
        let threshold = threshold(&deployer.contract).await.value;

        let log = response
            .decode_logs_with_type::<SetThresholdEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            SetThresholdEvent {
                previous_threshold,
                threshold,
            }
        );
        assert_eq!(previous_threshold, DEFAULT_THRESHOLD);
        assert_eq!(threshold, DEFAULT_THRESHOLD - 1);
        assert_eq!(final_nonce, initial_nonce + 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;
        let previous_threshold = threshold(&deployer.contract).await.value;

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().into(),
                nonce: initial_nonce,
                threshold: previous_threshold,
            }),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ThresholdCannotBeZero")]
    async fn when_threshold_is_zero() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let new_threshold = 0;

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().into(),
                nonce: initial_nonce,
                threshold: new_threshold,
            }),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, signatures, new_threshold).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn when_threshold_is_greater_than_approval_weight_of_all_owners() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let previous_threshold = threshold(&deployer.contract).await.value;

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().into(),
                nonce: initial_nonce,
                threshold: previous_threshold,
            }),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD + 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let previous_threshold = threshold(&deployer.contract).await.value;

        let tx_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().into(),
                nonce: initial_nonce,
                threshold: previous_threshold - 1,
            }),
        )
        .await
        .value
        .0;
        let tx_hash = Message::from_bytes(tx_hash);
        let mut signatures = transfer_signatures(private_key, tx_hash).await;
        signatures.pop();

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD - 1).await;
    }
}
