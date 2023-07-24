use crate::utils::{
    interface::core::{register, set_asset, set_owner},
    setup::{setup, REGISTER_DURATION},
};
use fuels::{
    prelude::{Address, ContractId},
    types::Identity,
};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::owner,
        setup::{string_to_ascii, OwnerChangedEvent},
    };

    #[tokio::test]
    async fn can_set_owner() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            ContractId::zeroed(),
        )
        .await;

        let previous_owner = owner(&instance, &acc1.name).await;

        assert_eq!(previous_owner.value.unwrap(), acc1.identity());

        let response = set_owner(&instance, &acc1.name, wallet_identity2.clone()).await;
        let new_owner = owner(&instance, &acc1.name).await;

        assert_eq!(new_owner.value.unwrap(), wallet_identity2);

        let log = response
            .decode_logs_with_type::<OwnerChangedEvent>()
            .unwrap();
        assert_eq!(
            log,
            vec![OwnerChangedEvent {
                name: string_to_ascii(&acc1.name),
                new_owner: wallet_identity2,
                previous_owner: acc1.identity()
            }]
        )
    }
}

mod revert {
    use super::*;

    // TODO: missing tests

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn cant_set_owner() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            ContractId::zeroed(),
        )
        .await;

        set_owner(
            &instance.with_account(wallet2).unwrap(),
            &acc1.name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
