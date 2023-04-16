mod passing {
    use crate::utils::{
        auction, bid, cancel_auction, change_asset, change_beneficiary, create_auction,
        get_contract_instance,
    };
    use fuels::{
        signers::WalletUnlocked,
        types::{ContractId, Identity},
    };

    #[tokio::test]
    async fn can_get_auction_details() {
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

        let auc = auction(&instance, 1).await;

        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);

        // Bid on the auction
        bid(&instance, 1, 300).await;

        // Check that the auction details have been updated
        let auc = auction(&instance, 1).await;

        // only the winner and the ended fields should have changed
        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, true);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, Some(Identity::Address(wallet.address().into())));
    }

    #[tokio::test]
    async fn auction_details_update_on_cancelling() {
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

        let auc = auction(&instance, 1).await;

        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);

        // Cancel the auction
        cancel_auction(&instance, 1).await;

        // Check that the auction details have been updated
        let auc = auction(&instance, 1).await;

        // only the ended field should have changed
        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, true);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);
    }

    #[tokio::test]
    async fn auction_details_update_on_asset_change() {
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

        let auc = auction(&instance, 1).await;

        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);

        // Change the asset
        change_asset(&instance, ContractId::from([1; 32]), 1).await;

        // Check that the auction details have been updated
        let auc = auction(&instance, 1).await;

        // only the asset_id field should have changed
        assert_eq!(auc.asset_id, ContractId::from([1; 32]));
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);
    }

    #[tokio::test]
    async fn auction_details_update_on_beneficiary_change() {
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

        let auc = auction(&instance, 1).await;

        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(wallet.address().into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);

        // Create a new wallet
        let new_wallet = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));
        let addr = new_wallet.address();

        // Change the beneficiary
        change_beneficiary(&instance, Identity::Address(addr.into()), 1).await;

        // Check that the auction details have been updated
        let auc = auction(&instance, 1).await;

        // only the beneficiary field should have changed
        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(auc.beneficiary, Identity::Address(addr.into()));
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);
    }
}

mod failing {
    use crate::utils::{auction, get_contract_instance};

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_get_auction_details_for_non_existent_auction() {
        let (instance, _wallet) = get_contract_instance().await;

        auction(&instance, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_get_auction_details_for_zero_auction_id() {
        let (instance, _wallet) = get_contract_instance().await;

        auction(&instance, 0).await;
    }
}
