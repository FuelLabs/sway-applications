contract;

// Our library dependencies
dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

// Standard library code
use std::{
    assert::require,
    context::{call_frames::msg_asset_id, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::*,
    revert::revert,
    storage::StorageMap,
    token::transfer,
    vec::Vec,
};

// Bring our code into scope
use abi::Escrow;
// use data_structures::{Asset, EscrowInfo, State, User};
use data_structures::*; // workaround to import Eq for State
use errors::{AccessError, ApproveError, CreationError, DepositError, StateError};
use events::{ApproveEvent, CreatedEscrowEvent, DepositEvent, NewUserEscrowEvent, ThresholdReachedEvent, WithdrawEvent};
use utils::sender_identity;

storage {
    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64, EscrowInfo>, 
    
    /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,

    /// Used to check the amount required to deposit for a specific asset in an escrow
    /// Map((ID, AssetId) => required deposit amount)
    required_deposit: StorageMap<(u64, ContractId), u64>, 

    /// State associated with the activity of a user for a specific escrow
    /// Map((ID, user address) => user data)
    users: StorageMap<(u64, Identity), User>, 
}

impl Escrow for Contract {
    /// Creates an internal representation of an escrow by setting the users and assets
    ///
    /// Instead of having a contract per escrow we create an internal representation for the data
    ///
    /// # Arguments
    ///
    /// - `assets` - The assets, with the required deposit amounts, that the campaign accepts
    /// - `users` - The users who are able to interact with the escrow
    ///
    /// # Reverts
    ///
    /// - When the amount of any asset required for deposit is set to 0
    #[storage(read, write)]
    fn create_escrow(assets: Vec<Asset>, users: Vec<Identity>) {
        // Start counting from 1 and keep incrementing for each new escrow
        storage.escrow_count += 1;

        // Set the assets that this escrow accepts
        // This allows the users to select which asset they want to deposit
        let mut asset_index = 0;
        while asset_index < assets.len() {
            // Workaround until `.unwrap()` can be called directly without annotating
            let asset: Option<Asset> = assets.get(asset_index);
            let asset = asset.unwrap();

            // It does not make sense to allow 0 deposits
            // Use 0 as a sentinel to check if the asset is a valid asset in the escrow
            require(0 < asset.amount, CreationError::DepositAmountCannotBeZero);

            storage.required_deposit.insert((storage.escrow_count, asset.id), asset.amount);
            asset_index += 1;
        }

        // Set the users that can interact with the escrow
        let mut user_index = 0;
        while user_index < users.len() {
            // Workaround until `.unwrap()` can be called directly without annotating
            let user: Option<Identity> = users.get(user_index);
            let user = user.unwrap();

            storage.users.insert((storage.escrow_count, user), User {
                approved: false, 
                asset: Option::None::<ContractId>(), 
                exists: true, 
                deposited: false
            });

            user_index += 1;

            // Allow UI to filter via user and then use the ID to get the escrow data
            log(NewUserEscrowEvent {
                identifier: storage.escrow_count, user
            });
        }

        let escrow = EscrowInfo {
            approval_count: 0,
            assets, 
            state: State::Pending,
            threshold: users.len(),
            users, 
        };

        storage.escrows.insert(storage.escrow_count, escrow);

        log(CreatedEscrowEvent { author: sender_identity(), escrow, identifier: storage.escrow_count });
    }

    /// Accepts a deposit from am authorized user for any of the assets specified in the escrow
    /// A successful deposit unlocks the approval functionality for that user
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the user is not an authorized user
    /// - When the escrow is not in the State::Pending state
    /// - When the user deposits and they still have their previous deposit in the escrow
    /// - When the user deposits an asset that has not been specified in the escrow
    /// - When the user sends an incorrect amount of an asset for the specified asset in the escrow
    #[storage(read, write)]
    fn deposit(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // Retrieve user data via the specified escrow ID
        let identity = sender_identity();
        let mut user = storage.users.get((identifier, identity));

        // Make sure caller has been specified as a user for this escrow
        require(user.exists, AccessError::UnauthorizedUser);

        // User can only deposit when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Prevent the user from depositing if they still have a deposit in the escrow
        // This means they can only deposit 1 asset at a time and we do not have to juggle their
        // deposits in order to refund them correctly
        require(!user.deposited, DepositError::AlreadyDeposited);

        // Check how much of the deposited asset is required
        let deposited_asset = msg_asset_id();
        let required_amount = storage.required_deposit.get((identifier, deposited_asset));

        // If the amount is 0 then this asset has not been specified at creation of the escrow
        require(required_amount != 0, DepositError::IncorrectAssetDeposited);

        // Check that the amount deposited is the amount specified at creation of the escrow
        // Exact amount reduces bytecode since we are not juggling calculations to ensure the correct
        // total is deposited
        require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

        // Update user state to indicate that they have deposited one of the assets
        user.asset = Option::Some(deposited_asset);
        user.deposited = true;

        // Overwrite the previous state in storage with the latest changes
        storage.users.insert((identifier, identity), user);

        log(DepositEvent {
            amount: required_amount, asset: deposited_asset, identifier, user: identity
        });
    }

    /// Updates the user state to indicate a user has approved
    /// Once all of the users approve the escrow will lock the approve() & deposit() functions
    /// leaving withdrawal as the last function unlocked
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When the user has not successfully deposited through the deposit() function
    /// - When the user approves again after they have already approved
    #[storage(read, write)]
    fn approve(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only approve when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Make sure caller has been specified as a user for this escrow
        let identity = sender_identity();
        let mut user = storage.users.get((identifier, identity));

        // User must deposit before they can approve
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // To prevent approval count manipulation and (some) event spam ensure they haven't approved
        require(!user.approved, ApproveError::AlreadyApproved);

        user.approved = true;

        storage.users.insert((identifier, identity), user);
        escrow.approval_count = escrow.approval_count + 1;

        log(ApproveEvent { identifier, user: identity });

        if escrow.threshold <= escrow.approval_count {
            // Everyone has approved so lock the escrow down from further use
            // Locks: deposit() & approve()
            escrow.state = State::Completed;

            log(ThresholdReachedEvent { identifier });
        }

        storage.escrows.insert(identifier, escrow);
    }

    /// Returns the deposited asset back to the user and resets their deposit & approval flags to false
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the user has not successfully deposited through the deposit() function
    #[storage(read, write)]
    fn withdraw(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // Retrieve data from whoever is trying to get access this escrow
        let identity = sender_identity();
        let mut user = storage.users.get((identifier, identity));

        // User can only withdraw their deposit if they have a deposit currently in the escrow
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // Safe to unwrap since the asset is set during the deposit
        let deposited_asset: Option<ContractId> = user.asset;
        let deposited_asset = deposited_asset.unwrap();

        // Retrieve the amount the escrow set for the asset
        // No need to validate asset since user can only deposit a specified asset
        let amount = storage.required_deposit.get((identifier, deposited_asset));

        // They're about to withdraw so reset their currently deposited asset back to None
        user.asset = Option::None::<ContractId>();
        user.deposited = false;
        user.approved = false;

        escrow.approval_count = escrow.approval_count - 1;

        storage.users.insert((identifier, identity), user);
        storage.escrows.insert(identifier, escrow);

        // Transfer the asset back to the user
        transfer(amount, deposited_asset, identity);

        log(WithdrawEvent { amount, asset: deposited_asset, identifier, user: identity });
    }
}
