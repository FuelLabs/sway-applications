mod success {

    use crate::utils::{
        interface::{core::create_campaign, info::campaign_info},
        setup::{identity, setup, CampaignInfo, CampaignState},
    };

    #[tokio::test]
    async fn returns_none() {
        let (_, user, _, _, _) = setup().await;

        let info = campaign_info(&user.contract, 1).await.value;
        assert!(matches!(info, Option::<CampaignInfo>::None));
    }

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

        let info = campaign_info(&author.contract, 1).await.value.unwrap();

        assert_eq!(info.asset, defaults.asset_id);
        assert_eq!(info.author, identity(author.wallet.address()).await);
        assert_eq!(info.beneficiary, defaults.beneficiary);
        assert!(matches!(info.state, CampaignState::Funding()));
        assert_eq!(info.deadline, defaults.deadline);
        assert_eq!(info.target_amount, defaults.target_amount);
        assert_eq!(info.total_pledge, 0);
    }
}
