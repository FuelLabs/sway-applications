use crate::utils::{
    interface::core::{cancel_campaign, claim_pledges, create_campaign, pledge},
    setup::{mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::campaign_info,
        setup::{identity, CampaignState, ClaimedEvent},
    };
    use fuels::tx::AssetId;

    #[tokio::test]
    async fn claims() {
        let (author, user, asset, _, defaults) = setup().await;
        let beneficiary = identity(author.wallet.address()).await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 4;

        mint(
            &asset.contract,
            defaults.target_amount,
            user.wallet.address(),
        )
        .await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        assert_eq!(
            0,
            author
                .wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response = claim_pledges(&author.contract, 1).await;
        let log = response.get_logs_with_type::<ClaimedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, ClaimedEvent { campaign_id: 1 });
        assert_eq!(
            defaults.target_amount,
            author
                .wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
        assert!(matches!(
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Claimed()
        ));
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_zero() {
        let (author, _, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        claim_pledges(&author.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
        let (author, _, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        claim_pledges(&author.contract, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UnauthorizedUser")]
    async fn when_sender_is_not_author() {
        let (author, user, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        claim_pledges(&user.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DeadlineNotReached")]
    async fn when_claiming_before_deadline() {
        let (author, user, asset, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 7;

        mint(
            &asset.contract,
            defaults.target_amount,
            user.wallet.address(),
        )
        .await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        // Reverts
        claim_pledges(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TargetNotReached")]
    async fn when_target_amount_is_not_reached() {
        let (author, _, _, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 2;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        claim_pledges(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyClaimed")]
    async fn when_claiming_more_than_once() {
        let (author, user, asset, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 4;

        mint(
            &asset.contract,
            defaults.target_amount,
            user.wallet.address(),
        )
        .await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        claim_pledges(&author.contract, 1).await;

        // Reverts
        claim_pledges(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignHasBeenCancelled")]
    async fn when_cancelled() {
        let (author, user, asset, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 5;

        mint(
            &asset.contract,
            defaults.target_amount,
            user.wallet.address(),
        )
        .await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        cancel_campaign(&author.contract, 1).await;

        // Reverts
        claim_pledges(&author.contract, 1).await;
    }
}
