use crate::utils::{
    interface::{deposit, total_assets, total_supply},
    setup::{defaults, deploy, get_wallet_balance, setup_nft, Deposit},
};
use fuels::types::Bits256;

mod success {

    use super::*;

    #[tokio::test]
    async fn creates_vault() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, _share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset1).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 1);
        assert_eq!(total_assets(&admin.f_nft).await, 0);
        assert_eq!(total_supply(&admin.f_nft, share_asset1).await, None);

        let response = deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset1).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 0);
        assert_eq!(total_assets(&admin.f_nft).await, 1);
        assert_eq!(
            total_supply(&admin.f_nft, share_asset1).await,
            Some(share_supply)
        );

        let log = response.decode_logs_with_type::<Deposit>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Deposit {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_1,
                vault_sub_id: Bits256(*vault_sub_id),
                deposited_amount: 1,
                minted_shares: 100_000_000,
            }
        );
    }

    #[tokio::test]
    async fn creates_multiple_vaults() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, share_asset2, share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset1).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 1);
        assert_eq!(total_assets(&admin.f_nft).await, 0);
        assert_eq!(total_supply(&admin.f_nft, share_asset1).await, None);

        let response = deposit(&admin.f_nft, nft_1, vault_admin, vault_sub_id).await;

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset1).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_1).await, 0);
        assert_eq!(total_assets(&admin.f_nft).await, 1);
        assert_eq!(
            total_supply(&admin.f_nft, share_asset1).await,
            Some(share_supply)
        );

        let log = response.decode_logs_with_type::<Deposit>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Deposit {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_1,
                vault_sub_id: Bits256(*vault_sub_id),
                deposited_amount: 1,
                minted_shares: 100_000_000,
            }
        );

        assert_eq!(get_wallet_balance(&admin.wallet, &share_asset2).await, 0);
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_2).await, 1);
        assert_eq!(total_supply(&admin.f_nft, share_asset2).await, None);

        let response = deposit(&admin.f_nft, nft_2, vault_admin, vault_sub_id).await;

        assert_eq!(
            get_wallet_balance(&admin.wallet, &share_asset2).await,
            share_supply
        );
        assert_eq!(get_wallet_balance(&admin.wallet, &nft_2).await, 0);
        assert_eq!(total_assets(&admin.f_nft).await, 2);
        assert_eq!(
            total_supply(&admin.f_nft, share_asset2).await,
            Some(share_supply)
        );

        let log = response.decode_logs_with_type::<Deposit>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            Deposit {
                caller: vault_admin,
                receiver: vault_admin,
                underlying_asset: nft_2,
                vault_sub_id: Bits256(*vault_sub_id),
                deposited_amount: 1,
                minted_shares: 100_000_000,
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
    #[should_panic(expected = "InvalidSRC20NFT")]
    async fn when_sending_more_than_one_amount() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, _share_asset1, _share_asset2, _share_supply) =
            defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        let call_params = CallParameters::new(2, AssetId::zeroed(), 1_000_000);
        let _ = admin
            .f_nft
            .methods()
            .deposit(vault_admin, Bits256(*vault_sub_id))
            .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value;
    }
}
