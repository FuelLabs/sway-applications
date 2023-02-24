<<<<<<< HEAD:fractional-NFT/project/fractional-NFT-contract/tests/functions/supply.rs
use crate::utils::{
    fractional_nft_abi_calls::{deposit, supply},
    nft_abi_calls::{approve, mint},
    test_helpers::{defaults, setup},
};
use fuels::{signers::Signer, types::Identity};

=======
>>>>>>> origin/master:fractional-NFT/project/contracts/fractional-NFT-contract/tests/functions/info/supply.rs
mod success {
    use crate::utils::{
        interface::{
            core::{
                fractional_nft::deposit,
                nft::{approve, mint},
            },
            info::fractional_nft::supply,
        },
        setup::{defaults, setup},
    };
    use fuels::{signers::Signer, types::Identity};

    #[tokio::test]
    async fn gets_supply() {
        let (_deployer, owner1, _owner2, fractional_nft_contract, nft_contract) = setup().await;
        let token_supply = defaults().await;

        let owner_identity = Identity::Address(owner1.wallet.address().into());
        let fractional_nft_identity = Identity::ContractId(fractional_nft_contract);

        mint(1, &owner1.nft, owner_identity.clone()).await;
        approve(Some(fractional_nft_identity.clone()), &owner1.nft, 0).await;

        assert_eq!(supply(&owner1.f_nft).await, 0);

        deposit(
            Some(owner_identity.clone()),
            &owner1.f_nft,
            nft_contract,
            token_supply,
            0,
        )
        .await;

        assert_eq!(supply(&owner1.f_nft).await, token_supply);
    }
}
