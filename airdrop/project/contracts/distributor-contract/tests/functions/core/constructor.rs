use crate::utils::{
    interface::core::airdrop_constructor,
    setup::{defaults, setup},
};
use fuels::types::Bits256;

mod success {

    use super::*;
    use crate::utils::{CreateAirdropEvent, interface::info::{end_block, merkle_root}};

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let provider = deploy_wallet.wallet.get_provider().unwrap();
        let root = Bits256([1u8; 32]);

        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 0);

        let response = airdrop_constructor(
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
        )
        .await;
        let log = response.get_logs_with_type::<CreateAirdropEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            CreateAirdropEvent {
                asset: asset.asset_id,
                end_block: claim_time,
                merkle_root: root
            }
        );

        assert_eq!(
            provider.latest_block_height().await.unwrap() + claim_time,
            end_block(&deploy_wallet.airdrop_distributor).await,
        );
        assert_eq!(merkle_root(&deploy_wallet.airdrop_distributor).await, root)
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AlreadyInitialized")]
    async fn when_already_initalized() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, _, _, _, _, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = Bits256([1u8; 32]);

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
