mod success {
    use crate::utils::{
        abi::{owner, register, set_owner},
        setup, REGISTER_DURATION, Account
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_owner() {
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

        let previous_owner = owner(&instance, &acc1.name).await;

        assert_eq!(previous_owner.0.value.unwrap(), acc1.identity());

        set_owner(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_owner = owner(&instance, &acc1.name).await;

        assert_eq!(new_owner.0.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use crate::utils::{abi::owner, setup};

    #[tokio::test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value")]
    async fn cant_get_owner() {
        let (instance, _id, _wallet, _wallet2) = setup().await;
        let name = String::from("SwaySway");

        let owner = owner(&instance, &name).await;
        owner.0.value.unwrap();
    }
}
