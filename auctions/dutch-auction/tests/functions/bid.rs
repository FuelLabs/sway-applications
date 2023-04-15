pub mod passing {
    use crate::utils::{auctions_won, bid, create_auction, get_contract_instance};
    use fuels::{
        signers::WalletUnlocked,
        types::{AssetId, ContractId, Identity},
    };

    #[tokio::test]
    async fn can_bid() {
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

        bid(&instance, 1, 400).await;

        assert_eq!(
            auctions_won(&instance, Identity::Address(wallet.address().into())).await,
            vec![1]
        );
    }

    #[tokio::test]
    async fn beneficiary_recieves_proceeds() {
        let (instance, wallet) = get_contract_instance().await;

        let wallet_balance_before = wallet.get_asset_balance(&AssetId::BASE).await.unwrap();

        // create a new wallet which will be the beneficiary
        let beneficiary = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));
        let beneficiary_address = beneficiary.address();

        // make sure the beneficiary has no balance
        assert_eq!(
            beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(),
            0
        );

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(beneficiary_address.into()),
            ContractId::zeroed(),
        )
        .await;

        // Of this 400, 100 should be returned back to wallet, and 300 should be sent to beneficiary
        bid(&instance, 1, 400).await;

        // beneficiary should have 300
        assert_eq!(
            beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(),
            300
        );

        // wallet should have 300 less (400 - 100)
        assert_eq!(
            wallet.get_asset_balance(&AssetId::BASE).await.unwrap(),
            wallet_balance_before - 300
        );
    }

    #[tokio::test]
    async fn user_recieves_refund_for_excess() {
        let (instance, wallet) = get_contract_instance().await;

        let wallet_balance_before = wallet.get_asset_balance(&AssetId::BASE).await.unwrap();

        // create a new wallet which will be the beneficiary
        let beneficiary = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));
        let beneficiary_address = beneficiary.address();

        // make sure the beneficiary has no balance
        assert_eq!(
            beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(),
            0
        );

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(beneficiary_address.into()),
            ContractId::zeroed(),
        )
        .await;

        // Of this 400, 100 should be returned back to wallet, and 300 should be sent to beneficiary
        bid(&instance, 1, 400).await;

        // beneficiary should have 300
        assert_eq!(
            beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(),
            300
        );

        // wallet should have 300 less (400 - 100)
        assert_eq!(
            wallet.get_asset_balance(&AssetId::BASE).await.unwrap(),
            wallet_balance_before - 300
        );
    }
}

mod failing {
    use crate::utils::{bid, create_auction, get_contract_instance};
    use fuels::{
        prelude::CallParameters,
        types::{AssetId, ContractId, Identity},
    };

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_bid_on_non_existant_auction() {
        let (instance, _wallet) = get_contract_instance().await;

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_bid_on_zero_auction() {
        let (instance, _wallet) = get_contract_instance().await;

        bid(&instance, 0, 400).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionAlreadyEnded")]
    async fn cannot_bid_on_auction_that_has_been_ended() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            3,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        // create blocks to end the auction
        wallet
            .get_provider()
            .unwrap()
            .produce_blocks(1, None)
            .await
            .unwrap();

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionAlreadyEnded")]
    async fn cannot_bid_on_auction_that_has_ended() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2, // Block height will be 1 at start, then 2 during sending of this tx
            3,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        // create blocks to end the auction
        wallet
            .get_provider()
            .unwrap()
            .produce_blocks(1, None)
            .await
            .unwrap();

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionNotYetStarted")]
    async fn cannot_bid_on_auction_that_has_not_started() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            4, // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),
        )
        .await;

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic(expected = "BidTooLow")]
    async fn cannot_bid_with_insufficient_funds() {
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

        bid(&instance, 1, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "WrongAssetSent")]
    async fn cannot_bid_with_incorrect_asset() {
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

        instance
            .methods()
            .bid(1)
            .call_params(
                CallParameters::default()
                    .set_amount(500)
                    .set_asset_id(AssetId::new([1; 32])),
            )
            .unwrap()
            .call()
            .await
            .unwrap();
    }
}
