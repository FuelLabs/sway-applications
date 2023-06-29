mod success {
    use crate::utils::{
        interface::{
            core::{
                nft::{approve, mint},
                token_distributor::create,
            },
            info::token_distributor::token_distribution,
        },
        setup::{defaults, setup, DistributionState},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_token_distribution_info() {
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

        let token_distribution_struct =
            token_distribution(&owner1.token_distributor, fractional_nft_contract).await;
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
        assert_eq!(token_distribution_struct.unwrap().token_price, token_price);
    }
}
