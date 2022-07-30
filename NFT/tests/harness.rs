// TODO: Awaiting SDK support with Options. Nearly every test will have the Option commented out and replaced by an Identity.
// Uncomment and remove when https://github.com/FuelLabs/fuels-rs/issues/415 is revolved.

mod utils;

use crate::utils::{Identity, TokenMetaData};
use fuels::prelude::*;
use utils::{
    abi_calls::{
        admin, approve, approved, balance_of, burn, constructor, is_approved_for_all, max_supply,
        meta_data, mint, owner_of, set_admin, set_approval_for_all, total_supply, transfer_from,
    },
    test_helpers::setup,
};

mod admin {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_admin() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // let new_admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let new_admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &new_admin, 1).await;

            assert_eq!(admin(&owner1.contract).await, new_admin);
        }

        #[tokio::test]
        async fn gets_admin_after_change() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // let new_admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let new_admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &new_admin, 1).await;

            assert_eq!(admin(&owner1.contract).await, new_admin);

            // let new_admin = Option::Some(minter.clone());
            let new_admin = Identity::Address(owner2.wallet.address());
            set_admin(&owner1.contract, &new_admin).await;

            assert_eq!(admin(&owner1.contract).await, new_admin);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_admin_not_set() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            admin(&owner1.contract).await;
        }
    }
}

mod approve {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn approves() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;

            assert_eq!(approved(&owner1.contract, 0).await, approved_identity);
        }

        #[tokio::test]
        async fn approves_mutliple() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 4).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(4, &owner1.contract, &minter).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;
            assert_eq!(approved(&owner1.contract, 0).await, approved_identity);

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 1).await;
            assert_eq!(approved(&owner1.contract, 1).await, approved_identity);

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 2).await;
            assert_eq!(approved(&owner1.contract, 2).await, approved_identity);

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 3).await;
            assert_eq!(approved(&owner1.contract, 3).await, approved_identity);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_owner_does_not_exist() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_map_to_existing_token() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;'
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner2.contract, 0).await;
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;

            assert_eq!(approved(&owner1.contract, 0).await, approved_identity);
        }

        #[tokio::test]
        async fn gets_approval_multiple() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 3).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(3, &owner1.contract, &minter).await;

            // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            let approved_identity = Identity::Address(owner2.wallet.address());
            approve(&approved_identity, &owner1.contract, 0).await;
            approve(&approved_identity, &owner1.contract, 1).await;
            approve(&approved_identity, &owner1.contract, 2).await;

            assert_eq!(approved(&owner1.contract, 0).await, approved_identity);
            assert_eq!(approved(&owner1.contract, 1).await, approved_identity);
            assert_eq!(approved(&owner1.contract, 2).await, approved_identity);
        }

        #[tokio::test]
        #[ignore]
        async fn gets_approval_for_none() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
        }

        #[tokio::test]
        #[ignore]
        async fn gets_approval_for_non_existing_token() {
            let (_deploy_wallet, _owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

            // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn gets_approval_for_none() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            approved(&owner1.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn gets_approval_for_non_existing_token() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            approved(&owner1.contract, 0).await;
        }
    }
}

mod balance_of {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_balance_of_owned() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        }

        #[tokio::test]
        async fn gets_balance_of_multiple_owned() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 4).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(4, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
        }

        #[tokio::test]
        async fn gets_balance_none_owned() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            let not_minter = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.contract, &not_minter).await, 0);
        }

        #[tokio::test]
        async fn gets_balance_before_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.contract, &balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.contract, &balance_identity_2).await, 0);
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            assert_eq!(total_supply(&owner1.contract).await, 1);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);

            burn(&owner1.contract, 0).await;

            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        }

        #[tokio::test]
        async fn burns_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 4).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(4, &owner1.contract, &minter).await;

            assert_eq!(total_supply(&owner1.contract).await, 4);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 4);

            burn(&owner1.contract, 0).await;
            assert_eq!(total_supply(&owner1.contract).await, 3);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 3);

            burn(&owner1.contract, 1).await;
            assert_eq!(total_supply(&owner1.contract).await, 2);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 2);

            burn(&owner1.contract, 2).await;
            assert_eq!(total_supply(&owner1.contract).await, 1);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);

            burn(&owner1.contract, 3).await;
            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_owner_does_not_exist() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            burn(&owner1.contract, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            burn(&owner1.contract, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            burn(&owner2.contract, 0).await;
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

            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 0);

            // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 1);
        }

        #[tokio::test]
        #[ignore]
        async fn initalizes_without_access_control() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 0);

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_initalized_twice() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;
            constructor(true, &deploy_wallet.contract, &admin, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_supply_is_zero() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 0).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        #[ignore]
        async fn panics_when_access_control_set_but_no_admin() {
            let (_deploy_wallet, _owner1, _owner2) = setup().await;

            // constructor(true, &deploy_wallet.contract, &Option::None(), 0).await;
        }
    }
}

