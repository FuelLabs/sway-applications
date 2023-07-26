use crate::utils::{
    interface::core::{
        nft::{approve, mint},
        token_distributor::create,
    },
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{
            fractional_nft::nft_info, nft::owner_of, token_distributor::token_distribution,
        },
        setup::DistributionState,
    };
    use fuels::{prelude::Bech32ContractId, tx::AssetId};

    #[tokio::test]
    async fn creates_token_distribution() {
        let (
            deployer,
            owner1,
            _owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);
        let token_distributor_identity = Identity::ContractId(token_distributor_contract);
        let provider = deployer.wallet.provider().unwrap();

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await,
            None
        );

        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(nft_struct.clone().unwrap().asset_id, nft_contract.clone());
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(token_distributor_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_asset,
            asset_contract.clone()
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().nft_asset_id,
            nft_contract.clone()
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().admin,
            Some(owner_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().reserve_price,
            Some(reserve_price)
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Started
        ));
        assert_eq!(token_distribution_struct.clone().unwrap().token_id, 0);
        assert_eq!(
            token_distribution_struct.clone().unwrap().token_price,
            token_price
        );
    }

    #[tokio::test]
    async fn creates_token_distribution_no_owner() {
        let (
            deployer,
            owner1,
            _owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);
        let provider = deployer.wallet.provider().unwrap();

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await,
            None
        );

        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            None,
            token_price,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(nft_struct.clone().unwrap().asset_id, nft_contract.clone());
        assert_eq!(nft_struct.clone().unwrap().admin, None);
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_asset,
            asset_contract.clone()
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().nft_asset_id,
            nft_contract.clone()
        );
        assert_eq!(token_distribution_struct.clone().unwrap().admin, None);
        assert_eq!(
            token_distribution_struct.clone().unwrap().reserve_price,
            Some(reserve_price)
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Started
        ));
        assert_eq!(token_distribution_struct.clone().unwrap().token_id, 0);
        assert_eq!(
            token_distribution_struct.clone().unwrap().token_price,
            token_price
        );
    }

    #[tokio::test]
    async fn creates_token_distribution_no_reserve() {
        let (
            deployer,
            owner1,
            _owner2,
            token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (_reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);
        let token_distributor_identity = Identity::ContractId(token_distributor_contract);
        let provider = deployer.wallet.provider().unwrap();

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            0
        );
        assert_eq!(owner_of(&owner1.nft, 0).await, Some(owner_identity.clone()));
        assert_eq!(
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await,
            None
        );

        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            None,
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
        assert_eq!(
            provider
                .get_contract_asset_balance(
                    &Bech32ContractId::from(token_distributor_contract),
                    AssetId::from(*fractional_nft_contract)
                )
                .await
                .unwrap(),
            token_supply
        );
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(nft_struct.clone().unwrap().asset_id, nft_contract.clone());
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(token_distributor_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_asset,
            asset_contract.clone()
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().external_deposits,
            0
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().nft_asset_id,
            nft_contract.clone()
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().admin,
            Some(owner_identity.clone())
        );
        assert_eq!(
            token_distribution_struct.clone().unwrap().reserve_price,
            None
        );
        assert!(matches!(
            token_distribution_struct.clone().unwrap().state,
            DistributionState::Started
        ));
        assert_eq!(token_distribution_struct.clone().unwrap().token_id, 0);
        assert_eq!(
            token_distribution_struct.clone().unwrap().token_price,
            token_price
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "DistributionAlreadyExists")]
    async fn when_initalized_twice() {
        let (
            _deployer,
            owner1,
            _owner2,
            _token_distributor_contract,
            fractional_nft_contract,
            nft_contract,
            asset_contract,
        ) = setup().await;
        let (reserve_price, token_price, token_supply, _purchase_amount, _asset_supply) =
            defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;

        create(
            &owner1.token_distributor,
            asset_contract,
            fractional_nft_contract,
            nft_contract,
            Some(reserve_price),
            Some(owner_identity.clone()),
            token_price,
            token_supply,
            0,
        )
        .await;
    }
}
