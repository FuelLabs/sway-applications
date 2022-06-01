contract;

use std::{
    address::Address,
    assert::require,
    b512::B512,
    chain::auth::Sender,
    context::{call_frames::contract_id, this_balance},
    contract_id::ContractId,
    ecr::{EcRecoverError, ec_recover_address},
    hash::sha256,
    logging::log,
    storage::StorageMap,
    result::*,
    revert::revert,
    token::{force_transfer, transfer_to_output}
};

use core::num::*;

abi MultiSignatureWallet {
    fn constructor(owner1: User, owner2: User, threshold: u64) -> bool;
    fn execute_transaction(to: Sender, value: u64, data: b256, signatures: [B512; 2]) -> bool;
    fn transfer(to: Sender, asset_id: ContractId, value: u64, data: b256, signatures: [B512; 2]) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn balance(asset_id: ContractId) -> u64;
    fn get_transaction_hash(to: Sender, value: u64, data: b256, nonce: u64) -> b256;
}

enum Error {
    ApprovalThresholdNotReached: (),
    CannotReinitialize: (),
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    NotAnOwner: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    WeightingCannotBeZero: (),
}

struct ExecutedEvent {
    to: Sender, 
    value: u64, 
    data: b256,
    nonce: u64
}

struct TransferEvent {
    to: Sender, 
    asset_id: ContractId, 
    value: u64,
    nonce: u64
}

struct User {
    identity: Sender,
    weight: u64
}

struct Tx {
    destination: b256,
    value: u64,
    data: b256,
    nonce: u64,
    contract_identifier: b256,
}

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions
    nonce: u64,

    /// The number of approvals (signatures * weight) required in order to execture a Tx
    threshold: u64,

    weighting: StorageMap<b256, u64>,
}

impl MultiSignatureWallet for Contract {
    /// Initializes the contract by setting the owners, owner weightings and initializes the nonce
    ///
    /// # Panics
    ///
    /// - When the constructor is called more than once
    /// - When the threshold is set to 0
    /// - When an owner has an approval weight of 0
    fn constructor(owner1: User, owner2: User, threshold: u64) -> bool {
        // TODO: when vectors are implemented change owners to be a Vec<Address>

        require(storage.nonce == 0, Error::CannotReinitialize);
        require(storage.threshold != 0, Error::ThresholdCannotBeZero);
        require(owner1.weight != 0, Error::WeightingCannotBeZero);
        require(owner2.weight != 0, Error::WeightingCannotBeZero);

        storage.weighting.insert(_get_address(owner1.identity), owner1.weight);
        storage.weighting.insert(_get_address(owner2.identity), owner2.weight);

        storage.nonce = 1;
        storage.threshold = threshold;
        true
    }

    /// Executes a Tx if the required signatures meet the restrictions on ownership and threshold approval
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the signer is not an owner
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    fn execute_transaction(to: Sender, value: u64, data: b256, signatures: [B512; 2]) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);

        let tx_hash = _get_transaction_hash(to, value, data, storage.nonce, contract_id());

        // The signers must have increasing values in order to check for duplicates or a zero-value
        let mut previous_signer = ~b256::min();

        let mut approval_count = 0;
        let mut index = 0;
        while index < 2 {
            let signer = match ec_recover_address(signatures[index], tx_hash) {
                Result::Ok(address) => address.value, _ => revert(42),
            };

            require(previous_signer < signer, Error::IncorrectSignerOrdering);

            let signer_weight = storage.weighting.get(signer);
            require(signer_weight != 0, Error::NotAnOwner);

            previous_signer = signer;
            approval_count = approval_count + signer_weight;

            // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
            // if storage.threshold <= approval_count {
            //     break;
            // }

            index = index + 1;
        }

        require(storage.threshold <= approval_count, Error::ApprovalThresholdNotReached);

        storage.nonce = storage.nonce + 1;

        // TODO: Execute https://github.com/FuelLabs/sway-applications/issues/22

        log(ExecutedEvent { to, value, data, nonce: storage.nonce - 1 });

        true
    }

    /// Transfers assets to outputs & contracts
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the balance of the asset being sent is less than the balance in the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the signer is not an owner
    /// - When the total approval count is less than the required threshold for execution
    fn transfer(to: Sender, asset_id: ContractId, value: u64, data: b256, signatures: [B512; 2]) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);
        require(value <= this_balance(asset_id), Error::InsufficientAssetAmount);

        let tx_hash = _get_transaction_hash(to, value, data, storage.nonce, contract_id());

        // The signers must have increasing values in order to check for duplicates or a zero-value
        let mut previous_signer = ~b256::min();

        let mut approval_count = 0;
        let mut index = 0;
        while index < 2 {
            let signer = match ec_recover_address(signatures[index], tx_hash) {
                Result::Ok(address) => address.value, _ => revert(42),
            };

            require(previous_signer < signer, Error::IncorrectSignerOrdering);

            let signer_weight = storage.weighting.get(signer);
            require(signer_weight != 0, Error::NotAnOwner);

            previous_signer = signer;
            approval_count = approval_count + signer_weight;

            // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
            // if storage.threshold <= approval_count {
            //     break;
            // }

            index = index + 1;
        }

        require(storage.threshold <= approval_count, Error::ApprovalThresholdNotReached);

        storage.nonce = storage.nonce + 1;

        match to {
            Sender::Address(address) => transfer_to_output(value, asset_id, address), Sender::ContractId(contract) => force_transfer(value, asset_id, contract), 
        };

        log(TransferEvent { to, asset_id, value, nonce: storage.nonce - 1 });

        true
    }

    /// Returns a boolean value indicating if the given address is an owner in the contract
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    fn is_owner(address: Address) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);
        storage.weighting.get(address.value) != 0
    }

    /// Returns the balance of the specified asset_id for this contract
    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    /// Used for verification of message
    fn get_transaction_hash(to: Sender, value: u64, data: b256, nonce: u64) -> b256 {
        // TODO: data > b256?
        _get_transaction_hash(to, value, data, nonce, contract_id())
    }
}

fn _get_transaction_hash(to: Sender, value: u64, data: b256, nonce: u64, self_id: ContractId) -> b256 {
    match to {
        Sender::Address(address) => sha256(Tx {
            contract_identifier: self_id.value, destination: address.value, value, data, nonce
        }), Sender::ContractId(asset_id) => sha256(Tx {
            contract_identifier: self_id.value, destination: asset_id.value, value, data, nonce
        }), 
    }
}

fn _get_address(user: Sender) -> b256 {
    match user {
        Sender::Address(address) => address.value, Sender::ContractId(address) => address.value, 
    }
}