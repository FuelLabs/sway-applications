mod success {
    use crate::utils::{
        abi::{identity, register, set_identity},
        setup, string_to_ascii, IdentityChangedEvent, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        let previous_identity = identity(&instance, &acc1.name).await;

        assert_eq!(previous_identity.0.value.unwrap(), acc1.identity(),);

        let response = set_identity(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &acc1.name).await;

        assert_eq!(new_identity.0.value.unwrap(), wallet_identity2);

        let log = response
            .0
            .get_logs_with_type::<IdentityChangedEvent>()
            .unwrap();
        assert_eq!(
            log,
            vec![IdentityChangedEvent {
                name: string_to_ascii(&acc1.name),
                new_identity: wallet_identity2,
                previous_identity: acc1.identity(),
            }]
        )
    }
}

mod revert {
    use crate::utils::{
        abi::{register, set_identity},
        setup, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    // TODO: missing tests

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn cant_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        set_identity(
            &instance.with_wallet(wallet2).unwrap(),
            &acc1.name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
