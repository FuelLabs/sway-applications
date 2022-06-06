contract;

// Our library dependencies
dep abi;
dep errors;
dep events;
dep data_structures;

// Standard library code
use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    // option::*, // enums not supported in storage
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

// Bring our code into scope
use abi::Escrow;
use errors::{AccessError, ApproveError, DepositError, InitError, StateError};
use events::{ApproveEvent, DepositEvent, ThresholdReachedEvent, WithdrawEvent};
use data_structures::{Asset, User};

// TODO: add enums when they are supported in storage
// enum State {
//     Void: (),
//     Pending: (),
//     Completed: (),
// }

storage {
    /// The asset and amount of asset the deposit() function will accept
    assets: StorageMap<ContractId, u64>, 
    
    /// Current number of successful calls to approve(), used to lock the contract at completion
    approval_count: u64,

    /// Default value used to indicate that a user is not a valid user in the contract
    sentinel: User,

    /// Mechanism used to manage the control flow of the contract
    // state: State // enum Eq not implemented for self
    state: u64,

    /// Required number of successful calls to approve() to mark the workflow as complete
    threshold: u64,

    /// State associated with the activity of each user
    users: StorageMap<Identity, User>, 
}

impl Escrow for Contract {
    /// Initializes the escrow with the users and the assets that the contract will accept
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor is called more than once
    /// - The amount of any asset being set is equal to 0
    /// - Any asset is the NATIVE_ASSET_ID
    fn constructor(users: [Identity; 2], assets: [Asset; 2]) {
        // require(storage.state == State::Void, Error::InitError(InitError::CannotReinitialize));
        require(storage.state == 0, InitError::CannotReinitialize);

        // Set the assets that this contract accepts
        let mut asset_index = 0;
        while asset_index < 2 {
            require(0 < assets[asset_index].amount, InitError::AssetAmountCannotBeZero);
            require(~ContractId::from(NATIVE_ASSET_ID) != assets[asset_index].id, InitError::AssetIdCannotBeZero);

            storage.assets.insert(assets[asset_index].id, assets[asset_index].amount);
        }

        // Define a sentinel to distinguish users in the contract vs random users
        storage.sentinel = User {
            approved: false, 
            asset: ~ContractId::from(NATIVE_ASSET_ID), // asset: Option::None::<ContractId>(), // enums not supported in storage
            exists: false, 
            deposited: false
        };

        // Set the users that can interact with the escrow
        // Notice the "exists" field is different from the sentinel
        let mut user_index = 0;
        while user_index < 2 {
            storage.users.insert(users[user_index], User {
                approved: false, 
                asset: ~ContractId::from(NATIVE_ASSET_ID), 
                // asset: Option::None::<ContractId>(), // enums not supported in storage
                exists: true, 
                deposited: false
            });
        }

        // TODO: when Vec is out, get its length and assign
        // Set the threshold to be the number of users
        // All users must approve
        storage.threshold = 2;

        // Flip the state to prevent future reinitialization
        storage.state = 1;
        // storage.state = State::Pending;
    }

    /// Accepts a deposit from an authorized user for any of the specified assets
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The required number of approvals has been reached and another deposit is made
    /// - The user is not an authorized user
    /// - The user deposits when they still have their previous deposit in the escrow
    /// - The user deposits an asset that has not been specified in the constructor
    /// - The user sends an incorrect amount of an asset for the specified asset in the constructor
    fn deposit() {
        // require(storage.state == State::Pending, Error::StateError(StateError::StateNotPending));
        require(storage.state == 1, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data != storage.sentinel, AccessError::UnauthorizedUser);
        require(!user_data.deposited, DepositError::AlreadyDeposited);

        let deposited_asset = msg_asset_id();
        let required_amount = storage.assets.get(deposited_asset);

        require(required_amount != 0, DepositError::IncorrectAssetDeposited);
        require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

        user_data.asset = deposited_asset;
        user_data.deposited = true;

        storage.users.insert(sender.unwrap(), user_data);

        log(DepositEvent {user: sender.unwrap(), asset: deposited_asset, amount: required_amount});
    }

