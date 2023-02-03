mod success {

    #[ignore]
    #[tokio::test]
    async fn executes_transfer() {}

    #[ignore]
    #[tokio::test]
    async fn executes_call_without_value() {}

    #[ignore]
    #[tokio::test]
    async fn executes_call_with_value() {}
}

mod revert {

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {}

    mod transfer {

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAnAssetId")]
        async fn transfer_requires_an_asset_id() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAValue")]
        async fn transfer_requires_a_value() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {}
    }

    mod call {

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CannotCallFunctionsOnAddresses")]
        async fn cannot_call_functions_on_addresses() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CallingFunctionsRequiresCalldata")]
        async fn calling_functions_requires_calldata() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "CallingFunctionsRequiresSingleValueTypeArg")]
        async fn calling_functions_requires_single_value_type_arg() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "TransferRequiresAnAssetId")]
        async fn transfer_requires_an_asset_id() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientAssetAmount")]
        async fn insufficient_asset_amount() {}

        #[ignore]
        #[tokio::test]
        #[should_panic(expected = "InsufficientApprovals")]
        async fn insufficient_approvals() {}
    }
}
