mod passing {
    use crate::utils::{auction, change_beneficiary, create_auction, get_contract_instance};
    use fuels::types::{ContractId, Identity};

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

        // Change the beneficiary
        change_beneficiary(
            &instance,
            Identity::ContractId(ContractId::from([1; 32])),
            1,
        )
        .await;

        // Check that the auction details have been updated
        let auc = auction(&instance, 1).await;

        // only the beneficiary field should have changed
        assert_eq!(auc.asset_id, ContractId::zeroed());
        assert_eq!(auc.author, Identity::Address(wallet.address().into()));
        assert_eq!(
            auc.beneficiary,
            Identity::ContractId(ContractId::from([1; 32]))
        );
        assert_eq!(auc.ended, false);
        assert_eq!(auc.end_time, 5);
        assert_eq!(auc.opening_price, 400);
        assert_eq!(auc.reserve_price, 100);
        assert_eq!(auc.start_time, 2);
        assert_eq!(auc.winner, None);
    }
}

mod failing {
    use crate::utils::{cancel_auction, change_beneficiary, create_auction, get_contract_instance};
    use fuels::{
        prelude::TxParameters,
        signers::WalletUnlocked,
        types::{AssetId, ContractId, Identity},
    };

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_update_beneficiary_for_invalid_id() {
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

        // Change the beneficiary
        change_beneficiary(
            &instance,
            Identity::ContractId(ContractId::from([1; 32])),
            2,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAuctionID")]
    async fn cannot_update_beneficiary_for_zero_id() {
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

        // Change the asset
        change_beneficiary(
            &instance,
            Identity::ContractId(ContractId::from([1; 32])),
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderNotAuthor")]
    async fn cannot_update_beneficiary_if_not_author() {
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

        // Create a new wallet
        let wallet2 = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));

        // Transfer some funds to the new wallet
        wallet
            .transfer(
                wallet2.address(),
                1000,
                AssetId::BASE,
                TxParameters::default(),
            )
            .await
            .unwrap();

        // Change the asset
        instance
            .with_wallet(wallet2)
            .unwrap()
            .methods()
            .change_beneficiary(Identity::ContractId(ContractId::from([1; 32])), 1)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionAlreadyEnded")]
    async fn cannot_update_beneficiary_if_auction_ended() {
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

        // End the auction
        cancel_auction(&instance, 1).await;

        // Change the asset
        change_beneficiary(
            &instance,
            Identity::ContractId(ContractId::from([1; 32])),
            1,
        )
        .await;
    }
}
