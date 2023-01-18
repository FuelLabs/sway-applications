use crate::utils::{
    abi_calls::{constructor, is_approved_for_all, set_approval_for_all},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn sets_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        assert_eq!(
            is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await,
            false
        );

        set_approval_for_all(true, &owner1.contract, operator.clone()).await;

        assert_eq!(
            is_approved_for_all(&owner1.contract, operator, owner.clone()).await,
            true
        );
    }

    #[tokio::test]
    async fn removes_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        assert_eq!(
            is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await,
            false
        );

        set_approval_for_all(true, &owner1.contract, operator.clone()).await;

        assert_eq!(
            is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await,
            true
        );

        set_approval_for_all(false, &owner1.contract, operator.clone()).await;

        assert_eq!(
            is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await,
            false
        );
    }
}
