mod success {

    use crate::utils::{
        interface::{
            core::{create_campaign, pledge},
            info::asset_info_by_id,
        },
        setup::{setup, AssetInfo},
    };

    #[tokio::test]
    async fn returns_none() {
        let (author, _, _, _, defaults) = setup().await;

        let asset_info = asset_info_by_id(&author.contract, &defaults.asset_id).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));
    }

    #[tokio::test]
    async fn returns_asset_info() {
        let (author, user, asset, _, defaults) = setup().await;

        let asset_info = asset_info_by_id(&author.contract, &defaults.asset_id).await;
        assert!(matches!(asset_info.value, Option::<AssetInfo>::None));

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
        assert_eq!(defaults.target_amount, asset_info.value.unwrap().amount);
    }
}
