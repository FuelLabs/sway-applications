use crate::utils::{
    interface::info::pledged,
    setup::{identity, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::core::{create_campaign, pledge},
        setup::mint,
    };

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
            .value;
        assert_eq!(1, info.id);
        assert_eq!(defaults.target_amount, info.amount);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_zero() {
        let (_, user, _, _, _) = setup().await;

        // Reverts
        pledged(&user.contract, 0, identity(user.wallet.address()).await).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidID")]
    async fn when_id_is_greater_than_number_of_pledges() {
        let (_, user, _, _, _) = setup().await;

        // Reverts
        pledged(&user.contract, 1, identity(user.wallet.address()).await).await;
    }
}
