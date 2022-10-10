use crate::utils::*;

mod success {
    use super::*;
    #[tokio::test]
    async fn can_register() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;
    }
}

mod revert {
    use super::*;
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_repeat_register() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, 5000).await;
        register(&instance, &name, 5000).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_infinite() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        register(&instance, &name, u64::MAX).await;
    }
}
