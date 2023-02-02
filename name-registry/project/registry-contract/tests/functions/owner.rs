mod success {
    use crate::utils::{
        abi::{owner, register, set_owner},
        setup, REGISTER_DURATION,
    };
    use fuels::{prelude::*, types::Identity};

    #[tokio::test]
    async fn can_get_owner() {
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

        let previous_owner = owner(&instance, &acc1.name).await;

        assert_eq!(previous_owner.value.unwrap(), acc1.identity());

        set_owner(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_owner = owner(&instance, &acc1.name).await;

        assert_eq!(new_owner.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use crate::utils::{abi::owner, setup};

    // TODO: missing test

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_get_owner() {
        let (instance, acc, _wallet2) = setup().await;
        let owner = owner(&instance, &acc.name).await;
        owner.value.unwrap();
    }
}
