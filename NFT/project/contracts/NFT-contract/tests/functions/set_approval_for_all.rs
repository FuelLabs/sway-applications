use crate::utils::{
    abi_calls::{constructor, is_approved_for_all, set_approval_for_all},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::OperatorEvent;

    #[tokio::test]
    async fn sets_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        assert!(!is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await);

        let response = set_approval_for_all(true, &owner1.contract, operator.clone()).await;
        let log = response.get_logs_with_type::<OperatorEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            OperatorEvent {
                approved: true,
                owner: owner.clone(),
                operator: operator.clone(),
            }
        );
        assert!(is_approved_for_all(&owner1.contract, operator, owner).await);
    }

    #[tokio::test]
    async fn removes_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let owner = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        assert!(!is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await);

        let response = set_approval_for_all(true, &owner1.contract, operator.clone()).await;
        let log = response.get_logs_with_type::<OperatorEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            OperatorEvent {
                approved: true,
                owner: owner.clone(),
                operator: operator.clone(),
            }
        );
        assert!(is_approved_for_all(&owner1.contract, operator.clone(), owner.clone()).await);

        let response = set_approval_for_all(false, &owner1.contract, operator.clone()).await;
        let log = response.get_logs_with_type::<OperatorEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            OperatorEvent {
                approved: false,
                owner: owner.clone(),
                operator: operator.clone(),
            }
        );
        assert!(!is_approved_for_all(&owner1.contract, operator, owner).await);
    }
}
