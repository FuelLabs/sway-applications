mod passing {
    use crate::utils::*;

    #[tokio::test]
    async fn can_extend() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;

        let old_expiry = expiry(&instance, &name).await;

        extend(&instance, &name, 5000).await;

        let new_expiry = expiry(&instance, &name).await;

        assert_eq!(old_expiry + 5000, new_expiry);
    }
}

mod failing {
    use crate::utils::*;
    #[tokio::test]
    #[should_panic]
    async fn cant_extend_insufficient_payment() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;

        extend(&instance, &name, u64::MAX).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_extend_name_not_registered() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        extend(&instance, &name, 5000).await;
    }
}
