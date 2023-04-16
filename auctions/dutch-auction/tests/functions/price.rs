pub mod passing {
    use crate::utils::{create_auction, get_contract_instance, price};
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    async fn price_is_correct() {
        let (instance, wallet) = get_contract_instance().await;

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

        assert_eq!(price(&instance, 1).await, 300); // Sending a query tx also increases block height

        assert_eq!(price(&instance, 1).await, 200);

        assert_eq!(price(&instance, 1).await, 100);

        assert_eq!(price(&instance, 1).await, 100); // Additional blocks will not affect price since reserve price is reached
    }
}

pub mod failing {
    use crate::utils::{create_auction, get_contract_instance, price};
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    #[should_panic = "InvalidAuctionID"]
    async fn cannot_get_zero_auction_id() {
        let (instance, wallet) = get_contract_instance().await;

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

        assert_eq!(price(&instance, 0).await, 300); // Auction ids start at 1, 0 is invalid
    }

    #[tokio::test]
    #[should_panic = "InvalidAuctionID"]
    async fn cannot_get_out_of_bounds_auction_id() {
        let (instance, wallet) = get_contract_instance().await;

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

        assert_eq!(price(&instance, 2).await, 300); // Auction ids start at 1, so 2 is invalid
    }
}
