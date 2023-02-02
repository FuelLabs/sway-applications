use crate::utils::{
    interface::{
        core::{constructor, set_weight},
        info::{approval_weight, nonce, weight_hash},
    },
    setup::{default_users, setup_env, transfer_signatures, VALID_SIGNER_PK},
};
use fuels::signers::fuel_crypto::Message;

mod success {

    use super::*;
    use crate::utils::setup::SetWeightEvent;

    #[tokio::test]
    async fn sets_weight() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let users = default_users();
        let mut user = users.get(0).unwrap().clone();

        constructor(&deployer.contract, users.clone()).await;

        user.weight += 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, user.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        let initial_weight = approval_weight(&deployer.contract, user.address.into())
            .await
            .value;

        let response = set_weight(&deployer.contract, None, signatures, user.clone()).await;

        let final_nonce = nonce(&deployer.contract).await.value;
        let final_weight = approval_weight(&deployer.contract, user.address.into())
            .await
            .value;

        let log = response.get_logs_with_type::<SetWeightEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, SetWeightEvent { user: user.clone() });
        assert_eq!(initial_nonce, 1);
        assert_eq!(final_nonce, initial_nonce + 1);
        assert_ne!(initial_weight, final_weight);
        assert_eq!(final_weight, user.weight);
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
        let user = users.get(0).unwrap().clone();

        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, user.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_weight(&deployer.contract, None, signatures, user.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let users = default_users();
        let mut user = users.get(0).unwrap().clone();

        constructor(&deployer.contract, users.clone()).await;

        user.weight += 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, user.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let mut signatures = transfer_signatures(private_key, tx_hash).await;
        signatures.pop();

        set_weight(&deployer.contract, None, signatures, user.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let users = default_users();
        let mut user = users.get(0).unwrap().clone();

        constructor(&deployer.contract, users.clone()).await;

        user.weight -= 1;

        let initial_nonce = nonce(&deployer.contract).await.value;
        let tx_hash = weight_hash(&deployer.contract, None, initial_nonce, user.clone())
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };
        let signatures = transfer_signatures(private_key, tx_hash).await;

        set_weight(&deployer.contract, None, signatures, user.clone()).await;
    }
}
