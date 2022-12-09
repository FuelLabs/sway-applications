library interface;

abi TokenDistributor {
    #[storage(read, write)]
    fn create(buy_asset: ContractId, fractional_nft: ContractId, nft: ContractId, reserve_price: u64, token_price: u64, token_supply: u64, token_id: u64);
}

abi FractionalNFT {
    #[storage(read, write)]
    fn deposit(nft: ContractId, owner: Identity, supply: u64, token_id: u64);
    #[storage(read)]
    fn nft() -> (Option<ContractId>, u64);
    #[storage(read)]
    fn owner() -> Option<Identity>;
    #[storage(read, write)]
    fn set_owner(new_owner: Identity);
    #[storage(read)]
    fn supply() -> u64;
    #[storage(read, write)]
    fn withdraw();
}
