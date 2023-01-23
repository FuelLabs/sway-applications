use crate::utils::{
    interface::{
        core::{cancel_transaction, constructor},
        info::nonce,
    },
    setup::{default_users, setup_env, DEFAULT_THRESHOLD, VALID_SIGNER_PK},
};

mod success {

    use super::*;
    use crate::utils::setup::CancelEvent;
    use fuels::prelude::Bits256;

    #[tokio::test]
    async fn cancels_transaction() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        let response = cancel_transaction(&deployer.contract).await;
        let log = response.get_logs_with_type::<CancelEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            CancelEvent {
                cancelled_nonce: initial_nonce,
                user: Bits256(deployer.wallet.address().hash.try_into().unwrap()),
            }
        );

        let final_nonce = nonce(&deployer.contract).await.value;

        assert_eq!(initial_nonce, 1);
        assert_eq!(final_nonce, initial_nonce + 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CanOnlyBeAccessedByAnOwner")]
    async fn not_an_owner() {
        let (_private_key, deployer, non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;

        cancel_transaction(&non_owner.contract).await;
    }
}
