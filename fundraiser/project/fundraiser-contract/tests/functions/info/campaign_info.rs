use crate::utils::{
    interface::{core::create_campaign, info::campaign_info},
    setup::setup,
};

mod success {

    use super::*;
    use crate::utils::setup::{identity, State};

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

        let info = campaign_info(&author.contract, 1).await.value;

        assert_eq!(info.asset, defaults.asset_id);
        assert_eq!(info.author, identity(author.wallet.address()).await);
        assert_eq!(info.beneficiary, defaults.beneficiary);
        assert_eq!(info.state, State::Funding());
        assert_eq!(info.deadline, defaults.deadline);
        assert_eq!(info.target_amount, defaults.target_amount);
        assert_eq!(info.total_pledge, 0);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_zero() {
        let (author, _, _, _, _) = setup().await;

        // Reverts
        campaign_info(&author.contract, 0).await.value;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_campaigns() {
        let (author, _, _, _, _) = setup().await;

        // Reverts
        campaign_info(&author.contract, 1).await;
    }
}
