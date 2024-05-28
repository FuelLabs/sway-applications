use crate::utils::setup;
use test_utils::{
    data_structures::ExchangeContractConfiguration, interface::amm::add_pool,
    setup::common::deploy_and_construct_exchange,
};

mod success {
    use super::*;
    use crate::utils::ordered_pair;
    use test_utils::interface::{amm::pool, RegisterPoolEvent};

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
        let response = add_pool(&amm_instance, pair, exchange.id).await;
        let log = response
            .decode_logs_with_type::<RegisterPoolEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_eq!(
            *event,
            RegisterPoolEvent {
                asset_pair: ordered_pair(pair),
                pool: exchange.id
            }
        );
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
        let response = add_pool(&amm_instance, pair, exchange.id).await;
        let log = response
            .decode_logs_with_type::<RegisterPoolEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let exchange_contract_id_in_storage = pool(&amm_instance, pair).await;

        assert_eq!(
            *event,
            RegisterPoolEvent {
                asset_pair: ordered_pair(pair),
                pool: exchange.id
            }
        );
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

        let response = add_pool(&amm_instance, pair_1, exchange_1.id).await;
        let log = response
            .decode_logs_with_type::<RegisterPoolEvent>()
            .unwrap();
        let event_1 = log.first().unwrap();

        let response = add_pool(&amm_instance, pair_2, exchange_2.id).await;
        let log = response
            .decode_logs_with_type::<RegisterPoolEvent>()
            .unwrap();
        let event_2 = log.first().unwrap();

        let exchange_contract_id_in_storage_of_pair_1 = pool(&amm_instance, pair_1).await;
        let exchange_contract_id_in_storage_of_pair_2 = pool(&amm_instance, pair_2).await;

        assert_eq!(
            *event_1,
            RegisterPoolEvent {
                asset_pair: ordered_pair(pair_1),
                pool: exchange_1.id
            }
        );
        assert_eq!(
            *event_2,
            RegisterPoolEvent {
                asset_pair: ordered_pair(pair_2),
                pool: exchange_2.id
            }
        );
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
    #[should_panic(expected = "BytecodeRootNotSet")]
    async fn when_uninitialized() {
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
    #[should_panic(expected = "BytecodeRootDoesNotMatch")]
    async fn when_byteroot_does_not_match() {
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
    #[should_panic(expected = "PairDoesNotDefinePool")]
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
