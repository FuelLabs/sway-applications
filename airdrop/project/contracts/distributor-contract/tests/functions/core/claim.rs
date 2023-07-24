use crate::utils::{
    interface::core::{airdrop_constructor, asset_constructor, claim, mint_to},
    setup::{build_tree, build_tree_manual, defaults, get_wallet_balance, setup},
};
use fuels::prelude::AssetId;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::claim_data, setup::leaves_with_depth, setup::ClaimEvent, setup::ClaimState,
    };
    use fuels::types::Identity;

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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone()).await,
            ClaimState::Unclaimed
        );

        let response = claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_a.clone(),
        )
        .await;

        let log = response.decode_logs_with_type::<ClaimEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ClaimEvent {
                to: identity_a.clone(),
                claimer: identity_a.clone(),
                amount: airdrop_leaves[key as usize].1
            }
        );
        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone()).await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
    }

    #[tokio::test]
    async fn claims_to_different_wallet() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (
            identity_a,
            identity_b,
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            get_wallet_balance(&wallet2.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone()).await,
            ClaimState::Unclaimed
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_b.clone()).await,
            ClaimState::Unclaimed
        );

        let response = claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            identity_b.clone(),
        )
        .await;

        let log = response.decode_logs_with_type::<ClaimEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ClaimEvent {
                to: identity_b.clone(),
                claimer: identity_a.clone(),
                amount: airdrop_leaves[key as usize].1
            }
        );
        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            get_wallet_balance(&wallet2.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone()).await,
            ClaimState::Claimed(airdrop_leaves[key as usize].1)
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_b.clone()).await,
            ClaimState::Unclaimed
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, depth) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_leaf, proof, root) = build_tree_manual(airdrop_leaves.clone(), depth, key).await;

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Unclaimed
        );

        let response = claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        let log = response.decode_logs_with_type::<ClaimEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ClaimEvent {
                to: airdrop_leaves[key as usize].0.clone(),
                claimer: airdrop_leaves[key as usize].0.clone(),
                amount: airdrop_leaves[key as usize].1
            }
        );
        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Unclaimed
        );

        let response = claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
        let log = response.decode_logs_with_type::<ClaimEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ClaimEvent {
                to: airdrop_leaves[key as usize].0.clone(),
                claimer: airdrop_leaves[key as usize].0.clone(),
                amount: airdrop_leaves[key as usize].1
            }
        );
        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );
        assert_eq!(
            claim_data(
                &deploy_wallet.airdrop_distributor,
                airdrop_leaves[key as usize].0.clone()
            )
            .await,
            ClaimState::Unclaimed
        );

        let response = claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
        let log = response.decode_logs_with_type::<ClaimEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ClaimEvent {
                to: airdrop_leaves[key as usize].0.clone(),
                claimer: airdrop_leaves[key as usize].0.clone(),
                amount: airdrop_leaves[key as usize].1
            }
        );
        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );
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

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ClaimPeriodNotActive")]
    async fn after_claim_period() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;
        let provider = deploy_wallet.wallet.provider().unwrap();

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        let _ = provider.produce_blocks(claim_time + 1, Option::None).await;

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            0
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;

        assert_eq!(
            get_wallet_balance(&wallet1.wallet, &AssetId::new(*asset.asset_id)).await,
            airdrop_leaves[key as usize].1
        );

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        let false_claim_quantity = 2;
        claim(
            false_claim_quantity,
            &wallet1.airdrop_distributor,
            key,
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

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        let false_claim_quantity = 2;
        claim(
            false_claim_quantity,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ClaimPeriodNotActive")]
    async fn when_not_initalized() {
        let (deploy_wallet, wallet1, wallet2, wallet3, _) = setup().await;
        let (_, _, _, _minter, key, _, _, airdrop_leaves, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, _root, _leaf, proof) = build_tree(key, airdrop_leaves.clone()).await;

        claim(
            airdrop_leaves[key as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[key as usize].0.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotEnoughTokens")]
    async fn when_not_enough_tokens_to_claim() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (_tree, root, _leaf, proof) = build_tree(key + 1, airdrop_leaves.clone()).await;

        asset_constructor(asset_supply, &asset.asset, minter.clone()).await;
        mint_to(asset_supply, &asset.asset, minter.clone()).await;

        airdrop_constructor(
            minter,
            1,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert!(airdrop_leaves[(key + 1) as usize].1 > 1);

        claim(
            airdrop_leaves[(key + 1) as usize].1,
            &wallet1.airdrop_distributor,
            key,
            proof.clone(),
            airdrop_leaves[(key + 1) as usize].0.clone(),
        )
        .await;
    }
}
