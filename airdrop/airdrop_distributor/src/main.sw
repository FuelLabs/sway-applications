contract;

dep abi;
dep data_structures;
dep events;
dep errors;
dep utils;

use abi::AirdropDistributor;
use data_structures::{AirdropData, Claim, State};
use events::{ClaimEvent, CreateEvent, ReClaimEvent};
use errors::{AccessError, InitError, StateError, VerificationError};
use utils::{create_claim_hash, sender_identity, verify_merkle_proof};
use std::{
    assert::require,
    block::height,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    storage::StorageMap,
    vec::Vec,
    token::transfer,
};

storage {
    airdrops: StorageMap<(ContractId, u64), Option<AirdropData>>,
    airdrop_count: StorageMap<ContractId, u64>,
    // TODO: This should be moved into the `AirdropDispersal` struct when strorage maps in structs
    // are supported
    claimed: StorageMap<(b256, ContractId, u64), bool>,
}

impl AirdropDistributor for Contract {
    /// This function will let users claim their airdrop
    ///
    /// # Reverts
    ///
    /// * When the airdrop does not map to an existing Airdrop.
    /// * When the claiming period has ended.
    /// * When the `to` `Identity` has already claimed.
    /// * When the merkle proof verification failed.
    #[storage(read, write)]fn claim(to: Identity, amount: u64, proof: Vec<b256>, token: ContractId, claim_id: u64) {
        let airdrop: Option<AirdropData> = storage.airdrops.get((token, claim_id));
        require(airdrop.is_some(), AccessError::AirdropDoesNotExist);
        let mut airdrop = airdrop.unwrap();
        
        let claim_hash = create_claim_hash(to, amount);
        require(!storage.claimed.get((claim_hash, token, claim_id)), AccessError::UserAlreadyClaimed);

        // Verify valid leaf
        require(verify_merkle_proof(airdrop.merkleRoot, claim_hash, proof), VerificationError::MerkleProofFailed);

        // Decrease the amount of tokens left
        airdrop.claim_remaining = airdrop.claim_remaining - amount;

        // Update the airdrop information
        storage.claimed.insert((claim_hash, token, claim_id), true);
        storage.airdrops.insert((token, claim_id), Option::Some(airdrop));

        // Send tokens to the Identity claiming
        transfer(amount, token, to);

        // Event
        log(ClaimEvent {
            to, amount, token, claim_id,
        });
    }

    /// Initalizes the contract
    ///
    /// # Reverts
    ///
    /// * The `claim_time` is set to zero.
    /// * The token amount provided in the transaction is zero.
    /// * The `token_contract` does not match the tokens provided in the transaction.
    #[storage(read, write)]fn create(token_contract: ContractId, merkleRoot: b256, admin: Identity, claim_time: u64) -> u64 {
        require(claim_time != 0, InitError::ClaimTimeCannotBeZero);
        require(msg_amount() != 0, InitError::AirdropAmountCannotBeZero);
        require(msg_asset_id() == token_contract, InitError::IncorrectTokenContract);
        
        let airdrop = AirdropData {
            admin,
            claim_remaining: msg_amount(),
            end_block: height() + claim_time,
            merkleRoot,
            state: State::Open,
            token_contract,
        };

        // Store the airdrop data
        storage.airdrops.insert((token_contract, storage.airdrop_count.get(token_contract)), Option::Some(airdrop));

        // Log Event
        log(CreateEvent {
            airdrop, claim_id: storage.airdrop_count.get(token_contract),
        });

        // Update the airdrop count and return the airdrop id
        storage.airdrop_count.insert(token_contract, storage.airdrop_count.get(token_contract) + 1);
        storage.airdrop_count.get(token_contract) - 1
    }

    /// This function will return the remaining tokens once the claim period has ended
    ///
    /// # Reverts
    /// 
    /// * When the `token_contract` and `claim_id` do not map to an airdrop
    /// * When the claim period has not ended
    /// * When the sender is not the admin
    #[storage(read, write)]fn reclaim(token_contract: ContractId, claim_id: u64) {
        // Make sure this airdrop exists
        let airdrop: Option<AirdropData> = storage.airdrops.get((token_contract, claim_id));
        require(airdrop.is_some(), AccessError::AirdropDoesNotExist);
        let mut airdrop = airdrop.unwrap();

        // Make sure the claiming period has ended
        require(height() >= airdrop.end_block, StateError::ClaimPeriodHasNotEnded);

        // Make sure that this is the admin
        let sender = sender_identity();
        require(sender == airdrop.admin, AccessError::SenderNotAdmin);

        // Update the auction states
        let claim_remaining = airdrop.claim_remaining;
        airdrop.state = State::Closed;
        airdrop.claim_remaining = 0;
        storage.airdrops.insert((token_contract, claim_id), Option::Some(airdrop));

        // Transfer tokens out of this contract
        transfer(claim_remaining, token_contract, sender);

        // Log Event
        log(ReClaimEvent {
            airdrop, claim_id,
        });
    }
}
