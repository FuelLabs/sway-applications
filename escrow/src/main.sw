contract;

// TODO:
//
//      deadline for withdrawal?
//
//      In the distant future it *should* be cheaper to interact with each escrow therefore this would
//      need to be split into a factory script and a single escrow contract each time

// Our library dependencies
dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::{Arbiter, ArbiterProposal, Asset, Buyer, EscrowInfo, Seller, State};
use errors::{CreationError, DepositError, StateError, UserError};
use events::{
    ChangedArbiterEvent,
    CreatedEscrowEvent,
    DepositEvent,
    DisputeEvent,
    PaymentTakenEvent,
    ProposedArbiterEvent,
    ResolvedDisputeEvent,
    ReturnedDepositEvent,
    TransferredToSellerEvent,
};
use interface::Escrow;
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
use utils::change_arbiter;

storage {
    /// If either party want to change the arbiter that data must be stored somewhere as "temporary"
    /// data. This does not belong in EscrowInfo hence a separate variable
    arbiter_proposal: StorageMap<u64, ArbiterProposal>,

    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64,
    EscrowInfo>, /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,
}

impl Escrow for Contract {
    #[storage(read, write)]fn change_arbiter(arbiter: Arbiter, identifier: u64) {
        // The assertions ensure that
        //  - Arbiter fee is greater than 0
        //  - The escrow has not been completed
        //  - Caller is either buyer or seller of escrow
        //  - Caller is not setting the buyer or seller as the new arbiter

        require(0 < arbiter.fee_amount, CreationError::ArbiterFeeCannotBeZero);

        let escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);

        let user = msg_sender().unwrap();

        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::UnauthorizedUser);
        require(arbiter.address != escrow.buyer.address, CreationError::ArbiterCannotBeBuyer);
        require(arbiter.address != escrow.seller.address, CreationError::ArbiterCannotBeSeller);

        let proposal = storage.arbiter_proposal.get(identifier);

        let user_proposal = if user == escrow.buyer.address { proposal.seller } else { proposal.buyer };
        let (escrow, proposal) = change_arbiter(arbiter, escrow, identifier, proposal, user, user_proposal);
        storage.arbiter_proposal.insert(identifier, proposal);
        storage.escrows.insert(identifier, escrow);
    }

    #[storage(read, write)]fn create_escrow(assets: Vec<Asset>, arbiter: Arbiter, buyer: Identity, deadline: u64) {
        // The assertions ensure that
        //  - At least 1 asset is specified for a deposit
        //  - Deadline is set in the future
        //  - Arbiter fee is greater than 0
        //  - Caller has deposited arbitration fee
        //  - Caller (assumed seller) is not setting the buyer or seller as the new arbiter
        //  - Any specified asset accepts a deposit that is greater than 0

        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(0 < arbiter.fee_amount, CreationError::ArbiterFeeCannotBeZero);
        require(arbiter.fee_amount == msg_amount(), CreationError::ArbiterFeeDoesNotMatchAmountSent);
        require(arbiter.address != buyer, CreationError::ArbiterCannotBeBuyer);
        require(arbiter.address != msg_sender().unwrap(), CreationError::ArbiterCannotBeSeller);

        let mut index = 0;
        while index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let escrow = EscrowInfo {
            arbiter: arbiter, 
            assets, 
            buyer: Buyer {
                address: buyer,
                asset: Option::None::<ContractId>(),
                deposited_amount: 0,
            },
            deadline, 
            disputed: false,
            seller: Seller {
                address: msg_sender().unwrap(),
            },
            state: State::Pending,
        };

        storage.escrows.insert(storage.escrow_count, escrow);
        storage.escrow_count += 1;

        log(CreatedEscrowEvent {
            escrow, identifier: storage.escrow_count - 1
        });
    }

    #[storage(read, write)]fn deposit(identifier: u64) {
        // The assertions ensure that
        //  - Escrow is active (within deadline and not completed)
        //  - Only the buyer can deposit
        //  - They can only deposit once and it must be the correct asset in its stated amount

        let mut escrow = storage.escrows.get(identifier);

        require(height() < escrow.deadline, DepositError::EscrowExpired);
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_none(), DepositError::AlreadyDeposited);

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       `.contains() -> bool / .position() -> u64` would clean up the loop
        let mut index = 0;
        while index < escrow.assets.len() {
            let asset = escrow.assets.get(index).unwrap();

            if asset.id == msg_asset_id() {
                require(asset.amount == msg_amount(), DepositError::IncorrectAssetAmount);
                escrow.buyer.asset = Option::Some(msg_asset_id());
                escrow.buyer.deposited_amount = msg_amount();
                break;
            }

            index += 1;
        }

        // User must deposit one of the specified assets in the correct amount
        require(escrow.buyer.asset.is_some(), DepositError::IncorrectAssetDeposited);

        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            asset: escrow.buyer.asset.unwrap(), identifier
        });
    }

    #[storage(read, write)]fn dispute(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The escrow is not already in dispute
        //  - Only the buyer can dispute
        //  - The buyer has made a deposit
        // Note that you can dispute even after the deadline

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(!escrow.disputed, UserError::AlreadyDisputed);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotDisputeBeforeDesposit);

        // Lock the escrow
        escrow.disputed = true;
        storage.escrows.insert(identifier, escrow);

        log(DisputeEvent {
            identifier
        });
    }

    #[storage(read, write)]fn resolve_dispute(identifier: u64, payment_amount: u64, user: Identity) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The escrow is in dispute
        //  - Only the arbiter can resolve
        //  - The deposit will be sent to either the buyer or seller
        //  - The buyer has made a deposit
        //  - The payment taken by the arbiter is not greater than the seller has deposited

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.disputed, UserError::NotDisputed);
        require(msg_sender().unwrap() == escrow.arbiter.address, UserError::UnauthorizedUser);
        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::InvalidRecipient);
        require(escrow.buyer.asset.is_some(), UserError::CannotResolveBeforeDesposit);
        require(payment_amount <= escrow.arbiter.fee_amount, UserError::ArbiterPaymentCannotBeGreaterThanDepositFromSeller);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), user);
        transfer(payment_amount, escrow.arbiter.asset, escrow.arbiter.address);

        if payment_amount != escrow.arbiter.fee_amount {
            transfer(escrow.arbiter.fee_amount - payment_amount, escrow.arbiter.asset, escrow.seller.address);
        }

        log(ResolvedDisputeEvent {
            identifier, user
        });
    }

    #[storage(read, write)]fn return_deposit(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - Only the seller can "return" the deposit from the escrow
        //  - The buyer has made a deposit
        // Note that you can dispute even after the deadline

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.buyer.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        log(ReturnedDepositEvent {
            identifier
        });
    }

    #[storage(read, write)]fn take_payment(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The call is made after the deadline of the escrow
        //  - The buyer has not initiated a dispute
        //  - Only the seller can call to take the payment
        //  - The buyer has deposited

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), UserError::CannotTakePaymentBeforeDeadline);
        require(!escrow.disputed, UserError::CannotTakePaymentDuringDispute);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        log(PaymentTakenEvent {
            identifier
        });
    }

    #[storage(read, write)]fn transfer_to_seller(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The buyer has made a deposit
        //  - Only the buyer can transfer their deposit

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        log(TransferredToSellerEvent {
            identifier
        });
    }
}
