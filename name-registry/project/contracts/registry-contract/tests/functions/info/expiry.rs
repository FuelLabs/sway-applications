use crate::utils::{interface::info::expiry, setup::setup};

mod success {
    use super::*;
    use crate::utils::{
        interface::core::{extend, register, set_asset},
        setup::{EXTEND_DURATION, REGISTER_DURATION},
    };
    use fuels::prelude::ContractId;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            ContractId::zeroed(),
        )
        .await;
        let previous_expiry_response = expiry(&instance, &acc.name).await;

        extend(&instance, &acc.name, EXTEND_DURATION, ContractId::zeroed()).await;

        let new_expiry_response = expiry(&instance, &acc.name).await;

        assert_eq!(
            previous_expiry_response.value.unwrap() + EXTEND_DURATION,
            new_expiry_response.value.unwrap()
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;

        let expiry = expiry(&instance, &acc.name).await;
        expiry.value.unwrap();
    }
}
