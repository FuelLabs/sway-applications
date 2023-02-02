use crate::utils::{
    interface::core::{claim_pledges, create_campaign, pledge, unpledge},
    setup::{mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{asset_info_by_count, campaign_info, pledge_count, pledged},
        setup::{identity, AssetInfo, UnpledgedEvent},
    };
    use fuels::tx::AssetId;

    #[tokio::test]
    async fn unpledges_full_amount() {
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

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
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response = unpledge(&user.contract, 1, defaults.target_amount).await;

        let log = response.get_logs_with_type::<UnpledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            UnpledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(0, asset_info.value.unwrap().amount);
        assert_eq!(
            0,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn unpledging_decrements_previous_amount() {
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

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

        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response = unpledge(&user.contract, 1, defaults.target_amount - 1).await;
        assert_eq!(
            defaults.target_amount - 1,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let log = response.get_logs_with_type::<UnpledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            UnpledgedEvent {
                amount: defaults.target_amount - 1,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(1, asset_info.value.unwrap().amount);

        assert_eq!(
            1,
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
        assert_eq!(1, info.amount);
    }

    #[tokio::test]
    async fn unpledges_from_different_campaigns() {
        let (author, user, asset, asset2, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));

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
            &asset2.id,
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        pledge(&user.contract, 2, &asset2, defaults.target_amount).await;

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

        let info1 = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();
        let info2 = pledged(&user.contract, 2, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();

        assert_eq!(1, info1.campaign_id);
        assert_eq!(2, info2.campaign_id);
        assert_eq!(defaults.target_amount, info1.amount);
        assert_eq!(defaults.target_amount, info2.amount);
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

        let response1 = unpledge(&user.contract, 1, defaults.target_amount).await;
        let response2 = unpledge(&user.contract, 2, defaults.target_amount).await;

        let log1 = response1.get_logs_with_type::<UnpledgedEvent>().unwrap();
        let log2 = response2.get_logs_with_type::<UnpledgedEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            UnpledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );
        assert_eq!(
            *event2,
            UnpledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 2,
                user: identity(user.wallet.address()).await
            }
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

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert_eq!(0, asset_info1.value.unwrap().amount);
        assert_eq!(0, asset_info2.value.unwrap().amount);
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
            2,
            pledge_count(&user.contract, identity(user.wallet.address()).await).await
        );

        let info1 = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();
        let info2 = pledged(&user.contract, 2, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();

        assert_eq!(1, info1.campaign_id);
        assert_eq!(2, info2.campaign_id);
        assert_eq!(0, info1.amount);
        assert_eq!(0, info2.amount);
    }

    #[tokio::test]
    async fn unpledges_total_pledge_when_attempting_to_unpledge_more_than_pledged() {
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

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
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let response = unpledge(&user.contract, 1, defaults.target_amount * 10).await;
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let log = response.get_logs_with_type::<UnpledgedEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            UnpledgedEvent {
                amount: defaults.target_amount,
                campaign_id: 1,
                user: identity(user.wallet.address()).await
            }
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(0, asset_info.value.unwrap().amount);
        assert_eq!(
            0,
            campaign_info(&author.contract, 1)
                .await
                .value
                .unwrap()
                .total_pledge
        );
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_zero() {
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
        unpledge(&user.contract, 0, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
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
        unpledge(&user.contract, 2, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AmountCannotBeZero")]
    async fn when_unpledging_zero_amount() {
        let (author, user, _, _, defaults) = setup().await;
        let target_amount = 0;

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;

        // Reverts
        unpledge(&user.contract, 1, target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyClaimed")]
    async fn after_claimed() {
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
        unpledge(&user.contract, 1, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserHasNotPledged")]
    async fn when_user_has_not_pledged() {
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
        unpledge(&user.contract, 1, defaults.target_amount).await;
    }
}
