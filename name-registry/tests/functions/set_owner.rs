mod success {
    use crate::utils::{
        abi::{owner, register, set_owner},
        setup, string_to_ascii, OwnerChangedEvent, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_set_owner() {
        let (instance, _id, wallet, wallet2) = setup().await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &name,
            REGISTER_DURATION,
            &wallet_identity,
            &wallet_identity,
        )
        .await;

        let previous_owner = owner(&instance, &name).await;

        assert_eq!(previous_owner.0.value.unwrap(), wallet_identity);

        let response = set_owner(&instance, &name, wallet_identity2.clone()).await;

        let new_owner = owner(&instance, &name).await;

        assert_eq!(new_owner.0.value.unwrap(), wallet_identity2);

        let log = instance
            .logs_with_type::<OwnerChangedEvent>(&response.0.receipts)
            .unwrap();
        assert_eq!(
            log,
            vec![OwnerChangedEvent {
                name: string_to_ascii(&name),
                new_owner: wallet_identity2,
                previous_owner: wallet_identity
            }]
        )
    }
}

mod revert {
    use crate::utils::{
        abi::{register, set_owner},
        setup, Account, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_set_owner() {
        let (instance, _id, wallet, wallet2) = setup().await;
        let acc1 = Account::new(wallet);
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        set_owner(
            &instance.with_wallet(wallet2).unwrap(),
            &acc1.name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
