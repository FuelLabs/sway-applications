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
    use crate::utils::setup::SetWeightsEvent;

    #[ignore]
    #[tokio::test]
    async fn sets_weights() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = default_users();

        constructor(&deployer.contract, users.clone()).await;

        users[0].weight += 1;
        users[1].weight -= 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, users.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        let response = set_weights(&deployer.contract, None, signatures, users.clone()).await;

        let log = response.get_logs_with_type::<SetWeightsEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, SetWeightsEvent { users });
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

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = default_users();

        constructor(&deployer.contract, users.clone()).await;

        users[0].weight += 1;
        users[1].weight -= 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, users.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_weights(&deployer.contract, None, signatures, users.clone()).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = default_users();

        constructor(&deployer.contract, users.clone()).await;

        users[0].weight -= 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, users.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_weights(&deployer.contract, None, signatures, users.clone()).await;
    }
}
