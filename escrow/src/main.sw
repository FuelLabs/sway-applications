contract;

// TODO: arbiter fees
//       make the seller deposit as well so that the user can get refunded some portion of the dispute?
//
//       over collateralize (additional fee / penalty) in case process is not smooth
//
//       In the distant future it *should* be cheaper to interact with each escrow therefore this would
//       need to be split into a factory script and a single escrow contract each time

/*
1. USER deposit() -> USER transfer_to_seller()
2. USER deposit() -> escrow expires -> SELLER take_payment()
3. USER deposit() -> USER dispute() -> locks escrow -> Arbiter resolve_dispute(user)

Only case 3 results in Arbiter getting a fee if fee != 0
*/

// Our library dependencies
dep contract_abi;
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
use contract_abi::Escrow;
use data_structures::{Asset, Buyer, EscrowInfo, Seller, State};
use errors::{CreationError, DepositError, StateError, UserError};
use events::{
    CreatedEscrowEvent,
    DepositEvent,
    DisputeEvent,
    PaymentTakenEvent,
    ResolvedDisputeEvent,
    TransferredToSellerEvent,
};

storage {
    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64,
    EscrowInfo>, /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,
}

impl Escrow for Contract {
    #[storage(read, write)]fn create_escrow(assets: Vec<Asset>, arbiter: Identity, arbiter_fee_percentage: u64, buyer: Identity, deadline: u64, seller: Identity) {
        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(arbiter_fee_percentage <= 100, CreationError::ArbiterFeeCannotExceed100Percent);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(arbiter != buyer, CreationError::ArbiterCannotBeBuyer);
        require(arbiter != seller, CreationError::ArbiterCannotBeSeller);

        let mut index = 0;
        while index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let escrow = EscrowInfo {
            arbiter, arbiter_fee_percentage, assets, buyer: Buyer {
                address: buyer,
                asset: Option::None::<ContractId>(),
                deposited_amount: 0,
                disputed: false,
            },
            deadline, disputed: false,
            seller: Seller {
                address: seller,
                disputed: false,
            },
            state: State::Pending,
        };

        storage.escrows.insert(storage.escrow_count, escrow);
        storage.escrow_count += 1;

        // TODO: should the author be in the EscrowInfo?
        log(CreatedEscrowEvent {
            author: msg_sender().unwrap(), escrow, identifier: storage.escrow_count - 1
        });
    }

    #[storage(read, write)]fn deposit(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only deposit when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Only the buyer should be able to deposit
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        // Prevent multiple deposits at once
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

                break;
            }

            index += 1;
        }

        require(escrow.buyer.asset.is_some(), DepositError::IncorrectAssetDeposited);

        // Update escrow state
        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            amount: msg_amount(), asset: msg_asset_id(), identifier, user: msg_sender().unwrap()
        });
    }

    #[storage(read, write)]fn transfer_to_seller(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only transfer when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Only the buyer can dictate when their deposit is ready to be sent
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        // If a party has initiated a dispute then the user cannot tranfer the funds
        require(!escrow.disputed, UserError::CannotTransferPaymentDuringDispute);

        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        // Conditions have been cleared, lock the escrow down and transfer funds to seller
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        // Note that there is no conditional check upon the deadline since it is unnecessary
        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(TransferredToSellerEvent {
            amount: escrow.buyer.deposited_amount, asset: escrow.buyer.asset.unwrap(), buyer: msg_sender().unwrap(), identifier, seller: escrow.seller.address
        });
    }

    #[storage(read, write)]fn take_payment(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only take payment when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // User cannot take payment unless the escrow has expired
        require(escrow.deadline < height(), UserError::CannotTakePaymentBeforeDeadline);

        // If a party has initiated a dispute then the user cannot suddenly take the funds
        require(!escrow.disputed, UserError::CannotTakePaymentDuringDispute);

        // Only the seller can transfer the payment to themselves
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);

        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        // Conditions have been cleared, lock the escrow down and transfer funds to seller
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(PaymentTakenEvent {
            amount: escrow.buyer.deposited_amount, asset: escrow.buyer.asset.unwrap(), buyer: escrow.buyer.address, identifier, seller: msg_sender().unwrap()
        });
    }

    #[storage(read, write)]fn dispute(identifier: u64) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only dispute when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Note that a dispute can happen before or after deadline therefore no check for deadline

        // Cannot dispute when already in the disputed state
        require(!escrow.disputed, UserError::AlreadyDisputed);

        // Only the buyer can initiate a dispute
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        require(escrow.buyer.asset.is_some(), UserError::CannotDisputeBeforeDesposit);

        escrow.disputed = true;
        escrow.buyer.disputed = true;
        storage.escrows.insert(identifier, escrow);

        log(DisputeEvent {
            identifier, user: msg_sender().unwrap()
        });
    }

    #[storage(read, write)]fn resolve_dispute(identifier: u64, user: Identity) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only dispute when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Note that a resolution cannot occur before or after deadline therefore no check for deadline

        // Cannot resolve a dispute if not in the disputed state
        require(escrow.disputed, UserError::NotDisputed);

        // Only the arbiter can resolve a dispute
        require(msg_sender().unwrap() == escrow.arbiter, UserError::UnauthorizedUser);

        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::InvalidRecipient);

        require(escrow.buyer.asset.is_some(), UserError::CannotResolveBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), user);
        // Add fee to arbiter
        // transfer(some fee ,some asset, escrow.arbiter);

        log(ResolvedDisputeEvent {
            identifier, user
        });
    }
}
