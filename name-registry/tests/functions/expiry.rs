mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        get_contract_instance//, *
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, _id, wallet, _wallet2) = get_contract_instance().await;

        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");

        let _registration_response = register(&instance, &name, 5000, &wallet_identity, &wallet_identity).await;
        let previous_expiry_response = expiry(&instance, &name).await;

        // let log_registration_event = instance.logs_with_type::<NameRegisteredEvent>(&registration_response.receipts).unwrap();
        // assert_eq!(log_registration_event, vec![NameRegisteredEvent { expiry: 5000, name: SizedAsciiString::<8>::new(name.to_owned()).unwrap(), identity: wallet_identity.clone(), owner: wallet_identity.clone()}]);

        extend(&instance, &name, 5000).await;

        let new_expiry_response = expiry(&instance, &name).await;

        assert_eq!(previous_expiry_response.value + 5000, new_expiry_response.value);
    }
}

mod revert {
    use crate::utils::{abi::*, *};

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_get_expiry() {
        let (instance, _id, _wallet, _wallet2) = get_contract_instance().await;

        let name = String::from("SwaySway");

        let _expiry = expiry(&instance, &name).await;
    }
}
