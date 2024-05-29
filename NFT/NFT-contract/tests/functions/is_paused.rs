use crate::utils::{
    interface::{constructor, is_paused, pause, unpause},
    setup::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn is_unpaused_by_default() {
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
    }

    #[tokio::test]
    async fn is_unpaused_when_not_initialzied() {
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

        assert!(!is_paused(&instance_1).await);
    }

    #[tokio::test]
    async fn switches_state_when_paused() {
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

        pause(&instance_1).await;

        assert!(is_paused(&instance_1).await);

        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);

        pause(&instance_1).await;

        assert!(is_paused(&instance_1).await);

        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);

        pause(&instance_1).await;

        assert!(is_paused(&instance_1).await);

        unpause(&instance_1).await;

        assert!(!is_paused(&instance_1).await);
    }
}
