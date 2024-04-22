use crate::utils::{
    interface::{constructor, owner},
    setup::{defaults, setup, State},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn initializes() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(owner(&instance_1).await, State::Initialized(owner_identity));
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialized")]
    async fn when_owner_already_set() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;
        constructor(&instance_1, owner_identity.clone()).await;
    }
}
