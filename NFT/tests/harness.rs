mod utils;

use crate::utils::{Identity, Option, TokenMetaData};
use fuels::prelude::*;
use utils::{
    abi_calls::{
        approve, approved, balance_of, burn, constructor, is_approved_for_all, max_supply,
        meta_data, mint, owner_of, set_admin, set_approval_for_all, total_supply, transfer_from,
    },
    test_helpers::setup,
};

mod approve {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn approves() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&approved_identity, &owner1.nft, 0).await;

            assert_eq!(approved(&owner1.nft, 0).await, approved_identity);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_map_to_existing_token() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&approved_identity, &owner1.nft, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&approved_identity, &owner2.nft, 0).await;
        }
    }
}

mod approved {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&approved_identity, &owner1.nft, 0).await;

            assert_eq!(approved(&owner1.nft, 0).await, approved_identity);
        }

        #[tokio::test]
        async fn gets_approval_for_none() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
            // assert_eq!(approved(&owner1.nft, 0).await, Option::None());
        }

        #[tokio::test]
        async fn gets_approval_for_non_existing_token() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
            // assert_eq!(approved(&owner1.nft, 0).await, Option::None());
        }
    }
}

mod balance_of {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_balance() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            assert_eq!(balance_of(&owner1.nft, &minter).await, 1);

            let not_minter = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.nft, &not_minter).await, 0);
        }

        #[tokio::test]
        async fn gets_balance_before_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, &balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.nft, &balance_identity_2).await, 0);
        }
    }
}

mod burn {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn burns() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            assert_eq!(total_supply(&owner1.nft).await, 0);

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            assert_eq!(total_supply(&owner1.nft).await, 1);

            burn(&owner1.nft, 0).await;

            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            burn(&owner1.nft, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            burn(&owner1.nft, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            burn(&owner2.nft, 0).await;
        }
    }
}

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn initalizes_with_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 0);

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(true, &deploy_wallet.nft, &admin, 1).await;

            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 1);
        }

        #[tokio::test]
        async fn initalizes_without_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 0);

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_initalized_twice() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;
            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_supply_is_zero() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_access_control_set_but_no_admin() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(true, &deploy_wallet.nft, &Option::None(), 0).await;
        }
    }
}

mod is_approved_for_all {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_approval_for_all() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(true, &owner1.nft, &operator).await;

            assert_eq! {is_approved_for_all(&owner1.nft, &operator, &owner).await, true};
            assert_eq! {is_approved_for_all(&owner1.nft, &owner, &operator).await, false};
        }
    }
}

mod max_supply {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_max_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(max_supply(&owner1.nft).await, 0);

            constructor(false, &deploy_wallet.nft, &Option::None(), 10).await;

            assert_eq!(max_supply(&owner1.nft).await, 10);
        }
    }
}

mod mint {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn mints() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 1);
            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);

            mint(1, &owner1.nft, &minter).await;

            assert_eq!(balance_of(&owner1.nft, &minter).await, 1);
            assert_eq!(owner_of(&owner1.nft, 0).await, Option::Some(minter.clone()));
            // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
            // assert_eq!(approved(&owner1.nft, 0).await, Option::None());
            assert_eq!(total_supply(&owner1.nft).await, 1);
            assert_eq!(max_supply(&owner1.nft).await, 1);
        }

        #[tokio::test]
        async fn mints_with_access() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            let admin = Option::Some(minter.clone());
            constructor(true, &deploy_wallet.nft, &admin, 1).await;

            assert_eq!(max_supply(&owner1.nft).await, 1);
            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);

            mint(1, &owner1.nft, &minter).await;

            assert_eq!(balance_of(&owner1.nft, &minter).await, 1);
            assert_eq!(owner_of(&owner1.nft, 0).await, Option::Some(minter.clone()));
            // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
            // assert_eq!(approved(&owner1.nft, 0).await, Option::None());
            assert_eq!(total_supply(&owner1.nft).await, 1);
            assert_eq!(max_supply(&owner1.nft).await, 1);
        }

        #[tokio::test]
        async fn mints_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 10).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(max_supply(&owner1.nft).await, 10);
            assert_eq!(total_supply(&owner1.nft).await, 0);
            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);

            mint(10, &owner1.nft, &minter).await;

            assert_eq!(balance_of(&owner1.nft, &minter).await, 10);
            assert_eq!(total_supply(&owner1.nft).await, 10);
            assert_eq!(max_supply(&owner1.nft).await, 10);
            for itterator in 0..10 {
                assert_eq!(
                    owner_of(&owner1.nft, itterator).await,
                    Option::Some(minter.clone())
                );
                // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
                // assert_eq!(approved(&owner1.nft, itterator).await, Option::None());
            }
        }

        #[tokio::test]
        async fn does_not_mint_when_not_initalized() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
            assert_eq!(max_supply(&owner1.nft).await, 1);
            assert_eq!(total_supply(&owner1.nft).await, 0);

            mint(0, &owner1.nft, &minter).await;

            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
            assert_eq!(total_supply(&owner1.nft).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minting_more_tokens_than_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(2, &owner1.nft, &minter).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minter_does_not_have_access() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let minter = Identity::Address(owner2.wallet.address());
            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(true, &deploy_wallet.nft, &admin, 1).await;

            mint(1, &owner2.nft, &minter).await;
        }
    }
}

