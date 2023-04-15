pub mod passing {
    use fuels::{types::{Identity, ContractId, AssetId}, signers::WalletUnlocked};
    use crate::utils::{create_auction, get_contract_instance, bid, auctions_won};

    #[tokio::test]
    async fn can_bid() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),            
        ).await;

        bid(&instance, 1, 400).await;

        assert_eq!(auctions_won(&instance, Identity::Address(wallet.address().into())).await, vec![1]);
    }

    #[tokio::test]
    async fn beneficiary_recieves_proceeds() {
        let (instance, wallet) = get_contract_instance().await;

        let wallet_balance_before = wallet.get_asset_balance(&AssetId::BASE).await.unwrap();

        // create a new wallet which will be the beneficiary
        let beneficiary = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));
        let beneficiary_address = beneficiary.address();

        // make sure the beneficiary has no balance
        assert_eq!(beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(), 0);

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(beneficiary_address.into()),
            ContractId::zeroed(),            
        ).await;

        // Of this 400, 100 should be returned back to wallet, and 300 should be sent to beneficiary
        bid(&instance, 1, 400).await;

        // beneficiary should have 300
        assert_eq!(beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(), 300);

        // wallet should have 300 less (400 - 100)
        assert_eq!(wallet.get_asset_balance(&AssetId::BASE).await.unwrap(), wallet_balance_before - 300);
    }

    #[tokio::test]
    async fn user_recieves_refund_for_excess() {
        let (instance, wallet) = get_contract_instance().await;

        let wallet_balance_before = wallet.get_asset_balance(&AssetId::BASE).await.unwrap();

        // create a new wallet which will be the beneficiary
        let beneficiary = WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));
        let beneficiary_address = beneficiary.address();

        // make sure the beneficiary has no balance
        assert_eq!(beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(), 0);

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(beneficiary_address.into()),
            ContractId::zeroed(),            
        ).await;

        // Of this 400, 100 should be returned back to wallet, and 300 should be sent to beneficiary
        bid(&instance, 1, 400).await;

        // beneficiary should have 300
        assert_eq!(beneficiary.get_asset_balance(&AssetId::BASE).await.unwrap(), 300);

        // wallet should have 300 less (400 - 100)
        assert_eq!(wallet.get_asset_balance(&AssetId::BASE).await.unwrap(), wallet_balance_before - 300);
    }
}

mod failing {
    use fuels::{types::{Identity, ContractId}};
    use crate::utils::{create_auction, get_contract_instance, bid};

    #[tokio::test]
    #[should_panic]
    async fn cannot_bid_on_non_existant_auction() {
        let (instance, _wallet) = get_contract_instance().await;

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cannot_bid_on_auction_that_has_ended() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            3,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),            
        ).await;

        // create blocks to end the auction
        wallet.get_provider().unwrap().produce_blocks(1, None).await.unwrap();

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cannot_bid_on_auction_that_has_not_started() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            4,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),            
        ).await;

        bid(&instance, 1, 400).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cannot_bid_with_insufficient_funds() {
        let (instance, wallet) = get_contract_instance().await;

        create_auction(
            &instance,
            400,
            100,
            2,  // Block height will be 1 at start, then 2 during sending of this tx
            5,
            Identity::Address(wallet.address().into()),
            ContractId::zeroed(),            
        ).await;

        bid(&instance, 1, 100).await;
    }
}