mod passing {
    use fuels::types::{ContractId, Identity};

    use crate::utils::{auction_count, bid, cancel_auction, create_auction, get_contract_instance};

    #[tokio::test]
    async fn can_get_auction_count() {
        let (instance, wallet) = get_contract_instance().await;

        assert_eq!(auction_count(&instance).await, 0); // Block height starts at 1, goes to 2 here

        create_auction(
            &instance,
            400,
            100,
            3, // 3
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(auction_count(&instance).await, 1); // 4

        create_auction(
            &instance,
            400,
            100,
            5, // 5
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(auction_count(&instance).await, 2); // 6

        create_auction(
            &instance,
            400,
            100,
            7, // 7
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(auction_count(&instance).await, 3); // 8
    }

    #[tokio::test]
    async fn cancellation_does_not_affect_count() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            3, // 3
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            4, // 4
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(auction_count(&instance).await, 3);

        cancel_auction(&instance, 2).await;

        assert_eq!(auction_count(&instance).await, 3);
    }

    #[tokio::test]
    async fn bidding_does_not_affect_count() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // 3
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            4, // 4
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(auction_count(&instance).await, 3);

        bid(&instance, 2, 400).await;

        assert_eq!(auction_count(&instance).await, 3);
    }
}