mod meta_data {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_meta_data() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 10).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            assert_eq!(meta_data(&owner1.nft, 0).await, TokenMetaData {});
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            meta_data(&owner1.nft, 1).await;
        }
    }
}

mod owner_of {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_owner_of() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            assert_eq!(owner_of(&owner1.nft, 0).await, Option::Some(minter.clone()));
        }

        #[tokio::test]
        async fn gets_owner_of_none() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            // Uncomment when https://github.com/FuelLabs/sway/issues/2238 is resolved
            // assert_eq!(owner_of(&owner1.nft, 0).await, Option::None());
        }
    }
}

mod set_admin {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn changes_admin() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(true, &deploy_wallet.nft, &admin, 1).await;

            let minter = Identity::Address(owner2.wallet.address());
            let new_admin = Option::Some(minter.clone());
            set_admin(&owner1.nft, &new_admin).await;

            mint(1, &owner2.nft, &minter).await;

            assert_eq!(balance_of(&owner2.nft, &minter).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            set_admin(&owner1.nft, &admin).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_admin_identity() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(true, &deploy_wallet.nft, &admin, 1).await;

            let new_admin = Option::Some(Identity::Address(owner2.wallet.address()));
            set_admin(&owner2.nft, &new_admin).await;
        }
    }
}

mod set_approval_for_all {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn sets_approval_for_all() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(true, &owner1.nft, &operator).await;

            assert_eq!(
                is_approved_for_all(&owner1.nft, &operator, &owner).await,
                true
            );
            assert_eq!(
                is_approved_for_all(&owner1.nft, &owner, &operator).await,
                false
            );
        }
    }
}

mod total_supply {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_total_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.nft).await, 0);

            constructor(false, &deploy_wallet.nft, &Option::None(), 10).await;

            assert_eq!(total_supply(&owner1.nft).await, 0);

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            assert_eq!(total_supply(&owner1.nft).await, 1);
        }
    }
}

mod transfer_from {

    use super::*;

    mod succes {

        use super::*;

        #[tokio::test]
        async fn transfers() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner1.nft, &minter, &to, 0).await;

            assert_eq!(owner_of(&owner1.nft, 0).await, Option::Some(to.clone()));

            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
            assert_eq!(balance_of(&owner2.nft, &to).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let to = Identity::Address(owner2.wallet.address());
            let approved_identity = Option::Some(to.clone());
            approve(&approved_identity, &owner1.nft, 0).await;

            transfer_from(&owner2.nft, &minter, &to, 0).await;

            assert_eq!(owner_of(&owner1.nft, 0).await, approved_identity);

            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
            assert_eq!(balance_of(&owner2.nft, &to).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_operator() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(true, &owner1.nft, &operator).await;

            transfer_from(&owner2.nft, &minter, &operator, 0).await;

            assert_eq!(
                owner_of(&owner1.nft, 0).await,
                Option::Some(operator.clone())
            );

            assert_eq!(balance_of(&owner1.nft, &minter).await, 0);
            assert_eq!(balance_of(&owner2.nft, &operator).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner1.nft, &from, &to, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(false, &deploy_wallet.nft, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.nft, &minter).await;

            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner2.nft, &minter, &to, 0).await;
        }
    }
}
