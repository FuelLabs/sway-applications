use crate::utils::{
    abi_calls::{create_campaign, user_campaign_count},
    test_helpers::{identity, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (author, _, _, _, _) = setup().await;

        assert_eq!(
            0,
            user_campaign_count(
                &author.contract,
                identity(author.wallet.address()).await
            )
            .await
        );
    }

    #[tokio::test]
    async fn returns_one() {
        let (author, _, _, _, defaults) = setup().await;

        assert_eq!(
            0,
            user_campaign_count(
                &author.contract,
                identity(author.wallet.address()).await
            )
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
            user_campaign_count(
                &author.contract,
                identity(author.wallet.address()).await
            )
            .await
        );
    }
}
