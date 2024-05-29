use crate::utils::{
    interface::{constructor, is_paused, pause, unpause},
    setup::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn unpauses() {
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

        constructor(&instance_1, owner_identity).await;

        pause(&instance_1).await;

        assert!(is_paused(&instance_1).await);

        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);
    }

    #[tokio::test]
    async fn stays_paused_when_called_twice() {
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

        constructor(&instance_1, owner_identity).await;

        pause(&instance_1).await;

        assert!(is_paused(&instance_1).await);

        unpause(&instance_1).await;
        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);
    }

    #[tokio::test]
    async fn unpaused_when_not_paused() {
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

        constructor(&instance_1, owner_identity).await;

        assert!(!is_paused(&instance_1).await);

        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_owner() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
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

        constructor(&instance_1, owner_identity).await;

        unpause(&instance_2).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_initialized() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            _owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        unpause(&instance_1).await;
    }
}
