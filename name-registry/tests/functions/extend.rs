mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        setup, string_to_ascii, RegistrationExtendedEvent, REGISTER_DURATION, EXTEND_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_extend() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");


        let (_, register_time) =
            register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;

        let previous_expiry = expiry(&instance, &name).await;

        let extend_response = extend(&instance, &name, EXTEND_DURATION).await;
        let log = instance
            .logs_with_type::<RegistrationExtendedEvent>(&extend_response.0.receipts)
            .unwrap();

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(
            previous_expiry.0.value.unwrap() + EXTEND_DURATION,
            new_expiry.0.value.unwrap()
        );
        assert_eq!(
            log,
            vec![RegistrationExtendedEvent {
                duration: EXTEND_DURATION,
                name: string_to_ascii(&name),
                new_expiry: register_time + REGISTER_DURATION + EXTEND_DURATION
            }]
        );
    }
}

mod revert {
    use crate::utils::{
        abi::{extend, register},
        setup, REGISTER_DURATION, EXTEND_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    #[should_panic]
    async fn cant_extend_insufficient_payment() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");


        register(&instance, &name, REGISTER_DURATION, &wallet_identity, &wallet_identity).await;

        extend(&instance, &name, u64::MAX).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_extend_name_not_registered() {
        let (instance, _id, _wallet, _wallet2) = setup().await;
        let name = String::from("SwaySway");

        
        extend(&instance, &name, EXTEND_DURATION).await;
    }
}
