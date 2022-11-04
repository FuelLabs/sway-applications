library  AssetHelpers;
use std::{
    contract_id::ContractId,
    vec::Vec,
    option::Option,
};

// TODO For time being there is no standard for fungible token so we are keeping them as ERC20 Later will change to the standards
//!currently it's dummy id when wfuel is added we will replace it
// Wraped FUEL ID
const _WFUEL: b256 =  0x8900c5bec4ca97d4febf9ceb4754a60d782abbf3cd815836c1872116f203f861;

// Sentinel value used to indicate WFUEL with wrapping/unwrapping semantics. The zero address is a good choice for
// multiple reasons: it is cheap to pass as a calldata argument, it is a known invalid token and non-contract, and
// it is an address Pools cannot register as a token.

const _FUEL: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

//Todo For time bieng since native token on fuel is called Native Asset So we are keeping eth till it has some name or symbol
fn _wfuel() -> ContractId {
    return ~ContractId::from(_WFUEL);
}

/*
    * Returns true if `asset` is the sentinel value that represents FUEL.
*/
pub fn _is_eth(asset: ContractId) -> bool {
    let asset = ~ContractId::from(_FUEL);
    return (asset == ~ContractId::from(_FUEL));
}

/*
    * Translates `asset` into an equivalent IERC20 token address. If `asset` represents FUEL, it will be translated
    * to the WFUEL contract.
*/
pub fn translate_to_ierc20(asset: ContractId) -> ContractId {
    if _is_eth(asset) {
        return _wfuel();
    }
    return _as_ierc20(asset);
}

/*
     * Same as `_translateToIERC20(IAsset)`, but for an entire array.
*/
pub fn translate_to_ierc20_second(asset: Vec<ContractId>) -> Vec<ContractId> {
    let mut tokens: Vec<ContractId> = ~Vec::new();
    let mut i: u64 = 0;
    while i < asset.len() {
        tokens.push(translate_to_ierc20(asset.get(i).unwrap()));
        i = i + 1;
    }
    return tokens;
}

/*
    * Interprets `asset` as an IERC20 token. This function should only be called on `asset` if `_isFUEL` previously
    * returned false for it, that is, if `asset` is guaranteed not to be the FUEL sentinel value.
*/
pub fn _as_ierc20(asset: ContractId) -> ContractId {
    return asset;
}
