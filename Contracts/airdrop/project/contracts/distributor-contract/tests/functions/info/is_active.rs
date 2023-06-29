mod success {
    use crate::utils::{
        interface::{
            core::{airdrop_constructor, asset_constructor, mint_to},
            info::is_active,
        },
        setup::{build_tree, defaults, setup},
    };

    #[tokio::test]
    async fn returns_active_state() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, _) = build_tree(key, airdrop_leaves.clone()).await;
        let provider = deploy_wallet.wallet.provider().unwrap();

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        assert!(!is_active(&deploy_wallet.airdrop_distributor,).await);

        airdrop_constructor(
            minter.clone(),
            asset_supply / 2,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert!(is_active(&deploy_wallet.airdrop_distributor,).await);

        let _ = provider.produce_blocks(claim_time + 1, Option::None).await;

        assert!(!is_active(&deploy_wallet.airdrop_distributor,).await);
    }
}
