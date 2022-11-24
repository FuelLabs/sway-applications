use crate::utils::{
    amm_abi_calls::{add_pool, pool},
    test_helpers::{deploy_and_construct_exchange_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn gets_some() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];

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
    async fn gets_none() {
        let (wallet, amm_instance, asset_pairs) = setup_and_initialize().await;
        let pair = asset_pairs[0];
        let another_pair = asset_pairs[1];

        let exchange_contract_id =
            deploy_and_construct_exchange_contract(&wallet, pair, None, None).await;

        add_pool(&amm_instance, pair, exchange_contract_id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;
        let non_existent_exchange_contract_id_in_storage = pool(&amm_instance, another_pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(
            exchange_contract_id_in_storage.unwrap(),
            exchange_contract_id
        );
        assert_eq!(non_existent_exchange_contract_id_in_storage, None);
    }
}
