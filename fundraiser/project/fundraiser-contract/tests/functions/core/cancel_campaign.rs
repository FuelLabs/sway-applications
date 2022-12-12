use crate::utils::{
    interface::core::{cancel_campaign, create_campaign},
    setup::{setup, CancelledCampaignEvent},
};

mod success {

    use super::*;
    use crate::utils::{interface::info::campaign_info, setup::State};

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
            campaign_info(&author.contract, 1).await.value.state,
            State::Funding()
        ));

        let response = cancel_campaign(&author.contract, 1).await;
        let log = response
            .get_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, CancelledCampaignEvent { id: 1 });
        assert!(matches!(
            campaign_info(&author.contract, 1).await.value.state,
            State::Cancelled()
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
            campaign_info(&author.contract, 1).await.value.state,
            State::Funding()
        ));
        assert!(matches!(
            campaign_info(&author.contract, 2).await.value.state,
            State::Funding()
        ));

        let response_one = cancel_campaign(&author.contract, 1).await;
        let response_two = cancel_campaign(&author.contract, 2).await;

        let log_one = response_one
            .get_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let log_two = response_two
            .get_logs_with_type::<CancelledCampaignEvent>()
            .unwrap();
        let event_one = log_one.get(0).unwrap();
        let event_two = log_two.get(0).unwrap();

        assert_eq!(*event_one, CancelledCampaignEvent { id: 1 });
        assert_eq!(*event_two, CancelledCampaignEvent { id: 2 });

        assert!(matches!(
            campaign_info(&author.contract, 1).await.value.state,
            State::Cancelled()
        ));
        assert!(matches!(
            campaign_info(&author.contract, 2).await.value.state,
            State::Cancelled()
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
        cancel_campaign(&author.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
        let (author, _, _, _, _) = setup().await;

        // Reverts
        cancel_campaign(&author.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UnauthorizedUser")]
    async fn when_sender_is_not_author() {
        let (author, user, _, _, defaults) = setup().await;
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
        cancel_campaign(&user.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignEnded")]
    async fn when_calling_after_deadline() {
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

        // Reverts
        cancel_campaign(&author.contract, 1).await;
    }
}
