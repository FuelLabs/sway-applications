use crate::utils::{
    fractional_nft_abi_calls::{deposit, nft_info, supply},
    nft_abi_calls::{approve, mint, owner_of},
    test_helpers::{defaults, setup},
};
use fuels::{prelude::Identity, signers::Signer, tx::AssetId};

mod success {

    use super::*;

    #[tokio::test]
    async fn deposits_nft() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner1
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(nft_struct, None);
        assert_eq!(supply(&owner1.f_nft).await, 0);

        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract.clone(),
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner1
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert!(nft_struct.is_some());
        assert_eq!(nft_struct.clone().unwrap().nft, nft_contract.clone());
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(owner_identity.clone())
        );
        assert_eq!(supply(&owner1.f_nft).await, token_supply);
    }

    #[tokio::test]
    async fn deposits_nft_no_owner() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner1
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(nft_struct, None);
        assert_eq!(supply(&owner1.f_nft).await, 0);

        deposit(None, &owner1.f_nft, nft_contract.clone(), token_supply, 0).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner1
                .wallet
                .get_asset_balance(&AssetId::new(*fractional_nft_contract))
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert!(nft_struct.is_some());
        assert_eq!(nft_struct.clone().unwrap().nft, nft_contract.clone());
        assert_eq!(nft_struct.clone().unwrap().admin, None);
        assert_eq!(supply(&owner1.f_nft).await, token_supply);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_already_deposited() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract.into());

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract.clone(),
            token_supply,
            0,
        )
        .await;
        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract.clone(),
            token_supply,
            0,
        )
        .await;
    }
}
