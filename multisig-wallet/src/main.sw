contract;

use std::{
    address::Address,
    assert::assert,
    chain::auth::{AuthError, Sender, msg_sender},
    contract_id::ContractId,
    hash::{HashMethod, hash_u64, hash_pair},
    panic::panic, 
    result::*,
    storage::{store, get},
};

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address, threshold: u64) -> bool;
    fn submit(destination: ContractId, value: u64, data: b256) -> bool;
    fn approve(nonce: u64) -> bool;
    fn revokeApproval(nonce: u64) -> bool;
    fn execute(nonce: u64) -> bool;
    fn isOwner(owner: Address) -> bool;
    fn getTransaction(nonce: u64) -> (b256, u64, b256, bool, u64);
}

struct SentinelTransaction {
    to: b256,
    value: b256,
    data: b256,
    executed: b256,
    approvals: b256
}

// TODO: add logging events

storage {
    state: u64,
    nonce: u64,
    threshold: u64,
    sentinel: SentinelTransaction,
}

impl MultiSignatureWallet for Contract {
    
    fn constructor(owner1: Address, owner2: Address, threshold: u64) -> bool {
        assert(storage.state == 0);

        // TODO: when vectors are implemented change owners to be a Vec<Address>
        store(owner1.value, true);
        store(owner2.value, true);

        storage.sentinel = SentinelTransaction {
            to:        hash_u64(1, HashMethod::Sha256),
            value:     hash_u64(2, HashMethod::Sha256),
            data:      hash_u64(3, HashMethod::Sha256),
            executed:  hash_u64(4, HashMethod::Sha256),
            approvals: hash_u64(5, HashMethod::Sha256)
        };

        storage.threshold = threshold;
        storage.state = 1;
        true
    }

    fn submit(destination: ContractId, value: u64, data: b256) -> bool {
        assert(storage.state == 1);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            assert(get(address.value));

            storage.nonce = storage.nonce + 1;

            let nonce_hash = hash_u64(storage.nonce, HashMethod::Sha256);

            store(hash_pair(nonce_hash, storage.sentinel.to, HashMethod::Sha256), destination.value);
            store(hash_pair(nonce_hash, storage.sentinel.value, HashMethod::Sha256), value);
            store(hash_pair(nonce_hash, storage.sentinel.data, HashMethod::Sha256), data);
        } else {
            panic(0);
        };

        true
    }

    fn approve(nonce: u64) -> bool {
        assert(storage.state == 1);
        assert(nonce != 0 && nonce <= storage.nonce);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            assert(get(address.value));

            let nonce_hash = hash_u64(nonce, HashMethod::Sha256);
            let has_approved: bool = get(hash_pair(nonce_hash, address.value, HashMethod::Sha256));

            assert(!has_approved);
            assert(!get(hash_pair(nonce_hash, storage.sentinel.executed, HashMethod::Sha256)));

            let approvals: u64 = get(hash_pair(nonce_hash, storage.sentinel.approvals, HashMethod::Sha256));

            store(hash_pair(nonce_hash, storage.sentinel.approvals, HashMethod::Sha256), approvals + 1);
            store(hash_pair(nonce_hash, address.value, HashMethod::Sha256), true);
        } else {
            panic(0);
        };

        true
    }

    fn revokeApproval(nonce: u64) -> bool {
        assert(storage.state == 1);
        assert(nonce != 0 && nonce <= storage.nonce);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            assert(get(address.value));

            let nonce_hash = hash_u64(nonce, HashMethod::Sha256);
            let has_approved: bool = get(hash_pair(nonce_hash, address.value, HashMethod::Sha256));

            assert(has_approved);
            assert(!get(hash_pair(nonce_hash, storage.sentinel.executed, HashMethod::Sha256)));

            let approvals: u64 = get(hash_pair(nonce_hash, storage.sentinel.approvals, HashMethod::Sha256));

            store(hash_pair(nonce_hash, storage.sentinel.approvals, HashMethod::Sha256), approvals - 1);
            store(hash_pair(nonce_hash, address.value, HashMethod::Sha256), false);
        } else {
            panic(0);
        };

        true
    }

    fn execute(nonce: u64) -> bool {
        assert(storage.state == 1);
        assert(nonce != 0 && nonce <= storage.nonce);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            assert(get(address.value));

            let nonce_hash = hash_u64(nonce, HashMethod::Sha256);
            let executed_hash = hash_pair(nonce_hash, storage.sentinel.executed, HashMethod::Sha256);

            // TODO: check approvals against threshold

            assert(!get(executed_hash));
            
            // TODO: execute
            
            store(executed_hash, true);
        } else {
            panic(0);
        };

        true
    }

    fn isOwner(owner: Address) -> bool {
        get(owner.value)
    }

    fn getTransaction(nonce: u64) -> (b256, u64, b256, bool, u64) {
        assert(nonce != 0 && nonce <= storage.nonce);

        let nonce_hash = hash_u64(nonce, HashMethod::Sha256);

        let to: b256 = get(hash_pair(nonce_hash, storage.sentinel.to, HashMethod::Sha256));
        let value: u64 = get(hash_pair(nonce_hash, storage.sentinel.value, HashMethod::Sha256));
        let data: b256 = get(hash_pair(nonce_hash, storage.sentinel.data, HashMethod::Sha256));
        let executed: bool = get(hash_pair(nonce_hash, storage.sentinel.executed, HashMethod::Sha256));
        let approvals: u64 = get(hash_pair(nonce_hash, storage.sentinel.approvals, HashMethod::Sha256));

        (to, value, data, executed, approvals)
    }
}
