use crate::utils::{
    interface::{deposit, withdraw},
    setup::{defaults, deploy, get_wallet_balance, setup_nft, Withdraw},
};
use fuels::types::Bits256;

mod success {

    use super::*;

    #[tokio::test]
    async fn withdraw_from_vault() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, _share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset1).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 0);

        let response = withdraw(
            &admin.f_nft,
            share_asset1,
            share_supply,
            vault_admin,
            nft_1,
            vault_sub_id,
        )
        .await;

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset1).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 1);

        let log = response.decode_logs_with_type::<Withdraw>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Withdraw {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_1,
                vault_sub_id: Bits256(*vault_sub_id),
                withdrawn_amount: 100_000_000,
                burned_shares: 100_000_000,
            }
        );
    }

    #[tokio::test]
    async fn withdraw_from_multiple_vaults() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        deposit(&admin.f_nft, nft_2, vault_admin, vault_sub_id).await;

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset1).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 0);

        let response = withdraw(
            &admin.f_nft,
            share_asset1,
            share_supply,
            vault_admin,
            nft_1,
            vault_sub_id,
        )
        .await;

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset1).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 1);
        let log = response.decode_logs_with_type::<Withdraw>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Withdraw {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_1,
                vault_sub_id: Bits256(*vault_sub_id),
                withdrawn_amount: 100_000_000,
                burned_shares: 100_000_000,
            }
        );

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset2).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_2).await, 0);

        let response = withdraw(
            &admin.f_nft,
            share_asset2,
            share_supply,
            vault_admin,
            nft_2,
            vault_sub_id,
        )
        .await;

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset2).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_2).await, 1);
        let log = response.decode_logs_with_type::<Withdraw>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Withdraw {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_2,
                vault_sub_id: Bits256(*vault_sub_id),
                withdrawn_amount: 100_000_000,
                burned_shares: 100_000_000,
            }
        );
    }
}

mod revert {

    use super::*;
    use fuels::{
        prelude::{AssetId, CallParameters, TxPolicies},
        programs::call_utils::TxDependencyExtension,
        types::Bits256,
    };

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_invalid_asset_sent() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, _share_asset1, _share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        let call_params = CallParameters::new(share_supply, AssetId::zeroed(), 1_000_000);
        let _ = admin
            .f_nft
            .methods()
            .withdraw(vault_admin, nft_1, Bits256(*vault_sub_id))
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value;
    }

    #[tokio::test]
    #[should_panic(expected = "AllSharesNotReturned")]
    async fn when_incorrect_amount_sent() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, _share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        withdraw(
            &admin.f_nft,
            share_asset1,
            share_supply - 1,
            vault_admin,
            nft_1,
            vault_sub_id,
        )
        .await;
    }
}
