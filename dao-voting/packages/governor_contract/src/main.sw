contract;

use governor_abi::Governor;

storage {
    var1: u64 = 0,
    var2: bool = false,
}

impl Governor for Contract {
    #[storage(write)]
    fn govern(var1: u64, var2: bool) {
        storage.var1 = var1;
        storage.var2 = var2;
    }

    #[storage(read)]
    fn vars() -> (u64, bool) {
        (
            storage.var1,
            storage.var2,
        )
    }
}
