library utils;

dep interface;

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

pub fn withdraw_fractional_nft(fractional_nft: ContractId) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft.value);
    f_nft_abi.withdraw();
}
