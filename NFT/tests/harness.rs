mod utils;

use utils::{
    abi_calls::{
        approve, approved, balance_of, burn, constructor, is_approved_for_all, mint, owner_of,
        set_admin, set_approval_for_all, total_supply, transfer_from,
    },
    test_helpers::{nft_identity_option, setup},
};
use crate::utils::{Option, Identity};
use fuels::{prelude::*};

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn initalizes_with_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1).await, 0);

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet, admin, true, 1).await;

            assert_eq!(total_supply(&owner1).await, 1);
        }

        #[tokio::test]
        async fn initalizes_without_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1).await, 0);

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            assert_eq!(total_supply(&owner1).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_initalized_twice() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;
            constructor(&deploy_wallet, Option::None(), false, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_supply_is_zero() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_access_control_set_but_no_admin() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), true, 0).await;
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 1);
        }

        #[tokio::test]
        async fn mints_with_access() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet, admin, true, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 1);
        }

        #[tokio::test]
        async fn mints_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 5).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 3).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 3);
        }

        #[tokio::test]
        async fn does_not_mint_when_not_initalized() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 0).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minting_more_tokens_than_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 2).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minter_does_not_have_access() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet, admin, true, 1).await;

            let minter = Identity::Address(owner2.wallet.address());
            mint(&owner2, minter, 1).await;
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
            constructor(&deploy_wallet, admin, true, 1).await;
            set_admin(&owner1, &owner2).await;

            let minter = Identity::Address(owner2.wallet.address());
            mint(&owner2, minter, 1).await;

            assert_eq!(balance_of(&owner2, &owner2).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            set_admin(&owner1, &owner1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_access_control_address() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet, admin, true, 1).await;

            set_admin(&owner2, &owner1).await;
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            approve(&owner1, &owner2, 1, true).await;

            assert_eq!(
                approved(&owner1, 1).await,
                nft_identity_option(&owner2).await
            );
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            approve(&owner2, &owner2, 1, true).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_approver_is_owner() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            approve(&owner1, &owner1, 1, true).await;
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 1);
            assert_eq!(balance_of(&owner1, &owner2).await, 0);
        }

        #[tokio::test]
        async fn gets_balance_before_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner1, &owner2).await, 0);
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            burn(&owner1, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let token_id = 0;

            burn(&owner1, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            let token_id = 2;

            burn(&owner1, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            burn(&owner2, 1).await;
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            approve(&owner1, &owner2, 1, true).await;

            assert_eq!(
                approved(&owner1, 1).await,
                nft_identity_option(&owner2).await
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

            constructor(&deploy_wallet, Option::None(), false, 10).await;

            assert_eq!(total_supply(&owner1).await, 10);
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;
            set_approval_for_all(&owner1, &owner1, &owner2, true).await;

            assert_eq! {is_approved_for_all(&owner1, &owner1, &owner2).await, true};
            assert_eq! {is_approved_for_all(&owner1, &owner2, &owner1).await, false};
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            assert_eq!(
                owner_of(&owner1, 1).await,
                nft_identity_option(&owner1).await
            );
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            set_approval_for_all(&owner1, &owner1, &owner2, true).await;

            assert_eq!(is_approved_for_all(&owner1, &owner1, &owner2).await, true);
            assert_eq!(is_approved_for_all(&owner1, &owner2, &owner1).await, false);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            set_approval_for_all(&owner2, &owner1, &owner2, true).await;
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

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            transfer_from(&owner1, &owner1, &owner2, 1).await;

            assert_eq!(
                owner_of(&owner1, 1).await,
                nft_identity_option(&owner2).await
            );
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            approve(&owner1, &owner2, 1, true).await;

            transfer_from(&owner2, &owner1, &owner2, 1).await;

            assert_eq!(
                owner_of(&owner1, 1).await,
                nft_identity_option(&owner2).await
            );
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_operator() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            set_approval_for_all(&owner1, &owner1, &owner2, true).await;

            transfer_from(&owner2, &owner1, &owner2, 1).await;

            assert_eq!(
                owner_of(&owner1, 1).await,
                nft_identity_option(&owner2).await
            );
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;
            let token_id = 0;

            transfer_from(&owner1, &owner1, &owner2, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1, minter, 1).await;

            transfer_from(&owner2, &owner1, &owner2, 1).await;
        }
    }
}
