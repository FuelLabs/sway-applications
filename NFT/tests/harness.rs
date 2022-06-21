mod utils;

use utils::{
    abi_calls::{
        approve,
        //approved,
        balance_of,
        burn,
        init,
        is_approved_for_all,
        mint,
        set_admin,
        set_approval_for_all,
        //owner_of,
        tokens_owned,
        total_supply,
        transfer,
    },
    test_helpers::setup,
};

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn initalizes() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            assert_eq!(total_supply(&owner1).await, 0);

            init(&deploy_wallet, &owner1, true, 1).await;

            assert_eq!(total_supply(&owner1).await, 1);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_initalized_twice() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, true, 1).await;
            init(&deploy_wallet, &owner1, true, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_supply_is_zero() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, true, 0).await;
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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 1);
        }

        #[tokio::test]
        async fn mints_with_access() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, true, 1).await;
            mint(&owner1, &owner1, 1).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 1);
        }

        #[tokio::test]
        async fn mints_multiple() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 5).await;
            mint(&owner1, &owner1, 3).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 3);
        }

        #[tokio::test]
        async fn does_not_mint_when_not_initalized() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 0).await;

            assert_eq!(balance_of(&owner1, &owner1).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, _owner2) = setup().await;
            mint(&owner1, &owner1, 1).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minting_more_tokens_than_supply() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 2).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_minter_does_not_have_access() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, true, 1).await;
            mint(&owner2, &owner2, 1).await;
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

            init(&deploy_wallet, &owner1, true, 1).await;
            set_admin(&owner1, &owner2).await;

            mint(&owner2, &owner2, 1).await;

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

            init(&deploy_wallet, &owner1, true, 1).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            approve(&owner1, &owner2, token_id, true).await;

            // assert_eq!(approved(&owner1, token_id).await, owner2.wallet.address());
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            approve(&owner2, &owner2, token_id, true).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_approver_is_owner() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            approve(&owner1, &owner1, token_id, true).await;
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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            burn(&owner1, token_id).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = 2;

            burn(&owner1, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            burn(&owner2, token_id).await;
        }
    }
}

// Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
// mod approved {

//     use super::*;

//     mod success {

//         use super::*;

//         #[tokio::test]
//         async fn gets_approval() {
//             let (deploy_wallet, owner1, owner2) = setup().await;

//             init(&deploy_wallet, &owner1, false, 1).await;
//             mint(&owner1, &owner1, 1).await;

//             let token_id = tokens_owned(&owner1, &owner1).await;

//             approve(&owner1, &owner2, token_id, true).await;

//             assert_eq!(approved(&owner1, token_id).await, owner2.wallet.address());
//         }
//     }
// }

mod tokens_owned {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_tokens() {
            let (deploy_wallet, owner1, _owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            assert_eq!(1, token_id);
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

            init(&deploy_wallet, &owner1, false, 10).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;
            set_approval_for_all(&owner1, &owner1, &owner2, true).await;

            assert_eq! {is_approved_for_all(&owner1, &owner1, &owner2).await, true};
            assert_eq! {is_approved_for_all(&owner1, &owner2, &owner1).await, false};
        }
    }
}

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
// mod owner_of {

//     use super::*;

//     mod success {

//         use super::*;

//         #[tokio::test]
//         async fn gets_owner_of() {
//             let (deploy_wallet, owner1, _owner2) = setup().await;

//             init(&deploy_wallet, &owner1, false, 1).await;
//             mint(&owner1, &owner1, 1).await;

//             let token_id = tokens_owned(&owner1, &owner1).await;

//             assert_eq!(owner_of(token_id).await, owner1.wallet.address());
//         }
//     }
// }

mod set_approval_for_all {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn sets_approval_for_all() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;

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

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            transfer(&owner1, &owner1, &owner2, token_id).await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
            // assert_eq!(owner_of(token_id).await, owner2.wallet.address());
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
            assert_eq!(tokens_owned(&owner2, &owner2).await, token_id);
            assert_eq!(tokens_owned(&owner1, &owner1).await, 0);
        }

        #[tokio::test]
        async fn transfers_by_approval() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            approve(&owner1, &owner2, token_id, true).await;

            transfer(&owner2, &owner1, &owner2, token_id).await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
            // assert_eq!(owner_of(token_id).await, owner2.wallet.address());
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
            assert_eq!(tokens_owned(&owner2, &owner2).await, token_id);
            assert_eq!(tokens_owned(&owner1, &owner1).await, 0);
        }

        #[tokio::test]
        async fn transfers_by_operator() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            set_approval_for_all(&owner1, &owner1, &owner2, true).await;

            transfer(&owner2, &owner1, &owner2, token_id).await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
            // assert_eq!(owner_of(token_id), owner2.wallet.address());
            assert_eq!(balance_of(&owner1, &owner1).await, 0);
            assert_eq!(balance_of(&owner2, &owner2).await, 1);
            assert_eq!(tokens_owned(&owner2, &owner2).await, token_id);
            assert_eq!(tokens_owned(&owner1, &owner1).await, 0);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_initalized() {
            let (_deploy_wallet, owner1, owner2) = setup().await;
            let token_id = 0;

            transfer(&owner1, &owner1, &owner2, token_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sender_is_not_owner() {
            let (deploy_wallet, owner1, owner2) = setup().await;

            init(&deploy_wallet, &owner1, false, 1).await;
            mint(&owner1, &owner1, 1).await;

            let token_id = tokens_owned(&owner1, &owner1).await;

            transfer(&owner2, &owner1, &owner2, token_id).await;
        }
    }
}
