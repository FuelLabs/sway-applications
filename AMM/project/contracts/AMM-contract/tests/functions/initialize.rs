use crate::utils::{
    amm_abi_calls::initialize,
    test_helpers::{
        bytecode_root, deploy_and_construct_exchange_contract, setup, setup_and_initialize,
    },
    EXCHANGE_CONTRACT_BYTECODE_ROOT,
};
use fuels::tx::ContractId;
use std::str::FromStr;

mod success {
    use super::*;

    #[tokio::test]
    async fn initializes() {
        let (_wallet, amm_instance, _asset_pairs) = setup().await;

        let hardcoded_bytecode_root =
            ContractId::from_str(EXCHANGE_CONTRACT_BYTECODE_ROOT).unwrap();
        let calculated_bytecode_root = bytecode_root().await;

        initialize(&amm_instance, hardcoded_bytecode_root).await;

        assert_eq!(calculated_bytecode_root, hardcoded_bytecode_root);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_already_initialized() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;

        let exchange =
            deploy_and_construct_exchange_contract(&wallet, asset_pairs[0], None, None).await;

        // already initialized
        initialize(&amm_instance, exchange.bytecode_root).await;
    }
}
