use crate::utils::{
    interface::{
        core::{constructor, set_threshold},
        info::{threshold, update_hash},
    },
    setup::{default_users, setup_env, transfer_signatures, DEFAULT_THRESHOLD, VALID_SIGNER_PK},
};
use fuels::{prelude::Bits256, signers::fuel_crypto::Message};

mod success {

    use super::*;
    use crate::utils::setup::{transfer_parameters, SetThresholdEvent};

    #[tokio::test]
    async fn sets_the_threshold() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();
        let (_receiver_wallet, _receiver, data) = transfer_parameters();
        let nonce = 1;

        constructor(&deployer.contract, default_users()).await;

        let previous_threshold = threshold(&deployer.contract).await.value;

        let tx_hash = update_hash(&deployer.contract, data, nonce).await.value.0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        let response =
            set_threshold(&deployer.contract, data, signatures, DEFAULT_THRESHOLD - 1).await;

        let threshold = threshold(&deployer.contract).await.value;

        let log = response.get_logs_with_type::<SetThresholdEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            SetThresholdEvent {
                previous_threshold,
                threshold,
            }
        );
        assert_eq!(previous_threshold, DEFAULT_THRESHOLD);
        assert_eq!(threshold, DEFAULT_THRESHOLD - 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        // Create any data just to call the function. The 1st require should panic so we do not care
        // about the data being passed in
        let data = Bits256([1u8; 32]);

        let tx_hash = update_hash(&deployer.contract, data, 0).await.value.0;

        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, data, signatures, DEFAULT_THRESHOLD - 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ThresholdCannotBeZero")]
    async fn when_threshold_is_zero() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        // Create any data just to call the function. The 1st require should panic so we do not care
        // about the data being passed in
        let data = Bits256([1u8; 32]);

        let tx_hash = update_hash(&deployer.contract, data, 0).await.value.0;

        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, data, signatures, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn when_threshold_is_greater_than_approval_weight_of_all_owners() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        // Create any data just to call the function. The 1st require should panic so we do not care
        // about the data being passed in
        let data = Bits256([1u8; 32]);

        let tx_hash = update_hash(&deployer.contract, data, 0).await.value.0;

        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_threshold(&deployer.contract, data, signatures, DEFAULT_THRESHOLD + 1).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        // TODO: forgot to do this
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();
        constructor(&deployer.contract, default_users()).await;

    }
}
