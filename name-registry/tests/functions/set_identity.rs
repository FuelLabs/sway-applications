mod success {
    use crate::utils::{
        abi::{identity, register, set_identity},
        setup, string_to_ascii, IdentityChangedEvent, REGISTER_DURATION
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, _id, wallet, _wallet2) = setup().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;

        let previous_identity = identity(&instance, &name).await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        assert_eq!(previous_identity.0.value.unwrap(), wallet_identity);

        let wallet2 = WalletUnlocked::new_random(None);
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        let response = set_identity(&instance, &name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &name).await;

        assert_eq!(new_identity.0.value.unwrap(), wallet_identity2);

        let log = instance
            .logs_with_type::<IdentityChangedEvent>(&response.0.receipts)
            .unwrap();
        assert_eq!(
            log,
            vec![IdentityChangedEvent {
                name: string_to_ascii(&name),
                new_identity: wallet_identity2,
                previous_identity: wallet_identity
            }]
        )
    }
}

mod revert {
    use crate::utils::{
        abi::{register, set_identity},
        setup, REGISTER_DURATION
    };
    use fuels::prelude::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_set_identity() {
        let (instance, _id, wallet, wallet2) = setup().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;

        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        set_identity(
            &instance.with_wallet(wallet2).unwrap(),
            &name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
