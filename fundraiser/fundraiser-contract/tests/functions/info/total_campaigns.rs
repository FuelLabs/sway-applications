mod success {

    use crate::utils::{
        interface::{core::create_campaign, info::total_campaigns},
        setup::setup,
    };

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