mod is_approved_for_all {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_approval_for_approved() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, false);

            set_approval_for_all(true, &owner1.contract, &operator).await;

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, true);
        }

        #[tokio::test]
        async fn gets_approval_for_unapproved() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(true, &owner1.contract, &operator).await;

            assert_eq!(is_approved_for_all(&owner1.contract, &owner, &operator).await, false);
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

            assert_eq!(max_supply(&owner1.contract).await, 0);

            // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 10).await;

            assert_eq!(max_supply(&owner1.contract).await, 10);
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 1);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

            mint(1, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
            assert_eq!(total_supply(&owner1.contract).await, 1);
            assert_eq!(max_supply(&owner1.contract).await, 1);
        }

        #[tokio::test]
        async fn mints_with_access() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            // let admin = Option::Some(minter.clone());
            let admin = minter.clone();
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            assert_eq!(max_supply(&owner1.contract).await, 1);
            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

            mint(1, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
            assert_eq!(total_supply(&owner1.contract).await, 1);
            assert_eq!(max_supply(&owner1.contract).await, 1);
        }

        #[tokio::test]
        async fn mints_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 4).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 4).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(max_supply(&owner1.contract).await, 4);
            assert_eq!(total_supply(&owner1.contract).await, 0);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

            mint(4, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
            assert_eq!(total_supply(&owner1.contract).await, 4);
            assert_eq!(max_supply(&owner1.contract).await, 4);

            // assert_eq!(
            //     owner_of(&owner1.contract, 0).await,
            //     Option::Some(minter.clone())
            // );
            // assert_eq!(
            //     owner_of(&owner1.contract, 1).await,
            //     Option::Some(minter.clone())
            // );
            // assert_eq!(
            //     owner_of(&owner1.contract, 2).await,
            //     Option::Some(minter.clone())
            // );
            // assert_eq!(
            //     owner_of(&owner1.contract, 3).await,
            //     Option::Some(minter.clone())
            // );
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            assert_eq!(owner_of(&owner1.contract, 1).await, minter.clone());
            assert_eq!(owner_of(&owner1.contract, 2).await, minter.clone());
            assert_eq!(owner_of(&owner1.contract, 3).await, minter.clone());
        }

        #[tokio::test]
        async fn mint_amount_is_zero() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 1);
            assert_eq!(total_supply(&owner1.contract).await, 0);

            mint(0, &owner1.contract, &minter).await;

            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(max_supply(&owner1.contract).await, 1);
            assert_eq!(total_supply(&owner1.contract).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_no_token_supply_set() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minting_more_tokens_than_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(max_supply(&owner1.contract).await + 1, &owner1.contract, &minter).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minter_does_not_have_access() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let minter = Identity::Address(owner2.wallet.address());
            // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            mint(1, &owner2.contract, &minter).await;
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 10).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            assert_eq!(
                meta_data(&owner1.contract, 0).await,
                TokenMetaData {
                    name: "Example".to_string()
                }
            );
        }

        #[tokio::test]
        async fn gets_meta_data_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 10).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(3, &owner1.contract, &minter).await;

            assert_eq!(
                meta_data(&owner1.contract, 0).await,
                TokenMetaData {
                    name: "Example".to_string()
                }
            );

            assert_eq!(
                meta_data(&owner1.contract, 1).await,
                TokenMetaData {
                    name: "Example".to_string()
                }
            );

            assert_eq!(
                meta_data(&owner1.contract, 2).await,
                TokenMetaData {
                    name: "Example".to_string()
                }
            );
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            meta_data(&owner1.contract, 1).await;
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        }

        #[tokio::test]
        async fn gets_owner_of_multiple() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 2).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 2).await;

            let minter1 = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter1).await;

            let minter2 = Identity::Address(owner2.wallet.address());
            mint(1, &owner1.contract, &minter2).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter1.clone()));
            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter2.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter1.clone());
            assert_eq!(owner_of(&owner1.contract, 1).await, minter2.clone());
        }

        #[tokio::test]
        #[ignore]
        async fn gets_owner_of_none() {
            let (_deploy_wallet, _owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::None());
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn gets_owner_of_none() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            owner_of(&owner1.contract, 0).await;
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

            // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner2.wallet.address());
            // let new_admin = Option::Some(minter.clone());
            let new_admin = minter.clone();
            set_admin(&owner1.contract, &new_admin).await;

            assert_eq!(balance_of(&owner2.contract, &minter).await, 0);

            mint(1, &owner2.contract, &minter).await;

            assert_eq!(balance_of(&owner2.contract, &minter).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_admin_not_set() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let admin = Identity::Address(owner1.wallet.address());
            set_admin(&owner1.contract, &admin).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_admin_identity() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            // let new_admin = Option::Some(Identity::Address(owner2.wallet.address()));
            let new_admin = Identity::Address(owner2.wallet.address());
            set_admin(&owner2.contract, &new_admin).await;
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, false);

            set_approval_for_all(true, &owner1.contract, &operator).await;

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, true);
        }

        #[tokio::test]
        async fn removes_approval_for_all() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, false);

            set_approval_for_all(true, &owner1.contract, &operator).await;

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, true);

            set_approval_for_all(false, &owner1.contract, &operator).await;

            assert_eq!(is_approved_for_all(&owner1.contract, &operator, &owner).await, false);
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 10).await;

            assert_eq!(total_supply(&owner1.contract).await, 0);

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            assert_eq!(total_supply(&owner1.contract).await, 1);
        }

        #[tokio::test]
        async fn gets_total_supply_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 10).await;

            assert_eq!(total_supply(&owner1.contract).await, 0);

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;
            assert_eq!(total_supply(&owner1.contract).await, 1);

            mint(1, &owner1.contract, &minter).await;
            assert_eq!(total_supply(&owner1.contract).await, 2);

            mint(2, &owner1.contract, &minter).await;
            assert_eq!(total_supply(&owner1.contract).await, 4);
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

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());

            mint(1, &owner1.contract, &minter).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            assert_eq!(balance_of(&owner2.contract, &to).await, 0);

            transfer_from(&owner1.contract, &minter, &to, 0).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(balance_of(&owner2.contract, &to).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            // let approved_identity = Option::Some(to.clone());
            let approved_identity = to.clone();

            mint(1, &owner1.contract, &minter).await;

            approve(&approved_identity, &owner1.contract, 0).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            assert_eq!(balance_of(&owner2.contract, &to).await, 0);

            transfer_from(&owner2.contract, &minter, &to, 0).await;

            assert_eq!(owner_of(&owner1.contract, 0).await, approved_identity);
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(balance_of(&owner2.contract, &to).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_operator() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());

            mint(1, &owner1.contract, &minter).await;

            set_approval_for_all(true, &owner1.contract, &operator).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            assert_eq!(balance_of(&owner2.contract, &operator).await, 0);

            transfer_from(&owner2.contract, &minter, &operator, 0).await;

            // assert_eq!(
            //     owner_of(&owner1.contract, 0).await,
            //     Option::Some(operator.clone())
            // );
            assert_eq!(owner_of(&owner1.contract, 0).await, operator.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(balance_of(&owner2.contract, &operator).await, 1);
        }

        #[tokio::test]
        async fn transfers_multiple() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 4).await;

            let minter = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());

            mint(4, &owner1.contract, &minter).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
            assert_eq!(balance_of(&owner2.contract, &to).await, 0);

            transfer_from(&owner1.contract, &minter, &to, 0).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 3);
            assert_eq!(balance_of(&owner2.contract, &to).await, 1);

            transfer_from(&owner1.contract, &minter, &to, 1).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 2);
            assert_eq!(balance_of(&owner2.contract, &to).await, 2);

            transfer_from(&owner1.contract, &minter, &to, 2).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
            assert_eq!(balance_of(&owner2.contract, &to).await, 3);

            transfer_from(&owner1.contract, &minter, &to, 3).await;

            // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
            assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
            assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
            assert_eq!(balance_of(&owner2.contract, &to).await, 4);
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
            transfer_from(&owner1.contract, &from, &to, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner_or_approved() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
            let admin = Identity::Address(owner1.wallet.address());
            constructor(true, &deploy_wallet.contract, &admin, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(1, &owner1.contract, &minter).await;

            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner2.contract, &minter, &to, 0).await;
        }
    }
}
