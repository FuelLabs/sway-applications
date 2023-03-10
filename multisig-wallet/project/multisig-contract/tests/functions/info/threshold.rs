mod success {

    use crate::utils::{
        interface::{core::constructor, info::threshold},
        setup::{default_users, setup_env, DEFAULT_THRESHOLD, VALID_SIGNER_PK},
    };

    #[tokio::test]
    async fn returns_threshold() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        let current_threshold = threshold(&deployer.contract).await.value;

        assert_eq!(DEFAULT_THRESHOLD, current_threshold);

        println!(
            "Estimated transaction cost: {:?}",
            &deployer
                .contract
                .methods()
                .threshold()
                .estimate_transaction_cost(Some(0.0))
                .await
                .unwrap()
        );
    }
}
