contract;

// TODO:
//      * enum Eq not implemented for self therefore using u64 for now
//      * change arrays to vec and update threshold in creation
//      * currently the UserEscrows does not handle the completed => active logic and you can only
//        have 1 active escrow at a time (for display purposes)

// Our library dependencies
dep abi;
dep data_structures;
dep errors;
dep events;

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
    token::{force_transfer_to_contract, transfer_to_output}
};

// Bring our code into scope
use abi::Escrow;
use data_structures::{Asset, EscrowData, State, User, UserEscrows};
use errors::{AccessError, ApproveError, CreationError, DepositError, InitError, StateError};
use events::{ApproveEvent, CreatedEscrowEvent, DepositEvent, ThresholdReachedEvent, WithdrawEvent};

// Note: mappings inside structs are not a thing therefore this mess exists
storage {
    /// State associated with the activity of a user for a specified EscrowData ID
    /// Map((EscrowData ID, Identity address) => user data for EscrowData)
    authorized_users: StorageMap<(u64, Identity), User>, 
    
    /// Required deposit amount of an asset for a specified EscrowData ID
    /// Map((EscrowData ID, EscrowData AssetId) => required deposit amount)
    deposit_amount: StorageMap<(u64, ContractId), u64>, 
    
    /// Metadata related to a newly created escrow via create_escrow()
    /// Map(ID => Data)
    escrows: StorageMap<u64, EscrowData>, 
    
    /// Number of created escrows
    /// Used for O(1) look up in mappings
    escrow_count: u64,

    /// Enum used to lock the constructor() to prevent re-initialization
    initialized: bool,

    /// Owner of the contract - passed into the constructor()
    /// Only the owner can create new escrows via create_escrow()
    owner: Identity,

    /// Completed and currently active escrows for a specified user
    user_escrows: StorageMap<Identity, UserEscrows>, 
}

impl Escrow for Contract {
    /// Sets the owner of the contract allowing them to create new escrows
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The constructor is called more than once
    fn constructor(owner: Identity) {
        require(!storage.initialized, InitError::CannotReinitialize);
        storage.owner = owner;
        storage.initialized = true;
    }

    /// Creates an internal representation of an escrow by setting the users and assets
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The calling identity is the 0x000.. asset
    /// - The calling identity is not the owner of the contract
    /// - The amount of any asset being set is equal to 0
    /// - Any asset is the NATIVE_ASSET_ID
    fn create_escrow(users: [Identity; 2], assets: [Asset; 2]) {
        let identity = unwrap_identity(msg_sender());
        // require(Identity::ContractId(~ContractId::from(NATIVE_ASSET_ID)) != identity, AccessError::UnauthorizedUser);
        // require(storage.owner == identity, AccessError::UnauthorizedUser);

        storage.escrow_count = storage.escrow_count + 1;

        // Set the assets that this escrow accepts
        let mut asset_index = 0;
        while asset_index < 2 {
            require(0 < assets[asset_index].amount, CreationError::AssetAmountCannotBeZero);
            require(~ContractId::from(NATIVE_ASSET_ID) != assets[asset_index].id, CreationError::AssetIdCannotBeZero);

            storage.deposit_amount.insert((storage.escrow_count, assets[asset_index].id), assets[asset_index].amount);
        }

        // Set the users that can interact with the escrow
        let mut user_index = 0;
        while user_index < 2 {
            storage.authorized_users.insert((storage.escrow_count, users[user_index]), User {
                approved: false, 
                asset: Option::None::<ContractId>(), 
                exists: true, 
                deposited: false
            });

            // Add a newly active escrow to the active list for the user
            let mut user_escrows = storage.user_escrows.get(users[user_index]);

            // TODO: second reminder, change to vec once it is out
            user_escrows.active = [storage.escrow_count];

            storage.user_escrows.insert(users[user_index], user_escrows);
        }

        let escrow = EscrowData {
            approval_count: 0,
            assets, state: State::Pending,
            threshold: 2, // Set the threshold to be the number of users. All users must approve
            users, 
        };

        storage.escrows.insert(storage.escrow_count, escrow);

        log(CreatedEscrowEvent {
            escrow, 
            identifier: storage.escrow_count
        });
    }

    /// Accepts a deposit from an authorized user for any of the specified assets
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The specified escrow identifier is not in the valid range of existing escrows
    /// - The escrow is not in the State::Pending state
    /// - The user is not an authorized user
    /// - The user deposits when they still have their previous deposit in the escrow
    /// - The user deposits an asset that has not been specified in the constructor
    /// - The user sends an incorrect amount of an asset for the specified asset in the escrow
    fn deposit(identifier: u64) {
        // Escrow must exist in order to retrieve valid data
        validate_id(identifier);

        let mut escrow = storage.escrows.get(identifier);

        // User can only deposit when the escrow state has not reached completion
        // require(escrow.state == State::Pending, StateError::StateNotPending);

        // Retrieve data from whoever is trying to get access for the specified identifier
        let identity = unwrap_identity(msg_sender());
        let mut user = storage.authorized_users.get((identifier, identity));

        // Make sure they are a specified user for this specific identifier
        require(user.exists, AccessError::UnauthorizedUser);

        // Prevent the user from depositing if they still have a deposit in the escrow
        require(!user.deposited, DepositError::AlreadyDeposited);

        // Check how much of the deposited asset is required
        let deposited_asset = msg_asset_id();
        let required_amount = storage.deposit_amount.get((identifier, deposited_asset));

        // If the amount is 0 then the asset is an invalid asset
        // This could be removed since the subsequent condition checks for the amount however the
        // distinction in the error msg is helpful
        require(required_amount != 0, DepositError::IncorrectAssetDeposited);

        // Check that the amount deposited is the amount specified at creation of the escrow
        require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

        // Update user state to indicate that they have deposited one of the assets
        user.asset = Option::Some(deposited_asset);
        user.deposited = true;

        // Overwrite the previous state in storage with the latest changes
        storage.authorized_users.insert((identifier, identity), user);

        log(DepositEvent {
            amount: required_amount, 
            asset: deposited_asset, 
            identifier, 
            user: identity
        });
    }

