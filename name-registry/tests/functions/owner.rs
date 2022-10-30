mod success {
    use crate::utils::{
        abi::{owner, register, set_owner},
        get_contract_instance,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_owner() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;

        let previous_owner = owner(&instance, &name).await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        assert_eq!(previous_owner.0.value.unwrap(), wallet_identity);

        let wallet2 = WalletUnlocked::new_random(None);
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        set_owner(&instance, &name, wallet_identity2.clone()).await;

        let new_owner = owner(&instance, &name).await;

        assert_eq!(new_owner.0.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use crate::utils::{abi::owner, get_contract_instance};

    #[tokio::test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value")]
    async fn cant_get_owner() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        let owner = owner(&instance, &name).await;
        owner.0.value.unwrap();
    }
}
