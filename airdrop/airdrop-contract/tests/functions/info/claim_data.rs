mod success {
    use crate::utils::{
        interface::{
            core::{airdrop_constructor, claim},
            info::claim_data,
        },
        setup::ClaimState,
        setup::{build_tree, build_tree_manual, defaults, setup},
    };

    #[tokio::test]
    async fn returns_claim_data() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset_id) = setup().await;
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
            _,
        ) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

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
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0
            )
            .await,
            ClaimState::Unclaimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_a,
        )
        .await;

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0
            )
            .await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset_id) = setup().await;
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
            _,
        ) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

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
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0
            )
            .await,
            ClaimState::Unclaimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_a,
        )
        .await;

        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0
            )
            .await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
    }
}
