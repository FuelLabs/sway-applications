mod success {

    use crate::utils::{
        interface::{core::constructor, info::approval_weight},
        setup::{default_users, setup_env, VALID_SIGNER_PK},
    };

    #[tokio::test]
    async fn gets_weight() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();
        let users = default_users();

        let initial_weight = approval_weight(&deployer.contract, users.first().unwrap().address)
            .await
            .value;

        constructor(&deployer.contract, users.clone()).await;

        let final_weight = approval_weight(&deployer.contract, users.first().unwrap().address)
            .await
            .value;

        assert_eq!(initial_weight, 0);
        assert_ne!(initial_weight, final_weight);
        assert_eq!(final_weight, users.first().unwrap().weight);
    }
}
