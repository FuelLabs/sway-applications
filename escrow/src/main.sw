contract;

// TODO: 3rd party arbitor address, fees, expire escrow

// Our library dependencies
dep abi;
dep data_structures;
dep errors;
dep events;

// Standard library code
use std::{
    assert::require,
    block::height,
    chain::auth::msg_sender,
    context::{call_frames::msg_asset_id, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    storage::StorageMap,
    token::transfer,
    vec::Vec,
};

// Bring our code into scope
use abi::Escrow;
use data_structures::{Asset, EscrowInfo, State, Buyer};
use errors::{AccessError, ApproveError, CreationError, DepositError, StateError};
use events::{ApproveEvent, CreatedEscrowEvent, DepositEvent, NewUserEscrowEvent, ThresholdReachedEvent, WithdrawEvent};

storage {
    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64, EscrowInfo>, 
    
    /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,
}

impl Escrow for Contract {
    /// Creates an internal representation of an escrow instead of deploying a contract per escrow
    /// 
    /// This sets the available addresses that can make a deposit (instead of forcing a single buyer), 
    /// the assets which the escrow accepts and the seller who will receive the funds
    ///
    /// # Arguments
    ///
    /// - `assets` - The assets, with the required deposit amounts, that the campaign accepts
    /// - `buyers` - A whitelist of addresses one of which much deposit funds into the escrow
    /// - `seller` - The seller of the product to whom the funds will be transferred to
    ///
    /// # Reverts
    ///
    /// - When the amount of any asset required for deposit is set to 0
    #[storage(read, write)]
    fn create_escrow(assets: Vec<Asset>, arbitor: Identity, arbitor_fee_percentage: u64, buyers: Vec<Identity>, deadline: u64, seller: Identity) {
        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(0 < buyers.len(), CreationError::UnspecifiedBuyers);
        require(arbitor_fee_percentage <= 100, CreationError::ArbitorFeeCannotExceed100Percent);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(seller != arbitor, CreationError::SellerCannotBeArbitor);
        
        let mut index = 0;
        while asset_index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let users: Vec<Buyer> = ~Vec::with_capacity(buyers.len());

        index = 0;
        while index < buyers.len() {
            let user = buyers.get(index).unwrap();
            require(user != arbitor, CreationError::BuyerCannotBeArbitor);

            users.push(Buyer {
                address: user,
                approved: false,
                asset: Option::None::<ContractId>(),
                deposited: false
            })

            index += 1;

            // Allow UI to filter via user and then use the ID to get the escrow data
            log(NewUserEscrowEvent {
                identifier: storage.escrow_count, user
            });
        }

        let escrow = EscrowInfo {
            arbitor,
            arbitor_fee_percentage,
            assets,
            buyers: users,
            deadline,
            state: State::Pending,
        };

        storage.escrows.insert(storage.escrow_count, escrow);
        storage.escrow_count += 1;

        log(CreatedEscrowEvent { author: msg_sender().unwrap(), escrow, identifier: storage.escrow_count - 1 });
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

        // User can only deposit when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Keep track of caller
        let identity = msg_sender().unwrap();

        // Validate identity as one of the specified buyers
        let valid_identity = false;

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       once vec has additional utility we won't need to create a new vec and reassign 
        let mut buyers: Vec<Buyer> = ~Vec::with_capacity(escrow.buyers.len());

        let mut index = 0;
        while index < escrow.buyers.len() {
            let buyer = escrow.buyers.get(index).unwrap();

            // Validate that no deposits are currently in the escrow
            // Only 1 address is meant to deposit from the whitelist
            require(!buyer.deposited, DepositError::AlreadyDeposited);

            // TODO: Since buyers are not validated to be only once in the Vec they can deposit multiples
            if identity == buyer.address {
                // Check how much of the deposited asset is required
                let deposited_asset = msg_asset_id();

                // TODO: loop over assets
                let required_amount = storage.required_deposit.get((identifier, deposited_asset));

                // If the amount is 0 then this asset has not been specified at creation of the escrow
                require(required_amount != 0, DepositError::IncorrectAssetDeposited);

                // Check that the amount deposited is the amount specified at creation of the escrow
                // Exact amount reduces bytecode since we are not juggling calculations to ensure the correct
                // total is deposited
                require(required_amount == msg_amount(), DepositError::IncorrectAssetAmount);

                valid_identity = true;

                // Update user state to indicate that they have deposited one of the assets
                buyer.asset = Option::Some(deposited_asset);
                buyer.deposited = true;
            }

            index += 1;
            buyers.push(buyer);
        }

        require(valid_identity, AccessError::UnauthorizedUser);

        escrow.buyers = buyers;

        // Update escrow state
        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            amount: msg_amount(), asset: msg_asset_id(), identifier, user: identity
        });
    }

    /// Updates the user state to indicate a user has approved
    /// Once all of the buyers approve the escrow will lock the approve() & deposit() functions
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
        let identity = msg_sender().unwrap();
        let mut user = storage.buyers.get((identifier, identity));

        // User must deposit before they can approve
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // To prevent approval count manipulation and (some) event spam ensure they haven't approved
        require(!user.approved, ApproveError::AlreadyApproved);

        user.approved = true;

        storage.buyers.insert((identifier, identity), user);
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
        let identity = msg_sender().unwrap();
        let mut user = storage.buyers.get((identifier, identity));

        // User can only withdraw their deposit if they have a deposit currently in the escrow
        // No need to check ".exists" since the deposit flag is only set after that check is made
        require(user.deposited, DepositError::DepositRequired);

        // Safe to unwrap since the asset is set during the deposit
        let deposited_asset = user.asset.unwrap();

        // Retrieve the amount the escrow set for the asset
        // No need to validate asset since user can only deposit a specified asset
        let amount = storage.required_deposit.get((identifier, deposited_asset));

        // They're about to withdraw so reset their currently deposited asset back to None
        user.asset = Option::None::<ContractId>();
        user.deposited = false;
        user.approved = false;

        escrow.approval_count -= 1;

        storage.buyers.insert((identifier, identity), user);
        storage.escrows.insert(identifier, escrow);

        // Transfer the asset back to the user
        transfer(amount, deposited_asset, identity);

        log(WithdrawEvent { amount, asset: deposited_asset, identifier, user: identity });
    }
}
