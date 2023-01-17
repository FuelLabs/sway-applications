use crate::utils::{
    interface::{constructor, nonce},
    test_helpers::{default_users, setup_env, DEFAULT_THRESHOLD},
    User, VALID_SIGNER_PK,
};

use fuels::prelude::*;

mod success {

    use super::*;

    #[tokio::test]
    async fn setup_with_constructor() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;

        assert_eq!(nonce(&deployer.contract).await.value, 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn cannot_reinitialize() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;

        constructor(&deployer.contract, default_users(), DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ThresholdCannotBeZero")]
    async fn threshold_cannot_be_zero() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users(), 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AddressCannotBeZero")]
    async fn address_cannot_be_zero() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = default_users();
        users[0] = User {
            address: Bits256::from_hex_str(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            weight: 3,
        };

        constructor(&deployer.contract, users, DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic(expected = "WeightingCannotBeZero")]
    async fn weighting_cannot_be_zero() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = default_users();
        users[0] = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 0,
        };

        constructor(&deployer.contract, users, DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TotalWeightCannotBeLessThanThreshold")]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let default_users = default_users();

        let mut default_total_weight = 0;
        for user in default_users.iter() {
            default_total_weight += user.weight;
        }

        constructor(&deployer.contract, default_users, default_total_weight + 1).await;
    }
}
