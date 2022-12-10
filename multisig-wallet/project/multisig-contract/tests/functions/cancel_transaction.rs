use crate::utils::{
    abi_calls::{cancel_transaction, constructor, nonce},
    test_helpers::setup_env,
    User, VALID_SIGNER_PK,
};

use fuels::prelude::*;

mod success {

    use super::*;

    #[tokio::test]
    async fn cancels_transaction() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 3,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 2,
        };
        let users = vec![fuel_user_1, evm_user_1];
        constructor(&contract, users, 5).await;

        let initial_nonce = nonce(&contract).await.value;

        cancel_transaction(&contract).await;

        let final_nonce = nonce(&contract).await.value;

        assert_eq!(initial_nonce, 1);
        assert_eq!(final_nonce, 2);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn not_an_owner() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 3,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 2,
        };
        let users = vec![fuel_user_1, evm_user_1];
        constructor(&contract, users, 5).await;

        let (non_owner_provider, _address) = setup_test_provider(vec![], vec![], None, None).await;
        let non_owner_wallet = WalletUnlocked::new_random(Some(non_owner_provider));

        contract
            .with_wallet(non_owner_wallet)
            .unwrap()
            .methods()
            .cancel_transaction()
            .call()
            .await
            .unwrap();
    }
}
