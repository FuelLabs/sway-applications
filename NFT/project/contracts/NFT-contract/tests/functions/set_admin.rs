use crate::utils::{
    abi_calls::{admin, constructor, set_admin},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::AdminEvent;

    #[tokio::test]
    async fn changes_admin() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!(admin(&owner1.contract).await, None);

        let minter = Identity::Address(owner1.wallet.address().into());
        let new_admin = Some(minter.clone());
        constructor(new_admin.clone(), &deploy_wallet.contract, Some(1)).await;

        assert_eq!(admin(&owner2.contract).await, new_admin.clone());

        let minter2 = Identity::Address(owner2.wallet.address().into());
        let new_admin2 = Some(minter2.clone());
        let response = set_admin(&owner1.contract, new_admin2.clone()).await;
        let log = response.get_logs_with_type::<AdminEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            AdminEvent {
                admin: new_admin2.clone()
            }
        );
        assert_eq!(admin(&owner2.contract).await, new_admin2);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NoContractAdmin")]
    async fn when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        let admin = Some(Identity::Address(owner1.wallet.address().into()));
        set_admin(&owner1.contract, admin.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderNotAdmin")]
    async fn when_not_admin_identity() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let new_admin = Some(minter.clone());
        constructor(new_admin.clone(), &deploy_wallet.contract, Some(1)).await;

        let new_admin2 = Some(Identity::Address(owner2.wallet.address().into()));
        set_admin(&owner2.contract, new_admin2.clone()).await;
    }
}
