contract;

abi EnglishAuction {
    fn test_function() -> bool;
}

impl EnglishAuction for Contract {
    fn test_function() -> bool {
        true
    }
}
