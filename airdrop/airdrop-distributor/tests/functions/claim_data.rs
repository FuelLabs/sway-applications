use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim, claim_data},
    airdropdistributor_mod::Identity as AirdropIdentity,
    simple_token_abi_calls::token_constructor,
    simpletoken_mod::Identity as TokenIdentity,
    test_helpers::{build_tree, build_tree_manual, setup},
};

mod success {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn returns_claim_data() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset, claim_time) = setup().await;

        let airdrop_leaves = [
            &(1, *wallet1.wallet.address().hash()),
            &(2, *wallet2.wallet.address().hash()),
            &(3, *wallet3.wallet.address().hash()),
        ];
        let key = 0;
        let num_leaves = 3;
        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        let (_tree, root, _leaf, proof) = build_tree(key, airdrop_leaves.to_vec()).await;

        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            asset.asset_id,
        )
        .await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.claimed,
            false
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.amount,
            0
        );

        claim(
            1,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            proof.clone(),
            identity.clone(),
            asset.asset_id,
        )
        .await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.claimed,
            true
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.amount,
            1
        );
    }

    #[tokio::test]
    async fn claims_manual_tree() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset, claim_time) = setup().await;

        let airdrop_leaves: [([u8; 32], u64); 3] = [
            (*wallet1.wallet.address().hash(), 1),
            (*wallet2.wallet.address().hash(), 2),
            (*wallet3.wallet.address().hash(), 3),
        ];
        let key = 0;
        let num_leaves = 3;
        let (root, proof1, proof2) = build_tree_manual(airdrop_leaves).await;

        let identity = AirdropIdentity::Address(wallet1.wallet.address().into());
        let minter = TokenIdentity::ContractId(deploy_wallet.contract_id);
        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            asset.asset_id,
        )
        .await;
        token_constructor(minter, &asset.token, 10).await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.claimed,
            false
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.amount,
            0
        );

        claim(
            1,
            &deploy_wallet.airdrop_distributor,
            key,
            num_leaves,
            [proof1, proof2].to_vec(),
            identity.clone(),
            asset.asset_id,
        )
        .await;

        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.claimed,
            true
        );
        assert_eq!(
            claim_data(&deploy_wallet.airdrop_distributor, identity.clone()).await.amount,
            1
        );
    }
}