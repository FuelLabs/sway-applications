mod passing {
    use crate::utils::*;
    #[tokio::test]
    async fn can_register() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;
    }
}

mod failing {
    use crate::utils::*;
    #[tokio::test]
    #[should_panic]
    async fn cant_repeat_register() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;
        register(&instance, &name, 5000).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_infinite() {
        let (instance, _id, _wallet) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, u64::MAX).await;
    }
}
