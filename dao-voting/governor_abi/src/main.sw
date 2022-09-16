library interface;

abi Governor {
    #[storage(write)]
    fn govern(var1: u64, var2: bool);
    #[storage(read)]
    fn vars() -> (u64, bool);
}
