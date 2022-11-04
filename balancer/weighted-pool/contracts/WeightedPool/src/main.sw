contract;

use std::{
    contract_id::ContractId,
    address::Address,

    vec::Vec,
};

abi WeightedPool {
    fn _get_normalized_weight(token: ContractId) -> u64;
    fn _get_normalized_weights() -> Vec<u64>;
    fn _get_max_tokens() -> u64;
    // fn _get_total_tokens() -> u64;
    fn _scaling_factor(token: ContractId) -> u64;
    fn _scaling_factors() -> Vec<u64>;
    fn _before_join_exit(
        preBalances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        protocolSwapFeePercentage: u64
    );
    fn _after_join_exit(
        isJoin: bool,
        preBalances: Vec<u64>,
        balanceDeltas: Vec<u64>,
        normalizedWeights: Vec<u64>
    );
}

use InvariantGrowthProtocolFees::InvariantGrowthProtocolFees;

storage {
    InvariantGrowthProtocolFees_contract_id: ContractId = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0, 
                                                          
}

const _MAX_TOKENS = 20;
const _TOTAL_TOKENS = 10000000;

const _TOKEN0: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN1: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN2: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN3: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN4: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN5: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN6: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN7: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN8: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN9: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN10: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN11: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN12: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN13: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN14: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN15: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN16: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN17: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN18: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
const _TOKEN19: b256 = 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;

// All token balances are normalized to behave as if the token had 18 decimals. We assume a token's decimals will
// not change throughout its lifetime, and store the corresponding scaling factor for each at construction time.
// These factors are always greater than or equal to one: tokens with more than 18 decimals are not supported.

const _SCALING_FACTOR0 = 10;
const _SCALING_FACTOR1 = 10;
const _SCALING_FACTOR2 = 10;
const _SCALING_FACTOR3 = 10;
const _SCALING_FACTOR4 = 10;
const _SCALING_FACTOR5 = 10;
const _SCALING_FACTOR6 = 10;
const _SCALING_FACTOR7 = 10;
const _SCALING_FACTOR8 = 10;
const _SCALING_FACTOR9 = 10;
const _SCALING_FACTOR10 = 10;
const _SCALING_FACTOR11 = 10;
const _SCALING_FACTOR12 = 10;
const _SCALING_FACTOR13 = 10;
const _SCALING_FACTOR14 = 10;
const _SCALING_FACTOR15 = 10;
const _SCALING_FACTOR16 = 10;
const _SCALING_FACTOR17 = 10;
const _SCALING_FACTOR18 = 10;
const _SCALING_FACTOR19 = 10;

const _NORMALIZED_WEIGHT0 = 10;
const _NORMALIZED_WEIGHT1 = 10;
const _NORMALIZED_WEIGHT2 = 10;
const _NORMALIZED_WEIGHT3 = 10;
const _NORMALIZED_WEIGHT4 = 10;
const _NORMALIZED_WEIGHT5 = 10;
const _NORMALIZED_WEIGHT6 = 10;
const _NORMALIZED_WEIGHT7 = 10;
const _NORMALIZED_WEIGHT8 = 10;
const _NORMALIZED_WEIGHT9 = 10;
const _NORMALIZED_WEIGHT10 = 10;
const _NORMALIZED_WEIGHT11 = 10;
const _NORMALIZED_WEIGHT12 = 10;
const _NORMALIZED_WEIGHT13 = 10;
const _NORMALIZED_WEIGHT14 = 10;
const _NORMALIZED_WEIGHT15 = 10;
const _NORMALIZED_WEIGHT16 = 10;
const _NORMALIZED_WEIGHT17 = 10;
const _NORMALIZED_WEIGHT18 = 10;
const _NORMALIZED_WEIGHT19 = 10;


fn _get_total_tokens() -> u64 {
    return _TOTAL_TOKENS;
}


impl WeightedPool for Contract {
        // BaseWeightedPool(
        //     vault,
        //     name,
        //     symbol,
        //     tokens,
        //     assetManagers,
        //     swapFeePercentage,
        //     pauseWindowDuration,
        //     bufferPeriodDuration,
        //     owner,
        //     false
        // )
    // {
    //     uint256 numTokens = tokens.length;
    //     InputHelpers.ensureInputLengthMatch(numTokens, normalizedWeights.length);

    //     _TOTAL_TOKENS = numTokens;

