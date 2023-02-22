use crate::utils::{
    interface::core::{
        fractional_nft::{deposit, set_admin},
        nft::{approve, mint},
    },
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::interface::info::{fractional_nft::nft_info, nft::owner_of};

    #[tokio::test]
    async fn sets_admin() {
        let (_deployer, owner1, owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner1_identity = Identity::Address(owner1.wallet.address().into());
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner1_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner1_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(owner1_identity.clone())
        );

        set_admin(&owner1.f_nft, Some(owner2_identity.clone())).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(owner2_identity.clone())
        );
    }

    #[tokio::test]
    async fn sets_admin_to_none() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner1_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner1_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner1_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(
            nft_struct.clone().unwrap().admin,
            Some(owner1_identity.clone())
        );

        set_admin(&owner1.f_nft, None).await;

        let nft_struct = nft_info(&owner1.f_nft).await;
        assert_eq!(
            owner_of(&owner1.nft, 0).await,
            Some(fractional_nft_identity.clone())
        );
        assert_eq!(nft_struct.clone().unwrap().admin, None);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NoNftDeposited")]
    async fn when_not_deposited() {
        let (_deployer, owner1, owner2, _fractional_nft_contract, _nft_contract) = setup().await;

        let owner2_identity = Identity::Address(owner2.wallet.address().into());

        set_admin(&owner1.f_nft, Some(owner2_identity.clone())).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotNftAdmin")]
    async fn when_not_admin() {
        let (_deployer, owner1, owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner1_identity = Identity::Address(owner1.wallet.address().into());
        let owner2_identity = Identity::Address(owner2.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner1_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;
        deposit(
            Some(owner1_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        set_admin(&owner2.f_nft, Some(owner2_identity.clone())).await;
    }
}
