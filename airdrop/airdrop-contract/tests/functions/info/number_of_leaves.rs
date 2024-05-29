mod success {

    use crate::utils::{
        interface::{core::airdrop_constructor, info::number_of_leaves},
        setup::{defaults, setup},
    };
    use fuels::types::Bits256;

    #[tokio::test]
    async fn returns_end_block() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset_id) = setup().await;
        let (_, _, _, minter, _, leaf_count, asset_supply, _, claim_time, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = Bits256([2u8; 32]);

        assert_eq!(
            0,
            number_of_leaves(&deploy_wallet.airdrop_distributor).await,
        );

        airdrop_constructor(
            minter,
            asset_supply / 2,
            asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            leaf_count,
        )
        .await;

        assert_eq!(
            leaf_count,
            number_of_leaves(&deploy_wallet.airdrop_distributor).await,
        );
    }
}
