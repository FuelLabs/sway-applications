use crate::utils::{
    amm_abi_calls::{add_pool, pool},
    test_helpers::{deploy_and_construct_exchange_contract, setup, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_when_asset_pair_is_in_same_order() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;

        // adding pair to the AMM contract in the same order as the constructed exchange contract
        add_pool(&amm_instance, pair, exchange.contract_id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(
            exchange_contract_id_in_storage.unwrap(),
            exchange.contract_id
        );
    }

    #[tokio::test]
    async fn adds_when_asset_pair_is_in_reverse_order() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;

        // adding pair to the AMM contract in the reverse order as the constructed exchange contract
        add_pool(&amm_instance, (pair.1, pair.0), exchange.contract_id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(
            exchange_contract_id_in_storage.unwrap(),
            exchange.contract_id
        );
    }

    #[tokio::test]
    async fn adds_more_than_once() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair_1 = asset_pairs[0];
        let pair_2 = asset_pairs[1];

        let exchange_1 = deploy_and_construct_exchange_contract(&wallet, pair_1, None, None).await;
        let exchange_2 =
            deploy_and_construct_exchange_contract(&wallet, pair_2, None, Some(1)).await;

        add_pool(&amm_instance, pair_1, exchange_1.contract_id).await;
        add_pool(&amm_instance, pair_2, exchange_2.contract_id).await;

        let exchange_contract_id_in_storage_of_pair_1 = pool(&amm_instance, pair_1).await;
        let exchange_contract_id_in_storage_of_pair_2 = pool(&amm_instance, pair_2).await;

        assert_ne!(exchange_contract_id_in_storage_of_pair_1, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_1.unwrap(),
            exchange_1.contract_id
        );
        assert_ne!(exchange_contract_id_in_storage_of_pair_2, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_2.unwrap(),
            exchange_2.contract_id
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_initialized() {
        let (wallet, amm_instance, asset_pairs) = setup().await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;

        add_pool(&amm_instance, pair, exchange.contract_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_exchange_contract_byteroot_invalid() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];

        let invalid_exchange =
            deploy_and_construct_exchange_contract(&wallet, pair, Some(true), None).await;

        add_pool(&amm_instance, pair, invalid_exchange.contract_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_exchange_contract_does_not_match_pair() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];
        let another_pair = asset_pairs[1];

        let _exchange = deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;
        let another_exchange =
            deploy_and_construct_exchange_contract(&wallet, another_pair, None, Some(1)).await;

        add_pool(&amm_instance, pair, another_exchange.contract_id).await;
    }
}
