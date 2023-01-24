mod success {

    use crate::utils::{
        interface::{core::create_campaign, info::campaign},
        setup::{identity, setup, Campaign},
    };

    #[tokio::test]
    async fn returns_none() {
        let (author, _, _, _, _) = setup().await;

        let campaign = campaign(&author.contract, 1, identity(author.wallet.address()).await).await;
        assert!(matches!(campaign.value, Option::<Campaign>::None));
    }

    #[tokio::test]
    async fn returns_info() {
        let (author, _, _, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 3;

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
            campaign(&author.contract, 1, identity(author.wallet.address()).await)
                .await
                .value
                .unwrap()
                .id
        );
    }
}
