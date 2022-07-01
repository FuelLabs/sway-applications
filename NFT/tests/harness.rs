mod utils;

use crate::utils::{Identity, Option};
use fuels::prelude::*;
use utils::{
    abi_calls::{
        approve, approved, balance_of, burn, constructor, is_approved_for_all, mint, owner_of,
        set_admin, set_approval_for_all, total_supply, transfer_from,
    },
    test_helpers::setup,
};

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn initalizes_with_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.nft).await, 0);

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet.nft, admin, true, 1).await;

            assert_eq!(total_supply(&owner1.nft).await, 1);
        }

        #[tokio::test]
        async fn initalizes_without_access_control() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1.nft).await, 0);

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            assert_eq!(total_supply(&owner1.nft).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_initalized_twice() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;
            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_supply_is_zero() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_access_control_set_but_no_admin() {
            let (deploy_wallet, _owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), true, 0).await;
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let balance_identity = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity).await, 1);
        }

        #[tokio::test]
        async fn mints_with_access() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet.nft, admin, true, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let balance_identity = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity).await, 1);
        }

        #[tokio::test]
        async fn mints_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 5).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 3).await;

            let balance_identity = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity).await, 3);
        }

        #[tokio::test]
        async fn does_not_mint_when_not_initalized() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 0).await;

            let balance_identity = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minting_more_tokens_than_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 2).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minter_does_not_have_access() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet.nft, admin, true, 1).await;

            let minter = Identity::Address(owner2.wallet.address());
            mint(&owner2.nft, minter, 1).await;
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
            constructor(&deploy_wallet.nft, admin, true, 1).await;

            let new_admin = Option::Some(Identity::Address(owner2.wallet.address()));
            set_admin(&owner1.nft, new_admin).await;

            let minter = Identity::Address(owner2.wallet.address());
            mint(&owner2.nft, minter, 1).await;

            let balance_identity = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner2.nft, balance_identity).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            set_admin(&owner1.nft, admin).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_access_control_address() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            constructor(&deploy_wallet.nft, admin, true, 1).await;

            let admin = Option::Some(Identity::Address(owner1.wallet.address()));
            set_admin(&owner2.nft, admin).await;
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&owner1.nft, approved_identity, 1).await;

            assert_eq!(
                approved(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner2.wallet.address()))
            );
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&owner2.nft, approved_identity, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_approver_is_owner() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let approved_identity = Option::Some(Identity::Address(owner1.wallet.address()));
            approve(&owner1.nft, approved_identity, 1).await;
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_1).await, 1);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_2).await, 0);
        }

        #[tokio::test]
        async fn gets_balance_before_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_2).await, 0);
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            burn(&owner1.nft, 1).await;

            let balance_identity = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;

            let token_id = 0;

            burn(&owner1.nft, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let token_id = 2;

            burn(&owner1.nft, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            burn(&owner2.nft, 1).await;
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&owner1.nft, approved_identity, 1).await;

            assert_eq!(
                approved(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner2.wallet.address()))
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

            constructor(&deploy_wallet.nft, Option::None(), false, 10).await;

            assert_eq!(total_supply(&owner1.nft).await, 10);
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(&owner1.nft, owner, operator, true).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            assert_eq! {is_approved_for_all(&owner1.nft, owner, operator).await, true};

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            assert_eq! {is_approved_for_all(&owner1.nft, operator, owner).await, false};
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            assert_eq!(
                owner_of(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner1.wallet.address()))
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(&owner1.nft, owner, operator, true).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            assert_eq!(is_approved_for_all(&owner1.nft, owner, operator).await, true);

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            assert_eq!(is_approved_for_all(&owner1.nft, operator, owner).await, false);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(&owner2.nft, owner, operator, true).await;
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

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner1.nft, from, to, 1).await;

            assert_eq!(
                owner_of(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner2.wallet.address()))
            );

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner2.nft, balance_identity_2).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let approved_identity = Option::Some(Identity::Address(owner2.wallet.address()));
            approve(&owner1.nft, approved_identity, 1).await;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner2.nft, from, to, 1).await;

            assert_eq!(
                owner_of(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner2.wallet.address()))
            );

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner2.nft, balance_identity_2).await, 1);
        }

        #[tokio::test]
        async fn transfers_by_operator() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let owner = Identity::Address(owner1.wallet.address());
            let operator = Identity::Address(owner2.wallet.address());
            set_approval_for_all(&owner1.nft, owner, operator, true).await;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner2.nft, from, to, 1).await;

            assert_eq!(
                owner_of(&owner1.nft, 1).await,
                Option::Some(Identity::Address(owner2.wallet.address()))
            );

            let balance_identity_1 = Identity::Address(owner1.wallet.address());
            assert_eq!(balance_of(&owner1.nft, balance_identity_1).await, 0);

            let balance_identity_2 = Identity::Address(owner2.wallet.address());
            assert_eq!(balance_of(&owner2.nft, balance_identity_2).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_does_not_exist() {
            let (_deploy_wallet, owner1, owner2) = setup().await;
            let token_id = 0;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner1.nft, from, to, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            constructor(&deploy_wallet.nft, Option::None(), false, 1).await;

            let minter = Identity::Address(owner1.wallet.address());
            mint(&owner1.nft, minter, 1).await;

            let from = Identity::Address(owner1.wallet.address());
            let to = Identity::Address(owner2.wallet.address());
            transfer_from(&owner2.nft, from, to, 1).await;
        }
    }
}
