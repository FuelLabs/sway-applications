use crate::utils::{
    abi_calls::{constructor, mint_to},
    test_helpers::setup,
    Identity,
};
use fuels::{signers::Signer, tx::AssetId};

mod success {

    use super::*;

    #[tokio::test]
    async fn mints_to_one_wallet() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );

        mint_to(10, &deployer.simple_token, identity.clone()).await;

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
        let (deployer, wallet2) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(10, &deployer.simple_token, identity.clone()).await;
        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            10
        );

        let identity2 = Identity::Address(wallet2.wallet.address().into());
        assert_eq!(
            wallet2
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );
        mint_to(15, &deployer.simple_token, identity2.clone()).await;
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
    async fn mints_all_tokens() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            0
        );

        mint_to(100, &deployer.simple_token, identity.clone()).await;

        assert_eq!(
            deployer
                .wallet
                .get_asset_balance(&AssetId::new(*deployer.asset_id))
                .await
                .unwrap(),
            100
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_sender_not_minter() {
        let (deployer, false_minter) = setup().await;

        let minter_identity = Identity::Address(deployer.wallet.address().into());
        constructor(minter_identity.clone(), &deployer.simple_token, 100).await;

        let false_minter_identity = Identity::Address(false_minter.wallet.address().into());
        mint_to(
            10,
            &false_minter.simple_token,
            false_minter_identity.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_mint_more_than_supply() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;

        mint_to(101, &deployer.simple_token, identity.clone()).await;
    }
}
