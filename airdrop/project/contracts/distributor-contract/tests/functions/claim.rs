use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim, claim_data},
    simple_asset_abi_calls::asset_constructor,
    test_helpers::{build_tree, build_tree_manual, defaults, leaves_with_depth, setup},
};
use fuels::{tx::AssetId, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn claims() {
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

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            0
        );
        assert!(
            !claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            identity_a.clone(),
        )
        .await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            airdrop_leaves[key as usize].1
        );
        assert!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, depth) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_leaf, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            0
        );
        assert!(
            !claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            airdrop_leaves[key as usize].1
        );
        assert!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );
    }

    #[tokio::test]
    async fn claims_manual_tree_2_depth() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, identity_b, identity_c, minter, key, _, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let identity_vec: Vec<Identity> =
            vec![identity_a.clone(), identity_b.clone(), identity_c.clone()];

        let depth = 2;
        let airdrop_leaves = leaves_with_depth(depth, identity_vec.clone()).await;
        let num_leaves = airdrop_leaves.len().try_into().unwrap();
        let (_leaf, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            0
        );
        assert!(
            !claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            airdrop_leaves[key as usize].1
        );
        assert!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );
    }

    #[tokio::test]
    async fn claims_manual_tree_16_depth() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, identity_b, identity_c, minter, key, _, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let identity_vec: Vec<Identity> =
            vec![identity_a.clone(), identity_b.clone(), identity_c.clone()];

        let depth = 16;
        let airdrop_leaves = leaves_with_depth(depth, identity_vec.clone()).await;
        let num_leaves = airdrop_leaves.len().try_into().unwrap();
        let (_leaf, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            0
        );
        assert!(
            !claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            airdrop_leaves[key as usize].1
        );
        assert!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ClaimPeriodHasEnded")]
    async fn after_claim_period() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        airdrop_constructor(asset.asset_id, 1, &deploy_wallet.airdrop_distributor, root).await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserAlreadyClaimed")]
    async fn when_claim_twice() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserAlreadyClaimed")]
    async fn when_claim_twice_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, depth) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            0
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        assert_eq!(
            wallet1
                .wallet
                .get_asset_balance(&AssetId::new(*asset.asset_id))
                .await
                .unwrap(),
            airdrop_leaves[key as usize].1
        );

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "MerkleProofFailed")]
    async fn when_failed_merkle_verification() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        let false_claim_quantity = 2;
        claim(
            false_claim_quantity,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "MerkleProofFailed")]
    async fn when_failed_merkle_verification_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, depth) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        let false_claim_quantity = 2;
        claim(
            false_claim_quantity,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ClaimPeriodHasEnded")]
    async fn when_not_initalized() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _minter, key, num_leaves, _, airdrop_leaves, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, _root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        claim(
            airdrop_leaves[key as usize].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }
}
