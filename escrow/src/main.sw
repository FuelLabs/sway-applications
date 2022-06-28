contract;

// TODO: 3rd party arbitor address, fees, expire escrow
//       make the seller deposit as well so that the user can get refunded some portion of the dispute
//       over collateralize (additional fee / penalty) in case process is not smooth

/*
The "!" indicates escrow is locked / completed
1. USER deposit() -> USER transfer_to_seller() !
2. USER deposit() -> escrow expires -> SELLER take_payment() !
3. USER deposit() -> USER disputes() -> escrow locks -> escrow expires -> ARBITOR resolve_dispute(user) !
4. USER deposit() -> USER dispute() -> locks escrow -> USER transfer_to_seller() !

Only case 3 results in ARBITOR getting a fee if fee != 0
*/

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
use data_structures::{Asset, Buyer, EscrowInfo, Seller, State};
use errors::{AccessError, ApproveError, CreationError, DepositError, StateError};
use events::{ApproveEvent, CreatedEscrowEvent, DepositEvent, NewUserEscrowEvent, ThresholdReachedEvent, WithdrawEvent, PaymentTakenEvent, TransferredToSellerEvent};

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
    fn create_escrow(assets: Vec<Asset>, arbitor: Identity, arbitor_fee_percentage: u64, buyer: Identity, deadline: u64, seller: Identity) {
        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(arbitor_fee_percentage <= 100, CreationError::ArbitorFeeCannotExceed100Percent);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(arbitor != seller, CreationError::ArbitorCannotBeSeller);
        require(arbitor != buyer, CreationError::ArbitorCannotBeBuyer);
        
        let mut index = 0;
        while asset_index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let escrow = EscrowInfo {
            arbitor,
            arbitor_fee_percentage,
            assets,
            buyer: Buyer {
                address: buyer,
                asset: Option::None::<ContractId>(),
                deposited_amount: 0,
                disputed: false,
            },
            deadline,
            disputed: false,
            seller: Seller {
                address: seller,
                disputed: false,
            }
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
    /// - When the escrow is not in the State::Pending state
    /// - When the user is not an authorized user (buyer)
    /// - When the user deposits and they still have their previous deposit in the escrow
    /// - When the user sends an incorrect amount of an asset for the specified asset in the escrow
    /// - When the user deposits an asset that has not been specified in the escrow
    #[storage(read, write)]
    fn deposit(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier); 

        // User can only deposit when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_none(), DepositError::AlreadyDeposited);

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       `.contains()` would clean up the loop
        let mut index = 0;
        while index < escrow.assets.len() {
            let asset = escrow.assets.get(index).unwrap();

            if asset.id == msg_asset_id() {
                // Check that the amount deposited is the amount specified at creation of the escrow
                // Exact amount reduces bytecode since we are not juggling calculations to ensure the
                // correct total is deposited
                require(asset.amount == msg_amount(), DepositError::IncorrectAssetAmount);

                // Update user state to indicate that they have deposited one of the assets
                escrow.buyer.asset = Option::Some(asset.id);
                escrow.buyer.deposited_amount = asset.amount;

                // TODO: https://github.com/FuelLabs/sway/pull/2112
                // break
            }

            index += 1;
        }

        require(escrow.buyer.asset.is_some(), DepositError::IncorrectAssetDeposited);

        // Update escrow state
        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            amount: msg_amount(), 
            asset: msg_asset_id(), 
            identifier, 
            user: msg_sender().unwrap(),
        });
    }

    /// After a buyer deposits they can transfer the deposit to the seller
    ///
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When the user is not an authorized user (buyer)
    #[storage(read, write)]
    fn transfer_to_seller(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier); 

        // User can only transfer when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Only the buyer can dictate when their deposit is ready to be sent
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        // Conditions have been cleared, lock the escrow down and transfer funds to seller
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(TransferredToSellerEvent { 
            amount: escrow.buyer.deposited_amount, 
            asset: escrow.buyer.asset.unwrap(), 
            buyer: msg_sender().unwrap(), 
            identifier, 
            seller: escrow.seller.address,
        });
    }

    /// If a user has deposited but not transferred in time & they have not disputed then the seller
    /// can take payment themselves
    //
    /// # Arguments
    ///
    /// - `identifier` - Unique escrow identifier in the storage.escrow_count range
    ///
    /// # Reverts
    ///
    /// - When the escrow is not in the State::Pending state
    /// - When a user attempts to take payment before the deadline
    /// - When a user attempts to take payment during a dispute
    /// - When the user is not an authorized user (seller)
    #[storage(read, write)]
    fn take_payment(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier); 

        // User can only take payment when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), UserError::CannotTakePaymentBeforeDeadline)
        require(!escrow.disputed, UserError::CannotTakePaymentDuringDispute);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(PaymentTakenEvent {
            amount: escrow.buyer.deposited_amount, 
            asset: escrow.buyer.asset.unwrap(), 
            buyer: escrow.buyer.address, 
            identifier, 
            seller: msg_sender().unwrap(),
        })
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

    // oh fuck, I can't believe you've done this
    fn dispute(identifier: u64) {}

    // if a dispute has been filed and the escrow expires then the arbitor can choose who the funds are sent to
    fn resolve_dispute(identifier: u64, user: Identity) {}

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
