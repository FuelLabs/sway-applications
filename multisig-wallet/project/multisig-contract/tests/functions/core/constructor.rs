use crate::utils::{
    interface::{core::constructor, info::nonce},
    setup::{default_users, setup_env, VALID_SIGNER_PK},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn setup_with_constructor() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        assert_eq!(nonce(&deployer.contract).await.value, 0);

        constructor(&deployer.contract, default_users()).await;

        assert_eq!(nonce(&deployer.contract).await.value, 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn cannot_reinitialize() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;
        constructor(&deployer.contract, default_users()).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "ThresholdCannotBeZero")]
    async fn threshold_cannot_be_zero() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        // TODO: Inject a different config time constant value to change the THRESHOLD when SDK supports
        constructor(&deployer.contract, default_users()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut default_users = default_users();

        for user in default_users.iter_mut() {
            // set weights to the lowest value so that they are lower than the current default threshold
            user.weight = 1;
        }

        constructor(&deployer.contract, default_users).await;
    }
}
