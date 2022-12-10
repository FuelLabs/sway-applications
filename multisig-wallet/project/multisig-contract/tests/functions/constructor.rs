use crate::utils::{
    abi_calls::{constructor, nonce},
    test_helpers::{constructor_users, setup_env, DEFAULT_THRESHOLD},
    User, VALID_SIGNER_PK,
};

use fuels::prelude::*;

mod success {

    use super::*;

    #[tokio::test]
    async fn setup_with_constructor() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;

        assert_eq!(nonce(&contract).await.value, 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn cannot_reinitialize() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn threshold_cannot_be_zero() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&contract, constructor_users(), 0).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn address_cannot_be_zero() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = constructor_users();
        users[0] = User {
            address: Bits256::from_hex_str(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            weight: 3,
        };

        constructor(&contract, users, DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn weighting_cannot_be_zero() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let mut users = constructor_users();
        users[0] = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 0,
        };

        constructor(&contract, users, DEFAULT_THRESHOLD).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn total_weight_cannot_be_less_than_threshold() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&contract, constructor_users(), 100).await;
    }
}
