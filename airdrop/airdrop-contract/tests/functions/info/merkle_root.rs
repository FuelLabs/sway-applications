use crate::utils::{interface::info::merkle_root, setup::setup};

mod success {

    use super::*;
    use crate::utils::{interface::core::airdrop_constructor, setup::defaults};
    use fuels::types::Bits256;

    #[tokio::test]
    async fn returns_root() {
        let (deploy_wallet, wallet1, wallet2, wallet3, asset_id) = setup().await;
        let (_, _, _, minter, _, num_leaves, asset_supply, _, claim_time, _, _) =
            defaults(&deploy_wallet, &wallet1, &wallet2, &wallet3).await;
        let root = Bits256([2u8; 32]);

        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor).await,
            Option::None
        );

        airdrop_constructor(
            minter,
            asset_supply / 2,
            asset_id,
            claim_time,
            &deploy_wallet.airdrop_distributor,
            root,
            num_leaves,
        )
        .await;

        assert_eq!(
            merkle_root(&deploy_wallet.airdrop_distributor)
                .await
                .unwrap(),
            root
        )
    }
}
