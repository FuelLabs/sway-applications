use crate::utils::{
    abi_calls::{asset_count, create_campaign},
    test_helpers::setup,
};

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
