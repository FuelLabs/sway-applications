library;

use ::interface::FractionalNFT;

pub fn create_fractional_nft(
    admin: Option<Identity>,
    fractional_nft_id: ContractId,
    asset_id: ContractId,
    supply: u64,
    token_id: u64,
) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft_id.value);
    f_nft_abi.deposit(admin, asset_id, supply, token_id);
}

pub fn fractional_nft_supply(fractional_nft_id: ContractId) -> u64 {
    let f_nft_abi = abi(FractionalNFT, fractional_nft_id.value);
    f_nft_abi.supply()
}

pub fn withdraw_fractional_nft(fractional_nft_id: ContractId, to: Identity) {
    let f_nft_abi = abi(FractionalNFT, fractional_nft_id.value);
    f_nft_abi.withdraw(to);
}
