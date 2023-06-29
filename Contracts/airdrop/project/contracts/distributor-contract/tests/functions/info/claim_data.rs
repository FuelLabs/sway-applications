mod success {
    use crate::utils::{
        interface::{
            core::{airdrop_constructor, asset_constructor, claim, mint_to},
            info::claim_data,
        },
        setup::ClaimState,
        setup::{build_tree, build_tree_manual, defaults, setup},
    };

    #[tokio::test]
    async fn returns_claim_data() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (
            identity_a,
            _,
            _,
            minter,
            key,
            num_leaves,
            asset_supply,
            airdrop_leaves,
            claim_time,
            _,
        ) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

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

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Unclaimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_a.clone(),
        )
        .await;

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (
            identity_a,
            _,
            _,
            minter,
            key,
            num_leaves,
            asset_supply,
            airdrop_leaves,
            claim_time,
            depth,
        ) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

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

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Unclaimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_a.clone(),
        )
        .await;

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
    }
}
