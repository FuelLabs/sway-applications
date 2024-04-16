use crate::utils::{
    interface::{burn, constructor, mint, pause, total_assets, total_supply},
    setup::{defaults, get_wallet_balance, setup},
};
use fuels::prelude::{CallParameters, TxPolicies, BASE_ASSET_ID};

mod success {

    use super::*;

    #[tokio::test]
    async fn burn_assets() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(0));
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[tokio::test]
    async fn burns_multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            asset_id_3,
            sub_id_1,
            sub_id_2,
            sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;
        mint(&instance_1, other_identity.clone(), sub_id_1, 1).await;
        mint(&instance_1, other_identity.clone(), sub_id_2, 1).await;
        mint(&instance_1, other_identity, sub_id_3, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_3).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 3);

        burn(&instance_2, asset_id_1, sub_id_1, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_3).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(0));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 3);

        burn(&instance_2, asset_id_2, sub_id_2, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 0);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_3).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(0));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(0));
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 3);

        burn(&instance_2, asset_id_3, sub_id_3, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 0);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_3).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(0));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(0));
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(0));
        assert_eq!(total_assets(&instance_1).await, 3);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotEnoughCoins")]
    async fn when_not_enough_coins() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;

        let call_params = CallParameters::new(0, asset_id_1, 1_000_000);
        instance_2
            .methods()
            .burn(sub_id_1, 1)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotEnoughCoins")]
    async fn when_invalid_asset() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;

        let call_params = CallParameters::new(1, BASE_ASSET_ID, 1_000_000);
        instance_2
            .methods()
            .burn(sub_id_1, 1)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotEnoughCoins")]
    async fn when_invalid_sub_id() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;

        let call_params = CallParameters::new(1, asset_id_1, 1_000_000);
        instance_2
            .methods()
            .burn(sub_id_2, 1)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Paused")]
    async fn when_contract_is_paused() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;
        pause(&instance_1).await;

        burn(&instance_2, asset_id_1, sub_id_1, 1).await;
    }
}
