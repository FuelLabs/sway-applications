use crate::utils::{
    interface::core::{cancel_campaign, create_campaign, pledge},
    setup::{mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{asset_info_by_count, campaign_info, pledge_count, pledged},
        setup::{identity, AssetInfo, PledgedEvent},
    };
    use fuels::tx::AssetId;

    #[tokio::test]
    async fn pledges() {
        let (author, user, asset, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));

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
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        assert_eq!(
            0,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            0,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response = pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let log = response.get_logs_with_type::<PledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            PledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.unwrap().amount);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            1,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );

        let info = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();

        assert_eq!(1, info.campaign_id);
        assert_eq!(defaults.target_amount, info.amount);
    }

    #[tokio::test]
    async fn pledge_increments_previous_amount() {
        let (author, user, asset, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));

        mint(
            &asset.contract,
            defaults.target_amount * 2,
            user.wallet.address(),
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

        assert_eq!(
            0,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            0,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );
        assert_eq!(
            defaults.target_amount * 2,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response1 = pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let log = response1.get_logs_with_type::<PledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            PledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.unwrap().amount);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            1,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );

        let info = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();

        assert_eq!(1, info.campaign_id);
        assert_eq!(defaults.target_amount, info.amount);

        let response2 = pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let log = response2.get_logs_with_type::<PledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            PledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount * 2, asset_info.value.unwrap().amount);
        assert_eq!(
            defaults.target_amount * 2,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            1,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );

        let info = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();

        assert_eq!(1, info.campaign_id);
        assert_eq!(defaults.target_amount * 2, info.amount);
    }

    #[tokio::test]
    async fn pledges_to_different_campaigns() {
        let (author, user, asset, asset2, defaults) = setup().await;
        let asset2_id = asset2.id;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert!(matches!(asset_info1.value, Option::<AssetInfo>::None));
        assert!(matches!(asset_info2.value, Option::<AssetInfo>::None));

        mint(
            &asset.contract,
            defaults.target_amount,
            user.wallet.address(),
        )
        .await;
        mint(
            &asset2.contract,
            defaults.target_amount,
            user.wallet.address(),
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
        create_campaign(
            &author.contract,
            &asset2_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        assert_eq!(
            0,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            0,
            campaign_info(&author.contract, 2)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            0,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );

        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset2.id))
                .await
                .unwrap()
        );

        let response1 = pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        let response2 = pledge(&user.contract, 2, &asset2, defaults.target_amount).await;

        let log1 = response1.get_logs_with_type::<PledgedEvent>().unwrap();
        let log2 = response2.get_logs_with_type::<PledgedEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            PledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );
        assert_eq!(
            *event2,
            PledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 2,
                user: identity(user.wallet.address()).await
            }
        );

        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset2.id))
                .await
                .unwrap()
        );

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert_eq!(defaults.target_amount, asset_info1.value.unwrap().amount);
        assert_eq!(defaults.target_amount, asset_info2.value.unwrap().amount);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 2)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            2,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );
        assert_eq!(
            1,
            pledged(&user.contract, 1, identity(user.wallet.address()).await)
                .await
                .value
                .unwrap()
                .campaign_id
        );
        assert_eq!(
            2,
            pledged(&user.contract, 2, identity(user.wallet.address()).await)
                .await
                .value
                .unwrap()
                .campaign_id
        );
        assert_eq!(
            defaults.target_amount,
            pledged(&user.contract, 1, identity(user.wallet.address()).await)
                .await
                .value
                .unwrap()
                .amount
        );
        assert_eq!(
            defaults.target_amount,
            pledged(&user.contract, 2, identity(user.wallet.address()).await)
                .await
                .value
                .unwrap()
                .amount
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_zero() {
        let (author, user, asset, _, defaults) = setup().await;

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
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        pledge(&user.contract, 0, &asset, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
        let (author, user, asset, _, defaults) = setup().await;

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
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        pledge(&user.contract, 2, &asset, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignEnded")]
    async fn when_pledging_after_deadline() {
        let (author, user, asset, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 3;

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
        pledge(&user.contract, 1, &asset, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn when_pledging_incorrect_asset() {
        let (author, user, _, asset2, defaults) = setup().await;

        mint(
            &asset2.contract,
            defaults.target_amount,
            user.wallet.address(),
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

        // Reverts
        pledge(&user.contract, 1, &asset2, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn when_pledging_zero_amount() {
        let (author, user, asset, _, defaults) = setup().await;

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
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        pledge(&user.contract, 1, &asset, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CampaignHasBeenCancelled")]
    async fn when_pledging_to_cancelled_campaign() {
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
        cancel_campaign(&author.contract, 1).await;

        // Reverts
        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
    }
}
