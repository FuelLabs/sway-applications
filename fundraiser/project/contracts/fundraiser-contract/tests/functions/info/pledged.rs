mod success {

    use crate::utils::{
        interface::{
            core::{create_campaign, pledge},
            info::pledged,
        },
        setup::{identity, mint, setup, Pledge},
    };

    #[tokio::test]
    async fn returns_none() {
        let (_, user, _, _, _) = setup().await;

        let info = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value;
        assert!(matches!(info, Option::<Pledge>::None));
    }

    #[tokio::test]
    async fn returns_info() {
        let (author, user, asset, _, defaults) = setup().await;
        let provider = author.wallet.get_provider().unwrap();
        let deadline = provider.latest_block_height().await.unwrap() + 4;

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

        let info = pledged(&user.contract, 1, identity(user.wallet.address()).await)
            .await
            .value
            .unwrap();
        assert_eq!(1, info.campaign_id);
        assert_eq!(defaults.target_amount, info.amount);
    }
}
