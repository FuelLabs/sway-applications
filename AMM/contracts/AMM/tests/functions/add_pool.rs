use crate::utils::{
    amm_abi_calls::{add_pool, pool},
    test_helpers::{deploy_and_construct_exchange_contract, setup, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn adds() {
        let (wallet, amm_instance, assets) = setup_and_initialize().await;
        let pair = (assets[0], assets[1]);
        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;
        add_pool(&amm_instance, pair, exchange_contract_id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;
        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(
            exchange_contract_id_in_storage.unwrap(),
            exchange_contract_id
        );
    }

    #[tokio::test]
    async fn adds_more_than_once() {
        let (wallet, amm_instance, assets) = setup_and_initialize().await;
        let pair_1 = (assets[0], assets[1]);
        let exchange_contract_id_1 =
            deploy_and_construct_exchange_contract(&wallet, pair_1, None, None).await;
        add_pool(&amm_instance, pair_1, exchange_contract_id_1).await;

        let pair_2 = (assets[1], assets[2]);
        let exchange_contract_id_2 =
            deploy_and_construct_exchange_contract(&wallet, pair_2, None, Some(1)).await;
        add_pool(&amm_instance, pair_2, exchange_contract_id_2).await;

        let exchange_contract_id_in_storage_of_pair_1 = pool(&amm_instance, pair_1).await;
        assert_ne!(exchange_contract_id_in_storage_of_pair_1, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_1.unwrap(),
            exchange_contract_id_1
        );

        let exchange_contract_id_in_storage_of_pair_2 = pool(&amm_instance, pair_2).await;
        assert_ne!(exchange_contract_id_in_storage_of_pair_2, None);
        assert_eq!(
            exchange_contract_id_in_storage_of_pair_2.unwrap(),
            exchange_contract_id_2
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_not_initialized() {
        let (wallet, amm_instance, assets) = setup().await;
        let pair = (assets[0], assets[1]);
        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;
        add_pool(&amm_instance, pair, exchange_contract_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_exchange_contract_byteroot_invalid() {
        let (wallet, amm_instance, assets) = setup_and_initialize().await;
        let pair = (assets[1], assets[2]);
        let invalid_exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, pair, Option::Some(false), None).await;
        add_pool(&amm_instance, pair, invalid_exchange_contract_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_exchange_contract_does_not_match_pair() {
        let (wallet, amm_instance, assets) = setup_and_initialize().await;
        let pair = (assets[0], assets[1]);
        let _exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;
        let another_pair = (assets[1], assets[2]);
        let another_exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, another_pair, None, Some(1)).await;
        add_pool(&amm_instance, pair, another_exchange_contract_id).await;
    }
}
