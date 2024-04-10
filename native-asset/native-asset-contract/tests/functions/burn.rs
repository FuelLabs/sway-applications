use crate::utils::{
    interface::{burn, constructor, mint, total_assets, total_supply},
    setup::{defaults, get_wallet_balance, setup},
};
use fuels::prelude::{CallParameters, TxPolicies};

mod success {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn burn_assets() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 50).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(50));
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[ignore]
    #[tokio::test]
    async fn burns_multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, asset_id_2, sub_id_1, sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;
        mint(&instance_1, other_identity.clone(), sub_id_1, 100).await;
        mint(&instance_1, other_identity, sub_id_2, 200).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, None);
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 50).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 50);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 200);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(50));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(200));
        assert_eq!(total_assets(&instance_1).await, 2);

        burn(&instance_2, asset_id_2, sub_id_2, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 50);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 100);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(50));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(100));
        assert_eq!(total_assets(&instance_1).await, 2);
    }

    #[ignore]
    #[tokio::test]
    async fn burn_to_zero() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 50).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(50));
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 25).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(25));
        assert_eq!(total_assets(&instance_1).await, 1);

        burn(&instance_2, asset_id_1, sub_id_1, 25).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(0));
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[ignore]
    #[tokio::test]
    async fn can_send_more_than_burn() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_assets(&instance_1).await, 1);

        let call_params = CallParameters::new(50, asset_id_1, 1_000_000);
        instance_2
            .methods()
            .burn(sub_id_1, 10)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(90));
        assert_eq!(total_assets(&instance_1).await, 1);
    }
}

mod revert {

    use super::*;

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "NotEnoughCoins")]
    async fn when_not_enough_coins() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        mint(&instance_1, other_identity, sub_id_1, 100).await;

        let call_params = CallParameters::new(101, asset_id_1, 1_000_000);
        instance_2
            .methods()
            .burn(sub_id_1, 101)
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }
}
