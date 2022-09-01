use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, claim},
    airdropdistributor_mod::Identity,
    simple_token_abi_calls::token_constructor,
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
        let identity = Identity::Address(wallet1.wallet.address().into());
        let (_tree, root, leaf, proof) = build_tree(airdrop_leaves.to_vec(), key).await;

        airdrop_constructor(10, &deploy_wallet.airdrop_distributor, root, asset.asset_id).await;
        token_constructor(deploy_wallet.contract_id, &asset.token, 10);

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            0
        );

        claim(1, &deploy_wallet.airdrop_distributor, key, num_leaves, proof, identity);

        assert_eq!(
            wallet1.wallet.get_asset_balance(&AssetId::new(*asset.asset_id)).await.unwrap(),
            1
        );
    }
}

mod revert {

    use super::*;

    #[ignore]
    #[tokio::test]
    #[should_panic]
    async fn panics_after_claim_period() {

    }

    #[ignore]
    #[tokio::test]
    #[should_panic]
    async fn panics_when_claim_twice() {

    }

    #[ignore]
    #[tokio::test]
    #[should_panic]
    async fn panics_when_failed_merkle_verification() {
        
    }
}
