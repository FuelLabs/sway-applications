use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim, claim_data},
    airdropdistributor_mod::Identity as AirdropIdentity,
    simple_asset_abi_calls::asset_constructor,
    simpleasset_mod::Identity as AssetIdentity,
    test_helpers::{build_tree, build_tree_manual, setup},
};

mod success {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn returns_claim_data() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset, claim_time) = setup().await;

        let identity_a = AirdropIdentity::Address(wallet1.wallet.address().into());
        let identity_b = AirdropIdentity::Address(wallet2.wallet.address().into());
        let identity_c = AirdropIdentity::Address(wallet3.wallet.address().into());
        let minter = AssetIdentity::ContractId(deploy_wallet.contract_id);
        let key = 0;
        let num_leaves = 3;
        let asset_supply = 10;
        let airdrop_leaves = [
            &(identity_a.clone(), 1),
            &(identity_b.clone(), 2),
            &(identity_c.clone(), 3),
        ];
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
            1,
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
            1
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset, claim_time) = setup().await;

        let identity_a = AirdropIdentity::Address(wallet1.wallet.address().into());
        let identity_b = AirdropIdentity::Address(wallet2.wallet.address().into());
        let identity_c = AirdropIdentity::Address(wallet3.wallet.address().into());
        let minter = AssetIdentity::ContractId(deploy_wallet.contract_id);
        let key = 0;
        let num_leaves = 3;
        let asset_supply = 10;
        let airdrop_leaves: [(AirdropIdentity, u64); 3] = [
            (identity_a.clone(), 1),
            (identity_b.clone(), 2),
            (identity_c.clone(), 3),
        ];
        let (root, proof1, proof2) = build_tree_manual(airdrop_leaves).await;

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
            1,
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
            1
        );
    }
}
