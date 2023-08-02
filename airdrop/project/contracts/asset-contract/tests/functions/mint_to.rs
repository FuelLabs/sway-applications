use crate::utils::{
    interface::{constructor, mint_to},
    setup::setup,
};
use fuels::types::Identity;

mod success {

    use super::*;
    use fuels::{accounts::ViewOnlyAccount, prelude::AssetId};

    #[tokio::test]
    async fn mints_to_one_wallet() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;

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

    #[tokio::test]
    async fn mints_to_multiple_wallets() {
        let (deployer, wallet2, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        let wallet2_identity = Identity::Address(wallet2.wallet.address().into());

        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(total_supply - 1, &deployer.simple_asset, identity.clone()).await;
        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            total_supply - 1
        );

        assert_eq!(
            wallet2
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(1, &deployer.simple_asset, wallet2_identity.clone()).await;
        assert_eq!(
            wallet2
                .wallet
                .get_asset_balance(&AssetId::new(*wallet2.asset_id))
                .await
                .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn mints_all_assets() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;

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
    #[should_panic(expected = "SenderNotPermittedToMint")]
    async fn when_sender_not_minter() {
        let (deployer, false_minter, total_supply) = setup().await;

        let minter_identity = Identity::Address(deployer.wallet.address().into());
        let false_minter_identity = Identity::Address(false_minter.wallet.address().into());

        constructor(
            total_supply,
            &deployer.simple_asset,
            minter_identity.clone(),
        )
        .await;

        mint_to(
            total_supply,
            &false_minter.simple_asset,
            false_minter_identity.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "GreaterThanMaximumSupply")]
    async fn when_mint_more_than_supply_in_one_transaction() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;

        mint_to(total_supply + 1, &deployer.simple_asset, identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "GreaterThanMaximumSupply")]
    async fn when_mint_more_than_supply_in_two_transactions() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;

        mint_to(total_supply - 1, &deployer.simple_asset, identity.clone()).await;
        mint_to(2, &deployer.simple_asset, identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderNotPermittedToMint")]
    async fn when_not_initalized() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());

        mint_to(total_supply, &deployer.simple_asset, identity.clone()).await;
    }
}
