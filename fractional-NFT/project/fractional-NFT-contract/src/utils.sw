library utils;

dep errors;

use errors::AccessError;
use sway_libs::nft::NFT;

/// Transfers an NFT from one `Identity` to another.
///
/// # Arguments
///
/// * `nft` - The NFT's contract id.
/// * `to` - The user which the NFT should be transfered to.
/// * `token_id` - The id number of the token.
pub fn transfer_nft(nft: ContractId, to: Identity, token_id: u64) {
    let nft_abi = abi(NFT, nft.value);
    nft_abi.transfer(to, token_id);
}
