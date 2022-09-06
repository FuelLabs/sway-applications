use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, end_block},
    test_helpers::setup,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_end_block() {
        let (deploy_wallet, _, _, _, asset, claim_time) = setup().await;
        let provider = deploy_wallet.wallet.get_provider().unwrap();

        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            [2u8; 32],
            asset.asset_id,
        )
        .await;

        assert_eq!(
            end_block(&deploy_wallet.airdrop_distributor).await,
            provider.latest_block_height().await.unwrap() + claim_time - 1
        );
    }
}
