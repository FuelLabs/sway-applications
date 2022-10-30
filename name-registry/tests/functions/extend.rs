mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        get_contract_instance, string_to_ascii, RegistrationExtendedEvent,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_extend() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        let (_, register_time) =
            register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;

        let previous_expiry = expiry(&instance, &name).await;

        let extend_response = extend(&instance, &name, 5000).await;
        let log = instance
            .logs_with_type::<RegistrationExtendedEvent>(&extend_response.0.receipts)
            .unwrap();

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(previous_expiry.0.value.unwrap() + 5000, new_expiry.0.value.unwrap());
        assert_eq!(
            log,
            vec![RegistrationExtendedEvent {
                duration: 5000,
                name: string_to_ascii(&name),
                new_expiry: register_time + 10000
            }]
        );
    }
}

mod revert {
    use crate::utils::{
        abi::{extend, register},
        get_contract_instance,
    };
    use fuels::prelude::*;
    #[tokio::test]
    #[should_panic]
    async fn cant_extend_insufficient_payment() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;

        extend(&instance, &name, u64::MAX).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_extend_name_not_registered() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        extend(&instance, &name, 5000).await;
    }
}