    //     // Ensure each normalized weight is above the minimum
    //     uint256 normalizedSum = 0;
    //     for (uint8 i = 0; i < numTokens; i++) {
    //         uint256 normalizedWeight = normalizedWeights[i];

    //         _require(normalizedWeight >= WeightedMath._MIN_WEIGHT, Errors.MIN_WEIGHT);
    //         normalizedSum = normalizedSum.add(normalizedWeight);
    //     }
    //     // Ensure that the normalized weights sum to ONE
    //     _require(normalizedSum == FixedPoint.ONE, Errors.NORMALIZED_WEIGHT_INVARIANT);

    //     _NORMALIZED_WEIGHT0 = normalizedWeights[0];
    //     _NORMALIZED_WEIGHT1 = normalizedWeights[1];
    //     _NORMALIZED_WEIGHT2 = numTokens > 2 ? normalizedWeights[2] : 0;
    //     _NORMALIZED_WEIGHT3 = numTokens > 3 ? normalizedWeights[3] : 0;
    //     _NORMALIZED_WEIGHT4 = numTokens > 4 ? normalizedWeights[4] : 0;
    //     _NORMALIZED_WEIGHT5 = numTokens > 5 ? normalizedWeights[5] : 0;
    //     _NORMALIZED_WEIGHT6 = numTokens > 6 ? normalizedWeights[6] : 0;
    //     _NORMALIZED_WEIGHT7 = numTokens > 7 ? normalizedWeights[7] : 0;
    //     _NORMALIZED_WEIGHT8 = numTokens > 8 ? normalizedWeights[8] : 0;
    //     _NORMALIZED_WEIGHT9 = numTokens > 9 ? normalizedWeights[9] : 0;
    //     _NORMALIZED_WEIGHT10 = numTokens > 10 ? normalizedWeights[10] : 0;
    //     _NORMALIZED_WEIGHT11 = numTokens > 11 ? normalizedWeights[11] : 0;
    //     _NORMALIZED_WEIGHT12 = numTokens > 12 ? normalizedWeights[12] : 0;
    //     _NORMALIZED_WEIGHT13 = numTokens > 13 ? normalizedWeights[13] : 0;
    //     _NORMALIZED_WEIGHT14 = numTokens > 14 ? normalizedWeights[14] : 0;
    //     _NORMALIZED_WEIGHT15 = numTokens > 15 ? normalizedWeights[15] : 0;
    //     _NORMALIZED_WEIGHT16 = numTokens > 16 ? normalizedWeights[16] : 0;
    //     _NORMALIZED_WEIGHT17 = numTokens > 17 ? normalizedWeights[17] : 0;
    //     _NORMALIZED_WEIGHT18 = numTokens > 18 ? normalizedWeights[18] : 0;
    //     _NORMALIZED_WEIGHT19 = numTokens > 19 ? normalizedWeights[19] : 0;

    //     // Immutable variables cannot be initialized inside an if statement, so we must do conditional assignments
    //     _TOKEN0 = tokens[0];
    //     _TOKEN1 = tokens[1];
    //     _TOKEN2 = numTokens > 2 ? tokens[2] : IERC20(0);
    //     _TOKEN3 = numTokens > 3 ? tokens[3] : IERC20(0);
    //     _TOKEN4 = numTokens > 4 ? tokens[4] : IERC20(0);
    //     _TOKEN5 = numTokens > 5 ? tokens[5] : IERC20(0);
    //     _TOKEN6 = numTokens > 6 ? tokens[6] : IERC20(0);
    //     _TOKEN7 = numTokens > 7 ? tokens[7] : IERC20(0);
    //     _TOKEN8 = numTokens > 8 ? tokens[8] : IERC20(0);
    //     _TOKEN9 = numTokens > 9 ? tokens[9] : IERC20(0);
    //     _TOKEN10 = numTokens > 10 ? tokens[10] : IERC20(0);
    //     _TOKEN11 = numTokens > 11 ? tokens[11] : IERC20(0);
    //     _TOKEN12 = numTokens > 12 ? tokens[12] : IERC20(0);
    //     _TOKEN13 = numTokens > 13 ? tokens[13] : IERC20(0);
    //     _TOKEN14 = numTokens > 14 ? tokens[14] : IERC20(0);
    //     _TOKEN15 = numTokens > 15 ? tokens[15] : IERC20(0);
    //     _TOKEN16 = numTokens > 16 ? tokens[16] : IERC20(0);
    //     _TOKEN17 = numTokens > 17 ? tokens[17] : IERC20(0);
    //     _TOKEN18 = numTokens > 18 ? tokens[18] : IERC20(0);
    //     _TOKEN19 = numTokens > 19 ? tokens[19] : IERC20(0);

