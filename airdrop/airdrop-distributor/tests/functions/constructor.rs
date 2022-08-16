use crate::utils::{
    airdrop_distributor_abi_calls::{constructor, end_block, merkle_root},
    test_helpers::setup,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, _, _, _, asset) = setup().await;

        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 0);
        assert_eq!(merkle_root(&deploy_wallet.airdrop_distributor).await, [0u8; 32]);

        constructor(10, &deploy_wallet.airdrop_distributor, [1u8; 32], asset.asset_id).await;

        // TODO: Get block height and add 10
        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 14);
        assert_eq!(merkle_root(&deploy_wallet.airdrop_distributor).await, [1u8; 32])
        // TODO: Get contract ID that was deployed
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_already_initalized() {
        let (deploy_wallet, _, _, _, asset) = setup().await;

        constructor(10, &deploy_wallet.airdrop_distributor, [1u8; 32], asset.asset_id).await;

        assert_eq!(merkle_root(&deploy_wallet.airdrop_distributor).await, [1u8; 32]);

        constructor(10, &deploy_wallet.airdrop_distributor, [1u8; 32], asset.asset_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_claim_time_zero() {
        let (deploy_wallet, _, _, _, asset) = setup().await;

        constructor(0, &deploy_wallet.airdrop_distributor, [1u8; 32], asset.asset_id).await;
    }
}
