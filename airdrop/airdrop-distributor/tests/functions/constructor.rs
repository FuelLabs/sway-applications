use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, end_block, merkle_root},
    test_helpers::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let provider = deploy_wallet.wallet.get_provider().unwrap();
        let root = [1u8; 32];

        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 0);

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;

        assert_eq!(
            end_block(&deploy_wallet.airdrop_distributor).await,
            provider.latest_block_height().await.unwrap() + claim_time - 1
        );
        assert_eq!(merkle_root(&deploy_wallet.airdrop_distributor).await, root)
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_already_initalized() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = [1u8; 32];

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;

        let false_claim_time = 10;
        airdrop_constructor(
            asset.asset_id,
            false_claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
    }
}
