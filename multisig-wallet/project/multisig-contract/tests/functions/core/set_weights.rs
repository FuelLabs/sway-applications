use crate::utils::{
    interface::{core::constructor, info::nonce},
    setup::{default_users, setup_env, VALID_SIGNER_PK},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn adds_two_owners() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;

        assert_eq!(nonce(&deployer.contract).await.value, 1);
    }
}

mod revert {

    use super::*;
    use crate::utils::setup::User;
    use fuels::prelude::Bits256;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn cannot_reinitialize() {
        let (_private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&deployer.contract, default_users()).await;
        constructor(&deployer.contract, default_users()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();
        constructor(&deployer.contract, default_users()).await;
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

        constructor(&deployer.contract, users).await;
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

        constructor(&deployer.contract, users).await;
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
