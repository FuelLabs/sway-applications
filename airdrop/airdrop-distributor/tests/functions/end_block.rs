use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, end_block},
    test_helpers::{defaults, setup},
};

mod success {

    use fuels::prelude::Bits256;

    use super::*;

    #[tokio::test]
    async fn returns_end_block() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let provider = deploy_wallet.wallet.get_provider().unwrap();
        let root = Bits256([2u8; 32]);

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;

        assert_eq!(
            provider.latest_block_height().await.unwrap() + claim_time,
            end_block(&deploy_wallet.airdrop_distributor).await,
        );
    }
}
