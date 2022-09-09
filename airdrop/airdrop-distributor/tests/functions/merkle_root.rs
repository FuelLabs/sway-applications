use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, merkle_root},
    test_helpers::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_root() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time) = defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = [2u8; 32];

        airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;

        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor).await,
            root
        )
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, _, _, _, _) = setup().await;

        merkle_root(&deploy_wallet.airdrop_distributor).await;
    }
}
