library utils;

dep data_structures;
dep interface;

use data_structures::TokenDistribution;
use interface::FractionalNFT;

pub fn create_fractional_nft(
    admin: Option<Identity>,
    fractional_nft: ContractId,
    nft: ContractId,
    supply: u64,
    token_id: u64,
) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.deposit(admin, nft, supply, token_id);
}

pub fn fractional_nft_supply(fractional_nft: ContractId) -> u64 {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.supply()
}

pub fn withdraw_fractional_nft(fractional_nft: ContractId, to: Identity) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.withdraw(to);
}
