mod passing {
    use crate::utils::{
        active_auctions_of_author, cancel_auction, create_auction, get_contract_instance,
    };
    use fuels::types::{ContractId, Identity};

    #[tokio::test]
    async fn can_cancel_auction() {
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

        cancel_auction(&instance, 1).await;
    }

    #[tokio::test]
    async fn cancelling_updates_active_auctions_of_author() {
        let (instance, wallet) = get_contract_instance().await;

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

        // creating another auction to make sure that the first auction is the one that is cancelled
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

        cancel_auction(&instance, 1).await;

        assert_eq!(
            active_auctions_of_author(&instance, Identity::Address(wallet.address().into())).await,
            vec![2]
        );
    }
}

mod failing {
    use crate::utils::{bid, cancel_auction, create_auction, get_contract_instance};
    use fuels::{
        prelude::TxParameters,
        signers::WalletUnlocked,
        types::{AssetId, ContractId, Identity},
    };

    #[tokio::test]
    #[should_panic(expected = "SenderNotAuthor")]
    async fn only_author_can_cancel_auction() {
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

        let random_wallet =
            WalletUnlocked::new_random(Some(wallet.get_provider().unwrap().clone()));

        wallet
            .transfer(
                random_wallet.address(),
                1000,
                AssetId::BASE,
                TxParameters::default(),
            )
            .await
            .unwrap();

        instance
            .with_wallet(random_wallet)
            .unwrap()
            .methods()
            .cancel_auction(1)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionAlreadyEnded")]
    async fn cannot_cancel_ended_auction() {
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

        cancel_auction(&instance, 1).await;
    }
}
