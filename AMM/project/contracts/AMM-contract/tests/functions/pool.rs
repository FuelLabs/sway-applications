mod success {
    use crate::utils::setup;
    use test_utils::{
        data_structures::ExchangeContractConfiguration,
        interface::amm::{add_pool, pool},
        setup::common::deploy_and_construct_exchange,
    };

    #[tokio::test]
    async fn gets_some() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;

        add_pool(&amm_instance, pair, exchange.id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(exchange_contract_id_in_storage.unwrap(), exchange.id);
    }

    #[tokio::test]
    async fn gets_none() {
        let (wallet, amm_instance, asset_pairs) = setup(true).await;
        let pair = asset_pairs[0];
        let another_pair = asset_pairs[1];

        let exchange = deploy_and_construct_exchange(
            &wallet,
            &ExchangeContractConfiguration::new(Some(pair), None, None, None),
        )
        .await;

        add_pool(&amm_instance, pair, exchange.id).await;

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;
        let non_existent_exchange_contract_id_in_storage = pool(&amm_instance, another_pair).await;

        assert_ne!(exchange_contract_id_in_storage, None);
        assert_eq!(exchange_contract_id_in_storage.unwrap(), exchange.id);
        assert_eq!(non_existent_exchange_contract_id_in_storage, None);
    }
}
