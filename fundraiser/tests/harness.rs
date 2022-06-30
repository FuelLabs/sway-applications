mod utils;

use fuels::{signers::Signer, tx::AssetId};

use utils::{
    abi_calls::{
        asset_count, asset_info_by_count, asset_info_by_id, campaign, campaign_info,
        cancel_campaign, claim_pledges, create_campaign, pledge, pledge_count, pledged,
        total_campaigns, unpledge, user_campaign_count,
    },
    test_helpers::{mint, setup},
    Identity,
};

// TODO: Until the SDK supports block manipulation changing tests may break them because of the
//       specifically selected block deadlines so your test might be correct but the deadline is
//       messing up the test
//
//  - claim_pledges
//      - revert_when_claiming_before_deadline (need SDK to manipulate block height)
//  - pledges
//      - revert_when_pledging_after_deadline (need SDK to manipulate block height)
//
//      When logging is deserialized in the SDK, check logs are correct

mod create_campaign {

    use super::*;

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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
            );
            assert_eq!(
                1,
                campaign(
                    &author.contract,
                    1,
                    Identity::Address(author.wallet.address())
                )
                .await
                .value
                .id
            );
            assert_eq!(info.asset, defaults.asset_id);
            assert_eq!(info.author, Identity::Address(author.wallet.address()));
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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
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
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
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
        #[should_panic(expected = "Revert(42)")]
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
        #[should_panic(expected = "Revert(42)")]
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
}

mod cancel_campaign {

    use super::*;

    mod success {

        use super::*;

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

            assert_eq!(
                false,
                campaign_info(&author.contract, 1).await.value.cancelled
            );

            cancel_campaign(&author.contract, 1).await;

            assert_eq!(
                true,
                campaign_info(&author.contract, 1).await.value.cancelled
            );
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

            assert_eq!(
                false,
                campaign_info(&author.contract, 1).await.value.cancelled
            );

            assert_eq!(
                false,
                campaign_info(&author.contract, 2).await.value.cancelled
            );

            cancel_campaign(&author.contract, 1).await;

            assert_eq!(
                true,
                campaign_info(&author.contract, 1).await.value.cancelled
            );

            cancel_campaign(&author.contract, 2).await;