    //     _SCALING_FACTOR0 = _computeScalingFactor(tokens[0]);
    //     _SCALING_FACTOR1 = _computeScalingFactor(tokens[1]);
    //     _SCALING_FACTOR2 = numTokens > 2 ? _computeScalingFactor(tokens[2]) : 0;
    //     _SCALING_FACTOR3 = numTokens > 3 ? _computeScalingFactor(tokens[3]) : 0;
    //     _SCALING_FACTOR4 = numTokens > 4 ? _computeScalingFactor(tokens[4]) : 0;
    //     _SCALING_FACTOR5 = numTokens > 5 ? _computeScalingFactor(tokens[5]) : 0;
    //     _SCALING_FACTOR6 = numTokens > 6 ? _computeScalingFactor(tokens[6]) : 0;
    //     _SCALING_FACTOR7 = numTokens > 7 ? _computeScalingFactor(tokens[7]) : 0;
    //     _SCALING_FACTOR8 = numTokens > 8 ? _computeScalingFactor(tokens[8]) : 0;
    //     _SCALING_FACTOR9 = numTokens > 9 ? _computeScalingFactor(tokens[9]) : 0;
    //     _SCALING_FACTOR10 = numTokens > 10 ? _computeScalingFactor(tokens[10]) : 0;
    //     _SCALING_FACTOR11 = numTokens > 11 ? _computeScalingFactor(tokens[11]) : 0;
    //     _SCALING_FACTOR12 = numTokens > 12 ? _computeScalingFactor(tokens[12]) : 0;
    //     _SCALING_FACTOR13 = numTokens > 13 ? _computeScalingFactor(tokens[13]) : 0;
    //     _SCALING_FACTOR14 = numTokens > 14 ? _computeScalingFactor(tokens[14]) : 0;
    //     _SCALING_FACTOR15 = numTokens > 15 ? _computeScalingFactor(tokens[15]) : 0;
    //     _SCALING_FACTOR16 = numTokens > 16 ? _computeScalingFactor(tokens[16]) : 0;
    //     _SCALING_FACTOR17 = numTokens > 17 ? _computeScalingFactor(tokens[17]) : 0;
    //     _SCALING_FACTOR18 = numTokens > 18 ? _computeScalingFactor(tokens[18]) : 0;
    //     _SCALING_FACTOR19 = numTokens > 19 ? _computeScalingFactor(tokens[19]) : 0;
    // }

    fn _get_normalized_weight(token: ContractId) -> u64 {
        // prettier-ignore
        if (token == ~ContractId::from(_TOKEN0)) { return _NORMALIZED_WEIGHT0; }
        else if (token == ~ContractId::from(_TOKEN1)) { return _NORMALIZED_WEIGHT1; }
        else if (token == ~ContractId::from(_TOKEN2)) { return _NORMALIZED_WEIGHT2; }
        else if (token == ~ContractId::from(_TOKEN3)) { return _NORMALIZED_WEIGHT3; }
        else if (token == ~ContractId::from(_TOKEN4)) { return _NORMALIZED_WEIGHT4; }
        else if (token == ~ContractId::from(_TOKEN5)) { return _NORMALIZED_WEIGHT5; }
        else if (token == ~ContractId::from(_TOKEN6)) { return _NORMALIZED_WEIGHT6; }
        else if (token == ~ContractId::from(_TOKEN7)) { return _NORMALIZED_WEIGHT7; }
        else if (token == ~ContractId::from(_TOKEN8)) { return _NORMALIZED_WEIGHT8; }
        else if (token == ~ContractId::from(_TOKEN9)) { return _NORMALIZED_WEIGHT9; }
        else if (token == ~ContractId::from(_TOKEN10)) { return _NORMALIZED_WEIGHT10; }
        else if (token == ~ContractId::from(_TOKEN11)) { return _NORMALIZED_WEIGHT11; }
        else if (token == ~ContractId::from(_TOKEN12)) { return _NORMALIZED_WEIGHT12; }
        else if (token == ~ContractId::from(_TOKEN13)) { return _NORMALIZED_WEIGHT13; }
        else if (token == ~ContractId::from(_TOKEN14)) { return _NORMALIZED_WEIGHT14; }
        else if (token == ~ContractId::from(_TOKEN15)) { return _NORMALIZED_WEIGHT15; }
        else if (token == ~ContractId::from(_TOKEN16)) { return _NORMALIZED_WEIGHT16; }
        else if (token == ~ContractId::from(_TOKEN17)) { return _NORMALIZED_WEIGHT17; }
        else if (token == ~ContractId::from(_TOKEN18)) { return _NORMALIZED_WEIGHT18; }
        else if (token == ~ContractId::from(_TOKEN19)) { return _NORMALIZED_WEIGHT19; }
        else {
            // require(false, "INVALID_TOKEN");
            let tmp = 0;
            return tmp;
        }
    }


