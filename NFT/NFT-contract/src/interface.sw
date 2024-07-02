library;

abi Constructor {
    #[storage(read, write)]
    fn constructor(owner: Identity);
}

abi MaxSupply {
    fn max_supply() -> u64;
}