    /// Updates the user state to indicate a user has approved
    /// Once all of the users approve the escrow will lock the approve() & deposit() functions
    /// leaving withdrawal as the last function unlocked
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The specified escrow identifier is not in the valid range of existing escrows
    /// - The escrow is not in the State::Pending state
    /// - The user has not successfully deposited through the deposit() function
    /// - The user approves again after they have already approved
    fn approve(identifier: u64) {
        // Escrow must exist in order to retrieve valid data
        validate_id(identifier);

        let mut escrow = storage.escrows.get(identifier);

        // User can only approve when the escrow state has not reached completion
        // require(escrow.state == State::Pending, StateError::StateNotPending);

        // Retrieve data from whoever is trying to get access for the specified identifier
        let identity = unwrap_identity(msg_sender());
        let mut user = storage.authorized_users.get((identifier, identity));

        // User must deposit before they can approve
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // To prevent approval count manipulation and (some) event spam ensure they haven't approved
        require(!user.approved, ApproveError::AlreadyApproved);

        user.approved = true;

        storage.authorized_users.insert((identifier, identity), user);
        escrow.approval_count = escrow.approval_count + 1;

        log(ApproveEvent {
            approval_count: escrow.approval_count, 
            identifier, 
            user: identity
        });

        if escrow.threshold <= escrow.approval_count {
            // Everyone has approved so lock the escrow down from further use
            // Locks: deposit() & approve()
            escrow.state = State::Completed;

            log(ThresholdReachedEvent {
                identifier
            });
        }

        storage.escrows.insert(identifier, escrow);
    }

    /// Returns the deposited asset back to the user and resets their deposit & approval flags to false
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The specified escrow identifier is not in the valid range of existing escrows
    /// - The user has not successfully deposited through the deposit() function
    fn withdraw(identifier: u64) {
        // Escrow must exist in order to retrieve valid data
        validate_id(identifier);

        let mut escrow = storage.escrows.get(identifier);

        // Retrieve data from whoever is trying to get access for the specified identifier
        let identity = unwrap_identity(msg_sender());
        let mut user = storage.authorized_users.get((identifier, identity));

        // User can only withdraw their deposit if they have a deposit currently in the escrow
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // Safe to unwrap since the asset is set during the deposit
        // let deposited_asset = user.asset.unwrap();

        let deposited_asset = match user.asset {
            Option::Some(asset) => asset, 
            _ => revert(0), // temporary workaround for unimplemented feature / bug
        };

        // Retrieve the amount the specified escrow set for the asset
        // No need to validate asset since user can only deposit a specified asset
        let amount = storage.deposit_amount.get((identifier, deposited_asset));

        // They're about to withdraw so reset their currently deposited asset back to None
        user.asset = Option::None::<ContractId>();
        user.deposited = false;
        user.approved = false;
        storage.authorized_users.insert((identifier, identity), user);

        escrow.approval_count = escrow.approval_count - 1;
        storage.escrows.insert(identifier, escrow);

        // Transfer the asset back to the user
        match identity {
            Identity::Address(address) => transfer_to_output(amount, deposited_asset, address), 
            Identity::ContractId(address) => force_transfer_to_contract(amount, deposited_asset, address), 
        }

        log(WithdrawEvent {
            amount, 
            approval_count: escrow.approval_count, 
            asset: deposited_asset, 
            identifier, 
            user: identity, 
        });
    }

    /// Returns data regarding the state of a user i.e. whether they have deposited, approved, their
    /// chosen asset and whether they are a valid user
    ///
    /// # Panics
    ///
    /// The function will panic when
    /// - The specified escrow identifier is not in the valid range of existing escrows
    fn user_data(identifier: u64, user: Identity) -> User {
        validate_id(identifier);
        storage.authorized_users.get((identifier, user))
    }

    /// Returns data regarding the identifiers for active and completed escrows
    /// Works for all users - including ones not in the contract
    fn user_escrows(user: Identity) -> UserEscrows {
        storage.user_escrows.get(user)
    }

    /// Returns the meta data regarding a created escrow
    /// # Panics
    ///
    /// The function will panic when
    /// - The specified escrow identifier is not in the valid range of existing escrows
    fn escrow_data(identifier: u64) -> EscrowData {
        validate_id(identifier);
        storage.escrows.get(identifier)
    }
}

// Keep the code dry
fn validate_id(identifier: u64) {
    require(identifier != 0 && identifier <= storage.escrow_count, AccessError::InvalidIdentifier);
}

fn unwrap_identity(sender: Result<Identity, AuthError>) -> Identity {
    sender.unwrap()
}
