use crate::utils::setup;
use test_utils::{
    abi::amm::initialize,
    data_structures::ExchangeContractConfiguration,
    setup::common::{deploy_and_construct_exchange, exchange_bytecode_root},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn initializes() {
        let (_wallet, amm_instance, _asset_pairs) = setup(false).await;

        let calculated_bytecode_root = exchange_bytecode_root().await;
        initialize(&amm_instance, calculated_bytecode_root).await;
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"BytecodeRootAlreadySet\"")]
    async fn when_already_initialized() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(asset_pairs[0]), Some(true), None, None),
        )
        .await;

        // already initialized
        initialize(&amm_instance, exchange.bytecode_root.unwrap()).await;
    }
}
