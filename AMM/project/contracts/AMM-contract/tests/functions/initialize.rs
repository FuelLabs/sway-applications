use crate::utils::{
    amm_abi_calls::initialize,
    test_helpers::{
        bytecode_root_legitimate, deploy_and_construct_exchange_contract, setup,
        setup_and_initialize,
    },
};

mod success {
    use super::*;

    #[tokio::test]
    async fn initializes() {
        let (wallet, amm_instance, asset_pairs) = setup().await;

        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, asset_pairs[0], None, None).await;

        // assert that the AMM contract stores the correct exchange contract merkle root
        assert!(bytecode_root_legitimate().await);

        initialize(&amm_instance, exchange_contract_id).await;
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_already_initialized() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;

        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, asset_pairs[0], None, None).await;

        // already initialized
        initialize(&amm_instance, exchange_contract_id).await;
    }
}