    fn _get_normalized_weights() -> Vec<u64> {
        let totalTokens = _get_total_tokens();
        let mut normalizedWeights = ~Vec::new();

        normalizedWeights.push(_NORMALIZED_WEIGHT0);
        normalizedWeights.push(_NORMALIZED_WEIGHT1);
        if (totalTokens > 2) { normalizedWeights.push(_NORMALIZED_WEIGHT2); }
        if (totalTokens > 3) { normalizedWeights.push(_NORMALIZED_WEIGHT3); }
        if (totalTokens > 4) { normalizedWeights.push(_NORMALIZED_WEIGHT4); }
        if (totalTokens > 5) { normalizedWeights.push(_NORMALIZED_WEIGHT5); }
        if (totalTokens > 6) { normalizedWeights.push(_NORMALIZED_WEIGHT6); }
        if (totalTokens > 7) { normalizedWeights.push(_NORMALIZED_WEIGHT7); }
        if (totalTokens > 8) { normalizedWeights.push(_NORMALIZED_WEIGHT8); }
        if (totalTokens > 9) { normalizedWeights.push(_NORMALIZED_WEIGHT9); }
        if (totalTokens > 11) { normalizedWeights.push(_NORMALIZED_WEIGHT11); }
        if (totalTokens > 10) { normalizedWeights.push(_NORMALIZED_WEIGHT10); }
        if (totalTokens > 12) { normalizedWeights.push(_NORMALIZED_WEIGHT12); }
        if (totalTokens > 13) { normalizedWeights.push(_NORMALIZED_WEIGHT13); }
        if (totalTokens > 14) { normalizedWeights.push(_NORMALIZED_WEIGHT14); }
        if (totalTokens > 15) { normalizedWeights.push(_NORMALIZED_WEIGHT15); }
        if (totalTokens > 16) { normalizedWeights.push(_NORMALIZED_WEIGHT16); }
        if (totalTokens > 17) { normalizedWeights.push(_NORMALIZED_WEIGHT17); }
        if (totalTokens > 18) { normalizedWeights.push(_NORMALIZED_WEIGHT18); }
        if (totalTokens > 19) { normalizedWeights.push(_NORMALIZED_WEIGHT19); }

        let mut count = totalTokens;
        while count < normalizedWeights.len() {
            normalizedWeights.push(0);
        }
        return normalizedWeights;
    }

    fn _get_max_tokens() -> u64 {
        return _MAX_TOKENS;
    }

