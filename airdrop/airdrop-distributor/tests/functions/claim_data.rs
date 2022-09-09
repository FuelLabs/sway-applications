use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim, claim_data},
    simple_asset_abi_calls::asset_constructor,
    test_helpers::{build_tree, build_tree_manual, defaults, setup},
};

mod success {

    use super::*;

    // NOTE: This test is ignored as it uses the Fuel-Merkle crate. There is currently an 
    // incompatability with the Fuel-Merkle crate and the Sway-Libs Merkle Proof library.
    // The issue can be tracked here: https://github.com/FuelLabs/sway/issues/2594
    #[ignore]
    #[tokio::test]
    async fn returns_claim_data() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

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
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            false
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .amount,
            0
        );

        claim(
            airdrop_leaves[0].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            identity_a.clone(),
        )
        .await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            true
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .amount,
            airdrop_leaves[0].1
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (identity_a, _, _, minter, key, num_leaves, asset_supply, airdrop_leaves, claim_time) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;

        let (root, proof1, proof2) = build_tree_manual(airdrop_leaves.clone()).await;

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        asset_constructor(asset_supply, &asset.asset, minter).await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            false
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .amount,
            0
        );

        claim(
            airdrop_leaves[0].1,
            asset.asset_id,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            [proof1, proof2].to_vec(),
            identity_a.clone(),
        )
        .await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .claimed,
            true
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity_a.clone())
                .await
                .amount,
            airdrop_leaves[0].1
        );
    }
}
