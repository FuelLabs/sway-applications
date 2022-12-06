library interface;

abi FractionalNFT {
    #[storage(read, write)]
    fn constructor(nft: ContractId, owner: Identity, supply: u64, token_id: u64);
    #[storage(read)]
    fn supply() -> u64;
}
