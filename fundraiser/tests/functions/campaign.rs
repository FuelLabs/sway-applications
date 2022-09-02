use crate::utils::{
    abi_calls::{campaign, create_campaign},
    test_helpers::{identity, setup},
};

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
                identity(author.wallet.address()).await
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
            identity(author.wallet.address()).await
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
            identity(author.wallet.address()).await
        )
        .await;
    }
}
