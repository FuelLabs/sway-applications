use crate::utils::{
    abi_calls::{
        asset_info_by_count, campaign_info, claim_pledges, create_campaign, pledge, pledge_count,
        pledged, unpledge,
    },
    test_helpers::{mint, setup},
    Identity,
};
use fuels::{signers::Signer, tx::AssetId};

mod success {

    use super::*;

    #[tokio::test]
    async fn unpledges_full_amount() {
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        unpledge(&user.contract, 1, defaults.target_amount).await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(0, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            0,
            campaign_info(&author.contract, 1).await.value.total_pledge
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
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(defaults.target_amount, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);

        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            1,
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );

        let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
            .await
            .value;
        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount, info.amount);

        assert_eq!(
            0,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        unpledge(&user.contract, 1, defaults.target_amount - 1).await;

        assert_eq!(
            defaults.target_amount - 1,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(1, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);

        assert_eq!(
            1,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            1,
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );
        let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
            .await
            .value;
        assert_eq!(1, info.id);
        assert_eq!(1, info.amount);
    }

    #[tokio::test]
    async fn unpledges_from_different_campaigns() {
        let (author, user, asset, asset2, defaults) = setup().await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;
        assert_eq!(0, asset_info.value.amount);
        assert_eq!(false, asset_info.value.exists);

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
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            campaign_info(&author.contract, 2).await.value.total_pledge
        );
        assert_eq!(
            0,
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;
        pledge(&user.contract, 2, &asset2, defaults.target_amount).await;

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
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );

        let info1 = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
            .await
            .value;
        let info2 = pledged(&user.contract, 2, Identity::Address(user.wallet.address()))
            .await
            .value;

        assert_eq!(1, info1.id);
        assert_eq!(2, info2.id);
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

        unpledge(&user.contract, 1, defaults.target_amount).await;
        unpledge(&user.contract, 2, defaults.target_amount).await;

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

        assert_eq!(0, asset_info1.value.amount);
        assert_eq!(0, asset_info2.value.amount);
        assert_eq!(true, asset_info1.value.exists);
        assert_eq!(true, asset_info2.value.exists);
        assert_eq!(
            0,
            campaign_info(&author.contract, 1).await.value.total_pledge
        );
        assert_eq!(
            0,
            campaign_info(&author.contract, 2).await.value.total_pledge
        );
        assert_eq!(
            2,
            pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
        );

        let info1 = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
            .await
            .value;
        let info2 = pledged(&user.contract, 2, Identity::Address(user.wallet.address()))
            .await
            .value;

        assert_eq!(1, info1.id);
        assert_eq!(2, info2.id);
        assert_eq!(0, info1.amount);
        assert_eq!(0, info2.amount);
    }

    #[tokio::test]
    async fn unpledges_total_pledge_when_attempting_to_unpledge_more_than_pledged() {
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

        pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(defaults.target_amount, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            defaults.target_amount,
            campaign_info(&author.contract, 1).await.value.total_pledge
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

        unpledge(&user.contract, 1, defaults.target_amount * 10).await;

        assert_eq!(
            defaults.target_amount,
            user.wallet
                .get_asset_balance(&AssetId::from(*asset.id))
                .await
                .unwrap()
        );

        let asset_info = asset_info_by_count(&author.contract, 1).await;

        assert_eq!(0, asset_info.value.amount);
        assert_eq!(true, asset_info.value.exists);
        assert_eq!(
            0,
            campaign_info(&author.contract, 1).await.value.total_pledge
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
    #[should_panic(expected = "Revert(42)")]
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
    #[should_panic(expected = "Revert(42)")]
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
        unpledge(&user.contract, 1, defaults.target_amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
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
    #[should_panic(expected = "Revert(42)")]
    async fn after_claimed() {
        let (author, user, asset, _, defaults) = setup().await;
        let deadline = 6;

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
    #[should_panic(expected = "Revert(42)")]
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
