use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim},
    airdropdistributor_mod::Identity as AirdropIdentity,
    simple_token_abi_calls::token_constructor,
    simpletoken_mod::Identity as TokenIdentity,
    test_helpers::{build_tree, setup},
};
use fuel_merkle::common::Bytes32;
use fuels::{signers::Signer, tx::AssetId};
use sha2::{Digest, Sha256};

mod success {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn claims() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves = [&(1, *wallet1.wallet.address().hash()), &(2, *wallet2.wallet.address().hash()), &(3, *wallet3.wallet.address().hash())];
        let key = 0;
        let num_leaves = 3;
        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(10, &deploy_wallet.airdrop_distributor, root, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            0
        );

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, proof.clone(), identity.clone(), asset.asset_id).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves: [([u8; 32], u64); 3]= [(*wallet1.wallet.address().hash(), 1), (*wallet2.wallet.address().hash(), 2), (*wallet3.wallet.address().hash(), 3)];
        let key = 0;
        let num_leaves = 3;

        //            ABC
        //           /   \
        //          AB    C
        //         /  \
        //        A    B

        // Leaf A hash
        let leaf_u64: u64 = 0;
        let mut leaf_a = Sha256::new();
        leaf_a.update(&airdrop_leaves[0].0);
        leaf_a.update(&airdrop_leaves[0].1.to_be_bytes());
        let leaf_a: Bytes32 = leaf_a.finalize().try_into().unwrap();

        let mut merkle_leaf_a = Sha256::new();
        merkle_leaf_a.update(leaf_u64.to_be_bytes());
        merkle_leaf_a.update(&leaf_a);
        let leaf_a_hash: Bytes32 = merkle_leaf_a.finalize().try_into().unwrap();

        // Leaf B hash
        let mut leaf_b = Sha256::new();
        leaf_b.update(&airdrop_leaves[1].0);
        leaf_b.update(&airdrop_leaves[1].1.to_be_bytes());
        let leaf_b: Bytes32 = leaf_b.finalize().try_into().unwrap();

        let mut merkle_leaf_b = Sha256::new();
        merkle_leaf_b.update(leaf_u64.to_be_bytes());
        merkle_leaf_b.update(&leaf_b);
        let leaf_b_hash: Bytes32 = merkle_leaf_b.finalize().try_into().unwrap();

        // leaf C hash
        let mut leaf_c = Sha256::new();
        leaf_c.update(&airdrop_leaves[2].0);
        leaf_c.update(&airdrop_leaves[2].1.to_be_bytes());
        let leaf_c: Bytes32 = leaf_c.finalize().try_into().unwrap();

        let mut merkle_leaf_c = Sha256::new();
        merkle_leaf_c.update(leaf_u64.to_be_bytes());
        merkle_leaf_c.update(&leaf_c);
        let leaf_c_hash: Bytes32 = merkle_leaf_c.finalize().try_into().unwrap();

        // Node AB hash
        let node_u64: u64 = 1;
        let mut node_ab = Sha256::new();
        node_ab.update(node_u64.to_be_bytes());
        node_ab.update(&leaf_a_hash);
        node_ab.update(&leaf_b_hash);
        let node_ab_hash: Bytes32 = node_ab.finalize().try_into().unwrap();

        // Root hash
        let mut node_abc = Sha256::new();
        node_abc.update(node_u64.to_be_bytes());
        node_abc.update(&node_ab_hash);
        node_abc.update(&leaf_c_hash);
        let node_abc_hash: Bytes32 = node_abc.finalize().try_into().unwrap();

        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        airdrop_constructor(15, &deploy_wallet.airdrop_distributor, node_abc_hash, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            0
        );

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, [leaf_b_hash, leaf_c_hash].to_vec(), identity.clone(), asset.asset_id).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            1
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_after_claim_period() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves = [&(1, *wallet1.wallet.address().hash()), &(2, *wallet2.wallet.address().hash()), &(3, *wallet3.wallet.address().hash())];
        let key = 0;
        let num_leaves = 3;
        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(1, &deploy_wallet.airdrop_distributor, root, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, proof.clone(), identity.clone(), asset.asset_id).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_claim_twice() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves = [&(1, *wallet1.wallet.address().hash()), &(2, *wallet2.wallet.address().hash()), &(3, *wallet3.wallet.address().hash())];
        let key = 0;
        let num_leaves = 3;
        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(10, &deploy_wallet.airdrop_distributor, root, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, proof.clone(), identity.clone(), asset.asset_id).await;
        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, proof.clone(), identity.clone(), asset.asset_id).await;
    }

    // TODO: This test will be removed and replaced by `panics_when_claim_twice()` when 
    // https://github.com/FuelLabs/sway/issues/2594 is resolved
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_claim_twice_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves: [([u8; 32], u64); 3]= [(*wallet1.wallet.address().hash(), 1), (*wallet2.wallet.address().hash(), 2), (*wallet3.wallet.address().hash(), 3)];
        let key = 0;
        let num_leaves = 3;

        //            ABC
        //           /   \
        //          AB    C
        //         /  \
        //        A    B

        // Leaf A hash
        let leaf_u64: u64 = 0;
        let mut leaf_a = Sha256::new();
        leaf_a.update(&airdrop_leaves[0].0);
        leaf_a.update(&airdrop_leaves[0].1.to_be_bytes());
        let leaf_a: Bytes32 = leaf_a.finalize().try_into().unwrap();

        let mut merkle_leaf_a = Sha256::new();
        merkle_leaf_a.update(leaf_u64.to_be_bytes());
        merkle_leaf_a.update(&leaf_a);
        let leaf_a_hash: Bytes32 = merkle_leaf_a.finalize().try_into().unwrap();

        // Leaf B hash
        let mut leaf_b = Sha256::new();
        leaf_b.update(&airdrop_leaves[1].0);
        leaf_b.update(&airdrop_leaves[1].1.to_be_bytes());
        let leaf_b: Bytes32 = leaf_b.finalize().try_into().unwrap();

        let mut merkle_leaf_b = Sha256::new();
        merkle_leaf_b.update(leaf_u64.to_be_bytes());
        merkle_leaf_b.update(&leaf_b);
        let leaf_b_hash: Bytes32 = merkle_leaf_b.finalize().try_into().unwrap();

        // leaf C hash
        let mut leaf_c = Sha256::new();
        leaf_c.update(&airdrop_leaves[2].0);
        leaf_c.update(&airdrop_leaves[2].1.to_be_bytes());
        let leaf_c: Bytes32 = leaf_c.finalize().try_into().unwrap();

        let mut merkle_leaf_c = Sha256::new();
        merkle_leaf_c.update(leaf_u64.to_be_bytes());
        merkle_leaf_c.update(&leaf_c);
        let leaf_c_hash: Bytes32 = merkle_leaf_c.finalize().try_into().unwrap();

        // Node AB hash
        let node_u64: u64 = 1;
        let mut node_ab = Sha256::new();
        node_ab.update(node_u64.to_be_bytes());
        node_ab.update(&leaf_a_hash);
        node_ab.update(&leaf_b_hash);
        let node_ab_hash: Bytes32 = node_ab.finalize().try_into().unwrap();

        // Root hash
        let mut node_abc = Sha256::new();
        node_abc.update(node_u64.to_be_bytes());
        node_abc.update(&node_ab_hash);
        node_abc.update(&leaf_c_hash);
        let node_abc_hash: Bytes32 = node_abc.finalize().try_into().unwrap();

        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        airdrop_constructor(15, &deploy_wallet.airdrop_distributor, node_abc_hash, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            0
        );

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, [leaf_b_hash, leaf_c_hash].to_vec(), identity.clone(), asset.asset_id).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            1
        );

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, [leaf_b_hash, leaf_c_hash].to_vec(), identity.clone(), asset.asset_id).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_failed_merkle_verification() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves = [&(1, *wallet1.wallet.address().hash()), &(2, *wallet2.wallet.address().hash()), &(3, *wallet3.wallet.address().hash())];
        let key = 0;
        let num_leaves = 3;
        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(1, &deploy_wallet.airdrop_distributor, root, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        claim(2, &deploy_wallet.airdrop_distributor, key, num_leaves, proof.clone(), identity.clone(), asset.asset_id).await;
    }

    // TODO: This test will be removed and replaced by `panics_when_failed_merkle_verification()` when 
    // https://github.com/FuelLabs/sway/issues/2594 is resolved
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_failed_merkle_verification_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;

        let airdrop_leaves: [([u8; 32], u64); 3]= [(*wallet1.wallet.address().hash(), 1), (*wallet2.wallet.address().hash(), 2), (*wallet3.wallet.address().hash(), 3)];
        let key = 0;
        let num_leaves = 3;

        //            ABC
        //           /   \
        //          AB    C
        //         /  \
        //        A    B

        // Leaf A hash
        let leaf_u64: u64 = 0;
        let mut leaf_a = Sha256::new();
        leaf_a.update(&airdrop_leaves[0].0);
        leaf_a.update(&airdrop_leaves[0].1.to_be_bytes());
        let leaf_a: Bytes32 = leaf_a.finalize().try_into().unwrap();

        let mut merkle_leaf_a = Sha256::new();
        merkle_leaf_a.update(leaf_u64.to_be_bytes());
        merkle_leaf_a.update(&leaf_a);
        let leaf_a_hash: Bytes32 = merkle_leaf_a.finalize().try_into().unwrap();

        // Leaf B hash
        let mut leaf_b = Sha256::new();
        leaf_b.update(&airdrop_leaves[1].0);
        leaf_b.update(&airdrop_leaves[1].1.to_be_bytes());
        let leaf_b: Bytes32 = leaf_b.finalize().try_into().unwrap();

        let mut merkle_leaf_b = Sha256::new();
        merkle_leaf_b.update(leaf_u64.to_be_bytes());
        merkle_leaf_b.update(&leaf_b);
        let leaf_b_hash: Bytes32 = merkle_leaf_b.finalize().try_into().unwrap();

        // leaf C hash
        let mut leaf_c = Sha256::new();
        leaf_c.update(&airdrop_leaves[2].0);
        leaf_c.update(&airdrop_leaves[2].1.to_be_bytes());
        let leaf_c: Bytes32 = leaf_c.finalize().try_into().unwrap();

        let mut merkle_leaf_c = Sha256::new();
        merkle_leaf_c.update(leaf_u64.to_be_bytes());
        merkle_leaf_c.update(&leaf_c);
        let leaf_c_hash: Bytes32 = merkle_leaf_c.finalize().try_into().unwrap();

        // Node AB hash
        let node_u64: u64 = 1;
        let mut node_ab = Sha256::new();
        node_ab.update(node_u64.to_be_bytes());
        node_ab.update(&leaf_a_hash);
        node_ab.update(&leaf_b_hash);
        let node_ab_hash: Bytes32 = node_ab.finalize().try_into().unwrap();

        // Root hash
        let mut node_abc = Sha256::new();
        node_abc.update(node_u64.to_be_bytes());
        node_abc.update(&node_ab_hash);
        node_abc.update(&leaf_c_hash);
        let node_abc_hash: Bytes32 = node_abc.finalize().try_into().unwrap();

        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        airdrop_constructor(15, &deploy_wallet.airdrop_distributor, node_abc_hash, asset.asset_id).await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            0
        );

        claim(2, &deploy_wallet.airdrop_distributor, key, num_leaves, [leaf_b_hash, leaf_c_hash].to_vec(), identity.clone(), asset.asset_id).await;
    }
}
