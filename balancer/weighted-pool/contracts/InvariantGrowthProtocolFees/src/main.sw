contract;

use std::{
    vec::Vec,   
    option::Option,
};

use WeightedMath::{_calculate_invariant, _calc_due_protocol_swap_fee_bpt_amount};

// storage {
//     WeightedMath_contract_id: ContractId = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0,
// }

abi InvariantGrowthProtocolFees {
    #[storage(read)]fn get_last_invariant() -> u64;
    #[storage(read)]fn _before_join_exit(
        preBalances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        protocolSwapFeePercentage: u64
    );
    #[storage(write)]fn _after_join_exit(
        isJoin: bool,
        preBalances: Vec<u64>,
        balanceDeltas: Vec<u64>,
        normalizedWeights: Vec<u64>
    );
}

// dummy value for totalSupply of pool token
const TOTAL_SUPPLY = 100000;


// This Pool pays protocol fees by measuring the growth of the invariant between joins and exits. Since weights are
// immutable, the invariant only changes due to accumulated swap fees, which saves gas by freeing the Pool
// from performing any computation or accounting associated with protocol fees during swaps.
// This mechanism requires keeping track of the invariant after the last join or exit.
// const _lastPostJoinExitInvariant = 10;
storage {
    _lastPostJoinExitInvariant: u64 = 10,
}

impl InvariantGrowthProtocolFees for Contract {
    /**
     * @dev Returns the value of the invariant after the last join or exit operation.
     */
    #[storage(read)]fn get_last_invariant() -> u64 {
        return storage._lastPostJoinExitInvariant;
    }

    #[storage(read)]
    fn _before_join_exit(
        preBalances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        protocolSwapFeePercentage: u64
    ) {
        // Before joins and exits, we measure the growth of the invariant compared to the invariant after the last join
        // or exit, which will have been caused by swap fees, and use it to mint BPT as protocol fees. This dilutes all
        // LPs, which means that new LPs will join the pool debt-free, and exiting LPs will pay any amounts due
        // before leaving.

        // We return immediately if the fee percentage is zero (to avoid unnecessary computation), or when the pool is
        // paused (to avoid complex computation during emergency withdrawals).
        let _isNotPaused: bool = true;
        // import this function from TemporarilyPausable contract
        if ((protocolSwapFeePercentage == 0) || !_isNotPaused) {
            return;
        }

        let preJoinExitInvariant = _calculate_invariant(normalizedWeights, preBalances);

        let toMint = _calc_due_protocol_swap_fee_bpt_amount(
            TOTAL_SUPPLY,
            storage._lastPostJoinExitInvariant,
            preJoinExitInvariant,
            protocolSwapFeePercentage
        );


        // call this function from BasePool contract
        // _payProtocolFees(toMint);
    }

    #[storage(write)]
    fn _after_join_exit(
        isJoin: bool,
        preBalances: Vec<u64>,
        balanceDeltas: Vec<u64>,
        normalizedWeights: Vec<u64>
    ) {
        // After all joins and exits we store the post join/exit invariant in order to compute growth due to swap fees
        // in the next one.
        let mut tmp = preBalances;
        // Compute the post balances by adding or removing the deltas. Note that we're allowed to mutate preBalances.
        let mut count = 0;
        while count < preBalances.len() {
            // Cannot optimize calls with a fn selector: there are 2- and 3-argument versions of SafeMath.sub
            if isJoin {
                tmp.push(preBalances.get(count).unwrap() + balanceDeltas.get(count).unwrap());
            }
            else {
                tmp.push(preBalances.get(count).unwrap() - balanceDeltas.get(count).unwrap());
            }
        }

        let postJoinExitInvariant = _calculate_invariant(normalizedWeights, tmp);
        storage._lastPostJoinExitInvariant = postJoinExitInvariant;
    }
}
