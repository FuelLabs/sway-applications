use crate::utils::{
    interface::{deposit, max_withdrawable, withdraw},
    setup::{defaults, deploy, setup_nft},
};
use fuels::tx::Bytes32;

mod success {

    use super::*;

    #[tokio::test]
    async fn increments_on_depoist() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, _share_asset1, _share_asset2, _share_supply) = defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        assert_eq!(max_withdrawable(&admin.f_nft, nft_1, vault_sub_id).await, Some(0));

        deposit(
            &admin.f_nft,
            nft_1,
            vault_admin,
            vault_sub_id
        )
        .await;

        assert_eq!(max_withdrawable(&admin.f_nft, nft_1, vault_sub_id).await, Some(1));
    }

    #[tokio::test]
    async fn decrements_on_withdraw() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, share_asset1, _share_asset2, share_supply) = defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(
            &admin.f_nft,
            nft_1,
            vault_admin.clone(),
            vault_sub_id
        )
        .await;

        assert_eq!(max_withdrawable(&admin.f_nft, nft_1, vault_sub_id).await, Some(1));

        withdraw(
            &admin.f_nft,
            share_asset1, 
            share_supply,
            vault_admin,
            nft_1,
            vault_sub_id
        )
        .await;

        assert_eq!(max_withdrawable(&admin.f_nft, nft_1, vault_sub_id).await, Some(0));
    }
        
    #[tokio::test]
    async fn none_with_invalid_sub_id() {
        let (_deployer, admin, f_nft_id, nft_id) = deploy().await;
        let (nft_1, nft_2) = setup_nft(&admin.wallet, &admin.nft, nft_id).await;
        let (vault_sub_id, vault_admin, _share_asset1, _share_asset2, _share_supply) = defaults(&admin.wallet, nft_1, nft_2, f_nft_id);

        deposit(
            &admin.f_nft,
            nft_1,
            vault_admin,
            vault_sub_id
        )
        .await;

        assert_eq!(max_withdrawable(&admin.f_nft, nft_1, Bytes32::new([1u8; 32])).await, None);
    }
}
