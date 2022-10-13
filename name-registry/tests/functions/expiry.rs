mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        get_contract_instance,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;

        let previous_expiry = expiry(&instance, &name).await;

        extend(&instance, &name, 5000).await;

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(previous_expiry + 5000, new_expiry);
    }
}

mod revert {
    use crate::utils::{abi::*, *};

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_get_expiry() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        let _expiry = expiry(&instance, &name).await;
    }
}
