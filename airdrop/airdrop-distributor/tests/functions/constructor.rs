use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, end_block, merkle_root},
    test_helpers::setup,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, _, _, _, asset, claim_time) = setup().await;
        let provider = deploy_wallet.wallet.get_provider().unwrap();

        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 0);

        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            [1u8; 32],
            asset.asset_id,
        )
        .await;

        assert_eq!(
            end_block(&deploy_wallet.airdrop_distributor).await,
            provider.latest_block_height().await.unwrap() + claim_time - 1
        );
        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor).await,
            [1u8; 32]
        )
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_already_initalized() {
        let (deploy_wallet, _, _, _, asset, claim_time) = setup().await;

        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            [1u8; 32],
            asset.asset_id,
        )
        .await;

        airdrop_constructor(
            10,
            &deploy_wallet.airdrop_distributor,
            [1u8; 32],
            asset.asset_id,
        )
        .await;
    }
}
