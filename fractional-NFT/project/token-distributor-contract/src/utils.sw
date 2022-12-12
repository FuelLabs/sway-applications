library utils;

dep data_structures;
dep interface;

use data_structures::TokenDistribution;
use interface::FractionalNFT;

pub fn create_fractional_nft(
    fractional_nft: ContractId,
    nft: ContractId,
    owner: Identity,
    supply: u64,
    token_id: u64,
) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.deposit(nft, owner, supply, token_id);
}

pub fn fractional_nft_owner(fractional_nft: ContractId) -> Option<Identity> {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.owner()
}

pub fn fractional_nft_supply(fractional_nft: ContractId) -> u64 {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.supply()
}

pub fn require_fractional_nft_exists(fractional_nft: Option<TokenDistribution>) -> TokenDistribution {
    require(fractional_nft.is_some(), "Fractional NFT distribution doesn't exist");
    fractional_nft.unwrap()
}

pub fn withdraw_fractional_nft(fractional_nft: ContractId) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.withdraw();
}
