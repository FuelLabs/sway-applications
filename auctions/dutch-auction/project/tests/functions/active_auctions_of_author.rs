mod passing {
    use crate::utils::{
        active_auctions_of_author, bid, cancel_auction, create_auction, get_contract_instance,
    };
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    async fn can_get_active_auctions_of_author() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and bid on the second one
        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 2 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 3 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        bid(&instance, 2, 400).await;

        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1, 3]
        );
    }

    #[tokio::test]
    async fn can_get_active_auctions_of_author_with_no_active_auctions() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and bid on all of them
        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 2 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 3 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        bid(&instance, 1, 400).await;
        bid(&instance, 2, 400).await;
        bid(&instance, 3, 400).await;

        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            Vec::<u64>::new()
        );
    }

    #[tokio::test]
    async fn can_get_active_auctions_of_author_with_no_auctions() {
        let (instance, wallet) = get_contract_instance().await;

        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            Vec::<u64>::new()
        );
    }

    #[tokio::test]
    async fn active_auctions_updates_on_cancelling() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and cancel the second one
        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 2 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        create_auction(
            &instance,
            400,
            100,
            5, // Block height will be 1 at start, becomes 3 during sending of this tx
            8,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        cancel_auction(&instance, 2).await;

        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1, 3]
        );
    }
}
