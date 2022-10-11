use crate::utils::{abi::*, *};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_extend() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));

        let name = String::from("SwaySway");

        register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;

        let old_expiry = expiry(&instance, &name).await;

        extend(&instance, &name, 5000).await;

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(old_expiry + 5000, new_expiry);
    }
}

mod revert {
    use super::*;
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
