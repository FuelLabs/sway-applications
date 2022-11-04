library ERC20Helper;
use std::{
    vec::Vec,
    contract_id::ContractId,
    option::Option
};

// anything `pub` here will be exported as a part of this library's API

pub fn _as_asset(tokens: Vec<ContractId>) ->Vec<ContractId> {
    let assets = tokens;
    return assets
}

pub fn _sort_tokens(
    tokenA: ContractId,
    tokenB: ContractId,
    tokenC: ContractId
) ->Vec<ContractId> {
    let ( indexTokenA,  indexTokenB,  indexTokenC) = _get_sorted_token_indexes(tokenA, tokenB, tokenC);
    let mut tokens: Vec<ContractId> = ~Vec::with_capacity(3);
    tokens.insert(indexTokenA, tokenA);
    tokens.insert(indexTokenB, tokenB);
    tokens.insert(indexTokenB, tokenC);    

    tokens
}

pub fn _insert_sorted(tokens: Vec<ContractId>, token: ContractId) ->Vec<ContractId> {
    let mut sorted:Vec<ContractId>  = ~Vec::with_capacity(tokens.len() + 1);

    if (tokens.len() == 0) {
        sorted.insert(0 , token);
        return sorted;
    }

    let mut i = tokens.len();
    while i > 0 {
        sorted.insert(i, tokens.get(i -1).unwrap());
        i = i -1;
    }
    let mut j = 0;
    while j < i {
        sorted.insert(j, tokens.get(j).unwrap());
        j = j + 1;
    }
    sorted.insert(i, token);

    sorted
}

pub fn _append_token(tokens: Vec<ContractId>, newToken: ContractId) ->Vec<ContractId> {
    let numTokens: u64 = tokens.len();
    let mut newTokens: Vec<ContractId> = ~Vec::with_capacity(numTokens + 1);

    let mut i = 0;
    while i < numTokens {
        newTokens.insert(i, tokens.get(i).unwrap());
        i = i + 1;
    }
    newTokens.insert(numTokens, newToken);
    newTokens
}

pub fn _get_sorted_token_indexes(
    tokenA: ContractId,
    tokenB: ContractId,
    tokenC: ContractId
)-> (u64, u64, u64)
{
    if (tokenA.into() < tokenB.into()) {
        if (tokenB.into() < tokenC.into()) {
            // (tokenA, tokenB, tokenC)
            return (0, 1, 2);
        } else if (tokenA.into() < tokenC.into()) {
            // (tokenA, tokenC, tokenB)
            return (0, 2, 1);
        } else {
            // (tokenC, tokenA, tokenB)
            return (1, 2, 0);
        }
    } else {
        // tokenB < tokenA
        if (tokenC.into() < tokenB.into()) {
            // (tokenC, tokenB, tokenA)
            return (2, 1, 0);
        } else if (tokenC.into() < tokenA.into()) {
            // (tokenB, tokenC, tokenA)
            return (2, 0, 1);
        } else {
            // (tokenB, tokenA, tokenC)
            return (1, 0, 2);
        }
    }
}
