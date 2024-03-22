mod success {

    use crate::utils::{
        interface::{core::constructor, info::nonce},
        setup::{default_users, setup_env, VALID_SIGNER_PK},
    };

    #[tokio::test]
    async fn gets_nonce() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_nonce = nonce(&deployer.contract).await.value;

        constructor(&deployer.contract, default_users()).await;

        let final_nonce = nonce(&deployer.contract).await.value;

        assert_eq!(initial_nonce, 0);
        assert_eq!(final_nonce, initial_nonce + 1);
    }
}
