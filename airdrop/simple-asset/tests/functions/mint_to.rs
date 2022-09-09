use crate::utils::{
    abi_calls::{constructor, mint_to},
    test_helpers::setup,
    Identity,
};
use fuels::tx::AssetId;

mod success {

    use super::*;

    #[tokio::test]
    async fn mints_to_one_wallet() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );

        mint_to(10, &deployer.simple_asset, identity.clone()).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            10
        );
    }

    #[tokio::test]
    async fn mints_to_multiple_wallets() {
        let (deployer, wallet2, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        let wallet2_identity = Identity::Address(wallet2.wallet.address().into());

        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(10, &deployer.simple_asset, identity.clone()).await;
        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            10
        );

        assert_eq!(
            wallet2
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(15, &deployer.simple_asset, wallet2_identity.clone()).await;
        assert_eq!(
            wallet2
                .wallet
                .get_asset_balance(&AssetId::new(*wallet2.asset_id))
                .await
                .unwrap(),
            15
        );
    }

    #[tokio::test]
    async fn mints_all_assets() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );

        mint_to(total_supply, &deployer.simple_asset, identity.clone()).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            total_supply
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_sender_not_minter() {
        let (deployer, false_minter, total_supply) = setup().await;

        let minter_identity = Identity::Address(deployer.wallet.address().into());
        let false_minter_identity = Identity::Address(false_minter.wallet.address().into());

        constructor(
            minter_identity.clone(),
            &deployer.simple_asset,
            total_supply,
        )
        .await;

        mint_to(
            10,
            &false_minter.simple_asset,
            false_minter_identity.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_mint_more_than_supply() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;

        mint_to(total_supply + 1, &deployer.simple_asset, identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_not_initalized() {
        let (deployer, _, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());

        mint_to(10, &deployer.simple_asset, identity.clone()).await;
    }
}
