use crate::utils::*;
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;

        let old_identity = identity(&instance, &name).await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        assert_eq!(old_identity, wallet_identity);

        let wallet2 = WalletUnlocked::new_random(None);
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        set_identity(&instance, &name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &name).await;

        assert_eq!(new_identity, wallet_identity2);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_set_identity() {
        let (instance, _id, _wallet, wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;

        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        set_identity(
            &instance._with_wallet(wallet2).unwrap(),
            &name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
