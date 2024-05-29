mod success {

    use crate::utils::{
        interface::{core::airdrop_constructor, info::end_block},
        setup::{defaults, setup},
    };
    use fuels::types::Bits256;

    #[tokio::test]
    async fn returns_end_block() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset_id) = setup().await;
        let (_, _, _, minter, _, num_leaves, asset_supply, _, claim_time, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let provider = deploy_wallet.wallet.provider().unwrap();
        let root = Bits256([2u8; 32]);

        assert_eq!(0, end_block(&deploy_wallet.airdrop_distributor).await,);

        airdrop_constructor(
            minter,
            asset_supply / 2,
            asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            provider.latest_block_height().await.unwrap() + claim_time,
            end_block(&deploy_wallet.airdrop_distributor).await,
        );
    }
}
