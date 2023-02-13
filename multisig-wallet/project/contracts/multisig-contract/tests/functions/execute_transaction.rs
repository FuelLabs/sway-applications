mod success {

    #[ignore]
    #[tokio::test]
    async fn executes_transaction() {
        // Currently ignored and unimplemented, awaiting the ability to implement https://github.com/FuelLabs/sway-applications/issues/22
    }
}

mod revert {

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        // Currently ignored and unimplemented, awaiting the ability to implement https://github.com/FuelLabs/sway-applications/issues/22
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientAssetAmount")]
    async fn insufficient_asset_amount() {
        // Currently ignored and unimplemented, awaiting the ability to implement https://github.com/FuelLabs/sway-applications/issues/22
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "IncorrectSignerOrdering")]
    async fn incorrect_signer_ordering() {
        // Currently ignored and unimplemented, awaiting the ability to implement https://github.com/FuelLabs/sway-applications/issues/22
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        // Currently ignored and unimplemented, awaiting the ability to implement https://github.com/FuelLabs/sway-applications/issues/22
    }
}
