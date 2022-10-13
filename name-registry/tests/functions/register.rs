mod success {
    use fuels::prelude::*;
    use crate::utils::{abi::register, get_contract_instance};
    #[tokio::test]
    async fn can_register() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;
    }
}

mod revert {
    use fuels::prelude::*;
    use crate::utils::{abi::register, get_contract_instance};
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_repeat_register() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;
        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_infinite() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

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