    /// Updates the user state to indicate a user has approved
    /// Once all of the users approve the escrow will lock the approval and deposit functions leaving
    /// withdrawal as the last function unlocked
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The required number of approvals has been reached and another approval is made
    /// - The user is not an authorized user
    /// - The user has not successfully deposited through the deposit() function
    /// - The user approves again after they have already approved
    fn approve() {
        // require(storage.state == State::Pending, Error::StateNotPending);
        require(storage.state == 1, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data != storage.sentinel, AccessError::UnauthorizedUser);
        require(user_data.deposited, DepositError::DepositRequired);
        require(!user_data.approved, ApproveError::AlreadyApproved);

        user_data.approved = true;

        storage.users.insert(sender.unwrap(), user_data);
        storage.approval_count = storage.approval_count + 1;

        log(ApproveEvent {user: sender.unwrap(), count: storage.approval_count});

        if storage.threshold <= storage.approval_count {
            // storage.state = State::Completed;
            storage.state = 2;
            log(ThresholdReachedEvent {});
        }
    }

    /// Returns the deposited asset back to the user and resets their approval to false
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    /// - The user is not an authorized user
    /// - The user has not successfully deposited through the deposit() function
    /// - The user attemps to withdraw after they have withdrawn and/or the contract is locked
    fn withdraw() {
        // require(storage.state == State::Pending, Error::StateError(StateError::StateNotPending));
        require(storage.state == 1 || storage.state == 2, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data != storage.sentinel, AccessError::UnauthorizedUser);
        require(user_data.deposited, DepositError::DepositRequired);

        let deposited_asset = user_data.asset;
        let required_amount = storage.assets.get(deposited_asset);

        user_data.asset = ~ContractId::from(NATIVE_ASSET_ID);
        user_data.deposited = false;
        user_data.approved = false;

        storage.users.insert(sender.unwrap(), user_data);

        if storage.state == 1 {
            storage.approval_count = storage.approval_count - 1;
        }

        match sender.unwrap() {
            Identity::Address(address) => {
                transfer_to_output(storage.assets.get(user_data.asset), user_data.asset, address);
            },
            Identity::ContractId(address) => {
                force_transfer_to_contract(storage.assets.get(user_data.asset), user_data.asset, address);
            }
        }

        log(WithdrawEvent {
            user: sender.unwrap(), 
            asset: deposited_asset, 
            amount: required_amount, 
            approval_count: storage.approval_count
        });
    }

    /// Returns a boolean indicating whether the asset has been specified in the constructor, a u64
    /// indicating the amount of asset currently in the contract
    fn get_balance(asset: ContractId) -> (bool, u64) {
        // TODO: once Vec is implemented return a Vec of Asset structs where each asset contains
        //       the asset ID and the amount of asset in the contract then remove param
        (storage.assets.get(asset) == 0, this_balance(asset))
    }

    /// Returns data regarding the state of a user i.e. whether they have deposited, approved, their
    /// chosen asset and whether they are a valid user
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor has not been called to initialize
    fn get_user_data(user: Identity) -> User {
        // require(storage.state != State::Void, Error::StateError(StateError::StateNotInitialized));
        require(storage.state != 0, StateError::StateNotInitialized);

        let user_data = storage.users.get(user);

        if storage.state == 2 && user_data.exists {
            User {
                approved: true,
                asset: user_data.asset,
                exists: user_data.exists,
                deposited: true,
            }
        } else {
            user_data
        }
    }

    /// Returns a value indicating the current state of the escrow
    ///
    /// # State
    ///
    /// 0 = The constructor has yet to be called to initialize the contract state
    /// 1 = The constructor has been called to initialize the contract and is pending the deposit &
    ///     approval from both parties
    /// 2 = All parties have deposited and approved and the escrow has completed its purpose
    fn get_state() -> u64 {
        storage.state
    }
}
