#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod utils;

use utils::{
    abi_calls::{
        balance, constructor, execute_transaction, nonce, owner, transfer, transaction_hash,
    },
    test_helpers::setup,
};

mod balance {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn returns_zero_for_nonexistent_asset() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        async fn returns_correct_balance() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }
}

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn initializes() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_nonce_is_not_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_threshold_is_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_user_identity_is_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_user_weight_is_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
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
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_nonce_is_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_unrecoverable_public_key() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_incorrect_signer_ordering() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_insufficient_approval_count() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }
}

mod nonce {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn returns_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        async fn returns_one() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }
}

mod owner {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        #[ignore]
        async fn returns_info_for_existing_owner() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        async fn returns_info_for_non_owner() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
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
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }

    mod revert {

        use super::*;

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_nonce_is_zero() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_transferring_more_than_contract_balance() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_unrecoverable_public_key() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_incorrect_signer_ordering() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }

        #[tokio::test]
        #[ignore]
        #[should_panic]
        async fn when_insufficient_approval_count() {
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
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
            let (multisig, wallet1, wallet2, wallet3) = setup().await;
        }
    }
}
