mod passing {
    use fuels::types::{ContractId, Identity};

    use crate::utils::{
        auctions_of_author, bid, cancel_auction, create_auction, get_contract_instance,
    };

    #[tokio::test]
    async fn can_get_auctions_of_author() {
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

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await, // 3
            vec![1]
        );

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
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await, // 5
            vec![1, 2]
        );

        create_auction(
            &instance,
            400,
            100,
            6, // 6
            20,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1, 2, 3]
        );
    }

    #[tokio::test]
    async fn can_get_auctions_of_author_with_no_auctions() {
        let (instance, wallet) = get_contract_instance().await;

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            Vec::<u64>::new()
        );
    }

    #[tokio::test]
    async fn cancelling_does_not_affect_auctions_of_author() {
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

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );

        cancel_auction(&instance, 1).await;

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );
    }

    #[tokio::test]
    async fn bidding_does_not_affect_auctions_of_author() {
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

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );

        bid(&instance, 1, 400).await;

        assert_eq!(
            auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );
    }
}
