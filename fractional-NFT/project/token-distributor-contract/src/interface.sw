library interface;

abi TokenDistributor {
    #[storage(read, write)]
    fn close(fractional_nft: ContractId);
    #[storage(read, write)]
    fn create(buy_asset: ContractId, fractional_nft: ContractId, nft: ContractId, reserve_price: u64, token_price: u64, token_supply: u64, token_id: u64);
    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft: ContractId);
    #[storage(read, write)]
    fn request_return(fractional_nft: ContractId);
    #[storage(read)]
    fn sell(fractional_nft: ContractId);
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
