use crate::utils::{
    abi_calls::{admin, constructor, max_supply, tokens_minted},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::{AdminEvent, SupplyEvent};

    #[tokio::test]
    async fn initalizes_with_access_control_and_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let admin_identity = Some(Identity::Address(owner1.wallet.address().into()));
        let response = constructor(admin_identity.clone(), &deploy_wallet.contract, Some(1)).await;
        let log1 = response.get_logs_with_type::<AdminEvent>().unwrap();
        let log2 = response.get_logs_with_type::<SupplyEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            AdminEvent {
                admin: admin_identity.clone()
            }
        );
        assert_eq!(*event2, SupplyEvent { supply: Some(1) });
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(admin(&owner1.contract).await, admin_identity.clone());
    }

    #[tokio::test]
    async fn initalizes_without_access_control() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let response = constructor(None, &deploy_wallet.contract, Some(1)).await;
        let log1 = response.get_logs_with_type::<AdminEvent>().unwrap();
        let log2 = response.get_logs_with_type::<SupplyEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(*event1, AdminEvent { admin: None });
        assert_eq!(*event2, SupplyEvent { supply: Some(1) });
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(admin(&owner1.contract).await, None);
    }

    #[tokio::test]
    async fn initalizes_without_max_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let admin_identity = Some(Identity::Address(owner1.wallet.address().into()));
        let response = constructor(admin_identity.clone(), &deploy_wallet.contract, None).await;
        let log1 = response.get_logs_with_type::<AdminEvent>().unwrap();
        let log2 = response.get_logs_with_type::<SupplyEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(
            *event1,
            AdminEvent {
                admin: admin_identity.clone()
            }
        );
        assert_eq!(*event2, SupplyEvent { supply: None });
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, admin_identity.clone());
    }

    #[tokio::test]
    async fn initalizes_without_access_control_and_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let response = constructor(None, &deploy_wallet.contract, None).await;
        let log1 = response.get_logs_with_type::<AdminEvent>().unwrap();
        let log2 = response.get_logs_with_type::<SupplyEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(*event1, AdminEvent { admin: None });
        assert_eq!(*event2, SupplyEvent { supply: None });
        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn when_initalized_twice() {
        let (deploy_wallet, _owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;
        constructor(None, &deploy_wallet.contract, Some(1)).await;
    }
}
