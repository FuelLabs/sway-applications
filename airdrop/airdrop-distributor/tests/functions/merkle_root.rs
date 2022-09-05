use crate::utils::{
    airdrop_distributor_abi_calls::{airdrop_constructor, merkle_root},
    test_helpers::setup,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_root() {
        let (deploy_wallet, _, _, _, asset, claim_time) = setup().await;

        airdrop_constructor(
            claim_time,
            &deploy_wallet.airdrop_distributor,
            [2u8; 32],
            asset.asset_id,
        )
        .await;

        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor).await,
            [2u8; 32]
        )
    }
}