            assert_eq!(
                true,
                campaign_info(&author.contract, 2).await.value.cancelled
            );
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
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
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_greater_than_number_of_campaigns() {
            let (author, _, _, _, _) = setup().await;

            // Reverts
            cancel_campaign(&author.contract, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_sender_is_not_author() {
            let (author, user, _, _, defaults) = setup().await;
            let deadline = 4;

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
        #[should_panic(expected = "Revert(42)")]
        async fn when_calling_after_deadline() {
            let (author, _, _, _, defaults) = setup().await;
            let deadline = 3;

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
        #[should_panic(expected = "Revert(42)")]
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
}

mod claim_pledges {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn claims() {
            let (author, user, asset, _, defaults) = setup().await;
            let beneficiary = Identity::Address(author.wallet.address());
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

            claim_pledges(&author.contract, 1).await;

            assert_eq!(
                defaults.target_amount,
                author
                    .wallet
                    .get_asset_balance(&AssetId::from(*asset.id))
                    .await
                    .unwrap()
            );
            assert_eq!(campaign_info(&author.contract, 1).await.value.claimed, true);
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
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
        #[should_panic(expected = "Revert(42)")]
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
        #[should_panic(expected = "Revert(42)")]
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

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn when_claiming_before_deadline() {
        //     let (author, user, asset, _, defaults) = setup().await;
        //     let deadline = 5;

        //     mint(&asset.contract, defaults.target_amount, user.wallet.address()).await;
        //     create_campaign(
        //         &author.contract,
        //         &defaults.asset_id,
        //         &defaults.beneficiary,
        //         deadline,
        //         defaults.target_amount,
        //     )
        //     .await;
        //     pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        //     // TODO: shift block height to be before deadline

        //     // Reverts
        //     claim_pledges(&author.contract, 1).await;
        // }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_target_amount_is_not_reached() {
            let (author, _, _, _, defaults) = setup().await;
            let deadline = 1;

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
        #[should_panic(expected = "Revert(42)")]
        async fn when_claiming_more_than_once() {
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
            pledge(&user.contract, 1, &asset, defaults.target_amount).await;
            claim_pledges(&author.contract, 1).await;

            // Reverts
            claim_pledges(&author.contract, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_cancelled() {
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
            cancel_campaign(&author.contract, 1).await;

            // Reverts
            claim_pledges(&author.contract, 1).await;
        }
    }
}

mod pledge {

    use super::*;

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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );

            let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );

            let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );

            let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
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
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );
            assert_eq!(
                1,
                pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
                    .await
                    .value
                    .id
            );
            assert_eq!(
                2,
                pledged(&user.contract, 2, Identity::Address(user.wallet.address()))
                    .await
                    .value
                    .id
            );
            assert_eq!(
                defaults.target_amount,
                pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
                    .await
                    .value
                    .amount
            );
            assert_eq!(
                defaults.target_amount,
                pledged(&user.contract, 2, Identity::Address(user.wallet.address()))
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

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn when_pledging_after_deadline() {
        //     let (author, user, asset, _, defaults) = setup().await;
        //     let deadline = 5;

        //     mint(&asset.contract, defaults.target_amount, user.wallet.address()).await;
        //     create_campaign(
        //         &author.contract,
        //         &defaults.asset_id,
        //         &defaults.beneficiary,
        //         deadline,
        //         defaults.target_amount,
        //     )
        //     .await;
        //     pledge(&user.contract, 1, &asset, defaults.target_amount).await;

        //     // TODO: shift block height to be after deadline

        //     // Reverts
        //     pledge(&user.contract, 1, &asset, 0).await;
        // }

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
}

mod unpledge {

    use super::*;

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
}

mod total_campaigns {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero() {
            let (author, _, _, _, _) = setup().await;

            assert_eq!(0, total_campaigns(&author.contract).await);
        }

        #[tokio::test]
        async fn returns_one() {
            let (author, _, _, _, defaults) = setup().await;

            assert_eq!(0, total_campaigns(&author.contract).await);
            create_campaign(
                &author.contract,
                &defaults.asset_id,
                &defaults.beneficiary,
                defaults.deadline,
                defaults.target_amount,
            )
            .await;
            assert_eq!(1, total_campaigns(&author.contract).await);
        }
    }
}

mod campaign_info {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_info() {
            let (author, _, _, _, defaults) = setup().await;

            create_campaign(
                &author.contract,
                &defaults.asset_id,
                &defaults.beneficiary,
                defaults.deadline,
                defaults.target_amount,
            )
            .await;

            let info = campaign_info(&author.contract, 1).await.value;

            assert_eq!(info.asset, defaults.asset_id);
            assert_eq!(info.author, Identity::Address(author.wallet.address()));
            assert_eq!(info.beneficiary, defaults.beneficiary);
            assert_eq!(info.cancelled, false);
            assert_eq!(info.claimed, false);
            assert_eq!(info.deadline, defaults.deadline);
            assert_eq!(info.target_amount, defaults.target_amount);
            assert_eq!(info.total_pledge, 0);
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_zero() {
            let (author, _, _, _, _) = setup().await;

            // Reverts
            campaign_info(&author.contract, 0).await.value;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_greater_than_number_of_campaigns() {
            let (author, _, _, _, _) = setup().await;

            // Reverts
            campaign_info(&author.contract, 1).await;
        }
    }
}

mod user_campaign_count {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero() {
            let (author, _, _, _, _) = setup().await;

            assert_eq!(
                0,
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
            );
        }

        #[tokio::test]
        async fn returns_one() {
            let (author, _, _, _, defaults) = setup().await;

            assert_eq!(
                0,
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
            );
            create_campaign(
                &author.contract,
                &defaults.asset_id,
                &defaults.beneficiary,
                defaults.deadline,
                defaults.target_amount,
            )
            .await;
            assert_eq!(
                1,
                user_campaign_count(&author.contract, Identity::Address(author.wallet.address()))
                    .await
            );
        }
    }
}

mod campaign {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_info() {
            let (author, _, _, _, defaults) = setup().await;
            let deadline = 6;

            create_campaign(
                &author.contract,
                &defaults.asset_id,
                &defaults.beneficiary,
                deadline,
                defaults.target_amount,
            )
            .await;

            assert_eq!(
                1,
                campaign(
                    &author.contract,
                    1,
                    Identity::Address(author.wallet.address())
                )
                .await
                .value
                .id
            );
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_zero() {
            let (author, _, _, _, _) = setup().await;

            // Reverts
            campaign(
                &author.contract,
                0,
                Identity::Address(author.wallet.address()),
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_greater_than_number_of_campaigns() {
            let (author, _, _, _, _) = setup().await;

            // Reverts
            campaign(
                &author.contract,
                1,
                Identity::Address(author.wallet.address()),
            )
            .await;
        }
    }
}

mod pledge_count {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero() {
            let (_, user, _, _, _) = setup().await;

            assert_eq!(
                0,
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );
        }

        #[tokio::test]
        async fn returns_one() {
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

            pledge(&user.contract, 1, &asset, defaults.target_amount).await;
            assert_eq!(
                1,
                pledge_count(&user.contract, Identity::Address(user.wallet.address())).await
            );
        }
    }
}

mod pledged {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_info() {
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

            let info = pledged(&user.contract, 1, Identity::Address(user.wallet.address()))
                .await
                .value;
            assert_eq!(1, info.id);
            assert_eq!(defaults.target_amount, info.amount);
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_zero() {
            let (_, user, _, _, _) = setup().await;

            // Reverts
            pledged(&user.contract, 0, Identity::Address(user.wallet.address())).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn when_id_is_greater_than_number_of_pledges() {
            let (_, user, _, _, _) = setup().await;

            // Reverts
            pledged(&user.contract, 1, Identity::Address(user.wallet.address())).await;
        }
    }
}

mod asset_count {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero() {
            let (author, _, _, _, _) = setup().await;

            assert_eq!(0, asset_count(&author.contract).await);
        }

        #[tokio::test]
        async fn returns_one() {
            let (author, _, _, _, defaults) = setup().await;

            assert_eq!(0, asset_count(&author.contract).await);
            create_campaign(
                &author.contract,
                &defaults.asset_id,
                &defaults.beneficiary,
                defaults.deadline,
                defaults.target_amount,
            )
            .await;
            assert_eq!(1, asset_count(&author.contract).await);
        }
    }
}

mod asset_info_by_id {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_asset_does_not_exist_info() {
            let (author, _, _, _, defaults) = setup().await;

            let asset_info = asset_info_by_id(&author.contract, &defaults.asset_id).await;
            assert_eq!(0, asset_info.value.amount);
            assert_eq!(false, asset_info.value.exists);
        }

        #[tokio::test]
        async fn returns_asset_info() {
            let (author, user, asset, _, defaults) = setup().await;

            let asset_info = asset_info_by_id(&author.contract, &defaults.asset_id).await;
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
            pledge(&user.contract, 1, &asset, defaults.target_amount).await;

            let asset_info = asset_info_by_id(&author.contract, &defaults.asset_id).await;
            assert_eq!(defaults.target_amount, asset_info.value.amount);
            assert_eq!(true, asset_info.value.exists);
        }
    }
}

mod asset_info_by_count {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_asset_does_not_exist_info() {
            let (author, _, _, _, _) = setup().await;

            let asset_info = asset_info_by_count(&author.contract, 1).await;
            assert_eq!(0, asset_info.value.amount);
            assert_eq!(false, asset_info.value.exists);
        }

        #[tokio::test]
        async fn returns_asset_info() {
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
            pledge(&user.contract, 1, &asset, defaults.target_amount).await;

            let asset_info = asset_info_by_count(&author.contract, 1).await;
            assert_eq!(defaults.target_amount, asset_info.value.amount);
            assert_eq!(true, asset_info.value.exists);
        }
    }
}
