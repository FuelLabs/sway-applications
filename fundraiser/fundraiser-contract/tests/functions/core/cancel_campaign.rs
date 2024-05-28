use crate::utils::{
    interface::core::{cancel_campaign, create_campaign},
    setup::setup,
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::campaign_info,
        setup::{CampaignState, CancelledCampaignEvent},
    };

    #[tokio::test]
    async fn cancels() {
        let (author, _, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        assert!(matches!(
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Funding
        ));

        let response = cancel_campaign(&author.contract, 1).await;
        let log = response
            .decode_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, CancelledCampaignEvent { campaign_id: 1 });
        assert!(matches!(
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Cancelled
        ));
    }

    #[tokio::test]
    async fn cancels_different_campaigns() {
        let (author, _, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        assert!(matches!(
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Funding
        ));
        assert!(matches!(
            campaign_info(&author.contract, 2)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Funding
        ));

        let response1 = cancel_campaign(&author.contract, 1).await;
        let response2 = cancel_campaign(&author.contract, 2).await;

        let log1 = response1
            .decode_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(*event1, CancelledCampaignEvent { campaign_id: 1 });
        assert_eq!(*event2, CancelledCampaignEvent { campaign_id: 2 });

        assert!(matches!(
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Cancelled
        ));
        assert!(matches!(
            campaign_info(&author.contract, 2)
                .await
                .value
                .unwrap()
                .state,
            CampaignState::Cancelled
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

        cancel_campaign(&author.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
        let (author, _, _, _, _) = setup().await;

        cancel_campaign(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UnauthorizedUser")]
    async fn when_sender_is_not_author() {
        let (author, user, _, _, defaults) = setup().await;
        let provider = author.wallet.provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 2;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline.into(),
            defaults.target_amount,
        )
        .await;

        cancel_campaign(&user.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignEnded")]
    async fn when_calling_after_deadline() {
        let (author, _, _, _, defaults) = setup().await;
        let provider = author.wallet.provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 2;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline.into(),
            defaults.target_amount,
        )
        .await;

        cancel_campaign(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignHasBeenCancelled")]
    async fn when_calling_after_already_cancelled() {
        let (author, _, _, _, defaults) = setup().await;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        cancel_campaign(&author.contract, 1).await;

        cancel_campaign(&author.contract, 1).await;
    }
}
