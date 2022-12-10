use crate::utils::{
    abi_calls::{constructor, nonce},
    test_helpers::{constructor_users, setup_env, DEFAULT_THRESHOLD},
    VALID_SIGNER_PK,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_nonce() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&contract).await.value;

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;

        let final_nonce = nonce(&contract).await.value;

        assert_eq!(initial_nonce, 0);
        assert_eq!(final_nonce, 1);
    }
}
