use crate::utils::{
    interface::{constructor, set_decimals},
    setup::{defaults, setup},
};

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ValueAlreadySet")]
    async fn when_attempting_to_set_decimals() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        set_decimals(&instance_2, asset_id_1, 9u8).await;
    }
}
