mod success {
    use crate::utils::{
        abi::register, setup, string_to_ascii, NameRegisteredEvent, REGISTER_DURATION,
    }; 
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_register() {
        let (instance, _id, wallet, _wallet2) = setup().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        let response = register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;
        let log = instance
            .logs_with_type::<NameRegisteredEvent>(&response.0.receipts)
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: response.1 + REGISTER_DURATION,
                name: string_to_ascii(&name),
                owner: wallet_identity.clone(),
                identity: wallet_identity
            }]
        )
    }
}

mod revert {
    use crate::utils::{abi::register, setup, REGISTER_DURATION,};
    use fuels::prelude::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_repeat_register() {
        let (instance, _id, wallet, _wallet2) = setup().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;
        register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_infinite() {
        let (instance, _id, wallet, _wallet2) = setup().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(
            &instance,
            &name,
            u64::MAX,
            &wallet_identity,
            &wallet_identity,
        )
        .await;
    }
}
