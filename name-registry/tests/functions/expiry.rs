use crate::utils::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;

        let old_expiry = expiry(&instance, &name).await;

        extend(&instance, &name, 5000).await;

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(old_expiry + 5000, new_expiry);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(0)")]
    async fn cant_get_expiry() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        let _expiry = expiry(&instance, &name).await;
    }
}
