use crate::utils::{
    interface::{constructor, nonce},
    test_helpers::{default_users, setup_env, DEFAULT_THRESHOLD},
    VALID_SIGNER_PK,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_nonce() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;

        let final_nonce = nonce(&deployer.contract).await.value;

        assert_eq!(initial_nonce, 0);
        assert_eq!(final_nonce, initial_nonce + 1);
    }
}
