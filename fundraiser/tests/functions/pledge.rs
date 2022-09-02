use crate::utils::{
    abi_calls::{
        asset_info_by_count, campaign_info, cancel_campaign, create_campaign, pledge, pledge_count,
        pledged,
    },
    test_helpers::{identity, mint, setup},
};
use fuels::tx::AssetId;

mod success {

    use super::*;

    #[tokio::test]
    async fn pledges() {
        let (author, user, asset, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(false, asset_info.value.exists);

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
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );
        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            1,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );

        let info = pledged(
            &user.contract,
            1,
            identity(user.wallet.address()).await
        )
        .await
        .value;

        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount, info.amount);
    }

    #[tokio::test]
    async fn pledge_increments_previous_amount() {
        let (author, user, asset, _, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(false, asset_info.value.exists);

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
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );

        assert_eq!(
            defaults.target_amount * 2,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            1,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );

        let info = pledged(
            &user.contract,
            1,
            identity(user.wallet.address()).await
        )
        .await
        .value;

        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount, info.amount);

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount * 2, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            defaults.target_amount * 2,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            1,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );

        let info = pledged(
            &user.contract,
            1,
            identity(user.wallet.address()).await
        )
        .await
        .value;

        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount * 2, info.amount);
    }

    #[tokio::test]
    async fn pledges_to_different_campaigns() {
        let (author, user, asset, asset2, defaults) = setup().await;
        let asset2_id = asset2.id;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(false, asset_info1.value.exists);
        assert_eq!(false, asset_info2.value.exists);

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
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            campaign_info(&author.contract, 2).await.value.total_pledge
        );
        assert_eq!(
            0,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        pledge(&user.contract, 2, &asset2, defaults.target_amount).await;

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

        assert_eq!(defaults.target_amount, asset_info1.value.amount);
        assert_eq!(defaults.target_amount, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(true, asset_info2.value.exists);

        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 2).await.value.total_pledge
        );
        assert_eq!(
            2,
            pledge_count(
                &user.contract,
                identity(user.wallet.address()).await
            )
            .await
        );
        assert_eq!(
            1,
            pledged(
                &user.contract,
                1,
                identity(user.wallet.address()).await
            )
            .await
            .value
            .id
        );
        assert_eq!(
            2,
            pledged(
                &user.contract,
                2,
                identity(user.wallet.address()).await
            )
            .await
            .value
            .id
        );
        assert_eq!(
            defaults.target_amount,
            pledged(
                &user.contract,
                1,
                identity(user.wallet.address()).await
            )
            .await
            .value
            .amount
        );
        assert_eq!(
            defaults.target_amount,
            pledged(
                &user.contract,
                2,
                identity(user.wallet.address()).await
            )
            .await
            .value
            .amount
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
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
    #[should_panic(expected = "Revert(42)")]
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
    #[ignore]
    async fn when_pledging_after_deadline() {
        let (author, user, asset, _, defaults) = setup().await;
        let deadline = 5;

        mint(&asset.contract, defaults.target_amount, user.wallet.address()).await;
        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            deadline,
            defaults.target_amount,
        )
        .await;
        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        // TODO: shift block height to be after deadline

        // Reverts
        pledge(&user.contract, 1, &asset, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
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
    #[should_panic(expected = "Revert(42)")]
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
    #[should_panic(expected = "Revert(42)")]
    async fn when_pledging_to_cancelled_campaign() {
        let (author, user, asset, _, defaults) = setup().await;
        let deadline = 5;

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
