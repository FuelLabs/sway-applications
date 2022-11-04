contract;

dep data_structures;
dep interface;

use std::{
    contract_id::ContractId,
    address::Address,
};

abi BaseMinimalSwapInfoPool {
    fn on_swap(request: SwapRequest, balanceTokenIn: u64, balanceTokenOut: u64) -> u64;
}


struct SwapRequest {
    // IVault.SwapKind kind;
    tokenIn: ContractId,
    tokenOut: ContractId,
    amount: u64,
    // Misc data
    poolId: b256,
    lastChangeBlock: u64,
    from: Address,
    to: Address,
    userData: b256,
}

use data_structures::{SwapRequest, SwapKind};
use interface::BaseMinimalSwapInfoPool;

impl BaseMinimalSwapInfoPool for Contract {
    fn on_swap(
        request: SwapRequest,
        balanceTokenIn: u64,
        balanceTokenOut: u64
    ) -> u64 {
        let scalingFactorTokenIn = _scalingFactor(request.tokenIn);
        let scalingFactorTokenOut = _scalingFactor(request.tokenOut);

        let balanceTokenIn = _upscale(balanceTokenIn, scalingFactorTokenIn);
        let balanceTokenOut = _upscale(balanceTokenOut, scalingFactorTokenOut);

        match request.kind {
            SwapKind::GIVEN_IN => {
                // Fees are subtracted before scaling, to reduce the complexity of the rounding direction analysis.
                request.amount = _subtractSwapFeeAmount(request.amount);

                // All token amounts are upscaled.
                request.amount = _upscale(request.amount, scalingFactorTokenIn);

                let amountOut = _onSwapGivenIn(request, balanceTokenIn, balanceTokenOut);

                // amountOut tokens are exiting the Pool, so we round down.
                _downscaleDown(amountOut, scalingFactorTokenOut)
            } 
            SwapKind::GIVEN_OUT => {
                // All token amounts are upscaled.
                request.amount = _upscale(request.amount, scalingFactorTokenOut);

                let amountIn = _onSwapGivenOut(request, balanceTokenIn, balanceTokenOut);

                // amountIn tokens are entering the Pool, so we round up.
                let amountIn = _downscaleUp(amountIn, scalingFactorTokenIn);

                // Fees are added after scaling happens, to reduce the complexity of the rounding direction analysis.
                _addSwapFeeAmount(amountIn)
            }
        }
    }

    /*
     * @dev Called when a swap with the Pool occurs, where the amount of tokens entering the Pool is known.
     *
     * Returns the amount of tokens that will be taken from the Pool in return.
     *
     * All amounts inside `swapRequest`, `balanceTokenIn`, and `balanceTokenOut` are upscaled. The swap fee has already
     * been deducted from `swapRequest.amount`.
     *
     * The return value is also considered upscaled, and will be downscaled (rounding down) before returning it to the
     * Vault.
     */

    // pub fn _onSwapGivenIn(
    //     swapRequest: SwapRequest,
    //     balanceTokenIn: u64,
    //     balanceTokenOut: u64
    // ) -> u64;

    /*
     * @dev Called when a swap with the Pool occurs, where the amount of tokens exiting the Pool is known.
     *
     * Returns the amount of tokens that will be granted to the Pool in return.
     *
     * All amounts inside `swapRequest`, `balanceTokenIn`, and `balanceTokenOut` are upscaled.
     *
     * The return value is also considered upscaled, and will be downscaled (rounding up) before applying the swap fee
     * and returning it to the Vault.
     */
    // pub fn _onSwapGivenOut(
    //     swapRequest: swapRequest,
    //     balanceTokenIn: u64,
    //     balanceTokenOut: u64
    // ) -> u64;
}

