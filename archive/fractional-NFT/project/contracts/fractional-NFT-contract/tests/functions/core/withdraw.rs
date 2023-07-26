use crate::utils::{
    interface::core::{
        fractional_nft::{deposit, withdraw},
        nft::{approve, mint},
    },
    setup::{defaults, setup},
};
use fuels::{
    accounts::Account,
    prelude::{Bech32ContractId, TxParameters},
    tx::AssetId,
    types::Identity,
};

mod success {

    use super::*;
    use crate::utils::interface::info::{fractional_nft::nft_info, nft::owner_of};
    use fuels::accounts::ViewOnlyAccount;

    #[tokio::test]
    async fn withdraws_nft() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract,
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
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(owner_identity.clone())
        );

        let _ = owner1
            .wallet
            .force_transfer_to_contract(
                &Bech32ContractId::from(fractional_nft_contract),
                token_supply,
                AssetId::new(*fractional_nft_contract),
                TxParameters::default(),
            )
            .await;
        withdraw(&owner1.f_nft, nft_contract, owner_identity.clone()).await;

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
        assert_eq!(nft_struct.clone().unwrap().admin, None);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NoNftDeposited")]
    async fn when_not_deposited() {
        let (_deployer, owner1, _owner2, _fractional_nft_contract, nft_contract) = setup().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());

        withdraw(&owner1.f_nft, nft_contract, owner_identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotNftAdmin")]
    async fn when_not_owner() {
        let (_deployer, owner1, owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        let _ = owner1
            .wallet
            .force_transfer_to_contract(
                &Bech32ContractId::from(fractional_nft_contract),
                token_supply,
                AssetId::new(*fractional_nft_contract),
                TxParameters::default(),
            )
            .await;
        withdraw(&owner2.f_nft, nft_contract, owner_identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotNftAdmin")]
    async fn when_no_owner() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(None, &owner1.f_nft, nft_contract, token_supply, 0).await;

        let _ = owner1
            .wallet
            .force_transfer_to_contract(
                &Bech32ContractId::from(fractional_nft_contract),
                token_supply,
                AssetId::new(*fractional_nft_contract),
                TxParameters::default(),
            )
            .await;
        withdraw(&owner1.f_nft, nft_contract, owner_identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "SupplyNotReturned")]
    async fn when_tokens_not_returned() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        withdraw(&owner1.f_nft, nft_contract, owner_identity.clone()).await;
    }
}
