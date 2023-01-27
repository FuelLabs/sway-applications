use crate::utils::{
    interface::{
        core::{constructor, set_weights},
        info::{nonce, weight_hash},
    },
    setup::{default_users, setup_env, transfer_signatures, VALID_SIGNER_PK},
};

use fuels::signers::fuel_crypto::Message;

mod success {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn sets_weights() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;
        let users = default_users();

        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, users.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_weights(&deployer.contract, None, signatures, users).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();
        constructor(&deployer.contract, default_users()).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanZero")]
    async fn total_weight_cannot_be_less_than_zero() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut default_users = default_users();

        for user in default_users.iter_mut() {
            // set weights to the lowest value so that they are lower than the current default threshold
            user.weight = 1;
        }

        constructor(&deployer.contract, default_users).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut default_users = default_users();

        for user in default_users.iter_mut() {
            // set weights to the lowest value so that they are lower than the current default threshold
            user.weight = 1;
        }

        constructor(&deployer.contract, default_users).await;
    }
}
