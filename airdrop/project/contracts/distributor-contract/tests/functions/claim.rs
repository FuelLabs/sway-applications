use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim, claim_data},
    simple_asset_abi_calls::asset_constructor,
    test_helpers::{build_tree, build_tree_manual, defaults, leaves_with_depth, setup},
};
use fuels::{tx::AssetId, types::Identity};

mod success {

    use super::*;

    // NOTE: This test is ignored as it uses the Fuel-Merkle crate. There is currently an
    // incompatability with the Fuel-Merkle crate and the Sway-Libs Merkle Proof library.
    // The issue can be tracked here: https://github.com/FuelLabs/sway/issues/2594
    #[ignore]
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
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            false
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
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            true
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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            false
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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            true
        );
    }

    #[tokio::test]
    async fn claims_manual_tree_2_depth() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, identity_b, identity_c, minter, key, _, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let mut identity_vec: Vec<Identity> = Vec::new();
        identity_vec.push(identity_a.clone());
        identity_vec.push(identity_b.clone());
        identity_vec.push(identity_c.clone());

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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            false
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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            true
        );
    }

    #[tokio::test]
    async fn claims_manual_tree_16_depth() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, identity_b, identity_c, minter, key, _, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let mut identity_vec: Vec<Identity> = Vec::new();
        identity_vec.push(identity_a.clone());
        identity_vec.push(identity_b.clone());
        identity_vec.push(identity_c.clone());

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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            false
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
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await
            .claimed,
            true
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

    // NOTE: This test is ignored as it uses the Fuel-Merkle crate. There is currently an
    // incompatability with the Fuel-Merkle crate and the Sway-Libs Merkle Proof library.
    // The issue can be tracked here: https://github.com/FuelLabs/sway/issues/2594
    #[ignore]
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

    // TODO: This test will be removed and replaced by `panics_when_claim_twice()` when
    // https://github.com/FuelLabs/sway/issues/2594 is resolved
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

    // NOTE: This test is ignored as it uses the Fuel-Merkle crate. There is currently an
    // incompatability with the Fuel-Merkle crate and the Sway-Libs Merkle Proof library.
    // The issue can be tracked here: https://github.com/FuelLabs/sway/issues/2594
    #[ignore]
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

    // TODO: This test will be removed and replaced by `panics_when_failed_merkle_verification()` when
    // https://github.com/FuelLabs/sway/issues/2594 is resolved
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
