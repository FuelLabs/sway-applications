use crate::utils::{
    abi_calls::{
        asset_info_by_count, campaign, campaign_info, create_campaign, total_campaigns,
        user_campaign_count,
    },
    test_helpers::{identity, setup},
};

mod success {

    use super::*;

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

        create_campaign(
            &author.contract,
            &defaults.asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount,
        )
        .await;
        let info = campaign_info(&author.contract, 1).await.value;
        let asset_info = asset_info_by_count(&author.contract, 1).await;
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
        assert_eq!(info.cancelled, false);
        assert_eq!(info.claimed, false);
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
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(false, asset_info2.value.exists);

        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.target_amount
        );
        assert_eq!(
            defaults.target_amount * 2,
            campaign_info(&author.contract, 2).await.value.target_amount
        );
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
            &asset_id,
            &defaults.beneficiary,
            defaults.deadline,
            defaults.target_amount * 2,
        )
        .await;

        let asset_info1 = asset_info_by_count(&author.contract, 1).await;
        let asset_info2 = asset_info_by_count(&author.contract, 2).await;

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(true, asset_info2.value.exists);

        assert_eq!(2, total_campaigns(&author.contract).await);
        assert_eq!(
            2,
            user_campaign_count(&author.contract, identity(author.wallet.address()).await).await
        );
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.target_amount
        );
        assert_eq!(
            defaults.target_amount * 2,
            campaign_info(&author.contract, 2).await.value.target_amount
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
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
    #[should_panic(expected = "Revert(18446744073709486080)")]
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
