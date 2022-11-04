contract;

abi Vault {
    fn test_function() -> bool;
}


// The `Vault` is Balancer V2's core contract. A single instance of it exists for the entire network, and it is the
// entity used to interact with Pools by Liquidity Providers who join and exit them, Traders who swap, and Asset
// Managers who withdraw and deposit tokens.
//
// The `Vault`'s source code is split among a number of sub-contracts, with the goal of improving readability and making
// understanding the system easier. Most sub-contracts have been marked as `abstract` to explicitly indicate that only
// the full `Vault` is meant to be deployed.
//
// Roughly speaking, these are the contents of each sub-contract:
//
//  - `AssetManagers`: Pool token Asset Manager registry, and Asset Manager interactions.
//  - `Fees`: set and compute protocol fees.
//  - `FlashLoans`: flash loan transfers and fees.
//  - `PoolBalances`: Pool joins and exits.
//  - `PoolRegistry`: Pool registration, ID management, and basic queries.
//  - `PoolTokens`: Pool token registration and registration, and balance queries.
//  - `Swaps`: Pool swaps.
//  - `UserBalance`: manage user balances (Internal Balance operations and external balance transfers)
//  - `VaultAuthorization`: access control, relayers and signature validation.
//
// Additionally, the different Pool specializations are handled by the `GeneralPoolsBalance`,
// `MinimalSwapInfoPoolsBalance` and `TwoTokenPoolsBalance` sub-contracts, which in turn make use of the
// `BalanceAllocation` library.
//
// The most important goal of the `Vault` is to make token swaps use as little gas as possible. This is reflected in a
// multitude of design decisions, from minor things like the format used to store Pool IDs, to major features such as
// the different Pool specialization settings.
//
// Finally, the large number of tasks carried out by the Vault means its bytecode is very large, close to exceeding
// the contract size limit imposed by EIP 170 (https://eips.ethereum.org/EIPS/eip-170). Manual tuning of the source code
// was required to improve code generation and bring the bytecode size below this limit. This includes extensive
// utilization of `internal` functions (particularly inside modifiers), usage of named return arguments, dedicated
// storage access methods, dynamic revert reason generation, and usage of inline assembly, to name a few.


use Swaps::Swaps;
use FlashLoans::FlashLoans;

storage {
    swaps_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
    flash_loans_contract_id: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
}

abi Vault {
    #[storage(read)]
    fn swap(
        singleSwap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64
    ) -> u64;
    #[storage(read)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64
    ) -> Vec<u64>;
    #[storage(read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement
    ) -> Vec<u64>;
    #[storage(read)]
    fn flash_loan(
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        userData: Vec<b256>,
    );
}

impl Vault for Contract {
    // fn set_paused(paused: bool){
    //     _set_paused(paused);
    // }

    // // solhint-disable-next-line func-name-mixedcase
    // fn WETH() -> IWETH {
    //     return _WETH();
    // }

    // swaps contract functions
    #[storage(read)]
    fn swap(
        singleSwap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64
    ) -> u64
    {
        let x = abi(Swaps, swaps_contract_id);
        return x.swap(singleSwap, funds, limit, deadline);
    }

     #[storage(read)]
    fn batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement,
        limits: Vec<u64>,
        deadline: u64
    ) -> Vec<u64> 
    {
        let x = abi(Swaps, swaps_contract_id);
        return x.batch_swap(kind, swaps, assets, funds, limits, deadline);
    }

    #[storage(read)]
    #[storage(read)]
    fn query_batch_swap(
        kind: SwapKind,
        swaps: Vec<BatchSwapStep>,
        assets: Vec<ContractId>,
        funds: FundManagement
    ) -> Vec<u64> 
    {
        let x = abi(Swaps, swaps_contract_id);
        return x.query_batch_swap(kind, swaps, assets, funds);
    }

    #[storage(read)]
    fn flash_loan(
        tokens: Vec<ContractId>,
        amounts: Vec<u64>,
        userData: Vec<b256>,
    )
    {
        let x = abi(FlashLoans, flash_loans_contract_id);
        return x.flash_loan(tokens, amounts, userData);
    }
    
}
