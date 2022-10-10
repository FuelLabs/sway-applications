use crate::utils::{
    abi_calls::{create_campaign, pledge, pledged},
    test_helpers::{identity, mint, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_info() {
        let (author, user, asset, _, defaults) = setup().await;
        let deadline = 7;

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
            .value;
        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount, info.amount);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_id_is_zero() {
        let (_, user, _, _, _) = setup().await;

        // Reverts
        pledged(&user.contract, 0, identity(user.wallet.address()).await).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_id_is_greater_than_number_of_pledges() {
        let (_, user, _, _, _) = setup().await;

        // Reverts
        pledged(&user.contract, 1, identity(user.wallet.address()).await).await;
    }
}
