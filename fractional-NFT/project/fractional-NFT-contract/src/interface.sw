library interface;

dep data_structures;

use data_structures::NFTInfo;

abi FractionalNFT {
    #[storage(read, write)]
    fn deposit(nft: ContractId, owner: Option<Identity>, supply: u64, token_id: u64);
    #[storage(read)]
    fn nft_info() -> Option<NFTInfo>;
    #[storage(read, write)]
    fn set_owner(new_owner: Option<Identity>);
    #[storage(read)]
    fn supply() -> u64;
    #[storage(read, write)]
    fn withdraw(to: Identity);
}
