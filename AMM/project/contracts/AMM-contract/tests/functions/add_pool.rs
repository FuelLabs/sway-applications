use crate::utils::setup;
use test_utils::{
    abi::amm::{add_pool, pool},
    data_structures::ExchangeContractConfiguration,
    setup::common::deploy_and_construct_exchange,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_when_asset_pair_is_in_same_order() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;

        // adding pair to the AMM contract in the same order as the constructed exchange contract
        add_pool(&amm_instance, pair, exchange.id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(exchange_contract_id_in_storage.unwrap(), exchange.id);
    }

    #[tokio::test]
    async fn adds_when_asset_pair_is_in_reverse_order() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;

        // adding pair to the AMM contract in the reverse order as the constructed exchange contract
        add_pool(&amm_instance, (pair.1, pair.0), exchange.id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(exchange_contract_id_in_storage.unwrap(), exchange.id);
    }

    #[tokio::test]
    async fn adds_more_than_once() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair_1 = asset_pairs[0];
        let pair_2 = asset_pairs[1];

        let exchange_1 = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair_1), None, None, None),
        )
        .await;
        let exchange_2 = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair_2), None, None, Some([1u8; 32])),
        )
        .await;

        add_pool(&amm_instance, pair_1, exchange_1.id).await;
        add_pool(&amm_instance, pair_2, exchange_2.id).await;

        let exchange_contract_id_in_storage_of_pair_1 = pool(&amm_instance, pair_1).await;
        let exchange_contract_id_in_storage_of_pair_2 = pool(&amm_instance, pair_2).await;

        assert_ne!(exchange_contract_id_in_storage_of_pair_1, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_1.unwrap(),
            exchange_1.id
        );
        assert_ne!(exchange_contract_id_in_storage_of_pair_2, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_2.unwrap(),
            exchange_2.id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"BytecodeRootNotSet\"")]
    async fn when_not_initialized() {
        let (wallet, amm_instance, asset_pairs) = setup(false).await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;

        add_pool(&amm_instance, pair, exchange.id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"BytecodeRootDoesNotMatch\"")]
    async fn when_exchange_contract_byteroot_invalid() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];

        let invalid_exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, Some(true), None),
        )
        .await;

        add_pool(&amm_instance, pair, invalid_exchange.id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"PairDoesNotDefinePool\"")]
    async fn when_exchange_contract_does_not_match_pair() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];
        let another_pair = asset_pairs[1];

        let _exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;
        let another_exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(another_pair), None, None, Some([1u8; 32])),
        )
        .await;

        add_pool(&amm_instance, pair, another_exchange.id).await;
    }
}
