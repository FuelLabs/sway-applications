use crate::utils::{
    abi_calls::{campaign_info, cancel_campaign, create_campaign},
    test_helpers::setup,
};

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
