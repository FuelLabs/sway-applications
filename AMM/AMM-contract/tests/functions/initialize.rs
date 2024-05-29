use crate::utils::setup;
use test_utils::interface::amm::initialize;

mod success {
    use super::*;
    use fuels::types::Bits256;
    use test_utils::{
        interface::SetExchangeBytecodeRootEvent, setup::common::exchange_bytecode_root,
    };

    #[tokio::test]
    async fn initializes() {
        let (_wallet, amm_instance, _asset_pairs) = setup(false).await;

        let calculated_bytecode_root = exchange_bytecode_root().await;

        let response = initialize(&amm_instance, calculated_bytecode_root).await;
        let log = response
            .decode_logs_with_type::<SetExchangeBytecodeRootEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            SetExchangeBytecodeRootEvent {
                root: Bits256::from_hex_str(&calculated_bytecode_root.to_string()).unwrap()
            }
        );
    }
}

mod revert {
    use super::*;
    use test_utils::{
        data_structures::ExchangeContractConfiguration,
        setup::common::deploy_and_construct_exchange,
    };

    #[tokio::test]
    #[should_panic(expected = "BytecodeRootAlreadySet")]
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
