mod success {
    use crate::utils::{
        abi::register, setup, string_to_ascii, Account, NameRegisteredEvent, REGISTER_DURATION,
    };

    #[tokio::test]
    async fn can_register() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let acc = Account::new(wallet);

        let response = register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
        let log = instance
            .logs_with_type::<NameRegisteredEvent>(&response.0.receipts)
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: response.1 + REGISTER_DURATION,
                name: string_to_ascii(&acc.name),
                owner: acc.identity().clone(),
                identity: acc.identity()
            }]
        )
    }
}

mod revert {
    use crate::utils::{abi::register, setup, Account, REGISTER_DURATION};
    use fuels::prelude::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_repeat_register() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");

        register(
            &instance,
            &name,
            REGISTER_DURATION,
            &wallet_identity,
            &wallet_identity,
        )
        .await;
        register(
            &instance,
            &name,
            REGISTER_DURATION,
            &wallet_identity,
            &wallet_identity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_max_duration() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let acc = Account::new(wallet);

        register(
            &instance,
            &acc.name,
            u64::MAX,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
    }
}
