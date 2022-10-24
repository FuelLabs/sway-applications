use crate::utils::{
    amm_abi_calls::initialize,
    test_helpers::{deploy_and_construct_exchange_contract, setup, setup_and_initialize},
};

mod success {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn initializes() {
        let (wallet, amm_instance, assets) = setup().await;
        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, (assets[0], assets[1]), None).await;
        initialize(&amm_instance, exchange_contract_id).await;
        // no way to compute the bytecode using the SDK for now
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_already_initialized() {
        let (wallet, amm_instance, assets) = setup_and_initialize().await;
        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, (assets[0], assets[1]), None).await;

        initialize(&amm_instance, exchange_contract_id).await;
        initialize(&amm_instance, exchange_contract_id).await;
    }
}
