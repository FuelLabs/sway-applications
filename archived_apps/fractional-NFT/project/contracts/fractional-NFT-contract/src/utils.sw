library;

use ::nft::NFT;

/// Transfers an NFT from one `Identity` to another.
///
/// # Arguments
///
/// * `asset_id` - The NFT's contract id.
/// * `to` - The user which the NFT should be transfered to.
/// * `token_id` - The id number of the token.
pub fn transfer_nft(asset_id: ContractId, to: Identity, token_id: u64) {
    let nft_abi = abi(NFT, asset_id.value);
    nft_abi.transfer(to, token_id);
}
