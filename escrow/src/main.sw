contract;

// TODO: enum Eq not implemented for self therefore will not compile
//       change arrays to vec and update threshold in constructor

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
    option::*,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output},
};

// Bring our code into scope
use abi::Escrow;
use errors::{AccessError, ApproveError, DepositError, InitError, StateError};
use events::{ApproveEvent, DepositEvent, ThresholdReachedEvent, WithdrawEvent};
use data_structures::{Asset, State, User};

storage {
    /// The asset and amount of asset the deposit() function will accept
    assets: StorageMap<ContractId, u64>, 
    
    /// Current number of successful calls to approve(), used to lock the contract at completion
    approval_count: u64,

    /// Mechanism used to manage the control flow of the contract
    state: State,

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
        require(storage.state == State::Void, InitError::CannotReinitialize);

        // Set the assets that this contract accepts
        let mut asset_index = 0;
        while asset_index < 2 {
            require(0 < assets[asset_index].amount, InitError::AssetAmountCannotBeZero);
            require(~ContractId::from(NATIVE_ASSET_ID) != assets[asset_index].id, InitError::AssetIdCannotBeZero);

            storage.assets.insert(assets[asset_index].id, assets[asset_index].amount);
        }

        // Set the users that can interact with the escrow
        let mut user_index = 0;
        while user_index < 2 {
            storage.users.insert(users[user_index], User {
                approved: false, 
                asset: Option::None::<ContractId>(),
                exists: true, 
                deposited: false
            });
        }

        // Set the threshold to be the number of users
        // All users must approve
        storage.threshold = 2;

        // Flip the state to prevent future reinitialization
        storage.state = State::Pending;
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
        require(storage.state == State::Pending, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data.exists, AccessError::UnauthorizedUser);
        require(!user_data.deposited, DepositError::AlreadyDeposited);

        let deposited_asset = msg_asset_id();
        let required_amount = storage.assets.get(deposited_asset);

        require(required_amount != 0, DepositError::IncorrectAssetDeposited);
        require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

        user_data.asset = Option::Some(deposited_asset);
        user_data.deposited = true;

        storage.users.insert(sender.unwrap(), user_data);

        log(DepositEvent {amount: required_amount, asset: deposited_asset, user: sender.unwrap()});
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
        require(storage.state == State::Pending, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data.deposited, DepositError::DepositRequired);
        require(!user_data.approved, ApproveError::AlreadyApproved);

        user_data.approved = true;

        storage.users.insert(sender.unwrap(), user_data);
        storage.approval_count = storage.approval_count + 1;

        log(ApproveEvent {count: storage.approval_count, user: sender.unwrap()});

        if storage.threshold <= storage.approval_count {
            storage.state = State::Completed;
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
        require(storage.state != State::Void, StateError::StateNotPending);

        let sender: Result<Identity, AuthError> = msg_sender();
        let mut user_data = storage.users.get(sender.unwrap());

        require(user_data.deposited, DepositError::DepositRequired);

        let deposited_asset = user_data.asset.unwrap();
        let amount = storage.assets.get(deposited_asset);

        user_data.asset = Option::None::<ContractId>();

        if storage.state == State::Pending {
            user_data.deposited = false;
            user_data.approved = false;
            storage.approval_count = storage.approval_count - 1;
        }

        storage.users.insert(sender.unwrap(), user_data);

        match sender.unwrap() {
            Identity::Address(address) => {
                transfer_to_output(amount, deposited_asset, address);
            },
            Identity::ContractId(address) => {
                force_transfer_to_contract(amount, deposited_asset, address);
            }
        }

        log(WithdrawEvent {
            amount,
            approval_count: storage.approval_count,
            asset: deposited_asset, 
            user: sender.unwrap(),
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
        require(storage.state != State::Void, StateError::StateNotInitialized);
        storage.users.get(user)
    }

    /// Returns a value indicating the current state of the escrow
    ///
    /// # State
    ///
    /// Void      = The constructor has yet to be called to initialize the contract state
    /// Pending   = The constructor has been called to initialize the contract and is pending the 
    ///             deposit & approval from both parties
    /// Completed = All parties have deposited and approved and the escrow has completed its purpose
    fn get_state() -> State {
        storage.state
    }
}
