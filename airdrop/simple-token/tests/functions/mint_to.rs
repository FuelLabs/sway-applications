use crate::utils::{
    abi_calls::{constructor, mint_to},
    test_helpers::setup,
    Identity
};
use fuels::{signers::Signer, tx::AssetId};

mod success {

    use super::*;

    #[tokio::test]
    async fn mints() {
        let deployer = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;

        assert_eq!(
            deployer.wallet.get_asset_balance(&AssetId::new(*deployer.asset_id)).await.unwrap(),
            0
        );

        mint_to(10, &deployer.simple_token, identity.clone()).await;

        assert_eq!(
            deployer.wallet.get_asset_balance(&AssetId::new(*deployer.asset_id)).await.unwrap(),
            10
        );
    }
}
