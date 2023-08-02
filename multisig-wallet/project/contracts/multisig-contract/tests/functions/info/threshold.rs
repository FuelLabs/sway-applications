mod success {

    use crate::utils::{
        interface::{core::constructor, info::threshold},
        setup::{default_users, setup_env, DEFAULT_THRESHOLD, VALID_SIGNER_PK},
    };

    #[tokio::test]
    async fn returns_threshold() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let initial_threshold = threshold(&deployer.contract).await.value;

        constructor(&deployer.contract, default_users()).await;

        let current_threshold = threshold(&deployer.contract).await.value;

        assert_eq!(0, initial_threshold);
        assert_eq!(DEFAULT_THRESHOLD, current_threshold);
        assert_ne!(DEFAULT_THRESHOLD, initial_threshold);
    }
}
