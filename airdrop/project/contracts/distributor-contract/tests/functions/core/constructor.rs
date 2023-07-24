use crate::utils::{
    interface::core::{airdrop_constructor, asset_constructor, mint_to},
    setup::{defaults, setup},
};
use fuels::types::Bits256;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{end_block, merkle_root},
        setup::CreateAirdropEvent,
    };

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, admin, _, num_leaves, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let provider = deploy_wallet.wallet.provider().unwrap();
        let root = Bits256([1u8; 32]);

        asset_constructor(asset_supply, &asset.asset, admin.clone()).await;
        mint_to(asset_supply, &asset.asset, admin.clone()).await;

        assert_eq!(end_block(&deploy_wallet.airdrop_distributor).await, 0);

        let response = airdrop_constructor(
            admin.clone(),
            asset_supply,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;
        let log = response
            .decode_logs_with_type::<CreateAirdropEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            CreateAirdropEvent {
                admin,
                asset: asset.asset_id,
                end_block: claim_time,
                merkle_root: root,
                number_of_leaves: num_leaves
            }
        );

        assert_eq!(
            (provider.latest_block_height().await.unwrap() as u64) + claim_time,
            end_block(&deploy_wallet.airdrop_distributor).await,
        );
        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor)
                .await
                .unwrap(),
            root
        )
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AlreadyInitialized")]
    async fn when_already_initalized() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, _, _, admin, _, num_leaves, asset_supply, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = Bits256([1u8; 32]);

        asset_constructor(asset_supply, &asset.asset, admin.clone()).await;
        mint_to(asset_supply, &asset.asset, admin.clone()).await;

        airdrop_constructor(
            admin.clone(),
            asset_supply / 2,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        airdrop_constructor(
            admin,
            asset_supply / 2,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotAirdropZeroTokens")]
    async fn when_no_tokens_provided() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset) = setup().await;
        let (_, admin, _, _, _, num_leaves, _, _, claim_time, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = Bits256([1u8; 32]);

        airdrop_constructor(
            admin,
            0,
            asset.asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;
    }
}
