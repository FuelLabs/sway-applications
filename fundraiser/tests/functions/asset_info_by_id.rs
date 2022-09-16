use crate::utils::{
    abi_calls::{asset_info_by_id, create_campaign, pledge},
    test_helpers::{mint, setup},
};

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