    /**
     * @dev Returns the scaling factor for one of the Pool's tokens. Reverts if `token` is not a token registered by the
     * Pool.
     */
    fn _scaling_factor(token: ContractId) -> u64 {
        // prettier-ignore
        if (token == ~ContractId::from(_TOKEN0)) { return _SCALING_FACTOR0; }
        else if (token == ~ContractId::from(_TOKEN1)) { return _SCALING_FACTOR1; }
        else if (token == ~ContractId::from(_TOKEN2)) { return _SCALING_FACTOR2; }
        else if (token == ~ContractId::from(_TOKEN3)) { return _SCALING_FACTOR3; }
        else if (token == ~ContractId::from(_TOKEN4)) { return _SCALING_FACTOR4; }
        else if (token == ~ContractId::from(_TOKEN5)) { return _SCALING_FACTOR5; }
        else if (token == ~ContractId::from(_TOKEN6)) { return _SCALING_FACTOR6; }
        else if (token == ~ContractId::from(_TOKEN7)) { return _SCALING_FACTOR7; }
        else if (token == ~ContractId::from(_TOKEN8)) { return _SCALING_FACTOR8; }
        else if (token == ~ContractId::from(_TOKEN9)) { return _SCALING_FACTOR9; }
        else if (token == ~ContractId::from(_TOKEN10)) { return _SCALING_FACTOR10; }
        else if (token == ~ContractId::from(_TOKEN11)) { return _SCALING_FACTOR11; }
        else if (token == ~ContractId::from(_TOKEN12)) { return _SCALING_FACTOR12; }
        else if (token == ~ContractId::from(_TOKEN13)) { return _SCALING_FACTOR13; }
        else if (token == ~ContractId::from(_TOKEN14)) { return _SCALING_FACTOR14; }
        else if (token == ~ContractId::from(_TOKEN15)) { return _SCALING_FACTOR15; }
        else if (token == ~ContractId::from(_TOKEN16)) { return _SCALING_FACTOR16; }
        else if (token == ~ContractId::from(_TOKEN17)) { return _SCALING_FACTOR17; }
        else if (token == ~ContractId::from(_TOKEN18)) { return _SCALING_FACTOR18; }
        else if (token == ~ContractId::from(_TOKEN19)) { return _SCALING_FACTOR19; }
        else {
            // require(false, "INVALID_TOKEN");
            let tmp = 0;
            return tmp;
        }
    }

    fn _scaling_factors() -> Vec<u64> {
        let totalTokens = _get_total_tokens();
        let mut scalingFactors = ~Vec::new();

        scalingFactors.push(_SCALING_FACTOR0);
        scalingFactors.push(_SCALING_FACTOR1);
        if (totalTokens > 2) { scalingFactors.push(_SCALING_FACTOR2); } 
        if (totalTokens > 3) { scalingFactors.push(_SCALING_FACTOR3); } 
        if (totalTokens > 4) { scalingFactors.push(_SCALING_FACTOR4); } 
        if (totalTokens > 5) { scalingFactors.push(_SCALING_FACTOR5); } 
        if (totalTokens > 6) { scalingFactors.push(_SCALING_FACTOR6); } 
        if (totalTokens > 7) { scalingFactors.push(_SCALING_FACTOR7); } 
        if (totalTokens > 8) { scalingFactors.push(_SCALING_FACTOR8); } 
        if (totalTokens > 9) { scalingFactors.push(_SCALING_FACTOR9); } 
        if (totalTokens > 10) { scalingFactors.push(_SCALING_FACTOR10); } 
        if (totalTokens > 11) { scalingFactors.push(_SCALING_FACTOR11); } 
        if (totalTokens > 12) { scalingFactors.push(_SCALING_FACTOR12); } 
        if (totalTokens > 13) { scalingFactors.push(_SCALING_FACTOR13); } 
        if (totalTokens > 14) { scalingFactors.push(_SCALING_FACTOR14); } 
        if (totalTokens > 15) { scalingFactors.push(_SCALING_FACTOR15); } 
        if (totalTokens > 16) { scalingFactors.push(_SCALING_FACTOR16); } 
        if (totalTokens > 17) { scalingFactors.push(_SCALING_FACTOR17); } 
        if (totalTokens > 18) { scalingFactors.push(_SCALING_FACTOR18); } 
        if (totalTokens > 19) { scalingFactors.push(_SCALING_FACTOR19); } 

        let mut count = scalingFactors.len();
        while count < totalTokens {
            scalingFactors.push(0);
            count = count + 1;
        }

        return scalingFactors;
    }

    // InvariantGrowthProtocolFees

    fn _before_join_exit(
        preBalances: Vec<u64>,
        normalizedWeights: Vec<u64>,
        protocolSwapFeePercentage: u64
    ) {
        let x = abi(InvariantGrowthProtocolFees, InvariantGrowthProtocolFees_contract_id);
        x._before_join_exit(preBalances, normalizedWeights, protocolSwapFeePercentage);
    }

    fn _after_join_exit(
        isJoin: bool,
        preBalances: Vec<u64>,
        balanceDeltas: Vec<u64>,
        normalizedWeights: Vec<u64>
    ) {
        let x = abi(InvariantGrowthProtocolFees, InvariantGrowthProtocolFees_contract_id);
        x._after_join_exit(isJoin, preBalances, balanceDeltas, normalizedWeights);
    }
}
