use crate::utils::{
    abi_calls::{create_campaign, user_campaign_count},
    test_helpers::setup,
    Identity,
};
use fuels::signers::Signer;

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (author, _, _, _, _) = setup().await;

        assert_eq!(
            0,
            user_campaign_count(&author.contract, Identity::Address(author.wallet.address())).await
        );
    }

    #[tokio::test]
    async fn returns_one() {
        let (author, _, _, _, defaults) = setup().await;

        assert_eq!(
            0,
            user_campaign_count(&author.contract, Identity::Address(author.wallet.address())).await
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
            user_campaign_count(&author.contract, Identity::Address(author.wallet.address())).await
        );
    }
}
