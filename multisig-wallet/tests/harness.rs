#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod utils;

use fuels::signers::Signer;

use utils::{
    abi_calls::{
        balance, constructor, execute_transaction, nonce, owner, transfer, transaction_hash,
    },
    Owner,
    test_helpers::{deposit, mint, setup},
    User,
};

mod balance {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero_for_nonexistent_asset() {
            let (multisig, _, asset) = setup().await;
            assert_eq!(0, balance(asset.id, &multisig.contract).await);
        }

        #[tokio::test]
        async fn returns_correct_balance() {
            let (multisig, wallets, asset) = setup().await;

            assert_eq!(0, balance(asset.id, &multisig.contract).await);

            mint(100, &asset.contract, &wallets.users[0]).await;
            deposit(100, asset.id, multisig.id, &wallets.users[0]).await;

            assert_eq!(100, balance(asset.id, &multisig.contract).await);
        }
    }
}

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn initializes() {
            let (multisig, wallets, _) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 1 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            assert_eq!(0, nonce(&multisig.contract).await);
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[0]).await);
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[1]).await);
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[2]).await);

            constructor(&multisig.contract, 2, users.clone()).await;

            assert_eq!(1, nonce(&multisig.contract).await);
            assert_eq!(Owner { weight: users.get(0).unwrap().weight }, owner(&multisig.contract, &wallets.users[0]).await);
            assert_eq!(Owner { weight: users.get(1).unwrap().weight }, owner(&multisig.contract, &wallets.users[1]).await);
            assert_eq!(Owner { weight: users.get(2).unwrap().weight }, owner(&multisig.contract, &wallets.users[2]).await);
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_nonce_is_not_zero() {
            let (multisig, wallets, asset) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 1 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            constructor(&multisig.contract, 2, users.clone()).await;
            constructor(&multisig.contract, 2, users).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_threshold_is_zero() {
            let (multisig, wallets, asset) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 1 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            constructor(&multisig.contract, 0, users.clone()).await;
        }

        #[tokio::test]
        #[should_panic]
        async fn when_user_weight_is_zero() {
            let (multisig, wallets, asset) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 0 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            constructor(&multisig.contract, 2, users.clone()).await;
        }
    }
}

mod execute_transaction {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn executes() {
            // TODO: add call to call function in order to execute data
            let (multisig, wallets, asset) = setup().await;
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_nonce_is_zero() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_unrecoverable_public_key() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_incorrect_signer_ordering() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_insufficient_approval_count() {
            let (multisig, wallets, asset) = setup().await;
        }
    }
}

mod nonce {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_zero() {
            let (multisig, _, _) = setup().await;
            assert_eq!(0, nonce(&multisig.contract).await);
        }

        #[tokio::test]
        async fn returns_one() {
            let (multisig, wallets, asset) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 1 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            assert_eq!(0, nonce(&multisig.contract).await);

            constructor(&multisig.contract, 2, users.clone()).await;

            assert_eq!(1, nonce(&multisig.contract).await);
        }
    }
}

mod owner {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn returns_info_for_existing_owner() {
            let (multisig, wallets, _) = setup().await;
            let users = vec![
                User { identity: wallets.users[0].address().into(), weight: 1 },
                User { identity: wallets.users[1].address().into(), weight: 1 },
                User { identity: wallets.users[2].address().into(), weight: 2 },
            ];

            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[0]).await);
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[1]).await);
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[2]).await);

            constructor(&multisig.contract, 2, users.clone()).await;

            assert_eq!(Owner { weight: users.get(0).unwrap().weight }, owner(&multisig.contract, &wallets.users[0]).await);
            assert_eq!(Owner { weight: users.get(1).unwrap().weight }, owner(&multisig.contract, &wallets.users[1]).await);
            assert_eq!(Owner { weight: users.get(2).unwrap().weight }, owner(&multisig.contract, &wallets.users[2]).await);
        }

        #[tokio::test]
        async fn returns_info_for_non_owner() {
            let (multisig, wallets, _) = setup().await;
            assert_eq!(Owner { weight: 0 }, owner(&multisig.contract, &wallets.users[0]).await);
        }
    }
}

mod transfer {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn transfers() {
            let (multisig, wallets, asset) = setup().await;
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_nonce_is_zero() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_transferring_more_than_contract_balance() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_unrecoverable_public_key() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_incorrect_signer_ordering() {
            let (multisig, wallets, asset) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_insufficient_approval_count() {
            let (multisig, wallets, asset) = setup().await;
        }
    }
}

mod transaction_hash {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn returns_hash() {
            let (multisig, wallets, asset) = setup().await;
        }
    }
}
