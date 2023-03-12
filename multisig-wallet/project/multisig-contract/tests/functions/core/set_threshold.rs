use crate::utils::{
    interface::{
        core::{constructor, set_threshold},
        info::{compute_hash, nonce, threshold},
    },
    setup::{
        compute_signatures, default_users, setup_env, Threshold, TypeToHash, DEFAULT_THRESHOLD,
        VALID_SIGNER_PK,
    },
};
use fuels::signers::fuel_crypto::Message;

mod success {

    use super::*;
    use crate::utils::setup::SetThresholdEvent;

    #[tokio::test]
    async fn sets_the_threshold() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let initial_threshold = threshold(&deployer.contract).await.value;

        let threshold_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
                nonce: initial_nonce,
                threshold: initial_threshold - 1,
            }),
        )
        .await
        .value
        .0;
        let threshold_hash = unsafe { Message::from_bytes_unchecked(threshold_hash) };
        let signatures = compute_signatures(private_key, threshold_hash).await;

        let response = set_threshold(
            &deployer.contract,
            signatures.clone(),
            DEFAULT_THRESHOLD - 1,
        )
        .await;

        let final_nonce = nonce(&deployer.contract).await.value;
        let final_threshold = threshold(&deployer.contract).await.value;

        let log = response.get_logs_with_type::<SetThresholdEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            SetThresholdEvent {
                previous_threshold: initial_threshold,
                threshold: final_threshold,
            }
        );

        assert_eq!(initial_threshold, DEFAULT_THRESHOLD);
        assert_eq!(final_threshold, DEFAULT_THRESHOLD - 1);
        assert_eq!(final_nonce, initial_nonce + 1);

        println!(
            "Estimated transaction cost: {:?}",
            &deployer
                .contract
                .methods()
                .set_threshold(signatures, DEFAULT_THRESHOLD - 1)
                .estimate_transaction_cost(Some(0.0))
                .await
                .unwrap()
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;

        let threshold_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
                nonce: initial_nonce,
                threshold: DEFAULT_THRESHOLD - 1,
            }),
        )
        .await
        .value
        .0;
        let threshold_hash = unsafe { Message::from_bytes_unchecked(threshold_hash) };
        let signatures = compute_signatures(private_key, threshold_hash).await;

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD - 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ThresholdCannotBeZero")]
    async fn when_threshold_is_zero() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        let threshold_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
                nonce: initial_nonce,
                threshold: 0,
            }),
        )
        .await
        .value
        .0;
        let threshold_hash = unsafe { Message::from_bytes_unchecked(threshold_hash) };
        let signatures = compute_signatures(private_key, threshold_hash).await;

        set_threshold(&deployer.contract, signatures, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn when_threshold_is_greater_than_approval_weight_of_all_owners() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        let threshold_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
                nonce: initial_nonce,
                threshold: DEFAULT_THRESHOLD + 1,
            }),
        )
        .await
        .value
        .0;
        let threshold_hash = unsafe { Message::from_bytes_unchecked(threshold_hash) };
        let signatures = compute_signatures(private_key, threshold_hash).await;

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD + 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let initial_threshold = threshold(&deployer.contract).await.value;

        let threshold_hash = compute_hash(
            &deployer.contract,
            TypeToHash::Threshold(Threshold {
                contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
                nonce: initial_nonce,
                threshold: initial_threshold - 1,
            }),
        )
        .await
        .value
        .0;
        let threshold_hash = unsafe { Message::from_bytes_unchecked(threshold_hash) };
        let mut signatures = compute_signatures(private_key, threshold_hash).await;
        signatures.pop();

        set_threshold(&deployer.contract, signatures, DEFAULT_THRESHOLD - 1).await;
    }
}
