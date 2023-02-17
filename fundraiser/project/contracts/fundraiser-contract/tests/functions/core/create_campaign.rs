use crate::utils::{interface::core::create_campaign, setup::setup};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{
            asset_info_by_count, campaign, campaign_info, total_campaigns, user_campaign_count,
        },
        setup::{identity, AssetInfo, CampaignState, CreatedCampaignEvent},
    };

    #[tokio::test]
    async fn creates_a_campaign() {
        let (author, _, _, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));
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

        let info = campaign_info(&author.contract, 1).await.value.unwrap();
        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(
            *event,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info.clone(),
                campaign_id: 1
            }
        );
        assert_eq!(0, asset_info.value.unwrap().amount);

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
                .unwrap()
                .id
        );
        assert_eq!(info.asset, defaults.asset_id);
        assert_eq!(info.author, identity(author.wallet.address()).await);
        assert_eq!(info.beneficiary, defaults.beneficiary);
        assert!(matches!(info.state, CampaignState::Funding()));
        assert_eq!(info.deadline, defaults.deadline);
        assert_eq!(info.target_amount, defaults.target_amount);
        assert_eq!(info.total_pledge, 0);
    }

    #[tokio::test]
    async fn creates_two_campaigns_with_the_same_asset() {
        let (author, _, _, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));
        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(
            0,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );

        let response1 = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let response2 = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;
        let info1 = campaign_info(&author.contract, 1).await.value.unwrap();
        let info2 = campaign_info(&author.contract, 2).await.value.unwrap();

        let log1 = response1
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let log2 = response2
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info1.clone(),
                campaign_id: 1
            }
        );
        assert_eq!(
            *event2,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info2.clone(),
                campaign_id: 2
            }
        );

        assert_eq!(0, asset_info1.value.unwrap().amount);
        assert!(asset_info2.value.is_none());
        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(defaults.target_amount, info1.target_amount);
        assert_eq!(defaults.target_amount * 2, info2.target_amount);
    }

    #[tokio::test]
    async fn creates_two_campaigns_with_different_assets() {
        let (author, _, _, asset2, defaults) = setup().await;
        let asset_id = asset2.id;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 1).await;

        assert!(matches!(asset_info1.value, Option::<AssetInfo>::None));
        assert!(matches!(asset_info2.value, Option::<AssetInfo>::None));
        assert_eq!(0, total_campaigns(&author.contract).await);
        assert_eq!(
            0,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );

        let response1 = create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let response2 = create_campaign(
            &author.contract,
            &asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;
        let info1 = campaign_info(&author.contract, 1).await.value.unwrap();
        let info2 = campaign_info(&author.contract, 2).await.value.unwrap();

        let log1 = response1
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let log2 = response2
            .get_logs_with_type::<CreatedCampaignEvent>()
            .unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info1.clone(),
                campaign_id: 1
            }
        );
        assert_eq!(
            *event2,
            CreatedCampaignEvent {
                author: identity(author.wallet.address()).await,
                campaign_info: info2.clone(),
                campaign_id: 2
            }
        );

        assert_eq!(0, asset_info1.value.unwrap().amount);
        assert_eq!(0, asset_info2.value.unwrap().amount);

        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(defaults.target_amount, info1.target_amount);
        assert_eq!(defaults.target_amount * 2, info2.target_amount);
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
