use crate::utils::{
    abi_calls::{cancel_transaction, constructor, nonce},
    test_helpers::{constructor_users, setup_env, DEFAULT_THRESHOLD},
    VALID_SIGNER_PK,
};

use fuels::prelude::*;

mod success {

    use super::*;

    #[tokio::test]
    async fn cancels_transaction() {
        let (_private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;

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

        constructor(&contract, constructor_users(), DEFAULT_THRESHOLD).await;

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
