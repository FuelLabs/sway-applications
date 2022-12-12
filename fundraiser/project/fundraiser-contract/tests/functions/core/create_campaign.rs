use crate::utils::{interface::core::create_campaign, setup::setup};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{
            asset_info_by_count, campaign, campaign_info, total_campaigns, user_campaign_count,
        },
        setup::{identity, CreatedCampaignEvent, State},
    };

    #[tokio::test]
    async fn creates_a_campaign() {
        let (author, _, _, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(false, asset_info.value.exists);
        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(
            0,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );

        let response = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let log = response
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        let info = campaign_info(&author.contract, 1).await.value;
        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(
            *event,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info.clone(),
                id: 1
            }
        );
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);

        assert_eq!(1, total_campaigns(&author.contract).await);
        assert_eq!(
            1,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(
            1,
            campaign(&author.contract, 1, identity(author.wallet.address()).await)
                .await
                .value
                .id
        );
        assert_eq!(info.asset, defaults.asset_id);
        assert_eq!(info.author, identity(author.wallet.address()).await);
        assert_eq!(info.beneficiary, defaults.beneficiary);
        assert!(matches!(info.state, State::Funding()));
        assert_eq!(info.deadline, defaults.deadline);
        assert_eq!(info.target_amount, defaults.target_amount);
        assert_eq!(info.total_pledge, 0);
    }

    #[tokio::test]
    async fn creates_two_campaigns_with_the_same_asset() {
        let (author, _, _, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(false, asset_info.value.exists);
        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(
            0,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );

        let response_one = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let resposne_two = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;
        let info_one = campaign_info(&author.contract, 1).await.value;
        let info_two = campaign_info(&author.contract, 2).await.value;

        let log_one = response_one
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let log_two = resposne_two
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let event_one = log_one.get(0).unwrap();
        let event_two = log_two.get(0).unwrap();

        assert_eq!(
            *event_one,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info_one.clone(),
                id: 1
            }
        );
        assert_eq!(
            *event_two,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info_two.clone(),
                id: 2
            }
        );

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(false, asset_info2.value.exists);
        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(defaults.target_amount, info_one.target_amount);
        assert_eq!(defaults.target_amount * 2, info_two.target_amount);
    }

    #[tokio::test]
    async fn creates_two_campaigns_with_different_assets() {
        let (author, _, _, asset2, defaults) = setup().await;
        let asset_id = asset2.id;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(false, asset_info1.value.exists);
        assert_eq!(false, asset_info2.value.exists);
        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(
            0,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );

        let response_one = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let resposne_two = create_campaign(
            &author.contract,
            &asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;
        let info_one = campaign_info(&author.contract, 1).await.value;
        let info_two = campaign_info(&author.contract, 2).await.value;

        let log_one = response_one
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let log_two = resposne_two
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let event_one = log_one.get(0).unwrap();
        let event_two = log_two.get(0).unwrap();

        assert_eq!(
            *event_one,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info_one.clone(),
                id: 1
            }
        );
        assert_eq!(
            *event_two,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info_two.clone(),
                id: 2
            }
        );

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(true, asset_info2.value.exists);
        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(defaults.target_amount, info_one.target_amount);
        assert_eq!(defaults.target_amount * 2, info_two.target_amount);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "DeadlineMustBeInTheFuture")]
    async fn when_deadline_is_in_the_past() {
        let (author, _, _, _, defaults) = setup().await;
        let deadline = 0;

        // Reverts
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "TargetAmountCannotBeZero")]
    async fn when_target_amount_is_zero() {
        let (author, _, _, _, defaults) = setup().await;
        let target_amount = 0;

        // Reverts
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            target_amount,
        )
        .await;
    }
}
