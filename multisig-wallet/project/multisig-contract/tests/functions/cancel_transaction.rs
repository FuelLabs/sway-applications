use crate::utils::{
    abi_calls::{cancel_transaction, constructor, nonce},
    test_helpers::{constructor_users, setup_env, DEFAULT_THRESHOLD},
    VALID_SIGNER_PK,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn cancels_transaction() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        let initial_nonce = nonce(&deployer.contract).await.value;

        cancel_transaction(&deployer.contract).await;

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

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        cancel_transaction(&non_owner.contract).await;
    }
}
