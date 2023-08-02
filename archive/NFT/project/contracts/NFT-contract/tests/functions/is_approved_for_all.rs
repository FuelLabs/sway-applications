use crate::utils::{
    abi_calls::{constructor, is_approved_for_all, set_approval_for_all},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_approval_for_approved() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        assert!(!is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await);

        set_approval_for_all(true, &owner1.contract, operator.clone()).await;

        assert!(is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await);
    }

    #[tokio::test]
    async fn gets_approval_for_unapproved() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());
        set_approval_for_all(true, &owner1.contract, operator.clone()).await;

        assert!(!is_approved_for_all(&owner1.contract, owner.clone(), operator.clone()).await);
    }
}
