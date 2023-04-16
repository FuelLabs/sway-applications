mod passing {
    use crate::utils::{auctions_won, bid, create_auction, get_contract_instance};
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    async fn can_get_auctions_won() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and bid on the second one
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
            4, //4
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        bid(&instance, 2, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![2]
        );
    }

    #[tokio::test]
    async fn can_get_auctions_won_with_no_auctions_won() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and bid on none of them
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

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            Vec::<u64>::new()
        );
    }

    #[tokio::test]
    async fn can_get_auctions_won_with_no_auctions() {
        let (instance, wallet) = get_contract_instance().await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            Vec::<u64>::new()
        );
    }

    #[tokio::test]
    async fn auctions_won_updates_out_of_order() {
        let (instance, wallet) = get_contract_instance().await;

        // Create 3 auctions and bid on the third, then first, then second
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

        bid(&instance, 3, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![3]
        );

        bid(&instance, 1, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![3, 1]
        );

        bid(&instance, 2, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![3, 1, 2]
        );
    }
}
