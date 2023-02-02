use crate::utils::{
    abi_calls::{admin, constructor, set_admin},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_admin() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(admin(&owner1.contract).await, None);

        let new_admin = Some(Identity::Address(owner1.wallet.address().into()));
        constructor(new_admin.clone(), &deploy_wallet.contract, Some(1)).await;

        assert_eq!(admin(&owner1.contract).await, new_admin.clone());
    }

    #[tokio::test]
    async fn gets_admin_after_change() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!(admin(&owner1.contract).await, None);

        let new_admin = Some(Identity::Address(owner1.wallet.address().into()));
        constructor(new_admin.clone(), &deploy_wallet.contract, Some(1)).await;

        assert_eq!(admin(&owner1.contract).await, new_admin.clone());

        let new_admin2 = Some(Identity::Address(owner2.wallet.address().into()));
        set_admin(&owner1.contract, new_admin2.clone()).await;

        assert_eq!(admin(&owner1.contract).await, new_admin2.clone());
    }
}
